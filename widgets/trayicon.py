#!/usr/bin/env python
# -*- coding: utf-8 -*-

from PyQt5.QtWidgets import QSystemTrayIcon, QMenu, QAction
from PyQt5.QtGui import QIcon, QPixmap


class TrayIcon(QSystemTrayIcon):
    def __init__(self, window, parent=None):
        super().__init__()
        self.ui = window
        self.context_menu = None
        self.set_context_menu()
        self.setIcon(QIcon(QPixmap(":/img/ico/white_arrived.ico")))
        self.activated.connect(self.slot_on_icon_clicked)

    def set_context_menu(self):
        self.context_menu = QMenu()
        self.context_menu.addAction(QAction("显示", self))
        self.context_menu.addAction(QAction("退出", self))
        self.setContextMenu(self.context_menu)
        self.context_menu.triggered.connect(self.slot_on_menu_triggered)

    def slot_on_menu_triggered(self, action):
        if action.text() == "显示":
            self.ui.showNormal()
            self.ui.activateWindow()
            self.ui.show()
        elif action.text() == "退出":
            self.ui.accept()

    def slot_on_icon_clicked(self, reason):
        if reason == 2:
            if self.ui.isMinimized() or not self.ui.isVisible():
                self.ui.showNormal()
                self.ui.activateWindow()
                self.ui.show()
            else:
                self.ui.showMinimized()
                self.ui.show()
