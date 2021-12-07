#!/usr/bin/env python
# -*- coding: utf-8 -*-
"""
test_compressor.py

The unit test for compressor.py

D:\\Demonstration
----\\folder 1
------\\word.doc
----\\folder 2
------\\excel.xls
----\\folder 3
------\\picture.bmp
----\book.txt
"""
from utils.compressor import *

target_dir = 'D:\\Demonstration'
save_dir = 'D:\\Demonstration\\Backups'
test_compressor = Compressor()
test_compressor.compress_folder(target_dir, save_dir, 'zip', '123456')
