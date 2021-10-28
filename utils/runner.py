#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
runner

This module manages those schedule jobs created by the program.
"""

from apscheduler.schedulers.background import BackgroundScheduler
from apscheduler.executors.pool import ThreadPoolExecutor
from apscheduler.triggers.interval import IntervalTrigger
from apscheduler.events import EVENT_JOB_ERROR
from utils.monitor import *
import re
import logging


class ScheduleRunner(object):
    def __init__(self, syslog, signal):
        self.log = syslog
        self.compressor = Compressor(syslog, signal)
        executors = {
            'default': ThreadPoolExecutor(10)
        }
        self.runner = BackgroundScheduler(executors=executors)
        self.runner.start()
        self.runner_id_list = dict()
        self.monitor = DirMonitor(syslog, signal)

        self.aps_log = logging.getLogger('apscheduler')
        self.aps_log.setLevel(logging.INFO)
        fmt = logging.Formatter('%(asctime)s - %(pathname)s[line:%(lineno)d] - %(levelname)s: %(message)s')
        h = logging.FileHandler('log/aps.txt', 'a')
        h.setFormatter(fmt)
        self.aps_log.addHandler(h)
        self.runner.add_listener(self.schedule_listener, EVENT_JOB_ERROR)

    def compress(self, config):
        """
        :param config: The schedule config to do the compress job
        :return:
        """
        self.compressor.compress_single_folder(config['src path'], config['dst path'], config['compress format'])

    def create_time_interval_schedule(self, config):
        """
        :param config: The schedule config to create a time interval job
        :return:
        """
        numbers = int(re.findall("\\d+", config['time interval'])[0])
        if config['time interval'][-1:] == 'e':
            trigger_interval = IntervalTrigger(minutes=numbers)
            self.runner.add_job(self.compress, trigger=trigger_interval,
                                id=config['name'], args=[config])
        elif config['time interval'][-1:] == 'r':
            trigger_interval = IntervalTrigger(hours=numbers)
            self.runner.add_job(self.compress, trigger=trigger_interval,
                                id=config['name'], args=[config])

        self.runner_id_list.update({config['name']: ''})
        if self.log:
            schedule_id = config['name']
            schedule_path = config['src path']
            self.log.info(f'create time interval schedule: {schedule_id} {schedule_path}')

    def create_time_specific_schedule(self, config):
        pass

    def create_edit_trigger_watcher(self, config):
        """
        :param config: The schedule config to create a edit trigger job
        :return:
        """
        self.monitor.add_job(config['name'], config['src path'], config['dst path'], config['compress format'])
        if self.log:
            schedule_id = config['name']
            schedule_path = config['src path']
            self.log.info(f'create edit trigger watcher: {schedule_id} {schedule_path}')

    def delete_time_interval_schedule(self, schedule_id):
        """
        :param schedule_id: The schedule's id which you want to delete
        :return:
        """
        self.runner.remove_job(schedule_id)
        self.runner_id_list.pop(schedule_id)
        if self.log:
            self.log.info(f'delete time interval schedule: {schedule_id}')

    def delete_time_specific_schedule(self, schedule_id):
        pass

    def delete_edit_trigger_watcher(self, schedule_id):
        """
        :param schedule_id: The schedule's id which you want to delete
        :return:
        """
        self.monitor.remove_job(schedule_id)
        if self.log:
            self.log.info(f'delete time interval schedule: {schedule_id}')

    def edit_time_interval_schedule(self, schedule_id, config):
        """
        :param schedule_id: The schedule's id which you want to edit
        :param config: New config for the schedule
        :return:
        """
        if schedule_id in self.runner_id_list.keys():
            self.runner.remove_job(schedule_id)
        self.create_time_interval_schedule(config)

    def edit_time_specific_schedule(self, schedule_id):
        pass

    def edit_edit_trigger_watcher(self, schedule_id, config):
        """
        :param schedule_id: The schedule's id which you want to edit
        :param config: New config for the schedule
        :return:
        """
        if schedule_id in self.monitor.watch_dict.keys():
            self.monitor.remove_job(schedule_id)
        self.create_edit_trigger_watcher(config)

    def create_job(self, config):
        """
        :param config: The job's configuration which is going to be created
        :return:
        """
        if config['trigger type'] == 'time trigger':
            self.create_time_interval_schedule(config)
        elif config['trigger type'] == 'edit trigger':
            self.create_edit_trigger_watcher(config)

    def delete_job(self, config):
        """
        :param config: The job's configuration which is going to be deleted
        :return:
        """
        if config['trigger type'] == 'time trigger':
            self.delete_time_interval_schedule(config['name'])
        elif config['trigger type'] == 'edit trigger':
            self.delete_edit_trigger_watcher(config['name'])

    def edit_job(self, config):
        """
        :param config: The job's configuration which is going to be edited
        :return:
        """
        if config['trigger type'] == 'time trigger':
            self.edit_time_interval_schedule(config['name'], config)
        elif config['trigger type'] == 'edit trigger':
            self.edit_edit_trigger_watcher(config['name'], config)

    def load_schedule_job(self, configs):
        """
        :param configs: The jobs' configurations which is going to be loaded
        :return:
        """
        for index, value in enumerate(configs):
            if value['trigger type'] == 'time trigger':
                self.create_time_interval_schedule(value)
            elif value['trigger type'] == 'edit trigger':
                self.create_edit_trigger_watcher(value)

    def schedule_listener(self, event):
        """
        :param event: The apscheduler's event
        :return:
        """
        if event.code == EVENT_JOB_ERROR:
            self.aps_log.error(event.code)
