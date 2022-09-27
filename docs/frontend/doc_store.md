# Store

## 功能

初始化前端状态管理，包括软件设置，任务列表及任务统计



## Stores

### Mission

- state

  - mission_list&emsp;&emsp;&emsp; &emsp;&emsp;&emsp;&emsp;&emsp;// 任务列表
  - current_mission &emsp;&emsp;&emsp;&emsp;&emsp;&emsp;// 最近选择任务
  - current_drop_mission_path&emsp;// 最近拖放路径

- getters

- actions

  - initialize_mission_list

    - props
      - list: Array\<Mission>

    初始化任务列表

  - update_current_mission

    - props
      - id: string

    更新最近选择任务

  - update_current_drop_mission_path

    - props
      - path: string

    更新最近拖放路径

  - append_new_mission

    - props
      - data: Mission

    添加新任务

  - edit_mission

    - props
      - id: string
      - new_config: Mission

    修改任务

  - start_mission

    - props
      - id: string

    开始任务

  - stop_mission

    - props
      - id: string

    停止任务

  - delete_mission

    - props
      - id: string

    删除任务

  - update_mission_status

    - props
      - id: string
      - status: string

    更新任务状态

  - update_mission_info

    - props
      - id: string
      - info: MissionInfo

    更新任务信息

  - get_mission_status

    - props
      - id: string

    获取任务状态



### Setting

- state

  - is_initialized&emsp; &emsp;&emsp;&emsp;&emsp;&emsp;  // 是否初始化
  - is_auto_start&emsp;&emsp;&emsp;&emsp;&emsp;&emsp;// 开机自启动设置
  - is_light_theme&emsp; &emsp;&emsp;&emsp;&emsp;// 软件主题设置
  - is_close_to_tray&emsp; &emsp; &emsp;&emsp;// 最小化至托盘设置
  - language&emsp;  &emsp;&emsp;&emsp;&emsp;&emsp;&emsp;// 软件语言设置
  - is_password_protected&emsp;// 锁屏密码保护设置
  - monitor_delay &emsp;&emsp;&emsp;&emsp;&emsp;// 监控延时设置
  - software_version&emsp;&emsp;&emsp;&emsp;// 软件版本

- getters

- actions

  - initialize_settings

    - props
      - data: Setting

    初始化软件设置

  - update_auto_start

    - props
      - data: boolean

    更新开机自启动设置

  - update_theme

    - props
      - data: boolean

    更新软件主题

  - update_language

    - props
      - data: string

    更新软件语言

  - update_close_to_tray

    - props
      - data: boolean

    更新是否最小化至托盘

  - update_password_protect

    - props
      - data: boolean

    更新是否开启锁屏密码保护

  - update_monitor_delay

    - props
      - id: number

    更新监控延时

  - update_version

    - props
      - data: string

    更新软件版本



### Statistic

