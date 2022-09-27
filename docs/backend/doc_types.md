# data_types.rs

## 功能

声明结构体



## 结构体

### DropPathInfo

拖放文件/文件夹信息

```rust
pub path: String,          // 媒体路径
pub meta_type: String,     // 媒体类型 文件/文件夹
pub name: String,          // 文件名称
pub save_path: String      // 保存目录
```



### SavePathInfo

保存目录统计信息，包括备份数量和所占空间大小

```rust
pub count: Vec<f64>,       // 备份数量
pub size: Vec<f64>         // 备份所占空间大小
```



### Setting

程序设置相关参数

```rust
pub is_auto_start: bool,            // 是否开机自启动
pub is_light_theme: bool,           // 是否为浅色模式
pub is_password_protected: bool,    // 是否启用锁屏密码
pub password: String,               // 锁屏密码
pub is_close_to_tray: bool,         // 是否关闭至托盘
pub language: String,               // 选择语言，目前支持中/英 
pub monitor_delay: u16,             // 设置监控延时时间
// is_webdav_enable: bool,
// is_webdav_available: bool,
// webdav_host_address: String,
// webdav_username: String,
// webdav_password: String,
// is_samba_enable: bool,
// is_samba_available: bool,
// samba_host_address: String,
// samba_uername: String,
// samba_password: String,
// is_ftp_enable: bool,
// is_ftp_available: bool,
// ftp_host_address: String,
// ftp_username: String,
// ftp_password: String,
// pub is_update_available: bool,
pub software_version: String,        // 软件版本
```



### MissonConfig

任务相关设置

```rust
pub id: String,                      // 任务唯一 id，由 uuid 生成
pub name: String,					 // 任务名称
pub target_type: String,			 // 目标类型 文件/文件夹
pub from_path: String,				 // 目标源路径
pub to_path: String,				 // 目标保存路径
pub ignore_enable: bool,			 // 是否启用忽略
pub ignore_method: String,			 // 忽略策略
pub ignores: Vec<String>,			 // 自定义忽略内容
pub compress_enable: bool,			 // 是否启用压缩
pub compress_format: String,		 // 压缩格式
pub cron_enable: bool,				 // 是否启用 cron 定时
pub cron_expression: String,		 // cron 表达式
pub monitor_enable: bool,			 // 是否启用监控
pub restrict_save_days_enable: bool, // 是否限制保存时长，以天为单位
pub save_days: u64,					 // 设置保存时长
pub restrict_save_size_enable: bool, // 是否限制保存空间大小，以 MB 为单位
pub save_size: u64,					 // 设置保存空间大小
pub backup_method: String,           // 备份策略，目前仅支持全量
```



### MissionInfo

任务相关信息

```rust
pub status: String,					 // 任务状态
pub next_run_time: String,			 // 任务下次执行时间
pub last_trigger_time: String		 // 任务上次执行时间
```



### Mission

任务结构体

``` rust
pub config: MissionConfig,			 // 任务设置
pub info: MissionInfo,				 // 任务信息
```



### APPData

app 数据结构体

``` rust
pub setting: Setting,				 // 软件设置
pub list: Vec<Mission>				 // 任务列表
```



### Response

返回结构体

```rust
pub code: i32						 // 返回 code
pub data: T							 // 返回数据
pub msg: String						 // 返回信息
```

