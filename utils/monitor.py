#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
monitor.py

This module helps to monitor the change in specific folder.
"""

from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler
from watchdog.utils.dirsnapshot import DirectorySnapshot, DirectorySnapshotDiff
from utils.compressor import *
import threading


class EventHandler(FileSystemEventHandler):
    def __init__(self, job):
        FileSystemEventHandler.__init__(self)
        self.job_config = job

        self.log = None
        self.timer = None
        self.signal = None
        self.webdav = None
        self.samba = None
        self.compressor = Compressor()
        self.snapshot = DirectorySnapshot(job.src_path)
        self.on_any_error_callback = None
        self.on_sync_job_status_callback = None

    def add_log_handler(self, handler):
        """
        :param handler: logger handler
        :return: None
        """
        self.log = handler
        self.compressor.add_log_handler(handler)

    def add_webdav_handler(self, handler):
        """
        :param handler: webdav client(WebDavHandler class)
        :return: None
        """
        self.webdav = handler
        self.compressor.add_webdav_handler(handler)

    def add_samba_handler(self, handler):
        """
        :param handler: samba client(SambaHandler class)
        :return: None
        """
        self.samba = handler
        self.compressor.add_samba_handler(handler)

    def add_on_any_error_callback_handler(self, handler):
        """
        add error callback
        :param handler: error callback handler
        :return: None
        """
        self.on_any_error_callback = handler

    def add_on_sync_job_status_callback(self, handler):
        """
        add sync job status callback
        :param handler: sync job status callback
        :return: None
        """
        self.on_sync_job_status_callback = handler

    def on_any_event(self, event):
        if self.timer:
            self.timer.cancel()

        if str(self.job_config.dst_path).replace('/', '') in str(event.src_path).replace('/', '').replace('\\', ''):
            return

        self.timer = threading.Timer(3, self.check_snap_shot)
        self.timer.start()

    def check_snap_shot(self):
        """
        Check folder's snap shot and handle differs
        :return: None
        """
        snapshot = DirectorySnapshot(self.job_config.src_path)
        diff = DirectorySnapshotDiff(self.snapshot, snapshot)
        self.snapshot = snapshot
        self.timer = None
        is_file_changed = len(diff.files_moved) | len(diff.files_modified) | \
            len(diff.files_created) | len(diff.files_created)
        is_dirs_changed = len(diff.dirs_moved) | len(diff.dirs_modified) | \
            len(diff.dirs_created) | len(diff.dirs_deleted)
        is_any_changed = is_file_changed | is_dirs_changed
        if is_any_changed:
            msg = f'detect file or dir change in path {self.job_config.src_path}'
            if self.log is not None:
                self.log.info(msg)
            else:
                print(msg)
            monitor_execute_res = self.compressor.scheduled_compress(self.job_config)
            if monitor_execute_res == self.compressor.CompressError:
                err_msg = f'occurs error when execute monitor job {self.job_config.name}'
                if self.log is not None:
                    self.log.info(err_msg)
                else:
                    print(err_msg)
            else:
                if monitor_execute_res != 0x00:
                    self.on_any_error_callback(monitor_execute_res, 'Execute', job=self.job_config)
                last_trigger_time = time.strftime('%Y-%m-%d %H:%M:%S', time.localtime())
                self.job_config.last_trigger_time = last_trigger_time
                self.on_sync_job_status_callback(self.job_config, 'Execute')


class DirMonitor(object):
    def __init__(self):
        self.log = None
        self.signal = None
        self.watcher_dict = dict()
        self.observer = Observer()
        self.observer.start()
        self.webdav = None
        self.samba = None
        self.on_any_error_callback = None
        self.on_sync_job_status_callback = None

    def add_log_handler(self, handler):
        """
        :param handler: logger handler
        :return: None
        """
        self.log = handler

    def add_webdav_handler(self, handler):
        """
        :param handler: webdav client(WebDavHandler class)
        :return: None
        """
        self.webdav = handler

    def add_samba_handler(self, handler):
        """
        :param handler: samba client(SambaHandler class)
        :return: None
        """
        self.samba = handler

    def add_on_any_error_callback_handler(self, handler):
        """
         add error callback
         :param handler: error callback handler
         :return: None
         """
        self.on_any_error_callback = handler

    def add_on_sync_job_status_callback(self, handler):
        """
        add sync job status callback
        :param handler: sync job status callback
        :return: None
        """
        self.on_sync_job_status_callback = handler

    def start(self):
        pass

    def stop(self):
        self.observer.stop()

    def add_job(self, job_id, src_path):
        """
        if you'd like to use this function, Please Modify the EventHandler class first.
        Add a watcher for src path
        :param job_id: The job's id which you want to add
        :param src_path: The folder you want to monitor
        :return: None
        """
        event_handler = EventHandler(src_path)
        job_watch = self.observer.schedule(event_handler, src_path, True)
        self.watcher_dict.update({
            job_id: job_watch
        })

    def add_scheduled_job(self, job):
        """
        Add a watcher for job src path
        :param job: The job configuration
        :return: True if success else False
        """
        event = EventHandler(job)
        if self.log is not None:
            event.add_log_handler(self.log)
        if self.webdav is not None:
            event.add_webdav_handler(self.webdav)
        if self.samba is not None:
            event.add_samba_handler(self.samba)
        if self.on_any_error_callback is not None:
            event.add_on_any_error_callback_handler(self.on_any_error_callback)
        if self.on_sync_job_status_callback is not None:
            event.add_on_sync_job_status_callback(self.on_sync_job_status_callback)

        try:
            job_watcher = self.observer.schedule(event, job.src_path, True)
        except OSError:
            if self.log is not None:
                err_msg = f"occurs error when creating monitor job {job.src_path}"
                self.log.error(err_msg)
            return False
        else:
            self.watcher_dict.update({
                job.id: job_watcher
            })
            return True

    def remove_job(self, job_id):
        """
        :param job_id: The job's id which you want to remove
        :return: True if success else False
        """
        try:
            self.observer.unschedule(self.watcher_dict[job_id])
        except OSError:
            if self.log is not None:
                err_msg = f"occurs error when delete monitor job {self.watcher_dict[job_id].name}"
                self.log.error(err_msg)
            return False
        else:
            if job_id in self.watcher_dict.keys():
                self.watcher_dict.pop(job_id)
            return True
