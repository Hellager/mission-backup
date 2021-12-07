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
from utils.scheduler import *


test_scheduler = Scheduler()
test_scheduler.start()
test_scheduler.load_encrypted_data_from_file('test_config.txt')

while True:
    try:
        time.sleep(5)
    except KeyboardInterrupt:
        print("手动中止")
        test_scheduler.save_encrypted_data_to_file('test_config.txt')
        exit(0)
