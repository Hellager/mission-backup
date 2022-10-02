# command.rs

## 功能

tauri 前后端通信相关 command，包括改变软件设置及创建/删除计划等



## 函数

### check_path_valid

- params
  - path: &str
  - expected: &str
- return
  - bool

根据参数校验路径有效性，是否存在及是否符合 expected 类型



### close_splashscreen

- params
  - window: tauri::Window
- return

关闭加载界面



### get_drop_path_info

- params
  - path: &str
- return
  - Response\<DropPathInfo>

获取拖放的文件/文件夹路径信息



### is_program_initialized

- params
  - state_handler: State\<MissionHandlerWrapper>
- return
  - bool

检查程序是否初始化



### initialize_program_status

- params
  - state_handler: State\<MissionHandlerWrapper>
- return
  - bool

设置初始化状态



### initialize_data

- params
  - state_handler: State\<MissionHandlerWrapper>
- return
  - APPData

初始化 app 数据



### start_timing_save_data

- params
  - state_handler: State\<MissionHandlerWrapper>
- return

开始定期保存数据

###  timing_save_data

- params
  - state_handler: State\<MissionHandlerWrapper>
- return

执行定期保存数据

### update_list_info

- params
  - list: Vec\<Mission>
  - state_handler: State\<MissionHandlerWrapper>
- return

更新任务列表信息



### exit_program

- params
  - state_handler: State\<MissionHandlerWrapper>
- return

按照自定义流程退出程序



### close_to_tray

- params
  - app_handle: tauri::AppHandle
- return

最小化至托盘

### open_specific_folder_window

- params
  - path: &str
- return

打开路径所在窗口

### change_setting_is_auto_start

- params
  - auto_start: bool
  - state_handler: State\<MissionHandlerWrapper>
  - start_handler: State\<AutoStartHandlerWrapper>
- bool

更改开机自启动设置

### change_setting_is_light_theme

- params
  - is_light: bool
  - state_handler: State\<MissionHandlerWrapper>
- return
  - bool

更改场景设置，true 为浅色， false 为深色

### change_setting_is_close_to_tray

- params
  - close_to_tray: bool
  - state_handler: State\<MissionHandlerWrapper>
- return
  - bool

更改关闭窗口时的处理，退出程序或最小化至托盘

### change_window_state_by_system_tray

- params
  - state_handler: State\<MissionHandlerWrapper>
  - app_handle: tauri::AppHandle
- return
  - bool

由托盘触发的改变窗口状态

### change_setting_language

- params
  - lang: &str
  - state_handler: State\<MissionHandlerWrapper>
  - app_handle: tauri::AppHandle
- return
  - bool

更改程序当前语言

### change_setting_is_password_protect

- params
  - password_protect: bool
  - state_handler: State\<MissionHandlerWrapper>
- return
  - bool

更改程序锁屏密码保护是否开启

### change_setting_password

- params
  - old_password: &str
  - new_password: &str
  - state_handler: State\<MissionHandlerWrapper>
- return
  - bool

更改设置的锁屏密码

### change_monitor_delay

- params
  - delay: u16
  - state_handler: State\<MissionHandlerWrapper>
- return
  - bool

更改监控延时，避免频繁触发

### unlock

- params
  - password: &str
  - state_handler: State\<MissionHandlerWrapper>

- return
  - bool

解锁屏幕

### create_mission

- params
  - config: MissionConfig
  - state_handler: State\<MissionHandlerWrapper>
- return
  - Response\<Mission>

创建任务

### start_mission

- params
  - id: String
  - state_handler: State\<MissionHandlerWrapper>
- return
  - Response\<bool>

根据 id 启动任务

### stop_mission

- params
  - id: String
  - state_handler: State\<MissionHandlerWrapper>
- return
  - Response\<bool>

根据 id 停止任务

### edit_mission

- params
  - id: String
  - config: MissionConfig
  - state_handler: State\<MissionHandlerWrapper>
- return
  - Response\<Mission>

根据 id 和 config 修改原任务

### delete_mission

- params
  - id: String
  - state_handler: State\<MissionHandlerWrapper>
- return
  - Response\<bool>

根据 id 删除任务

### force_delete_mission

- params
  - id: String
  - state_handler: State\<MissionHandlerWrapper>
- return
  - Response\<bool>

根据 id 强制删除任务，仅删除列表中内容，防止 handler 中已删除但列表中仍存在

### get_mission_backups_status

- params
  - id: String
  - date_type: String
  - start_datetime: String
  - size_unit: String
  - state_handler: State\<MissionHandlerWrapper>
- return
  - Response\<SavePathInfo>

根据 id 和日期类型等参数获取当前任务备份状态，包括备份数和所占空间大小