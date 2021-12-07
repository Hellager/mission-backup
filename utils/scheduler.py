#!/usr/bin/env python
# -*- coding: utf-8 -*-
"""
scheduler.py

This module manages those schedule jobs created by the program.
"""
from apscheduler.schedulers.background import BackgroundScheduler
from apscheduler.executors.pool import ThreadPoolExecutor
from apscheduler.triggers.interval import IntervalTrigger
from apscheduler.events import EVENT_JOB_ERROR
from utils.collector import *
from utils.monitor import *
from utils.idworker import *
from utils.webdav import *
from utils.bsamba import *
import logging
import base64
import json


class Scheduler(object):
    def __init__(self):
        # init job list array
        self.job_dict = dict()

        # init compressor
        self.compressor = Compressor()

        # init monitor
        self.monitor = DirMonitor()

        # init id generator
        self.id_generator = IdWorker()

        # init webdav handler
        self.webdav = None

        # init samba handler
        self.samba = None

        # init background scheduler
        executors = {
            'default': ThreadPoolExecutor(10)
        }
        self.scheduler = BackgroundScheduler(executors=executors)

        # init logging
        if not os.path.exists(os.getcwd() + '\\syslog'):
            os.mkdir(os.getcwd() + '\\syslog')
        check_file = open('syslog\\log.txt', 'a')
        check_file.close()

        self.log = logging.getLogger('apscheduler')
        self.log.setLevel(logging.ERROR)
        file_log = logging.FileHandler('syslog\\log.txt', 'a')
        log_format = logging.Formatter('%(asctime)s - %(filename)s[line:%(lineno)d] - %(levelname)s: %(message)s')
        file_log.setFormatter(log_format)
        self.log.addHandler(file_log)
        self.scheduler.add_listener(self.schedule_listener, EVENT_JOB_ERROR)

        # init utils' log
        self.compressor.add_log_handler(self.log)
        self.monitor.add_log_handler(self.log)

        # init utils' callback
        self.monitor.add_on_any_error_callback_handler(self.on_any_error_callback)
        self.monitor.add_on_sync_job_status_callback(self.on_sync_job_status_callback)

        # init widget callback
        self.on_sync_widget_data_callback = None
        self.on_sync_widget_remote_callback = None
        self.on_sync_widget_error_callback = None

    def start(self):
        """
        Start scheduler
        :return: None
        """
        self.scheduler.start()
        self.monitor.start()

    def stop(self):
        """
        Stop scheduler
        :return: None
        """
        self.scheduler.shutdown()
        self.monitor.stop()

    def schedule_listener(self, event):
        """
        Log scheduler error
        :param event: Event passed from apscheduler
        :return:
        """
        if event.code == EVENT_JOB_ERROR:
            self.log.error(event.code)

    def add_webdav_handler(self, config):
        """
        Add webdav client from configuration
        :param config: Webdav configuration
        :return: True if success else False
        """
        if self.webdav is not None:
            self.webdav = None
        scheduler_webdav = WebDavHandler(config)
        scheduler_webdav.add_log_handler(self.log)
        res = scheduler_webdav.set_root_path(config['webdav_root_path'])
        if res is False:
            self.on_any_error_callback(0x02, 'Remote', remote=config)
        else:
            self.webdav = scheduler_webdav
            self.compressor.add_webdav_handler(self.webdav)
            self.monitor.add_webdav_handler(self.webdav)
            if self.log is not None:
                self.log.info('add webdav service success')
            else:
                print('add webdav service success')
        return res

    def add_samba_handler(self, config):
        """
        Add samba client from configuration
        :param config: Samba configuration
        :return: True if success else False
        """
        if self.samba is not None:
            self.samba = None
        scheduler_samba = SambaHandler()
        scheduler_samba.add_log_handler(self.log)
        res_connect = scheduler_samba.connect(config)
        res_set_root_path = scheduler_samba.set_root_path(config['samba_root_path'])
        res = res_connect and res_set_root_path
        if res is False:
            self.on_any_error_callback(0x08, 'Remote', remote=config)
        else:
            self.samba = scheduler_samba
            self.compressor.add_samba_handler(self.samba)
            self.monitor.add_samba_handler(self.samba)
            if self.log is not None:
                self.log.info('add samba service success')
            else:
                print('add samba service success')
        return res

    def create_job(self, config):
        """
        :param config: The job's configuration
        :return: res: A job class if create success else False
        """
        res = None
        if not os.path.exists(config['src_path']):
            err_msg = f"{config['name']}'s src_path {config['src_path']} does not exists"
            if self.log is not None:
                self.log.error(err_msg)
            else:
                print(err_msg)
            self.on_any_error_callback(0x01, 'Create', job=Plan(config))
            return False
        if config['is_time_job'] == 'True':
            res = self.create_time_interval_job(config)
        elif config['is_monitor_job'] == 'True':
            res = self.create_edit_trigger_job(config)
        if res is False:
            self.on_sync_widget_error_callback(0x01, 'Create')
        self.on_sync_job_status_callback(res, 'Create')
        return res

    def delete_job(self, job_id):
        """
        :param job_id: The job's id
        :return: None
        """
        if self.job_dict[job_id].is_time_job == 'True':
            self.delete_time_interval_job(job_id)
        elif self.job_dict[job_id].is_monitor_job == 'True':
            self.delete_edit_trigger_job(job_id)

    def edit_job(self, job_id, new_config):
        """
        :param job_id: The job's id
        :param new_config: The job's new config
        :return:
        """
        res = None
        create_time = self.job_dict[job_id].create_time
        if new_config['is_time_job'] != self.job_dict[job_id].is_time_job:
            if new_config['is_time_job'] == 'True':
                self.delete_edit_trigger_job(job_id)
                res = self.create_time_interval_job(new_config, job_id, create_time)
            else:
                self.delete_time_interval_job(job_id)
                res = self.create_edit_trigger_job(new_config, job_id, create_time)
            self.on_sync_job_status_callback(res, 'Edit')
            return

        if self.job_dict[job_id].is_time_job == 'True':
            res = self.edit_time_interval_job(job_id, create_time, new_config)
        elif self.job_dict[job_id].is_monitor_job == 'True':
            res = self.edit_edit_trigger_job(job_id, create_time, new_config)
        self.on_sync_job_status_callback(res, 'Edit')

    def compress(self, job):
        """
        :param job: Job configuration
        :return: None
        """
        res = self.compressor.scheduled_compress(job)
        if res == self.compressor.CompressError:
            err_msg = f'occurs error when execute {job.name}'
            if self.log is not None:
                self.log.error(err_msg)
            else:
                print(err_msg)
        else:
            if res != 0x00:
                self.on_any_error_callback(res, 'Execute', job=job)
            job.next_run_time = str(self.scheduler.get_job(job_id=job.id).next_run_time)[0:-13]
            self.on_sync_job_status_callback(job, 'Execute')

    def create_time_interval_job(self, config, config_id=None, create_time=None):
        """
        :param config: The schedule configuration
        :param config_id: Specific the job id
        :param create_time: Specific the job create time
        :return: job: A plan class
        """
        job = Plan(config, self.id_generator.get_id())
        if config_id is not None:
            job = Plan(config, config_id)
            job.create_time = create_time

        interval = None
        data = int(job.time_interval)
        if job.time_interval_unit == 'Minute':
            interval = IntervalTrigger(minutes=data)
        elif job.time_interval_unit == 'Hour':
            interval = IntervalTrigger(hours=data)

        res = self.scheduler.add_job(self.compress, trigger=interval,
                                     id=job.id, args=[job])
        msg = f'create time interval job: {job.name} {job.src_path}'
        job.next_run_time = str(res.next_run_time)[0:-13]
        job.last_trigger_time = 'None'
        self.log.info(msg)
        self.job_dict.update({
            job.id: job
        })
        return job

    def create_time_specific_job(self, config):
        """
        :param config: The schedule config
        :return:
        """
        pass

    def create_edit_trigger_job(self, config, config_id=None, create_time=None):
        """
        :param config: The schedule config
        :param config_id: Specific the job id
        :param create_time: Specific the job create time
        :return: job: a plan class with id
        """
        job = Plan(config, self.id_generator.get_id())
        if config_id is not None:
            job = Plan(config, config_id)
            job.create_time = create_time

        res = self.monitor.add_scheduled_job(job)
        if res is False:
            return False
        job.next_run_time = 'None'
        job.last_trigger_time = 'None'
        msg = f'create edit trigger job: {job.name} {job.src_path}'
        self.log.info(msg)
        self.job_dict.update({
            job.id: job
        })
        return job

    def delete_time_interval_job(self, job_id):
        """
        :param job_id: The schedule job's id
        :return:
        """
        self.scheduler.remove_job(job_id)
        msg = f'delete time interval job: {self.job_dict[job_id].name}'
        self.job_dict.pop(job_id)
        self.log.info(msg)

    def delete_time_specific_job(self, job_id):
        """
        :param job_id: The schedule job's id
        :return:
        """
        pass

    def delete_edit_trigger_job(self, job_id):
        """
        :param job_id: The schedule job's id
        :return:
        """
        self.monitor.remove_job(job_id)
        msg = f'delete edit trigger job: {self.job_dict[job_id].name}'
        self.job_dict.pop(job_id)
        self.log.info(msg)

    def edit_time_interval_job(self, origin_id, origin_create_time, new_config):
        """
        :param origin_id: Job's origin id
        :param origin_create_time: Job's origin create time
        :param new_config: Job's new configuration
        :return: res: a new job class with origin id
        """
        self.delete_time_interval_job(origin_id)
        res = self.create_time_interval_job(new_config, origin_id, origin_create_time)
        return res

    def edit_time_specific_job(self, origin_id, new_config):
        """
        :param origin_id: Job's origin id
        :param new_config: Job's new configuration
        :return:
        """
        pass

    def edit_edit_trigger_job(self, origin_id, origin_create_time, new_config):
        """
        :param origin_id: Job's origin id
        :param origin_create_time: Job's origin create time
        :param new_config: Job's new configuration
        :return: res: a new job class with origin id
        """
        self.delete_edit_trigger_job(origin_id)
        res = self.create_edit_trigger_job(new_config, origin_id, origin_create_time)
        return res

    def add_on_sync_widget_data_callback(self, handler):
        self.on_sync_widget_data_callback = handler

    def add_on_sync_widget_remote_callback(self, handler):
        self.on_sync_widget_remote_callback = handler

    def add_on_sync_widget_error_callback(self, handler):
        self.on_sync_widget_error_callback = handler

    def on_sync_job_status_callback(self, job, mode):
        """
        Sync job status
        :param job: Job configuration
        :param mode: Status mode -> Create Edit Execute
        :return: None
        """
        if mode != 'Create':
            if job.is_days_restrict == 'True':
                restrict_path_folder_numbers(job.dst_path, job.days_restriction)
            if job.is_size_restrict == 'True':
                restrict_size = 0
                if job.size_restriction_unit == 'MB':
                    restrict_size = int(job.size_restriction) * 1024 * 1024
                elif job.size_restriction_unit == 'GB':
                    restrict_size = int(job.size_restriction) * 1024 * 1024 * 1024
                restrict_path_folder_size(job.dst_path, restrict_size, self.log)
        job_backups_info = get_backups_dir_info(job.dst_path)
        job.backups_count = 'L:' + str(job_backups_info[0])
        job.backups_size = 'L:' + str(job_backups_info[1])
        if self.webdav is not None:
            if job.is_webdav_sync == 'True':
                webdav_backups_info = self.webdav.get_backups_dir_info(job)
                if webdav_backups_info is not False:
                    job.backups_count += ' W:' + str(webdav_backups_info[0])
                    job.backups_size += ' W:' + str(webdav_backups_info[1])
                else:
                    self.on_any_error_callback(0x01, 'Remote', job=job)
        if self.samba is not None:
            if job.is_samba_sync == 'True':
                samba_backups_info = self.samba.get_backups_dir_info(job)
                if samba_backups_info is not False:
                    job.backups_count += ' S:' + str(samba_backups_info[0])
                    job.backups_size += ' S:' + str(samba_backups_info[1])
                else:
                    self.on_any_error_callback(0x04, 'Remote', job=job)
        if mode == 'Edit':
            job.edit_time = time.strftime('%Y-%m-%d %H-%M-%S', time.localtime())
        if self.on_sync_widget_data_callback is not None:
            self.on_sync_widget_data_callback(job, mode)

    def on_any_error_callback(self, code, mode, job=None, remote=None):
        """
        Create:
            0x01 -> Create job failed
        Load:
            0x01 -> Load remote configuration failed
            0x02 -> Load job configuration failed
        Execute:
            0x01 -> Compress folder failed
            0x02 -> Upload to webdav failed
            0x04 -> Lack of webdav service
            0x08 -> Upload to samba failed
            0x10 -> Lack of samba service
        Remote:
            0x01 -> Get webdav folder info failed
            0x02 -> Add webdav client failed
            0x04 -> Get samba folder info failed
            0x08 -> Add samba client failed
        :param code: Error code
        :param mode: Error occurs mode
        :param job: Job configuration when execution error occurs
        :param remote: Remote configuration when remote error occurs
        :return:
        """
        self.on_sync_widget_error_callback(code, mode, job, remote)

    def load_encrypted_data_from_file(self, filepath):
        """
        :param filepath: The data file's filepath
        :return: None
        """
        if not os.path.exists(filepath):
            config_file = open(filepath, 'w')
            config_file.close()
            remote_config = {
                'is_any_webdav_service': 'False',
                'is_any_samba_service': 'False'
            }
            self.on_sync_widget_remote_callback(remote_config)
            return
        if not os.path.getsize(filepath):
            remote_config = {
                'is_any_webdav_service': 'False',
                'is_any_samba_service': 'False'
            }
            self.on_sync_widget_remote_callback(remote_config)
            return
        with open(filepath, 'r') as f:
            encrypted_data = f.read()
            data = json.loads(base64.decodebytes(encrypted_data.encode()).decode())

            remote_config = data['remote_config']
            try:
                if remote_config['is_any_webdav_service'] == 'True':
                    scheduler_webdav_config = {
                        'webdav_host_address': remote_config['webdav_host_address'],
                        'webdav_username': remote_config['webdav_username'],
                        'webdav_password': remote_config['webdav_password'],
                        'webdav_root_path': remote_config['webdav_root_path']
                    }
                    res = self.add_webdav_handler(scheduler_webdav_config)
                    if res is False:
                        remote_config['is_any_webdav_service'] = 'False'
                if remote_config['is_any_samba_service'] == 'True':
                    scheduler_samba_config = {
                        'samba_host_address': remote_config['samba_host_address'],
                        'samba_host_port': remote_config['samba_host_port'],
                        'samba_host_name': remote_config['samba_host_name'],
                        'samba_username': remote_config['samba_username'],
                        'samba_password': remote_config['samba_password'],
                        'samba_root_path': remote_config['samba_root_path']
                    }
                    res = self.add_samba_handler(scheduler_samba_config)
                    if res is False:
                        remote_config['is_any_samba_service'] = 'False'
                self.on_sync_widget_remote_callback(remote_config)
            except KeyError:
                self.on_any_error_callback(0x01, 'Load')
            try:
                job_config = data['job_config']
                for value in job_config.values():
                    if self.webdav is None:
                        value['is_webdav_sync'] = 'False'
                    if self.samba is None:
                        value['is_samba_sync'] = 'False'
                    self.create_job(value)
            except KeyError:
                self.on_any_error_callback(0x02, 'Load')

    def save_encrypted_data_to_file(self, filepath):
        """
        :param filepath: The data file's filepath
        :return:
        """
        remote_webdav_config = self.webdav.get_config_dict() if self.webdav is not None \
            else {'is_any_webdav_service': 'False'}
        remote_samba_config = self.samba.get_config_dict() if self.samba is not None \
            else {'is_any_samba_service': 'False'}
        remote_config = dict(remote_webdav_config, **remote_samba_config)
        job_config_dict = dict()
        for value in self.job_dict.values():
            job_config_dict.update({
                value.id: value.get_config_dict()
            })
        data_dict = dict()
        data_dict.update({
            "remote_config": remote_config,
            "job_config": job_config_dict
        })
        encode_dict = base64.encodebytes(json.dumps(data_dict, ensure_ascii=False).encode())
        with open(filepath, 'w') as f:
            f.write(encode_dict.decode())
