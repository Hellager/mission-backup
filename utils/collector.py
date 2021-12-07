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
    Get folder file numbers and folder size
    :param path: The folder path
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
                  which will only count those folders were named as "xx'xx"(x represents numbers)
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


def get_path_level_sub_folders(path):
    """
    Get path sub folder's numbers
    :param path: The folder path
    :return: nums: The sub folders' numbers
    """
    for root, dirs, files in os.walk(path):
        if root == path:
            res = len(dirs)
            return res


def restrict_path_folder_numbers(path, number):
    """
    Directly delete folder in path until under target number
    :param path: The folder path
    :param number: The sub folders' number you want
    :return: None
    """
    for root, dirs, files in os.walk(path):
        if root == path:
            res = len(dirs) - int(number)
            if res > 0:
                for index in range(0, res):
                    shutil.rmtree(root + '/' + dirs[index])
        else:
            continue


def restrict_path_folder_size(path, size, syslog='none'):
    """
    Directly delete files sorted by default in path until reach target size
    :param path: The folder path
    :param size: The target size(bytes)
    :param syslog: helps log error
    :return: None
    """
    current_size = get_backups_dir_info(path)[1]
    if current_size > int(size):
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
            restrict_path_folder_size(path, int(size))
