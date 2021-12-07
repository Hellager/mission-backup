#!/usr/bin/env python
# -*- coding: utf-8 -*-
"""
task.py

This module specific the task class.
"""
import time


class Plan(object):
    def __init__(self, data, plan_id=None):
        # basic config
        self.name = data['name']
        self.src_path = data['src_path']
        self.dst_path = data['dst_path']
        self.compress_format = data['compress_format']
        self.has_password = data['has_password']
        self.password = data['password']

        # time job config
        self.is_time_job = data['is_time_job']
        self.time_interval = data['time_interval']
        self.time_interval_unit = data['time_interval_unit']

        # monitor job config
        self.is_monitor_job = data['is_monitor_job']

        # days restrict config
        self.is_days_restrict = data['is_days_restrict']
        self.days_restriction = data['days_restriction']

        # size restrict config
        self.is_size_restrict = data['is_size_restrict']
        self.size_restriction = data['size_restriction']
        self.size_restriction_unit = data['size_restriction_unit']

        # webdav config
        self.is_webdav_sync = data['is_webdav_sync']

        # Samba config
        self.is_samba_sync = data['is_samba_sync']

        # basic info
        self.create_time = self.generate_create_time()
        self.edit_time = self.create_time
        self.next_run_time = data['next_run_time'] if 'next_run_time' in data.keys() else 'None'
        self.last_trigger_time = data['last_trigger_time'] if 'last_trigger_time' in data.keys() else 'None'

        # backups info
        self.backups_count = data['backups_count'] if 'backups_count' in data.keys() else '0 0 0'
        self.backups_size = data['backups_size'] if 'backups_size' in data.keys() else '0 0 0'

        # plan id
        self.id = 'None' if plan_id is None else str(plan_id)

        self.config_dict = dict()

    @staticmethod
    def generate_create_time():
        """
        Generate plan create time
        :return: time(str)
        """
        res = time.strftime('%Y-%m-%d-%H-%M', time.localtime())
        return res

    def update_edit_time(self):
        """
        Update plan edit time
        :return: None
        """
        res = time.strftime('%Y-%m-%d-%H-%M', time.localtime())
        self.edit_time = res

    def get_config_dict(self):
        """
        :return: plan configuration(dict)
        """
        self.config_dict.update({
            "name": self.name,
            "src_path": self.src_path,
            "dst_path": self.dst_path,
            "compress_format": self.compress_format,
            "has_password": self.has_password,
            "password": self.password,
            "is_time_job": self.is_time_job,
            "time_interval": self.time_interval,
            "time_interval_unit": self.time_interval_unit,
            "is_monitor_job": self.is_monitor_job,
            "is_days_restrict": self.is_days_restrict,
            "days_restriction": self.days_restriction,
            "is_size_restrict": self.is_size_restrict,
            "size_restriction": self.size_restriction,
            "size_restriction_unit": self.size_restriction_unit,
            "is_webdav_sync": self.is_webdav_sync,
            "is_samba_sync": self.is_samba_sync,
            "create_time": self.create_time,
            "edit_time": self.edit_time,
            "next_run_time": self.next_run_time,
            "last_trigger_time": self.last_trigger_time,
            "backups_count": self.backups_count,
            "backups_size": self.backups_size,
            "id": self.id
        })
        return self.config_dict

