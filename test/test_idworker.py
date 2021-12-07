#!/usr/bin/env python
# -*- coding: utf-8 -*-
"""
test_idworker.py

The unit test for idworker.py

D:\\Demonstration
----\\folder 1
------\\word.doc
----\\folder 2
------\\excel.xls
----\\folder 3
------\\picture.bmp
----\book.txt
"""
from utils.idworker import *

test_worker = IdWorker()
print(test_worker.get_id())
print(test_worker.get_id())
