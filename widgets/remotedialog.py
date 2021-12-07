#!/usr/bin/env python
# -*- coding: utf-8 -*-
from PyQt5.QtWidgets import QDialog, QMessageBox
from PyQt5.QtCore import pyqtSignal
from PyQt5.QtGui import QPalette, QMouseEvent
from PyQt5.QtCore import Qt, QFile
from widgets.ui.remotedialog import *


class RemoteDialog(QDialog):
    # init signal
    signal_transfer_remote_config = pyqtSignal(dict)

    def __init__(self):
        super().__init__()
        self.preset = "Dark"
        self.data_qss = None
        self.parent_page = None
        self.drag_move = False
        self.drag_pos = None
        self.app_geometry = None

        # init ui
        self.ui = Ui_Dialog()
        self.ui.setupUi(self)
        self.setSizeGripEnabled(True)
        self.setWindowFlags(Qt.FramelessWindowHint | Qt.WindowSystemMenuHint | Qt.WindowMinMaxButtonsHint)
        self.msg_box = QMessageBox()
        self.msg_box.setWindowTitle('配置错误')
        self.msg_box.setIcon(QMessageBox.Warning)
        self.msg_box.setStandardButtons(QMessageBox.Yes | QMessageBox.No)

        # init widget
        self.ui.checkBox_webdav.clicked.connect(self.update_remote)
        self.ui.checkBox_samba.clicked.connect(self.update_remote)
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
        palette = QPalette()
        if "black" in path:
            self.preset = "Dark"
        elif "silvery" in path:
            self.preset = "Light"
        self.setPalette(palette)

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

    def is_webdav_enabled(self, state):
        if state is True:
            self.ui.lineEdit_webdav_host_config.setEnabled(True)
            self.ui.lineEdit_webdav_username.setEnabled(True)
            self.ui.lineEdit_webdav_password.setEnabled(True)
        else:
            self.ui.lineEdit_webdav_host_config.setDisabled(True)
            self.ui.lineEdit_webdav_username.setDisabled(True)
            self.ui.lineEdit_webdav_password.setDisabled(True)

    def is_samba_enabled(self, state):
        if state is True:
            self.ui.lineEdit_samba_host_config.setEnabled(True)
            self.ui.lineEdit_samba_username.setEnabled(True)
            self.ui.lineEdit_samba_password.setEnabled(True)
        else:
            self.ui.lineEdit_samba_host_config.setDisabled(True)
            self.ui.lineEdit_samba_username.setDisabled(True)
            self.ui.lineEdit_samba_password.setDisabled(True)

    def clear_webdav_config(self):
        self.ui.lineEdit_webdav_host_config.clear()
        self.ui.lineEdit_webdav_username.clear()
        self.ui.lineEdit_webdav_password.clear()

    def clear_samba_config(self):
        self.ui.lineEdit_samba_host_config.clear()
        self.ui.lineEdit_samba_username.clear()
        self.ui.lineEdit_samba_password.clear()

    def update_remote(self):
        self.is_webdav_enabled(self.ui.checkBox_webdav.isChecked())
        self.is_samba_enabled(self.ui.checkBox_samba.isChecked())

    def slot_on_button_confirm_clicked(self):
        res = dict()
        res.update({
            'is_any_webdav_service': False,
            'is_any_samba_service': False,
        })
        res_webdav_config_allowed = True
        res_samba_config_allowed = True
        if self.ui.checkBox_webdav.isChecked() is True:
            res_webdav_config_allowed = self.check_webdav_config_available()
            if res_webdav_config_allowed:
                res['is_any_webdav_service'] = True
                webdav_host_config = self.ui.lineEdit_webdav_host_config.text().split(';')
                res.update({
                    'webdav_host_address': webdav_host_config[0],
                    'webdav_root_path': webdav_host_config[1],
                    'webdav_username': self.ui.lineEdit_webdav_username.text(),
                    'webdav_password': self.ui.lineEdit_webdav_password.text()
                })
            else:
                return
        if self.ui.checkBox_samba.isChecked() is True:
            res_samba_config_allowed = self.check_samba_config_available()
            if res_samba_config_allowed:
                res['is_any_samba_service'] = True
                samba_host_config = self.ui.lineEdit_samba_host_config.text().split(';')
                res.update({
                    'samba_host_address': samba_host_config[0],
                    'samba_host_port': samba_host_config[1],
                    'samba_host_name': samba_host_config[2],
                    'samba_root_path': samba_host_config[3],
                    'samba_username': self.ui.lineEdit_samba_username.text(),
                    'samba_password': self.ui.lineEdit_samba_password.text()
                    })
            else:
                return
        res_remote_config_allowed = (not self.ui.checkBox_webdav.isChecked()) or res_webdav_config_allowed or \
                                    (not self.ui.checkBox_samba.isChecked() or res_samba_config_allowed)
        if res_remote_config_allowed:
            self.signal_transfer_remote_config.emit(res)
        else:
            return
        self.accept()

    def slot_on_button_cancel_clicked(self):
        if self.ui.checkBox_webdav.isChecked():
            if self.check_webdav_config_available() is False:
                self.ui.checkBox_webdav.setChecked(False)
        if self.ui.checkBox_samba.isChecked():
            if self.check_samba_config_available() is False:
                self.ui.checkBox_samba.setChecked(False)
        self.reject()

    def load_config_data(self, data):
        if data['is_any_webdav_service'] == 'True':
            self.is_webdav_enabled(True)
            self.ui.checkBox_webdav.setChecked(True)
            webdav_config = data['webdav_host_address'] + ';' + data['webdav_root_path']
            self.ui.lineEdit_webdav_host_config.setText(webdav_config)
            self.ui.lineEdit_webdav_username.setText(data['webdav_username'])
            self.ui.lineEdit_webdav_password.setText(data['webdav_password'])
        else:
            self.clear_webdav_config()
            self.is_webdav_enabled(False)

        if data['is_any_samba_service'] == 'True':
            self.is_samba_enabled(True)
            self.ui.checkBox_samba.setChecked(True)
            samba_config = data['samba_host_address'] + ';' + data['samba_host_port'] + ';' + \
                data['samba_host_name'] + ';' + data['samba_root_path']
            self.ui.lineEdit_samba_host_config.setText(samba_config)
            self.ui.lineEdit_samba_username.setText(data['samba_username'])
            self.ui.lineEdit_samba_password.setText(data['samba_password'])
        else:
            self.clear_samba_config()
            self.is_samba_enabled(False)

    def check_webdav_config_available(self):
        if self.ui.checkBox_webdav.isChecked() is True:
            res = bool(self.ui.lineEdit_webdav_username.text()) and \
                  bool(self.ui.lineEdit_webdav_password.text()) and \
                  not bool(len(self.ui.lineEdit_webdav_host_config.text().split(';')) < 2)
            if not res:
                self.msg_box.setText('webdav配置不能有空或host config缺少参数！')
                self.msg_box.exec_()
            return res
        else:
            return True

    def check_samba_config_available(self):
        if self.ui.checkBox_samba.isChecked() is True:
            res = bool(self.ui.lineEdit_samba_username.text()) and \
                  bool(self.ui.lineEdit_samba_password.text()) and \
                  not bool(len(self.ui.lineEdit_samba_host_config.text().split(';')) < 4)
            if not res:
                self.msg_box.setText('samba配置不能有空或host config缺少参数！！')
                self.msg_box.exec_()
            return res
        else:
            return True
