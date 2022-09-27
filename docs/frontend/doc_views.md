# Views

## GlobalSetting

软件设置界面

### 函数

#### show_error_message

- props
- return

显示错误信息

#### toggle_dialog_cancel

- props
- return

处理对话框取消事件

#### toggle_change_auto_start

- porps
  - data: boolean
- return
- type: async

设置软件开机自启动

#### toggle_change_theme

- props
  - data: boolean
- return
- type: async

设置软件主题

#### toggle_change_language

- props
  - data: string
- return
- type: async

设置软件语言

#### toggle_change_close_event

- props
  - data: boolean
- return
- type: async

设置软件是否最小化至托盘或关闭

#### toggle_change_password_protect

- props
  - data: boolean
- return
- type: async

设置软件是否锁屏密码保护

#### toggle_change_password

- props
- return
- type: async

重设锁屏密码

#### toggle_change_monitor_delay

- props
- return
- type: async

设置软件监控延时



## MissionConfig

任务的新建及修改

### 函数

#### load_current_mission

- props
  - data: Mission
- return

加载最近选择任务数据

#### load_drop_mission

- props
  - path: string
- return
- type: async

加载拖放文件/文件夹数据

#### cancel

- props
- return

取消添加/修改任务

#### select_path

- props
  - target: string
  - from: string
- return
- type: async

选择文件/文件夹

#### toggle_is_cron

- props
  - formEl: FormInstance | undefined
- return

选择是否为 cron 定时任务

#### toggle_is_monitor

- props
- return

选择是否为监控任务



#### toggle_is_save_days_restrict

- props
- return

选择是否限制保存天数

#### toggle_is_save_size_restrict

- props
- return

选择是否限制保存空间大小

#### validate_cron

- props
  - rule: any
  - value: any
  - callback: any
- return

校验 cron 表达式

#### handle_change_save_days

- props
  - currentValue: number | typeof NaN
  - oldValue: number | typeof NaN
- return

修改保存时长

#### handle_change_save_size

- props
  - currentValue: number | typeof NaN
  - oldValue: number | typeof NaN
- return

修改保存空间大小

#### submit_form

- props
  - formEl: FormInstance | undefined
- return
- type: async

提交任务表单数据

#### check_exists_before_submit

- props
- return
- type: async

检验任务是否已存在与任务列表

#### check_path_valid

- props
- return
- type: async

检验路径是否有效

#### check_valid

- props
- return
- type: async

校验路径有效和任务是否已存在

#### submit_to_backend

- props
- return
- type: async

提交任务设置至后端

## StatisticPage

任务统计界面

### 函数

#### form_x_axis

- props
  - from: number
  - to: number
  - step: number
- return
  - Array\<number> | Array\<string>

生成 x 轴坐标

#### form_x_axis_with_datetime

- props
  - from: string
  - length: number
- return
  - Array\<string>

生成 x 轴坐标，以日期形式

#### change_date_picker_format

- props
  - val: any
- return

修改时间选择单位

#### change_current_mission

- props
  - val: any
- return

修改当前选择任务

#### select_date

- props
  - val: any
- return

选择时间

#### update_chart_option

- props
  - xAxis: number[] | string []
  - count: number []
  - size: number []
- return

更新图标数据





## Table

通过表格形式展示任务列表



### 函数

#### indexMethod

- props
  - index: number
- return
  - number

生成序列号



#### toggle_row_click

- props
  - row: any
- return

处理表格行单击事件，单击选择当前行对应任务

#### toggle_row_db_click

- props
  - row: any
- return

处理表格双击事件，双击跳转至任务编辑界面

#### toggle_edit_click

- props
- return

处理编辑按钮点击事件，点击编辑对应任务

#### toggle_stop_click

- props
- return

处理停止按钮点击事件，点击停止对应任务

#### toggle_delete_dlick

- props
- return

处理删除按钮点击事件，点击停止并删除对应任务