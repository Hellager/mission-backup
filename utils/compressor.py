#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
compressor

This module helps to compress folder to different formats.
The following formats are now supported:
    zip, tar.gz, tar.bz2, tar.xz, 7z.
"""
import os
import time
import shutil
import py7zr


class Compressor(object):
    def __init__(self, syslog, signal='none'):
        self.log = syslog
        # signal is specific for this program
        if not signal == 'none':
            self.signal = signal

    def compress_single_folder(self, src_path, dst_path, formatter):
        """
        :param src_path: The folder to be compressed
        :param dst_path: The folder where the compressed folder is saved
        :param formatter: Format for saving compressed files
        :return:
        """
        if not self.signal == 'none':
            self.signal(src_path, False)

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
                self.log.error(f'occurs error when compress folder {src_path} to {formatter}')
            else:
                shutil.rmtree(dst_path)
                self.log.info(f'compress {src_path} to {formatter}')
                if not self.signal == 'none':
                    self.signal(src_path, True)
                return True
        else:
            compress_path = compress_path + '.7z'
            with py7zr.SevenZipFile(compress_path, 'w') as zf:
                for dir_path, dir_names, file_names in os.walk(dst_path):
                    for filename in file_names:
                        file_path = os.path.join(dir_path, filename)
                        filename = os.path.join(dir_path, filename)
                        try:
                            zf.write(file_path, arcname=filename)
                        except(SystemExit, Exception, StopIteration, IOError):
                            self.log.error(f'occurs error when compress folder {src_path} to {formatter}')
                        else:
                            continue
            shutil.rmtree(dst_path)
            self.log.info(f'compress {src_path} to {formatter}')
            if not self.signal == 'none':
                self.signal(src_path, True)
            return True
