#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
test_webdav.py

The unit test for webdav.py
"""
from utils.webdav import *
from utils.plan import *

webdav_config = {
    'webdav_host_address': 'https://dav.jianguoyun.com/dav/',
    'webdav_username': 'username',
    'webdav_password': 'password',
    'webdav_root_path': 'root path'
}

target_dir = 'D:\\Demonstration'
save_dir = 'D:\\Demonstration\\Backups'

data = dict()
data.update({
    'name': 'test_compressor',
    'src_path': target_dir,
    'dst_path': save_dir,
    'compress_format': 'zip',
    'has_password': 'False',
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

    'is_webdav_sync': 'True',
    'is_samba_sync': 'False',

    "next_run_time": "",
    "last_trigger_time": "",

    "backups_count": "",
    "backups_size": "",
})

test_plan = Plan(data)
test_webdav = WebDavHandler(webdav_config)
test_webdav.set_root_path(webdav_config['webdav_root_path'])
test_webdav.upload("file src path", 'webdav store path')
# test_webdav.upload('D:\\Demonstration\\test.txt', '/{root path}/test.txt')
