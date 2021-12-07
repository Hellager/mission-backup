#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
bsamba.py

This module uploads file to samba server.
"""
from smb.SMBConnection import SMBConnection
from smb.base import NotConnectedError, OperationFailure


class SambaHandler(object):
    def __init__(self):
        self.log = None

        self.host_address = None
        self.host_name = None
        self.host_port = None
        self.username = None
        self.password = None
        self.status = None
        self.client = None
        self.root_path = None
        self.config_dict = dict()

    def add_log_handler(self, handler):
        """
        :param handler: logger handler
        :return: None
        """
        self.log = handler

    def get_config_dict(self):
        """
        :return: samba client configuration(dict)
        """
        self.config_dict.update({
            "is_any_samba_service": "True",
            "samba_host_address": self.host_address,
            "samba_host_port": self.host_port,
            "samba_username": self.username,
            "samba_password": self.password,
            "samba_host_name": self.host_name,
            "samba_root_path": self.root_path
        })
        return self.config_dict

    def connect(self, config):
        """
        Connect to samba server
        :param config: The samba host configuration
        :return: True if connect success else False
        """
        try:
            self.client = SMBConnection(config['samba_username'], config['samba_password'], '',
                                        config['samba_host_name'], use_ntlm_v2=True)
            self.client.connect(config['samba_host_address'], config['samba_host_port'])
            self.status = self.client.auth_result
        except (TimeoutError, OperationFailure, ConnectionRefusedError):
            err_msg = f"connect to {config['samba_host_address']}:{config['samba_host_port']} failed"
            if self.log is not None:
                self.log.error(err_msg)
            else:
                print(err_msg)
            return False
        else:
            self.host_address = config['samba_host_address']
            self.host_name = config['samba_host_name']
            self.host_port = config['samba_host_port']
            self.username = config['samba_username']
            self.password = config['samba_password']
            return True

    def disconnect(self):
        """
        Disconnect from samba server
        :return: None
        """
        if self.status:
            self.client.close()

    def exists(self, path):
        """
        :param path: The path to be checked whether it exists
        :return: True if path exists else False
        """
        res = False
        dir_name = path.split('/')[len(path.split('/')) - 1]
        for value in self.client.listPath(self.host_name, path.replace(dir_name, '')):
            if (value.filename == dir_name) and (value.isDirectory is True):
                res = True
                break
        return res

    def set_root_path(self, path):
        """
        :param path: The file stores' root path
        :return: True if success else False
        """
        res = True
        address_list = path.split('/')
        address_list.remove('')
        address_text = f''
        for value in address_list:
            address_text = address_text + f'/{value}'
            try:
                if not self.exists(address_text):
                    self.client.createDirectory(self.host_name, address_text)
            except (NotConnectedError, TimeoutError, OperationFailure):
                res = False
                err_msg = f'set root path {path} failed'
                if self.log is not None:
                    self.log.error(err_msg)
                else:
                    print(err_msg)
                return res
        self.root_path = path
        return res

    def get_dir_info(self, path):
        """
        :param path: The path to get its information about file numbers and size
        :return: [count(int), size(bytes)]
        """
        count = 0
        size = 0
        try:
            res = self.client.listPath(self.host_name, path)
            for index, dir_value in enumerate(res):
                if '.' not in dir_value.filename:
                    file_list = self.client.listPath(self.host_name, path + '/' + dir_value.filename)
                    for file_value in file_list:
                        if file_value.filename == '.':
                            continue
                        if file_value.filename == '..':
                            continue
                        count += 1
                        size += file_value.file_size
        except (NotConnectedError, TimeoutError, OperationFailure):
            res = False
            err_msg = f'samba get dir {path} info failed'
            if self.log is not None:
                self.log.error(err_msg)
            else:
                print(err_msg)
            return res
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

    def upload(self, from_path, to_path):
        """
        :param from_path: The upload file's path
        :param to_path: The upload file's save path which should contains the file name!
        :return: None
        """
        with open(from_path, 'rb') as f:
            self.client.storeFile(self.host_name, to_path, f)

    def scheduled_upload(self, src_path, job):
        """
        This function is specific for this program.
        It will upload file to 'self.root_path/Backup-Schedule/job.name/days/filename.xx'
        :param src_path: The upload file's source path
        :param job: The job configuration
        :return: True if success else False
        """
        days = src_path.split("\\")[-2]
        filename = src_path.split("\\")[-1]
        if not self.exists(f'{self.root_path}/Backup-Schedule'):
            self.client.createDirectory(self.host_name, f'{self.root_path}/Backup-Schedule')
        if not self.exists(f'{self.root_path}/Backup-Schedule/{job.name}'):
            self.client.createDirectory(self.host_name, f'{self.root_path}/Backup-Schedule/{job.name}')
        if not self.exists(f'{self.root_path}/Backup-Schedule/{job.name}/{days}'):
            self.client.createDirectory(self.host_name, f'{self.root_path}/Backup-Schedule/{job.name}/{days}')
        dst_path = f'{self.root_path}/Backup-Schedule/{job.name}/{days}/{filename}'
        try:
            with open(src_path, 'rb') as f:
                self.client.storeFile(self.host_name, dst_path, f)
        except (NotConnectedError, TimeoutError, OperationFailure, FileNotFoundError):
            err_msg = f'upload file to {dst_path} failed'
            if self.log is not None:
                self.log.error(err_msg)
            else:
                print(err_msg)
            return -1
        else:
            return True
