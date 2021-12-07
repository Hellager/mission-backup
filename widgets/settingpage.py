#!/usr/bin/env python
# -*- coding: utf-8 -*-
from PyQt5.QtWidgets import QDialog, QFileDialog, QMessageBox
from PyQt5.QtCore import pyqtSignal
from PyQt5.QtGui import QIcon, QPixmap, QMouseEvent
from PyQt5.QtCore import Qt, QFile
from widgets.ui.settingdialog import Ui_Dialog


class SettingPage(QDialog):
    # init signal
    signal_complete_config = pyqtSignal(dict)

    def __init__(self, name="setting"):
        super().__init__()
        # init parameter
        self.mode = "Create"
        self.preset = "Dark"
        self.is_max = False
        self.data_qss = None
        self.drag_move = False
        self.drag_pos = None
        self.app_geometry = None
        self.last_geometry = None
        self.check_existing_available = None
        self.has_samba_handler = None
        self.has_webdav_handler = None
        self.setting_id = None

        # init setting page ui
        self.ui = Ui_Dialog()
        self.ui.setupUi(self)
        self.setSizeGripEnabled(True)
        self.load_qss(":/style/qss/black.css")
        self.ui.label_name.setText(name)
        self.setWindowFlags(Qt.FramelessWindowHint | Qt.WindowSystemMenuHint | Qt.WindowMinMaxButtonsHint)
        self.msg_box = QMessageBox()
        self.msg_box.setWindowTitle('配置错误')
        self.msg_box.setIcon(QMessageBox.Warning)
        self.msg_box.setStandardButtons(QMessageBox.Yes)

        # init signal and slot connection
        self.ui.button_min.clicked.connect(self.slot_on_button_min_clicked)
        self.ui.button_close.clicked.connect(self.slot_on_button_close_clicked)
        self.ui.Button_Confirm.clicked.connect(self.slot_on_button_confirm_clicked)
        self.ui.Button_Cancel.clicked.connect(self.slot_on_button_close_clicked)
        self.ui.Button_ChooseSourceDir.clicked.connect(self.slot_on_button_choose_source_dir_clicked)
        self.ui.Button_ChooseSaveDir.clicked.connect(self.slot_on_button_choose_save_dir_clicked)
        self.ui.checkBox_hasPassword.clicked.connect(self.slot_on_checkbox_has_password_clicked)
        self.ui.radioButton_time_trigger.clicked.connect(self.slot_on_radiobutton_time_trigger_clicked)
        self.ui.radioButton_edit_trigger.clicked.connect(self.slot_on_radiobutton_edit_trigger_clicked)
        self.ui.checkBox_RestrictSaveTime.clicked.connect(self.slot_on_checkbox_restrict_save_time_clicked)
        self.ui.checkBox_RestrictSaveDirSize.clicked.connect(self.slot_on_checkbox_restrict_save_dir_size_clicked)
        self.ui.comboBox_CompressFormat.currentIndexChanged.connect(self.slot_on_combobox_compressformat_changed)

    def set_preset(self, mode):
        if mode is "Dark":
            self.load_qss(":/style/qss/black.css")
        elif mode is "Light":
            self.load_qss(":/style/qss/silvery.css")

    def set_handle_type(self, mode):
        self.mode = mode
        self.clear_page()
        if mode == 'Create':
            self.ui.label_name.setText("创建新计划")
        elif mode == 'Edit':
            self.ui.label_name.setText("编辑计划")

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
        self.ui.label_icon.setPixmap(QPixmap(f":/img/ico/{icon_prefix}_setting.ico"))
        self.ui.button_min.setIcon(QIcon(QPixmap(f":/img/ico/{icon_prefix}_minus.ico")))
        self.ui.button_close.setIcon(QIcon(QPixmap(f":/img/ico/{icon_prefix}_close.ico")))

    def clear_page(self):
        self.ui.lineEdit_ScheduleName.clear()
        self.ui.lineEdit_SourceDir.clear()
        self.ui.lineEdit_SaveDir.clear()
        self.ui.lineEdit_SaveDir.setDisabled(True)
        self.ui.comboBox_CompressFormat.setCurrentIndex(0)
        self.ui.checkBox_hasPassword.setChecked(False)
        self.ui.lineEdit_password.clear()
        self.ui.lineEdit_password.setDisabled(True)
        self.ui.radioButton_time_trigger.setChecked(True)
        self.ui.radioButton_edit_trigger.setChecked(False)
        self.ui.spinBox_InterveralTime.setValue(30)
        self.ui.comboBox_InterveralTimeUnit.setCurrentIndex(0)
        self.ui.checkBox_webdav.setChecked(False)
        self.ui.checkBox_samba.setChecked(False)
        self.ui.checkBox_RestrictSaveTime.setChecked(False)
        self.ui.checkBox_RestrictSaveDirSize.setChecked(False)
        self.ui.lineEdit_RestrictSize.setText('25')
        self.ui.lineEdit_RestrictSize.setDisabled(True)
        self.ui.lineEdit_RestrictDays.setDisabled(True)
        self.ui.comboBox_RestrictSizeUnit.setCurrentIndex(0)
        self.ui.comboBox_RestrictSizeUnit.setDisabled(True)

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

    def slot_on_button_min_clicked(self):
        self.showMinimized()

    def slot_on_checkbox_restrict_save_time_clicked(self):
        if self.ui.checkBox_RestrictSaveTime.isChecked() is True:
            self.ui.lineEdit_RestrictDays.setDisabled(False)
        else:
            self.ui.lineEdit_RestrictDays.setDisabled(True)

    def slot_on_checkbox_restrict_save_dir_size_clicked(self):
        if self.ui.checkBox_RestrictSaveDirSize.isChecked() is True:
            self.ui.lineEdit_RestrictSize.setDisabled(False)
            self.ui.comboBox_RestrictSizeUnit.setDisabled(False)
        else:
            self.ui.lineEdit_RestrictSize.setDisabled(True)
            self.ui.comboBox_RestrictSizeUnit.setDisabled(True)

    def slot_on_radiobutton_time_trigger_clicked(self):
        if self.ui.radioButton_time_trigger.isChecked() is True:
            self.ui.spinBox_InterveralTime.setDisabled(False)
            self.ui.comboBox_InterveralTimeUnit.setDisabled(False)
        else:
            self.ui.spinBox_InterveralTime.setDisabled(True)
            self.ui.comboBox_InterveralTimeUnit.setDisabled(True)

    def slot_on_radiobutton_edit_trigger_clicked(self):
        if self.ui.radioButton_edit_trigger.isChecked() is True:
            self.ui.spinBox_InterveralTime.setDisabled(True)
            self.ui.comboBox_InterveralTimeUnit.setDisabled(True)
        else:
            self.ui.spinBox_InterveralTime.setDisabled(False)
            self.ui.comboBox_InterveralTimeUnit.setDisabled(False)

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

    def slot_on_checkbox_has_password_clicked(self):
        if not ((self.ui.comboBox_CompressFormat.currentText() == 'zip') or
                (self.ui.comboBox_CompressFormat.currentText() == '7z')):
            self.ui.checkBox_hasPassword.setDisabled(True)
            self.ui.lineEdit_password.setDisabled(True)
        else:
            self.ui.checkBox_hasPassword.setEnabled(True)
            self.ui.lineEdit_password.setEnabled(True)
        if self.ui.checkBox_hasPassword.isChecked():
            self.ui.lineEdit_password.setEnabled(True)
        else:
            self.ui.lineEdit_password.setEnabled(False)

    def slot_on_combobox_compressformat_changed(self):
        if not ((self.ui.comboBox_CompressFormat.currentText() == 'zip') or
                (self.ui.comboBox_CompressFormat.currentText() == '7z')):
            self.ui.checkBox_hasPassword.setDisabled(True)
            self.ui.checkBox_hasPassword.setChecked(False)
            self.ui.lineEdit_password.setDisabled(True)
        else:
            self.ui.checkBox_hasPassword.setEnabled(True)

    def slot_on_button_confirm_clicked(self):
        res = dict()
        dict.update({
            'name': '',
            'src_path': '',
            'dst_path': '',
            'compress_format': '',
            'has_password': '',
            'password': '',
            'is_time_job': '',
            'time_interval': '',
            'time_interval_unit': '',
            'is_monitor_job': '',
            'is_days_restrict': '',
            'days_restriction': '',
            'is_size_restrict': '',
            'size_restriction': '',
            'size_restriction_unit': '',
            'is_webdav_sync': '',
            'is_samba_sync': '',
        })
        res['name'] = self.ui.lineEdit_ScheduleName.text()
        res['src_path'] = self.ui.lineEdit_SourceDir.text()
        res['dst_path'] = self.ui.lineEdit_SaveDir.text()
        res['compress_format'] = self.ui.comboBox_CompressFormat.currentText()
        res['has_password'] = str(self.ui.checkBox_hasPassword.isChecked())
        res['password'] = self.ui.lineEdit_password.text()
        res['is_time_job'] = str(self.ui.radioButton_time_trigger.isChecked())
        res['time_interval'] = self.ui.spinBox_InterveralTime.text()
        res['time_interval_unit'] = self.ui.comboBox_InterveralTimeUnit.currentText() \
            if res['is_time_job'] == 'True' else 'False'
        if res['time_interval_unit'] == '分钟':
            res['time_interval_unit'] = 'Minute'
        elif res['time_interval_unit'] == '小时':
            res['time_interval_unit'] = 'Hour'
        res['is_monitor_job'] = str(self.ui.radioButton_edit_trigger.isChecked())
        res['is_days_restrict'] = str(self.ui.checkBox_RestrictSaveTime.isChecked())
        res['days_restriction'] = self.ui.lineEdit_RestrictDays.text()
        res['is_size_restrict'] = str(self.ui.checkBox_RestrictSaveDirSize.isChecked())
        res['size_restriction'] = self.ui.lineEdit_RestrictSize.text()
        res['size_restriction_unit'] = self.ui.comboBox_RestrictSizeUnit.currentText()
        res['is_webdav_sync'] = 'True' if self.ui.checkBox_webdav.isChecked() else 'False'
        res['is_samba_sync'] = 'True' if self.ui.checkBox_samba.isChecked() else 'False'
        if self.mode == 'Edit':
            res.update({
                'id': self.setting_id
            })
        if self.check_current_available():
            if self.check_existing_available(res, self.mode):
                self.signal_complete_config.emit(res)
            else:
                self.msg_box.setText('与现存计划存在冲突！')
                self.msg_box.exec_()
                return
        else:
            return
        self.accept()

    def slot_on_button_close_clicked(self):
        self.accept()

    def load_schedule_setting(self, setting):
        self.setting_id = setting.id
        self.ui.lineEdit_ScheduleName.setText(setting.name)
        self.ui.lineEdit_SourceDir.setText(setting.src_path)
        self.ui.lineEdit_SaveDir.setText(setting.dst_path)
        type_index = ['zip', '7z', 'tar.gz', 'tar.bz2', 'tar.xz'].index(setting.compress_format)
        self.ui.comboBox_CompressFormat.setCurrentIndex(type_index)
        if setting.has_password == 'True':
            self.ui.checkBox_hasPassword.setChecked(True)
            self.ui.lineEdit_password.setText(setting.password)
        else:
            self.ui.checkBox_hasPassword.setChecked(False)
            self.slot_on_checkbox_has_password_clicked()
        if setting.is_time_job == 'True':
            self.ui.radioButton_time_trigger.setChecked(True)
            self.ui.spinBox_InterveralTime.setValue(int(setting.time_interval))
            self.ui.comboBox_InterveralTimeUnit.setCurrentIndex(['Minute', 'Hour'].index(setting.time_interval_unit))
            self.slot_on_radiobutton_time_trigger_clicked()
        else:
            self.ui.radioButton_edit_trigger.setChecked(True)
            self.slot_on_radiobutton_edit_trigger_clicked()
        if self.has_webdav_handler is True:
            self.ui.checkBox_webdav.setEnabled(True)
            if setting.is_webdav_sync == 'True':
                self.ui.checkBox_webdav.setChecked(True)
            else:
                self.ui.checkBox_webdav.setChecked(False)
        else:
            self.ui.checkBox_webdav.setEnabled(False)
            self.ui.checkBox_webdav.setChecked(False)
        if self.has_samba_handler is True:
            self.ui.checkBox_samba.setEnabled(True)
            if setting.is_samba_sync == 'True':
                self.ui.checkBox_samba.setChecked(True)
            else:
                self.ui.checkBox_samba.setChecked(False)
        else:
            self.ui.checkBox_samba.setEnabled(False)
            self.ui.checkBox_samba.setChecked(False)
        self.ui.lineEdit_RestrictDays.setText(str(setting.days_restriction))
        if setting.is_days_restrict == 'True':
            self.ui.lineEdit_RestrictDays.setEnabled(True)
            self.ui.checkBox_RestrictSaveTime.setChecked(True)
        else:
            self.ui.checkBox_RestrictSaveTime.setChecked(False)
            self.ui.lineEdit_RestrictDays.setEnabled(False)
        self.ui.lineEdit_RestrictSize.setText(str(setting.size_restriction))
        if setting.is_size_restrict == 'True':
            self.ui.checkBox_RestrictSaveDirSize.setChecked(True)
            self.ui.lineEdit_RestrictSize.setEnabled(True)
        else:
            self.ui.checkBox_RestrictSaveDirSize.setChecked(False)
            self.ui.lineEdit_RestrictSize.setEnabled(False)

    def check_current_available(self):
        is_available = True
        if (self.ui.lineEdit_ScheduleName.text() == '') or \
            (self.ui.lineEdit_SourceDir.text() == '') or \
                (self.ui.lineEdit_SaveDir.text() == ''):
            is_available = False
            self.msg_box.setText('计划名称/源目录/保存目录 不能有空!')
            self.msg_box.exec_()
        # check encryption available
        if self.ui.checkBox_hasPassword.isChecked() is True:
            if self.ui.comboBox_CompressFormat.currentIndex() >= 2:
                is_available = False
                self.msg_box.setText('当前压缩格式不支持加密!')
                self.msg_box.exec_()
            if self.ui.lineEdit_password.text() == '':
                is_available = False
                self.msg_box.setText('密码不能为空!')
                self.msg_box.exec_()
        # check webdav or samba available
        if self.has_webdav_handler is False:
            if self.ui.checkBox_webdav.isChecked():
                is_available = False
                self.msg_box.setText('Webdav服务未启用!')
                self.msg_box.exec_()
        if self.has_samba_handler is False:
            if self.ui.checkBox_samba.isChecked():
                is_available = False
                self.msg_box.setText('Samba服务未启用!')
                self.msg_box.exec_()
        return is_available

    def add_check_existing_available(self, handler):
        self.check_existing_available = handler
