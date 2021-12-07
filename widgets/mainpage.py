#!/usr/bin/env python
# -*- coding: utf-8 -*-
from PyQt5.QtWidgets import QDialog, QAbstractItemView, QTableView, QHeaderView, \
    QTableWidgetItem, QMenu, QAction, QMessageBox
from PyQt5.QtGui import QIcon, QCursor, QPixmap, QMouseEvent
from PyQt5.QtCore import Qt, QFile
from widgets.ui.maindialog import Ui_Dialog
from widgets.settingpage import SettingPage
from widgets.closedialog import CloseDialog
from widgets.remotedialog import RemoteDialog
from widgets.documentdialog import DocumentDialog
from widgets.trayicon import TrayIcon
from utils.scheduler import Scheduler
import re
from qrc.imgsrc import *


class MainPage(QDialog):
    def __init__(self, name="page"):
        super().__init__()
        # init parameter
        self.preset = "Dark"
        self.is_max = False
        self.data_qss = None
        self.drag_move = False
        self.drag_pos = None
        self.app_geometry = None
        self.last_geometry = None
        self.schedule_list = []

        # init main page ui
        self.ui = Ui_Dialog()
        self.ui.setupUi(self)
        self.tray_icon = TrayIcon(self)
        self.setSizeGripEnabled(True)
        self.load_qss(":/style/qss/black.css")
        self.ui.label_name.setText(name)
        self.setWindowFlags(Qt.FramelessWindowHint | Qt.WindowSystemMenuHint | Qt.WindowMinMaxButtonsHint)
        self.setting_page = SettingPage()
        self.close_dialog = CloseDialog()
        self.remote_dialog = RemoteDialog()
        self.document_dialog = DocumentDialog()
        self.setting_page.add_check_existing_available(self.check_among_existing_schedule_available)
        self.msg_box = QMessageBox()
        self.msg_box.setWindowTitle('发生错误!')
        self.msg_box.setIcon(QMessageBox.Critical)
        self.msg_box.setStandardButtons(QMessageBox.Yes)

        # init schedule table
        self.ui.schedule_table.setColumnCount(7)
        self.ui.schedule_table.setHorizontalHeaderLabels(['计划编号', '计划名', '状态', '源目录', '保存目录', '备份数', '备份大小'])
        self.ui.schedule_table.verticalHeader().setHidden(True)
        self.ui.schedule_table.setEditTriggers(QTableView.NoEditTriggers)
        self.ui.schedule_table.horizontalHeader().setSectionResizeMode(QHeaderView.Stretch)
        self.ui.schedule_table.setSelectionBehavior(QAbstractItemView.SelectRows)
        self.ui.schedule_table.setAlternatingRowColors(True)
        self.ui.schedule_table.resizeRowsToContents()
        self.ui.schedule_table.resizeColumnsToContents()
        self.ui.schedule_table.setContextMenuPolicy(Qt.CustomContextMenu)
        self.ui.schedule_table.customContextMenuRequested.connect(self.show_context_menu)

        # init preset menu
        self.set_preset_menu()
        self.ui.button_preset.triggered.connect(self.manage_preset)

        # init signal and slot
        self.ui.button_min.clicked.connect(self.slot_on_button_min_clicked)
        self.ui.button_max.clicked.connect(self.slot_on_button_max_clicked)
        self.ui.button_close.clicked.connect(self.slot_on_button_close_clicked)
        self.ui.schedule_table.doubleClicked.connect(self.slot_on_table_double_clicked)
        self.setting_page.signal_complete_config.connect(self.slot_on_complete_schedule_config)
        self.remote_dialog.signal_transfer_remote_config.connect(self.slot_on_receive_remote_config)

        # init scheduler
        self.scheduler = Scheduler()
        self.scheduler.start()
        self.scheduler.add_on_sync_widget_data_callback(self.manage_schedule_data)
        self.scheduler.add_on_sync_widget_remote_callback(self.manage_remote_config)
        self.scheduler.add_on_sync_widget_error_callback(self.slot_on_error_occurs)
        self.scheduler.load_encrypted_data_from_file('config.txt')

    def load_qss(self, path):
        qss_file = QFile(path)
        qss_file.open(QFile.ReadOnly)
        self.data_qss = qss_file.readAll().data().decode()
        qss_file.close()
        self.setStyleSheet(self.data_qss)
        icon_prefix = None
        if "black" in path:
            self.preset = "Dark"
            icon_prefix = "white"
        elif "silvery" in path:
            self.preset = "Light"
            icon_prefix = "dark"
        self.ui.label_icon.setPixmap(QPixmap(f":/img/ico/{icon_prefix}_arrived.ico"))
        self.ui.button_preset.setIcon(QIcon(QPixmap(f":/img/ico/{icon_prefix}_arrow_down.ico")))
        self.ui.button_min.setIcon(QIcon(QPixmap(f":/img/ico/{icon_prefix}_minus.ico")))
        self.ui.button_max.setIcon(QIcon(QPixmap(f":/img/ico/{icon_prefix}_max.ico")))
        self.ui.button_close.setIcon(QIcon(QPixmap(f":/img/ico/{icon_prefix}_close.ico")))

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

    def get_desktop_geometry(self, app):
        self.app_geometry = app.desktop().availableGeometry()

    def slot_on_button_min_clicked(self):
        self.showMinimized()

    def slot_on_button_max_clicked(self):
        icon_prefix = "dark" if self.preset is "Light" else "white"
        if self.is_max is True:
            self.setGeometry(self.last_geometry)
            self.ui.button_max.setIcon(QIcon(QPixmap(f":/img/ico/{icon_prefix}_max.ico")))
            self.is_max = False
        else:
            self.last_geometry = self.geometry()
            self.setGeometry(self.app_geometry)
            self.ui.button_max.setIcon(QIcon(QPixmap(f":/img/ico/{icon_prefix}_restore.ico")))
            self.is_max = True

    def slot_on_button_close_clicked(self):
        self.close_dialog.set_preset(self.preset)
        close_dialog_pos = self.pos()
        close_dialog_pos.setX(self.pos().x() + self.width() * 0.35)
        close_dialog_pos.setY(self.pos().y() + self.height() * 0.35)
        self.close_dialog.move(close_dialog_pos)
        res = self.close_dialog.exec()
        if res is 4:
            self.scheduler.save_encrypted_data_to_file('config.txt')
            self.accept()
        elif res is 3:
            self.hide()
            self.tray_icon.show()

    def set_preset_menu(self):
        preset_menu = QMenu()
        preset_menu.setStyleSheet(self.data_qss)
        icon_prefix = "white" if self.preset is "Dark" else "dark"
        preset_menu.addAction(QAction(QIcon(f":/img/ico/{icon_prefix}_day_time.ico"), "浅色场景", self))
        preset_menu.addAction(QAction(QIcon(f":/img/ico/{icon_prefix}_night_time.ico"), "深色场景", self))
        self.ui.button_preset.setMenu(preset_menu)

    def manage_preset(self, action):
        if action.text() == "浅色场景":
            self.load_qss(":/style/qss/silvery.css")
            self.set_preset_menu()
        elif action.text() == "深色场景":
            self.load_qss(":/style/qss/black.css")
            self.set_preset_menu()

    def show_context_menu(self):
        context_menu = QMenu()
        context_menu.setStyleSheet(self.data_qss)
        icon_prefix = "white" if self.preset is "Dark" else "dark"
        context_menu.addAction(QAction(QIcon(f":/img/ico/{icon_prefix}_add.ico"), "添加", self))
        context_menu.addAction(QAction(QIcon(f":/img/ico/{icon_prefix}_switch.ico"), "编辑", self))
        context_menu.addAction(QAction(QIcon(f":/img/ico/{icon_prefix}_minus.ico"), "删除", self))
        context_menu.addAction(QAction(QIcon(f":/img/ico/{icon_prefix}_connection.ico"), "远程", self))
        context_menu.addAction(QAction(QIcon(f":/img/ico/{icon_prefix}_book.ico"), "说明", self))
        context_menu.triggered.connect(self.manage_schedule)
        context_menu.exec_(QCursor.pos())

    def slot_on_receive_remote_config(self, config):
        if config['is_any_webdav_service'] is True:
            self.scheduler.add_webdav_handler(config)
            self.setting_page.ui.checkBox_webdav.setEnabled(True)
            self.setting_page.has_webdav_handler = True
        else:
            self.scheduler.webdav = None
            self.setting_page.has_webdav_handler = False
            self.setting_page.ui.checkBox_webdav.setEnabled(False)
        if config['is_any_samba_service'] is True:
            self.scheduler.add_samba_handler(config)
            self.setting_page.ui.checkBox_samba.setEnabled(True)
            self.setting_page.has_samba_handler = True
        else:
            self.scheduler.samba = None
            self.setting_page.has_samba_handler = False
            self.setting_page.ui.checkBox_samba.setEnabled(False)

    def manage_schedule(self, action):
        if len(self.ui.schedule_table.selectedItems()) is not 0:
            self.setting_page.get_desktop_geometry(self.app_geometry)
            self.setting_page.set_preset(self.preset)
            setting_page_pos = self.pos()
            setting_page_pos.setX(self.pos().x() + self.height() * 0.2)
            setting_page_pos.setY(self.pos().y() + self.height() * 0.1)
            self.setting_page.move(setting_page_pos)
            if action.text() == "编辑":
                self.setting_page.set_handle_type('Edit')
                target = self.schedule_list[self.ui.schedule_table.currentRow()]
                self.setting_page.setting_id = target.id
                self.setting_page.load_schedule_setting(target)
                self.setting_page.exec()
            elif action.text() == "删除":
                target = self.schedule_list[self.ui.schedule_table.currentRow()]
                self.scheduler.delete_job(target.id)
                self.ui.schedule_table.removeRow(self.ui.schedule_table.currentRow())
                # pass
        if action.text() == "添加":
            self.setting_page.get_desktop_geometry(self.app_geometry)
            self.setting_page.set_preset(self.preset)
            setting_page_pos = self.pos()
            setting_page_pos.setX(self.pos().x() + self.height() * 0.2)
            setting_page_pos.setY(self.pos().y() + self.height() * 0.1)
            self.setting_page.move(setting_page_pos)
            self.setting_page.set_handle_type('Create')
            self.setting_page.exec()
        elif action.text() == "远程":
            self.remote_dialog.get_desktop_geometry(self.app_geometry)
            self.remote_dialog.set_preset(self.preset)
            remote_dialog_pos = self.pos()
            remote_dialog_pos.setX(self.pos().x() + self.height() * 0.025)
            remote_dialog_pos.setY(self.pos().y() + self.height() * 0.3)
            self.remote_dialog.move(remote_dialog_pos)
            self.remote_dialog.exec()
        elif action.text() == '说明':
            self.document_dialog.get_desktop_geometry(self.app_geometry)
            self.document_dialog.set_preset(self.preset)
            document_dialog_pos = self.pos()
            document_dialog_pos.setX(self.pos().x() + self.height() * 0.2)
            document_dialog_pos.setY(self.pos().y() + self.height() * 0.1)
            self.document_dialog.move(document_dialog_pos)
            self.document_dialog.exec_()
            print('打开说明文档')

    def slot_on_complete_schedule_config(self, config):
        if self.setting_page.mode == 'Create':
            self.scheduler.create_job(config)
        elif self.setting_page.mode == 'Edit':
            self.scheduler.edit_job(self.setting_page.setting_id, config)

    def check_among_existing_schedule_available(self, config, mode):
        for value in self.schedule_list:
            if mode != 'Create':
                if value.id == config['id']:
                    continue
            if value.name == config['name']:
                return False
            if config['src_path'] in value.src_path:
                return False
        return True

    def slot_on_table_double_clicked(self):
        self.setting_page.get_desktop_geometry(self.app_geometry)
        self.setting_page.set_preset(self.preset)
        setting_page_pos = self.pos()
        setting_page_pos.setX(self.pos().x() + self.height() * 0.2)
        setting_page_pos.setY(self.pos().y() + self.height() * 0.1)
        self.setting_page.move(setting_page_pos)
        self.setting_page.set_handle_type('Edit')
        target = self.schedule_list[self.ui.schedule_table.currentRow()]
        self.setting_page.setting_id = target.id
        self.setting_page.load_schedule_setting(target)
        self.setting_page.exec()

    def manage_schedule_data(self, data, mode):
        target_index = None
        if mode == 'Create':
            self.schedule_list.append(data)
            self.ui.schedule_table.insertRow(self.ui.schedule_table.rowCount())
            target_index = self.ui.schedule_table.rowCount() - 1
        elif (mode == 'Edit') or (mode == 'Execute'):
            for index, value in enumerate(self.schedule_list):
                if value.id == data.id:
                    target_index = index
                    self.schedule_list[index] = data
                    break

        item_number = QTableWidgetItem(str(target_index + 1))
        item_number.setTextAlignment(Qt.AlignHCenter | Qt.AlignVCenter)
        item_name = QTableWidgetItem(data.name)
        item_state_text = '计时中' if data.is_time_job == 'True' else '监控中'
        item_state = QTableWidgetItem(item_state_text)
        item_src_path = QTableWidgetItem(data.src_path)
        item_dst_path = QTableWidgetItem(data.dst_path)
        item_backups_count = QTableWidgetItem(data.backups_count)
        row_backups_size = data.backups_size.split(' ')
        res_backups_size = ''
        for value in row_backups_size:
            size = re.findall(r'\d+', value)[0]
            if data.size_restriction_unit == 'MB' or data.size_restriction_unit == 'False':
                size = str(round(int(size) / 1024 / 1024, 2)) + ' MB'
            elif data.size_restriction_unit == 'GB':
                size = str(round(int(size) / 1024 / 1024 / 1024, 2)) + ' MB'
            res_backups_size += str(value)[0:2] + ' ' + size + '  '
        item_backups_size = QTableWidgetItem(res_backups_size)
        item_list = [item_number, item_name, item_state, item_src_path, item_dst_path,
                     item_backups_count, item_backups_size]
        # add tooltip
        item_name.setToolTip(data.name)
        if data.is_time_job == 'True':
            item_state.setToolTip('下次执行时间为' + data.next_run_time)
        elif data.is_monitor_job == 'True':
            item_state.setToolTip('最近修改时间为' + data.last_trigger_time)
        item_src_path.setToolTip(data.src_path)
        item_dst_path.setToolTip(data.dst_path)
        item_backups_count.setToolTip(data.backups_count.replace('L', '本地').replace('W', 'Webdav').replace('S', 'Samba'))
        item_backups_size.setToolTip(res_backups_size.replace('L', '本地').replace('W', 'Webdav').replace('S', 'Samba'))
        for index, value in enumerate(item_list):
            self.ui.schedule_table.setItem(target_index, index, value)

    def manage_remote_config(self, config):
        self.remote_dialog.load_config_data(config)
        if config['is_any_webdav_service'] == 'True':
            self.setting_page.has_webdav_handler = True
            self.setting_page.ui.checkBox_webdav.setEnabled(True)
        else:
            self.setting_page.has_webdav_handler = False
            self.setting_page.ui.checkBox_webdav.setEnabled(False)
        if config['is_any_samba_service'] == 'True':
            self.setting_page.has_samba_handler = True
            self.setting_page.ui.checkBox_samba.setEnabled(True)
        else:
            self.setting_page.has_samba_handler = False
            self.setting_page.ui.checkBox_samba.setEnabled(False)

    def slot_on_error_occurs(self, code, mode, job, remote):
        if mode == 'Create':
            self.msg_box.setText(f"创建任务 {job.name} 失败")
            self.msg_box.exec_()
        if mode == 'Load':
            if code == 0x01:
                self.msg_box.setText(f"导入远程配置失败")
                self.msg_box.exec_()
            elif code == 0x02:
                self.msg_box.setText(f"导入任务配置失败")
                self.msg_box.exec_()
        if mode == 'Execute':
            self.msg_box.setText(f"执行 {job.name} 出错 代码:{bin(code)}")
            self.msg_box.exec_()
        if mode == 'Remote':
            if code == 0x01:
                self.msg_box.setText(f"获取 {job.name} Webdav文件信息失败")
                self.msg_box.exec_()
            if code == 0x02:
                self.msg_box.setText(f"添加Webdav失败")
                self.msg_box.exec_()
                self.remote_dialog.ui.checkBox_webdav.setChecked(False)
                self.scheduler.webdav = None
                self.setting_page.has_webdav_handler = False
                self.setting_page.ui.checkBox_webdav.setEnabled(False)
            if code == 0x04:
                self.msg_box.setText(f"获取 {job.name} Samba文件信息失败")
                self.msg_box.exec_()
            if code == 0x8:
                self.msg_box.setText(f"添加Samba失败")
                self.msg_box.exec_()
                self.remote_dialog.ui.checkBox_samba.setChecked(False)
                self.scheduler.samba = None
                self.setting_page.has_samba_handler = False
                self.setting_page.ui.checkBox_samba.setEnabled(False)
