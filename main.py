#!/usr/bin/env python
# -*- coding: utf-8 -*-
"""
Time:     2021/10/27 17:24
Author:   Gu Jun jie
Version:  V 1.0.1
File:     main.py
Describe: an automatic backup software , Github link: https://github.com/Hellager/Backup-Schedule
"""
from PyQt5.QtWidgets import QApplication
from widgets.mainpage import *
import sys


if __name__ == '__main__':
    app = QApplication(sys.argv)
    mainWindow = MainPage("备份计划")
    mainWindow.show()
    mainWindow.get_desktop_geometry(app)
    sys.exit(app.exec_())
