#!/usr/bin/env python
# -*- coding: utf-8 -*-
"""
Time:     2021/10/27 17:24
Author:   Gu Jun jie
Version:  V 1.0.0
File:     main.py
Describe: an automatic backup software , Github link: https://github.com/Hellager/Backup-Schedule
"""
from PyQt5.QtWidgets import QApplication
from widgets.mymainwindow import *
import sys
import logging


if __name__ == '__main__':
    if not os.path.exists(os.getcwd()+'\\log'):
        os.mkdir('log')
    log_file = open('log\\syslog.txt', 'w')
    log_file.close()

    LOG_FORMAT = "%(asctime)s - %(levelname)s - %(message)s"
    DATE_FORMAT = "%Y/%m/%d %H:%M:%S"
    file_handler = logging.FileHandler('log/syslog.txt', 'a')
    stream_handler = logging.StreamHandler()
    logging.basicConfig(level=logging.INFO,
                        format=LOG_FORMAT, datefmt=DATE_FORMAT,
                        handlers=[file_handler, stream_handler])

    app = QApplication(sys.argv)
    mainWindow = MyMainWindow("备份计划", logging)
    mainWindow.show()
    sys.exit(app.exec_())
