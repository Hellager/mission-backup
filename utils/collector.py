#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
collector.py

This file provides functions to get folder information or handle them.
"""

import os
import re
import shutil
from os.path import join, getsize


def get_dir_info(path):
    """
    :param path: The folder where you want information about the number of its files and the its size
    :return: [nums(int), size(bytes)]
    """
    size = 0
    nums = 0
    for root, dirs, files in os.walk(path):
        size += sum([getsize(join(root, name)) for name in files])
        nums += len(files)
    return [nums, size]


def get_backups_dir_info(path):
    """
    This function is specific for this program
    :param path: The folder where you want information about the number of its files and the its size
    :return: [nums(int), size(bytes)]
    """
    size = 0
    nums = 0
    for root, dirs, files in os.walk(path):
        for name in files:
            if re.match(r'\d{2}'+'\''+r'\d{2}', name):
                size += getsize(join(root, name))
                nums += 1
    return [nums, size]


def restrict_path_folder_numbers(path, number):
    """
    :param path: The folder you want to restrict subfolders' numbers
    :param number: The subfolers' number you want
    :return:
    """
    for root, dirs, files in os.walk(path):
        if root == path:
            res = len(dirs) - number
            if res > 0:
                for index in range(0, res):
                    shutil.rmtree(root + '/' + dirs[index])
        else:
            continue


def restrict_path_folder_size(path, size, syslog='none'):
    """
    :param path: The folder you want to restrict its size
    :param size: The size you want in bytes
    :param syslog: helps the check error
    :return:
    """
    current_size = get_backups_dir_info(path)[1]
    if current_size > size:
        try:
            folder = os.listdir(path)[0]
        except IndexError:
            if syslog:
                syslog.error('restrict path folder size but no folder exist in path' + path)
        else:
            try:
                first_file = os.listdir(path + '\\' + folder)[0]
            except IndexError:
                shutil.rmtree(path + '\\' + folder)
            else:
                os.remove(path + '\\' + folder + '\\' + first_file)
            restrict_path_folder_size(path, size)
