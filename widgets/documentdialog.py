#!/usr/bin/env python
# -*- coding: utf-8 -*-

from PyQt5.QtWidgets import QDialog, QFrame
from PyQt5.QtGui import QPalette, QColor, QMouseEvent
from PyQt5.QtCore import Qt, QFile
from widgets.ui.documentdialog import *


class DocumentDialog(QDialog):
    def __init__(self):
        super().__init__()
        self.preset = "Dark"
        self.data_qss = None
        self.drag_move = False
        self.drag_pos = None
        self.app_geometry = None

        # init ui
        self.ui = Ui_Dialog()
        self.ui.setupUi(self)
        self.setWindowFlags(Qt.FramelessWindowHint | Qt.WindowSystemMenuHint | Qt.WindowMinMaxButtonsHint)
        self.ui.textEdit.setFrameShape(QFrame.NoFrame)

        # init signal and slot
        self.ui.button_Close.clicked.connect(self.slot_on_button_close_clicked)

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
        palette = QPalette()
        if "black" in path:
            self.preset = "Dark"
            palette.setColor(QPalette.Background, QColor("#484848"))
        elif "silvery" in path:
            self.preset = "Light"
            palette.setColor(QPalette.Background, QColor("#F5F5F5"))
        self.setPalette(palette)
        self.ui.textEdit.setFrameShape(QFrame.NoFrame)

    def mousePressEvent(self, a0: QMouseEvent):
        if a0.button() == Qt.LeftButton:
            self.drag_move = True
            self.drag_pos = a0.globalPos() - self.pos()
            a0.accept()

    def mouseMoveEvent(self, a0: QMouseEvent):
        if self.drag_move is True:
            self.move(a0.globalPos() - self.drag_pos)
            a0.accept()

    def mouseReleaseEvent(self, a0: QMouseEvent):
        self.drag_move = False

    def get_desktop_geometry(self, geometry):
        self.app_geometry = geometry

    def slot_on_button_close_clicked(self):
        self.accept()
