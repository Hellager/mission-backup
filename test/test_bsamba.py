#!/usr/bin/env python
# -*- coding: utf-8 -*-
"""
test_bsamba.py

This is the unit test for bsamba.py
"""

from utils.bsamba import *
from utils.plan import *

target_dir = 'D:\\Demonstration'
save_dir = 'D:\\Demonstration\\Backups'

data = dict()
data.update({
    'name': 'test_compressor',
    'src_path': target_dir,
    'dst_path': save_dir,
    'compress_format': '7z',
    'has_password': 'true',
    'password': '123456',

    'is_time_job': 'False',
    'time_interval': '30',
    'time_interval_unit': 'Minute',

    'is_monitor_job': 'False',

    'is_days_restrict': 'False',
    'days_restriction': '3',

    'is_size_restrict': 'False',
    'size_restriction': '25',
    'size_restriction_unit': 'MB',

    "is_webdav_sync": 'False',
    "is_samba_sync": 'True',

    "next_run_time": ' ',
    "last_trigger_time": ' ',

    "backups_count": "",
    "backups_size": "",
})
test_plan = Plan(data)

scheduler_samba_config = {
    'samba_host_address': 'ip address',
    'samba_host_port': 'port',
    'samba_host_name': 'remote name',
    'samba_username': 'username',
    'samba_password': 'password',
    'samba_root_path': 'root path for Backup-Schedule'
}
test_samba = SambaHandler()
test_samba.connect(scheduler_samba_config)
test_samba.set_root_path(scheduler_samba_config['samba_root_path'])
test_samba.upload('file src path', 'samba store path(with file name)')
# test_samba.upload('D:\\Demonstration\\test.txt', '/{host name}/test.txt')
test_samba.disconnect()
