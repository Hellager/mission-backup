#!/usr/bin/env python
# -*- coding: utf-8 -*-

from PyQt5.QtWidgets import QWidget, QFileDialog, QMessageBox
from PyQt5.QtCore import pyqtSignal
from PyQt5.QtGui import QCloseEvent
from widgets.settingpage import *
import jsonpath
import re


class MySettingPage(QWidget):
    # init signal
    signal_page_close = pyqtSignal()
    signal_create_new_schedule = pyqtSignal(dict)

    def __init__(self, main_window, name="SettingPage", syslog='none'):
        super().__init__()
        self.log = syslog
        self.type = 'create'
        # init ui
        self.parent_window = main_window
        self.ui = Ui_SettingPage()
        self.ui.setupUi(self)
        self.setWindowTitle(name)

        self.ui.label_RestrictTime.hide()
        self.ui.comboBox_RestrictTime.hide()
        self.ui.label_RestrictSize.hide()
        self.ui.lineEdit_RestrictSize.hide()
        self.ui.comboBox_RestrictSizeUnit.hide()
        self.ui.lineEdit_SaveDir.setEnabled(False)
        self.msg_box = QMessageBox(QMessageBox.Warning, '警告', '请完整填写相关参数！')

        # init signal and slot connect
        self.signal_page_close.connect(self.parent_window.slot_on_setting_page_close)
        self.signal_create_new_schedule.connect(self.parent_window.slot_on_new_schedule_create)
        self.ui.checkBox_RestrictSaveTime.clicked.connect(self.slot_on_checkbox_restrict_save_time_clicked)
        self.ui.checkBox_RestrictSaveDirSize.clicked.connect(self.slot_on_checkbox_restrict_save_dir_size_clicked)
        self.ui.comboBox_TriggerType.currentIndexChanged['int'].connect(self.slot_on_combobox_trigger_type_index_changed)
        self.ui.Button_ChooseSourceDir.clicked.connect(self.slot_on_button_choose_source_dir_clicked)
        self.ui.Button_ChooseSaveDir.clicked.connect(self.slot_on_button_choose_save_dir_clicked)
        self.ui.Button_Confirm.clicked.connect(self.slot_on_button_confirm_clicked)
        self.ui.Button_Cancel.clicked.connect(self.slot_on_button_cancel_clicked)

    def close_clear(self):
        self.ui.lineEdit_ScheduleName.clear()
        self.ui.lineEdit_SourceDir.clear()
        self.ui.lineEdit_SaveDir.clear()
        self.ui.comboBox_CompressType.setCurrentIndex(0)
        self.ui.comboBox_TriggerType.setCurrentIndex(0)
        self.ui.spinBox_InterveralTime.setValue(30)
        self.ui.comboBox_InterveralTimeUnit.setCurrentIndex(0)
        self.ui.checkBox_RestrictSaveTime.clearMask()
        self.ui.checkBox_RestrictSaveDirSize.clearMask()
        self.ui.lineEdit_RestrictSize.setText('1024')
        self.ui.comboBox_RestrictTime.setCurrentIndex(0)
        self.ui.comboBox_RestrictSizeUnit.setCurrentIndex(0)

    def closeEvent(self, a0: QCloseEvent) -> None:
        self.signal_page_close.emit()
        a0.accept()

    def slot_on_checkbox_restrict_save_time_clicked(self):
        if self.ui.checkBox_RestrictSaveTime.isChecked():
            self.ui.label_RestrictTime.show()
            self.ui.comboBox_RestrictTime.show()
        else:
            self.ui.label_RestrictTime.hide()
            self.ui.comboBox_RestrictTime.hide()

    def slot_on_checkbox_restrict_save_dir_size_clicked(self):
        if self.ui.checkBox_RestrictSaveDirSize.isChecked():
            self.ui.label_RestrictSize.show()
            self.ui.lineEdit_RestrictSize.show()
            self.ui.comboBox_RestrictSizeUnit.show()
        else:
            self.ui.label_RestrictSize.hide()
            self.ui.lineEdit_RestrictSize.hide()
            self.ui.comboBox_RestrictSizeUnit.hide()

    def slot_on_combobox_trigger_type_index_changed(self):
        if not self.ui.comboBox_TriggerType.currentIndex():
            self.ui.label_InterveralTime.show()
            self.ui.spinBox_InterveralTime.show()
            self.ui.comboBox_InterveralTimeUnit.show()
        else:
            self.ui.label_InterveralTime.hide()
            self.ui.spinBox_InterveralTime.hide()
            self.ui.comboBox_InterveralTimeUnit.hide()

    def slot_on_button_choose_source_dir_clicked(self):
        source_dir = QFileDialog.getExistingDirectory(self, "请选择备份目录", "/home",
                                                      QFileDialog.ShowDirsOnly | QFileDialog.DontResolveSymlinks)
        self.ui.lineEdit_SourceDir.setText(source_dir)
        self.ui.lineEdit_SaveDir.setText(source_dir+"/Backups")

    def slot_on_button_choose_save_dir_clicked(self):
        self.ui.lineEdit_SaveDir.setEnabled(True)
        save_dir = QFileDialog.getExistingDirectory(self, "请选择备份目录", self.ui.lineEdit_SourceDir.text(),
                                                    QFileDialog.ShowDirsOnly | QFileDialog.DontResolveSymlinks)
        self.ui.lineEdit_SaveDir.setText(save_dir+"/Backups")

    def slot_on_button_confirm_clicked(self):
        new_schedule = dict()
        dict.update({
            'name': '',
            'src path': '',
            'dst path': '',
            'compress format': '',
            'trigger type': '',
            'time interval': '',
            'time restrict': '',
            'size restrict': ''
        })
        new_schedule['name'] = self.ui.lineEdit_ScheduleName.text()
        new_schedule['src path'] = self.ui.lineEdit_SourceDir.text()
        new_schedule['dst path'] = self.ui.lineEdit_SaveDir.text()
        new_schedule['compress format'] = self.ui.comboBox_CompressType.currentText()

        if self.ui.comboBox_TriggerType.currentIndex():
            new_schedule['trigger type'] = 'edit trigger'
            new_schedule['time interval'] = 'NONE'
        else:
            new_schedule['trigger type'] = 'time trigger'
            new_schedule['time interval'] = str(self.ui.spinBox_InterveralTime.value()) + \
                                            [' minute', ' hour'][self.ui.comboBox_InterveralTimeUnit.currentIndex()]

        new_schedule['time restrict'] = (self.ui.comboBox_RestrictTime.currentText()[0] + ' day') \
            if self.ui.checkBox_RestrictSaveTime.isChecked() else 'NONE'
        new_schedule['size restrict'] = (self.ui.lineEdit_RestrictSize.text() + ' ' +
                                         self.ui.comboBox_RestrictSizeUnit.currentText()) \
            if self.ui.checkBox_RestrictSaveDirSize.isChecked() else 'NONE'

        if new_schedule['name'] == '' or new_schedule['src path'] == '' or new_schedule['dst path'] == ''\
                or (self.ui.spinBox_InterveralTime.value() == 0 and self.ui.comboBox_TriggerType.currentIndex() == 0) \
                or (int(self.ui.lineEdit_RestrictSize.text()) <= 0 and (self.ui.checkBox_RestrictSaveDirSize.isChecked())):
            self.msg_box.exec_()
            return

        if self.check_schedule(new_schedule):
            if self.type == 'create':
                self.signal_create_new_schedule.emit(new_schedule)
            elif self.type == 'edit':
                self.parent_window.slot_on_schedule_edit(new_schedule)
            self.signal_page_close.emit()
        else:
            self.msg_box.setText('重名或已存在子目录！')
            self.msg_box.exec_()
            return

    def slot_on_button_cancel_clicked(self):
        self.signal_page_close.emit()

    def slot_on_edit_schedule(self, index):
        self.ui.lineEdit_ScheduleName.setText(self.parent_window.schedule_list[index]['name'])
        self.ui.lineEdit_SourceDir.setText(self.parent_window.schedule_list[index]['src path'])
        self.ui.lineEdit_SaveDir.setText(self.parent_window.schedule_list[index]['dst path'])
        self.ui.comboBox_CompressType.setCurrentIndex(['zip', '7z', 'tar.gz', 'tar.bz2', 'tar.xz'].
                                                      index(self.parent_window.schedule_list[index]['compress format']))

        if self.parent_window.schedule_list[index]['trigger type'] == 'time trigger':
            self.ui.comboBox_TriggerType.setCurrentIndex(0)
        else:
            self.ui.comboBox_TriggerType.setCurrentIndex(1)
        if not self.parent_window.schedule_list[index]['time interval'] == 'NONE':
            value = int(re.findall("\\d+", self.parent_window.schedule_list[index]['time interval'])[0])
            self.ui.spinBox_InterveralTime.setValue(value)
        else:
            self.ui.spinBox_InterveralTime.setValue(0)

        if self.parent_window.schedule_list[index]['time restrict'] == 'NONE':
            self.ui.checkBox_RestrictSaveTime.setChecked(False)
        else:
            self.ui.checkBox_RestrictSaveTime.setChecked(True)
        if self.parent_window.schedule_list[index]['size restrict'] == 'NONE':
            self.ui.checkBox_RestrictSaveDirSize.setChecked(False)
        else:
            self.ui.checkBox_RestrictSaveDirSize.setChecked(True)

        if self.parent_window.schedule_list[index]['time interval'] == 'NONE':
            self.ui.comboBox_InterveralTimeUnit.setCurrentIndex(0)
        else:
            pos = ['e', 'r'].index(self.parent_window.schedule_list[index]['time interval'][-1:])
            self.ui.comboBox_InterveralTimeUnit.setCurrentIndex(pos)
        if self.parent_window.schedule_list[index]['time restrict'] == 'NONE':
            self.ui.comboBox_RestrictTime.setCurrentIndex(0)
        else:
            pos = [1, 3, 5, 7].index(int(re.findall("\\d+", self.parent_window.schedule_list[index]['time restrict'])[0]))
            self.ui.comboBox_RestrictTime.setCurrentIndex(pos)

        if self.parent_window.schedule_list[index]['size restrict'] == 'NONE':
            self.ui.lineEdit_RestrictSize.setText('0')
        else:
            value = re.findall("\\d+", self.parent_window.schedule_list[index]['size restrict'])[0]
            self.ui.lineEdit_RestrictSize.setText(value)
        if self.parent_window.schedule_list[index]['size restrict'] == 'NONE':
            self.ui.comboBox_RestrictSizeUnit.setCurrentIndex(0)
        else:
            pos = ['MB', 'GB'].index(self.parent_window.schedule_list[index]['size restrict'][-2:])
            self.ui.comboBox_RestrictSizeUnit.setCurrentIndex(pos)

        self.slot_on_checkbox_restrict_save_time_clicked()
        self.slot_on_checkbox_restrict_save_dir_size_clicked()

    def check_schedule(self, config):
        stat = True
        if not len(self.parent_window.schedule_list):
            return stat
        list_name = jsonpath.jsonpath(self.parent_window.schedule_list, '$..name')
        list_src_path = jsonpath.jsonpath(self.parent_window.schedule_list, '$..src path')
        list_dst_path = jsonpath.jsonpath(self.parent_window.schedule_list, '$..dst path')
        if self.type == 'edit':
            list_name.pop(self.parent_window.ui.Table_Schedule.currentIndex().row())
            list_src_path.pop(self.parent_window.ui.Table_Schedule.currentIndex().row())
            list_dst_path.pop(self.parent_window.ui.Table_Schedule.currentIndex().row())
        name_res = False if config['name'] in list_name else True
        src_res = False if any(config['src path'] in path for path in list_src_path) else True
        # dst_res = False if any(config['dst_path'] in path for path in list_dst_path) else True
        stat = name_res & src_res
        return stat
