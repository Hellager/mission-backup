#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
monitor

This module helps to monitor the change in specific folder.
It refers to the following author's code
————————————————
版权声明：本文为CSDN博主「天元浪子」的原创文章，遵循CC 4.0 BY-SA版权协议，转载请附上原文出处链接及本声明。
原文链接：https://blog.csdn.net/xufive/article/details/93847372
"""

from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler
from watchdog.utils.dirsnapshot import DirectorySnapshot, DirectorySnapshotDiff
from utils.compressor import *
import threading


class FileEventHandler(FileSystemEventHandler):
    def __init__(self, src_path, dst_path, formatter, syslog, signal):
        FileSystemEventHandler.__init__(self)
        self.monitor_path = src_path
        self.save_path = dst_path
        self.timer = None
        self.log = syslog
        self.compressor = Compressor(syslog, signal)
        self.compress_formatter = formatter
        self.snapshot = DirectorySnapshot(src_path)

    def on_any_event(self, event):
        if self.timer:
            self.timer.cancel()
        if str(self.save_path).replace('/', '') in str(event.src_path).replace('/', '').replace('\\', ''):
            return

        self.timer = threading.Timer(3, self.check_snap_shot)
        self.timer.start()

    def check_snap_shot(self):
        snapshot = DirectorySnapshot(self.monitor_path)
        diff = DirectorySnapshotDiff(self.snapshot, snapshot)
        self.snapshot = snapshot
        self.timer = None
        is_file_changed = len(diff.files_moved) | len(diff.files_modified) | len(diff.files_created) | len(diff.files_created)
        is_dirs_changed = len(diff.dirs_moved) | len(diff.dirs_modified) | len(diff.dirs_created) | len(diff.dirs_deleted)
        is_any_changed = is_file_changed | is_dirs_changed
        if is_any_changed:
            path = self.monitor_path
            self.log.info(f'detect file or dir change in path {path}')
            self.compressor.compress_single_folder(self.monitor_path, self.save_path, self.compress_formatter)


class DirMonitor(object):
    def __init__(self, syslog, signal):
        self.log = syslog
        self.signal = signal
        self.watch_dict = dict()
        self.observer = Observer()
        self.observer.start()

    def start(self):
        pass

    def stop(self):
        self.observer.stop()

    def add_job(self, job_id, src_path, dst_path, formatter):
        """
        :param job_id: The job's id which you want to add
        :param src_path: The folder you want to monitor
        :param dst_path: The folder to save compressed file from src_path
        :param formatter: The compressed format you want etc zip, tar.gz, tar.bz2, tar.xz, 7z.
        :return:
        """
        event_handler = FileEventHandler(src_path, dst_path, formatter, self.log, self.signal)
        job_watch = self.observer.schedule(event_handler, src_path, True)
        self.watch_dict.update({
            job_id: job_watch
        })

    def remove_job(self, job_id):
        """
        :param job_id: The job's id which you want to remove
        :return:
        """
        self.observer.unschedule(self.watch_dict[job_id])
