#!/usr/bin/env python
# -*- coding: utf-8 -*-

from PyQt5.QtWidgets import QMainWindow, QAbstractItemView, QTableView, QHeaderView, QTableWidgetItem
from PyQt5.QtGui import QIcon, QPixmap
from widgets.mainwindow import *
from widgets.mysettingpage import *
from utils.runner import *
from utils.collector import *
import json
import os


class MyMainWindow(QMainWindow):
    def __init__(self, name="MainWindow", syslog='none'):
        super().__init__()
        self.log = syslog

        # load exist schedule config file
        if os.path.exists('schedule.json'):
            with open(os.getcwd() + '\\schedule.json', 'r') as f:
                if not os.path.getsize(os.getcwd() + '\\schedule.json'):
                    self.schedule_list = []
                else:
                    self.schedule_list = json.load(f)
        else:
            self.schedule_list = []
            self.config_file = open(os.getcwd() + '\\schedule.json', 'w')
            self.config_file.close()

        # init ui
        self.ui = Ui_MainWindow()
        self.ui.setupUi(self)
        self.setWindowTitle(name)
        icon = QIcon()
        icon.addPixmap(QPixmap('img\\Syncronize-Folder.png'))
        self.setWindowIcon(icon)
        self.settingpage = MySettingPage(self, "设置界面")
        self.msg_box = QMessageBox(QMessageBox.Warning, '警告', '达到定时计划数量限制上限！')

        # init signal and slot connect
        self.ui.Button_Create.clicked.connect(self.slot_on_button_create_clicked)
        self.ui.Button_Edit.clicked.connect(self.slot_on_button_edit_clicked)
        self.ui.Button_Delete.clicked.connect(self.slot_on_button_delete_clicked)
        self.ui.Table_Schedule.doubleClicked.connect(self.slot_on_button_edit_clicked)

        # table schedule init
        self.ui.Table_Schedule.setColumnCount(6)
        self.ui.Table_Schedule.setHorizontalHeaderLabels(['计划名', '状态', '源目录', '保存目录', '备份数', '备份大小'])
        self.ui.Table_Schedule.setEditTriggers(QTableView.NoEditTriggers)
        self.ui.Table_Schedule.horizontalHeader().setSectionResizeMode(QHeaderView.Stretch)
        self.ui.Table_Schedule.setSelectionBehavior(QAbstractItemView.SelectRows)
        self.ui.Table_Schedule.setAlternatingRowColors(True)
        self.ui.Table_Schedule.resizeRowsToContents()
        self.ui.Table_Schedule.resizeColumnsToContents()

        # load exist schedule config file
        if len(self.schedule_list):
            self.load_config_file()

        # create schedule runner and load jobs
        self.runner = ScheduleRunner(self.log, self.update_stat)
        self.runner.load_schedule_job(self.schedule_list)

    def closeEvent(self, a0: QCloseEvent) -> None:
        if not self.settingpage.isHidden():
            self.settingpage.close_clear()
            self.settingpage.close()
        a0.accept()

    def slot_on_button_create_clicked(self):
        job_time_interval_nums = 0
        for value in self.schedule_list:
            if value['trigger type'] == 'time trigger':
                job_time_interval_nums += 1
                if job_time_interval_nums >= 10:
                    self.msg_box.exec_()
                    return
        self.settingpage.setWindowTitle('新建计划')
        self.settingpage.type = 'create'
        self.settingpage.show()
        self.setEnabled(False)

    def slot_on_button_delete_clicked(self):
        if not len(self.schedule_list):
            return
        index = self.ui.Table_Schedule.currentIndex().row()
        self.delete_schedule(index)

    def slot_on_button_edit_clicked(self):
        if not len(self.schedule_list):
            return
        index = self.ui.Table_Schedule.currentIndex().row()
        self.settingpage.setWindowTitle('编辑计划')
        self.settingpage.type = 'edit'
        self.settingpage.show()
        self.settingpage.slot_on_edit_schedule(index)
        self.setEnabled(False)

    def slot_on_new_schedule_create(self, data):
        self.schedule_list.append(data)
        self.append_schedule(data)
        self.runner.create_job(data)
        with open(os.getcwd() + '\\schedule.json', 'w') as f:
            json.dump(self.schedule_list, f)

    def slot_on_schedule_edit(self, data):
        self.edit_schedule(self.ui.Table_Schedule.currentIndex().row(), data)
        self.schedule_list[self.ui.Table_Schedule.currentIndex().row()] = data
        with open(os.getcwd() + '\\schedule.json', 'w') as f:
            json.dump(self.schedule_list, f)

    def slot_on_setting_page_close(self):
        if not self.settingpage.isHidden():
            self.settingpage.close_clear()
            self.settingpage.close()
        self.setEnabled(True)

    def append_schedule(self, schedule_config):
        self.ui.Table_Schedule.insertRow(self.ui.Table_Schedule.rowCount())
        table_data = self.handle_config_data(schedule_config)
        for index, value in enumerate(table_data):
            item = QTableWidgetItem(table_data[index])
            self.ui.Table_Schedule.setItem(self.ui.Table_Schedule.rowCount()-1, index, item)
            self.ui.Table_Schedule.item(self.ui.Table_Schedule.rowCount()-1, index).setToolTip(table_data[index])

    def edit_schedule(self, row, schedule_config):
        table_data = self.handle_config_data(schedule_config)
        for index, value in enumerate(table_data):
            item = QTableWidgetItem(table_data[index])
            self.ui.Table_Schedule.setItem(row, index, item)
            self.ui.Table_Schedule.item(row, index).setToolTip(table_data[index])
        self.runner.edit_job(schedule_config)

    def delete_schedule(self, index):
        self.ui.Table_Schedule.removeRow(index)
        self.runner.delete_job(self.schedule_list[index])
        self.schedule_list.pop(index)
        with open(os.getcwd() + '\\schedule.json', 'w') as f:
            json.dump(self.schedule_list, f)

    @staticmethod
    def handle_config_data(data):
        schedule_name = data['name']
        schedule_stat = '计时中' if data['trigger type'] == 'time trigger' else '监控中'
        schedule_src_path = data['src path']
        schedule_dst_path = data['dst path']
        schedule_backups_info = get_backups_dir_info(data['dst path'])
        schedule_backups_number = str(schedule_backups_info[0])
        schedule_backups_size = ''
        if data['size restrict'] == 'NONE' or data['size restrict'][-2:] == 'MB':
            schedule_backups_size = str(round(schedule_backups_info[1] / 1024 / 1024, 2)) + ' MB'
        elif data['size restrict'][-2:] == 'GB':
            schedule_backups_size = str(round(schedule_backups_info[1] / 1024 / 1024 / 1024, 2)) + ' MB'
        return [schedule_name, schedule_stat, schedule_src_path,
                schedule_dst_path, schedule_backups_number, schedule_backups_size]

    def load_config_file(self):
        for index, value in enumerate(self.schedule_list):
            self.append_schedule(value)

    def update_stat(self, src_path, stat):
        row = 0
        trigger = ''
        for index, value in enumerate(self.schedule_list):
            if self.schedule_list[index]['src path'] == src_path:
                row = index
                trigger = '计时中' if self.schedule_list[index]['trigger type'] == 'time trigger' else '监控中'
                break

        if not stat:
            self.ui.Table_Schedule.setItem(row, 1, QTableWidgetItem(str('备份中')))
        else:
            self.ui.Table_Schedule.setItem(row, 1, QTableWidgetItem(trigger))

            if not self.schedule_list[row]['time restrict'] == 'NONE':
                days = int(re.findall("\\d+", self.schedule_list[row]['time restrict'])[0])
                restrict_path_folder_numbers(self.schedule_list[row]['dst path'], days)
            if not self.schedule_list[row]['size restrict'] == 'NONE':
                size = int(re.findall("\\d+", self.schedule_list[row]['size restrict'])[0])
                if self.schedule_list[row]['size restrict'][-2:] == 'MB':
                    size = size * 1024 * 1024
                elif self.schedule_list[row]['size restrict'][-2:] == 'GB':
                    size = size * 1024 * 1024 * 1024
                restrict_path_folder_size(self.schedule_list[row]['dst path'], size, self.log)

            info = get_backups_dir_info(self.schedule_list[row]['dst path'])
            schedule_backups_size = ''
            if self.schedule_list[row]['size restrict'] == 'NONE' \
                    or self.schedule_list[row]['size restrict'][-2:] == 'MB':
                schedule_backups_size = str(round(info[1] / 1024 / 1024, 2)) + ' MB'
            elif self.schedule_list[row]['size restrict'][-2:] == 'GB':
                schedule_backups_size = str(round(info[1] / 1024 / 1024 / 1024, 2)) + ' GB'
            self.ui.Table_Schedule.setItem(row, 4, QTableWidgetItem(str(info[0])))
            self.ui.Table_Schedule.setItem(row, 5, QTableWidgetItem(schedule_backups_size))
