#!/usr/bin/env python
# -*- coding: utf-8 -*-

from PyQt5.QtWidgets import QDialog
from PyQt5.QtCore import Qt, QFile
from widgets.ui.closewidget import *


class CloseDialog(QDialog):
    def __init__(self):
        super().__init__()
        self.preset = "Dark"
        self.data_qss = None

        # init close
        self.ui = Ui_Dialog()
        self.ui.setupUi(self)
        self.setWindowFlags(Qt.FramelessWindowHint | Qt.WindowSystemMenuHint | Qt.WindowMinMaxButtonsHint)
        self.ui.button_mini.click()

        # init signal and slot
        self.ui.button_confirm.clicked.connect(self.slot_on_button_confirm_clicked)
        self.ui.button_cancel.clicked.connect(self.slot_on_button_cancel_clicked)

    def set_preset(self, mode):
        if mode is "Dark":
            self.load_qss(":/style/qss/black.css")
        elif mode is "Light":
            self.load_qss(":/style/qss/silvery.css")

    def load_qss(self, path):
        qss_file = QFile(path)
        qss_file.open(QFile.ReadOnly)
        self.data_qss = qss_file.readAll().data().decode()
        qss_file.close()
        self.setStyleSheet(self.data_qss)
        if "black" in path:
            self.preset = "Dark"
        elif "silvery" in path:
            self.preset = "Light"

    def slot_on_button_confirm_clicked(self):
        if self.ui.button_mini.isChecked() is True:
            self.done(3)
        else:
            self.done(4)

    def slot_on_button_cancel_clicked(self):
        self.reject()
