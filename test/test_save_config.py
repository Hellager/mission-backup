#!/usr/bin/env python
# -*- coding: utf-8 -*-

import base64
import json

encrypt_dict = {
    "remote_config": {
        "is_any_webdav_service": 'False',
        "is_any_samba_service": 'False',

        "webdav_host_address": "https://dav.jianguoyun.com/dav/",
        "webdav_username": "username",
        "webdav_password": "password",
        "webdav_root_path": "root depth",

        'samba_host_address': 'ip address',
        'samba_host_port': 'port',
        'samba_host_name': 'remote name',
        'samba_username': 'username',
        'samba_password': 'password',
        'samba_root_path': 'root path'
    },
    "job_config": {
        "1460074127015542784": {
            "name": "\u5b9a\u65f6\u8ba1\u5212",
            "src_path": "D:/Demonstration/folder 1",
            "dst_path": "D:/Demonstration/folder 1/Backups",
            "compress_format": "zip",
            "has_password": "False",
            "password": "None",

            "is_time_job": "True",
            "time_interval": "30",
            "time_interval_unit": "Minute",

            "is_monitor_job": "False",

            "is_days_restrict": "False",
            "days_restriction": "3",

            "is_size_restrict": "False",
            "size_restriction": "25",
            "size_restriction_unit": "MB",

            "is_webdav_sync": "True",
            "is_samba_sync": "False",

            "create_time": "2021-12-06-09-00",
            "edit_time": "2021-12-06-09-00",

            "next_run_time": "",
            "last_trigger_time": "",

            "backups_count": "12 7",
            "backups_size": "23940 13965",

            "id": "1460074127015542784"
        },
        "1460074127015542785": {
            "name": "\u76d1\u89c6\u8ba1\u5212",
            "src_path": "D:/Demonstration/folder 2",
            "dst_path": "D:/Demonstration/folder 2/Backups",
            "compress_format": "7z",
            "has_password": "False",
            "password": "None",

            "is_time_job": "False",
            "time_interval": "30",
            "time_interval_unit": "Minute",

            "is_monitor_job": "True",

            "is_days_restrict": "False",
            "days_restriction": "3",

            "is_size_restrict": "False",
            "size_restriction": "25",
            "size_restriction_unit": "MB",

            "is_webdav_sync": "False",
            "is_samba_sync": "True",

            "create_time": "2021-12-06-09-05",
            "edit_time": "2021-12-06-09-05",

            "next_run_time": "",
            "last_trigger_time": "",

            "backups_count": "0 0 0",
            "backups_size": "0",

            "id": "1460074127015542785"
        }
    }
}

print("加密")
encode_dict = base64.encodebytes(json.dumps(encrypt_dict, ensure_ascii=False).encode())
print(encode_dict.decode())

print("解密")
decode_dict = base64.decodebytes(encode_dict)
print(decode_dict.decode())

print("保存加密配置")
with open('test_config.txt', 'w') as f:
    f.write(encode_dict.decode())

print("读取加密配置")
with open('test_config.txt', 'r') as f:
    config_data = f.read()
    after_data = base64.decodebytes(config_data.encode())
    data_dict = json.loads(after_data.decode())
    print(json.dumps(data_dict, ensure_ascii=False, indent=4))
