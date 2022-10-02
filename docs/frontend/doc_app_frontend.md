# App

## 功能

根组件，负责路由界面的展示及后端事件监听



## 函数

### initialize_data

- props
- return

初始化程序数据

### initialize_program_status

- props
- return

初始化程序状态

### initialize_mission

- props
- return

初始化任务

### initialize_system_tray

- props
- return

初始化系统托盘，与软件设置同步

### initialize_page_theme

- props
- return

初始化软件主题，与软件设置同步



### initialize_timing_save_data

- props
- return

定期保存当前数据

### build_router_transitionname

- props
  - name: any
- return
  - string

转换路由动态过渡动画名，TS 情况下必需

### listen_to_any_error

- props
- return

监听后端任意错误事件

### listen_to_close_event

- props
- return

监听软件关闭事件

### listen_to_drop_event

- props
- return

监听拖放事件

### listen_to_system_tray_event

- props
- return

监听托盘事件

### listen_to_mission_status_update

- props
- return

监听任务状态更新事件

### listen_to_mission_info_update

- props
- return

监听任务信息更新事件

### listen_to_cron_mission_run_time_update

- props
- return

监听 cron 任务执行事件更新事件



### listen_to_timing_save_data

- props
- return

监听定时备份数据事件

