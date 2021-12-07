#!/usr/bin/env python
# -*- coding: utf-8 -*-
"""
test_collector.py

The unit test for collector.py

D:\\Demonstration
----\\folder 1
------\\word.doc
----\\folder 2
------\\excel.xls
----\\folder 3
------\\picture.bmp
----\book.txt
"""

from utils.collector import *

test_dir = 'D:\\Demonstration'

dir_info = get_dir_info(test_dir)
print(f'path {test_dir} \n'
      f'total file numbers {dir_info[0]} \n'
      f'total file size {dir_info[1]} byte \n')

dir_backups_info = get_backups_dir_info(test_dir)
print(f'path {test_dir} \n'
      f'total backups file numbers {dir_backups_info[0]} \n'
      f'total backups file size {dir_backups_info[1]} byte \n')

"""
WARNING!!! This function will directly delete those sub folders which were out of range !!!
Please check your path before testing for making sure that there should not be any important sub folders!!!
"""
origin_numbers = get_path_level_sub_folders(test_dir)
# restrict_path_folder_numbers(test_dir, origin_numbers)
restrict_number = get_path_level_sub_folders(test_dir)
print(f'path {test_dir} \n'
      f'origin folder numbers {origin_numbers} \n'
      f'after restrict numbers {restrict_number} \n')

"""
WARNING!!! This function will directly delete any files or folders sorted by default until reach the setting size!!!
Please check your path before testing for making sure that there should not be any important files or folders!!!
"""
origin_size = dir_info[1]
# restrict_path_folder_size(test_dir, origin_size)
after_size = get_dir_info(test_dir)[1]
print(f'path {test_dir} \n'
      f'origin folder size {origin_size} \n'
      f'after restrict size {after_size} \n')


