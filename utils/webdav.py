#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
webdav.py

This module uploads file to web device like jianguoyun.

https://skshetry.github.io/webdav4/reference/client.html
"""
from webdav4.client import Client
from webdav4.client import ClientError, HTTPError, HTTPStatusError
from webdav4.fsspec import WebdavFileSystem
from httpx import WriteTimeout, WriteError, ConnectError, ReadTimeout, ReadError, ProxyError


class WebDavHandler(object):
    def __init__(self, config):
        self.log = None
        self.host_address = 'https://dav.jianguoyun.com/dav/'
        self.username = config['webdav_username']
        self.password = config['webdav_password']
        self.root_path = None
        self.config_dict = dict()
        self.client = Client(base_url=self.host_address,
                             auth=(self.username, self.password))
        self.web_file_system = WebdavFileSystem(base_url=self.host_address,
                                                auth=(self.username, self.password),
                                                client=self.client)

    def add_log_handler(self, handler):
        """
        :param handler: logger handler
        :return: None
        """
        self.log = handler

    def get_config_dict(self):
        """
        :return: webdav client configuration(dict)
        """
        self.config_dict.update({
            "is_any_webdav_service": "True",
            "webdav_host_address": self.host_address,
            "webdav_username": self.username,
            "webdav_password": self.password,
            "webdav_root_path": self.root_path,
        })
        return self.config_dict

    def set_root_path(self, path):
        """
        :param path: The file stores' root path
        :return: True if success else False
        """
        try:
            if not self.client.exists(path):
                self.web_file_system.makedirs(path)
        except (ClientError, HTTPError, HTTPStatusError, AttributeError, ConnectError, ProxyError):
            err_msg = f'webdav set root path  {path} failed'
            if self.log is not None:
                self.log.error(err_msg)
            else:
                print(err_msg)
            return False
        else:
            self.root_path = path
            return True

    def get_dir_info(self, path):
        """
        :param path: The path to get its information about file numbers and size
        :return: [count(int), size(bytes)]
        """
        size = 0
        count = 0
        try:
            dir_list = self.web_file_system.ls(path)
            for index, dir_value in enumerate(dir_list):
                file_list = self.web_file_system.ls(dir_value['name'])
                for file in file_list:
                    count += 1
                    size += self.web_file_system.size(file['name'])
        except (ClientError, HTTPError, HTTPStatusError, AttributeError,
                FileNotFoundError, ConnectError, ReadTimeout, ReadError):
            err_msg = f'webdav get dir {path} info failed'
            if self.log is not None:
                self.log.error(err_msg)
            else:
                print(err_msg)
            return False
        else:
            return [count, size]
    
    def get_backups_dir_info(self, job):
        """
        This function is specific for this program.
        :param job: The job to get its information
        :return: [count(int), size(bytes)]
        """
        target_path = self.root_path + '/Backup-Schedule/' + job.name
        res = self.get_dir_info(target_path)
        return res

    def upload(self, src_path, dst_path):
        """
        :param src_path: The upload file's path
        :param dst_path: The upload file's save path which should contains the file name!
        :return: None
        """
        try:
            self.client.upload_file(from_path=src_path, to_path=dst_path, overwrite=False)
        except (ClientError, FileNotFoundError, ConnectError):
            err_msg = f'upload {dst_path} to webdav failed'
            if self.log is not None:
                self.log.error(err_msg)
            else:
                print(err_msg)

    def scheduled_upload(self, src_path, job):
        """
        This function is specific for this program.
        It will upload file to 'root_path/Backup-Schedule/job.name/days/filename.xx'
        :param src_path: The upload file's source path
        :param job: The job configuration
        :return: True if upload file success else False
        """
        res = True
        days = src_path.split("\\")[-2]
        filename = src_path.split("\\")[-1]
        if not self.client.exists(f'{self.root_path}/Backup-Schedule/{job.name}/{days}'):
            self.web_file_system.makedirs(f'{self.root_path}/Backup-Schedule/{job.name}/{days}')
        dst_path = f"{self.root_path}/Backup-Schedule/{job.name}/{days}/{filename}"
        try:
            self.web_file_system.put_file(lpath=src_path, rpath=dst_path)
        except (ClientError, FileNotFoundError, WriteTimeout, WriteError, ConnectError):
            res = False
            err_msg = f'upload {dst_path} to webdav failed'
            if self.log is not None:
                self.log.error(err_msg)
            else:
                print(err_msg)
            return res
        else:
            return res
