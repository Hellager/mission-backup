#!/usr/bin/env python
# -*- coding: utf-8 -*-
"""
compressor.py

This module helps to compress folder to different formats.
The following formats are now supported:
    zip, tar.gz, tar.bz2, tar.xz, 7z.
The following formats are now supported for encrypted:
    zip(only when you have Microsoft Visual C++ 14.0), 7z.
"""

import os
import shutil
import py7zr
from utils.plan import *


class Compressor(object):
    def __init__(self):
        self.log = None
        self.signal = None
        self.webdav = None
        self.samba = None

        # Error Code
        # x x x x x         1->Fail 0->Success
        # samba_service_res samba_sync_res webdav_service_res webdav_sync_res compress_res
        self.CompressError = 0x01
        self.MissingWebdavServiceError = 0x04
        self.WebdavSyncError = 0x02
        self.MissingSambaServiceError = 0x10
        self.SambaSyncError = 0x08

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
        if self.log is not None:
            self.webdav.add_log_handler(self.log)

    def add_samba_handler(self, handler):
        """
        :param handler: samba client(SambaHandler class)
        :return: None
        """
        self.samba = handler
        if self.log is not None:
            self.samba.add_log_handler(self.log)

    @staticmethod
    def compress_folder(src_path, dst_path, formatter, password=None):
        """
        :param src_path: The folder to be compressed
        :param dst_path: The folder where the compressed folder is saved
        :param formatter: Format for compressed files
        :param password: password for compressed files. It now only supports zip, 7z.
        :return: True if success else False
        """
        if formatter == 'tar.gz':
            formatter = 'gztar'
        elif formatter == 'tar.bz2':
            formatter = 'bztar'
        elif formatter == 'tar.xz':
            formatter = 'xztar'
        dst_path = dst_path + "\\" + time.strftime("%Y-%m-%d", time.localtime()) + "\\temp"
        if os.path.exists(dst_path):
            shutil.rmtree(dst_path)
        shutil.copytree(src_path, dst_path, ignore=shutil.ignore_patterns("BackUps"))

        compress_path = dst_path[:-5] + "\\" + time.strftime("%H'%M", time.localtime())
        if not formatter == '7z':
            try:
                shutil.make_archive(compress_path, formatter, dst_path)
            except(SystemExit, Exception, StopIteration, IOError):
                print(f'occurs error when compress folder {src_path} to {formatter}')
            else:
                shutil.rmtree(dst_path)
                print(f'compress {src_path} to {formatter}')
                return True
        else:
            save_path_for_7z = dst_path[:-5] + '\\temp\\'
            compress_path = compress_path + '.7z'
            with py7zr.SevenZipFile(compress_path, 'w', password=None if password is None else password) as zf:
                if password is not None:
                    zf.set_encrypted_header(True)
                for dir_path, dir_names, file_names in os.walk(dst_path):
                    for filename in file_names:
                        temp_path = dir_path.replace(save_path_for_7z, '')
                        file_path = os.path.join(dir_path, filename)
                        filename = os.path.join(temp_path, filename)
                        if temp_path == save_path_for_7z[0:-1]:
                            filename = filename.replace(save_path_for_7z, '')
                        try:
                            zf.write(file_path, arcname=filename)
                        except(SystemExit, Exception, StopIteration, IOError):
                            print(f'occurs error when compress folder {src_path} to {formatter}')
                        else:
                            continue
            shutil.rmtree(dst_path)
            print(f'compress {src_path} to {formatter}')
            return True

    def scheduled_compress(self, job):
        """
        :param job: The job configuration
        :return: 0x00 if success else error code
        """
        password = job.password
        formatter = job.compress_format
        if formatter == 'tar.gz':
            formatter = 'gztar'
        elif formatter == 'tar.bz2':
            formatter = 'bztar'
        elif formatter == 'tar.xz':
            formatter = 'xztar'
        handle_day = time.strftime("%Y-%m-%d", time.localtime())
        dst_path = job.dst_path.replace('/', '\\') + "\\" + handle_day + "\\temp"
        if os.path.exists(dst_path):
            shutil.rmtree(dst_path)
        shutil.copytree(job.src_path, dst_path, ignore=shutil.ignore_patterns("BackUps"))

        handle_time = time.strftime("%H'%M", time.localtime())
        compress_path = dst_path[:-5] + "\\" + handle_time
        remote_src_path = None
        if not formatter == '7z':
            try:
                shutil.make_archive(compress_path, formatter, dst_path)
            except(SystemExit, Exception, StopIteration, IOError):
                err_msg = f'occurs error when compress folder {job.src_path} to {formatter}'
                if self.log is not None:
                    self.log.error(err_msg)
                else:
                    print(err_msg)
                return self.CompressError
            else:
                remote_src_path = compress_path + '.' + job.compress_format
                remote_src_path = remote_src_path.replace("/", "\\")
        else:
            save_path_for_7z = dst_path[:-5] + '\\temp\\'
            compress_path = compress_path + '.7z'
            with py7zr.SevenZipFile(compress_path, 'w', password=None if password is None else password) as zf:
                if password is not None:
                    zf.set_encrypted_header(True)
                for dir_path, dir_names, file_names in os.walk(dst_path):
                    for filename in file_names:
                        temp_path = dir_path.replace(save_path_for_7z, '')
                        file_path = os.path.join(dir_path, filename)
                        filename = os.path.join(temp_path, filename)
                        if temp_path == save_path_for_7z[0:-1]:
                            filename = filename.replace(save_path_for_7z, '')
                        try:
                            zf.write(file_path, arcname=filename)
                        except(SystemExit, Exception, StopIteration, IOError):
                            err_msg = f'occurs error when compress folder {job.src_path} to {formatter}'
                            if self.log is not None:
                                self.log.error(err_msg)
                            else:
                                print(err_msg)
                            return self.CompressError
                        else:
                            continue
            remote_src_path = compress_path

        shutil.rmtree(dst_path)
        suc_msg = f'compress {job.src_path} to {formatter}'
        remote_webdav_execute_res = 0x00
        remote_samba_execute_res = 0x00
        time.sleep(3)
        remote_src_path.replace("\\", "")
        if self.log is not None:
            self.log.info(suc_msg)

        if job.is_samba_sync == 'True':
            if self.samba is not None:
                samba_upload_res = self.samba.scheduled_upload(remote_src_path, job)
                if samba_upload_res is False:
                    remote_samba_execute_res |= self.SambaSyncError
            else:
                err_msg = f'lack of samba service when execute {job.name}'
                if self.log is not None:
                    self.log.error(err_msg)
                else:
                    print(err_msg)
                remote_samba_execute_res |= self.MissingSambaServiceError | self.SambaSyncError
        if job.is_webdav_sync == 'True':
            if self.webdav is not None:
                webdav_upload_res = self.webdav.scheduled_upload(remote_src_path, job)
                if webdav_upload_res is False:
                    remote_webdav_execute_res |= self.WebdavSyncError
            else:
                err_msg = f'lack of webdav service when execute {job.name}'
                if self.log is not None:
                    self.log.error(err_msg)
                else:
                    print(err_msg)
                remote_webdav_execute_res |= self.MissingWebdavServiceError | self.WebdavSyncError

        remote_execute_res = remote_webdav_execute_res | remote_samba_execute_res
        return remote_execute_res
