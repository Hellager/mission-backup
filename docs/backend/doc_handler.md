# handler.rs

## 功能

负责声明相关 handler 结构体，用以完成对任务的监控/定时等操作及开机自启动功能的管理



## 函数

### initialize_cron_scheduler

- params
- return 
  - tokio_cron_scheduler::JobScheduler

初始化 cron 任务处理器



### MissionHandler

任务处理器结构体

``` rust
pub list: Vec<Mission>,                                   // 任务列表
pub setting: Setting,									  // app 设置
pub is_initialized: bool,								  // 是否初始化
pub cron_handler: JobScheduler,                           // cron 处理器
pub monitor_handler: Hotwatch,							  // 监控处理器
pub app_handler: AppHandle,								  // app 处理器
pub window_visiable: bool,								  // 窗口是否可见
pub cron_job_hashmap: HashMap<String, Uuid>,			  // cron 任务哈希表 任务名称-cron 任务id
pub working_mission_list: HashMap<String, bool>,		  // 活动中任务哈希表 任务名称-是否活动中
pub dir_increment: HashMap<String, Vec<String>>			  // 文件夹增量内容，暂为使用
```



### add_job_to_handler

- params
  - &mut self
  - config: data_types::MissionConfig
- return
  - Response\<Mission>

添加任务至处理器



### remove_job_from_handler

- params
  - &mut self
  - id: String
- return
  - Response\<bool>

从处理器删除任务



### update_mission_status

- params
  - status: &str
  - config: &data_types::MissionConfig
  - app_handle: &tauri::AppHandle
- return

更新任务状态，是否活动中/暂停



### update_mission_info

- params
  - config: &data_types::MissionConfig
  - app_handle: &tauri::AppHandle
- return

更新任务信息，包括状态和执行时间相关



### create_backups

- params
  - config: &data_types::MissionConfig
- return
  - bool

根据任务设置创建备份



### restrict_save_path

- params
  - config: &data_types::MissionConfig
- return

限制保存目录相关内容，包括天数和空间大小



### start_timing_save_data

- params
  - self
- return

开始定期保存数据



### AutoStartHandler

开机自启动结构体

``` rust
pub handler: AutoLaunch
```



### enable_auto_start

- params
  - &self
- return
  - bool

启用开机自启动



### disable_auto_start

- params
  - &self
- return 
  - bool

禁用开机自启动

