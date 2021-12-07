#!/usr/bin/env python
# -*- coding: utf-8 -*-
"""
test_monitor.py

The unit test for monitor.py

D:\\Demonstration
----\\folder 1
------\\word.doc
----\\folder 2
------\\excel.xls
----\\folder 3
------\\picture.bmp
----\book.txt
"""
from utils.monitor import *

target_dir = 'D:\\Demonstration'
save_dir = 'D:\\Demonstration\\Backups'

test_monitor = DirMonitor()
test_monitor.add_job('123456', target_dir)

while True:
    time.sleep(5)
