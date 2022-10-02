# app.rs

## 功能

负责 App 相关基本功能的实现及一些系统相关功能，如创建托盘，检查 webview 是否可用等



## 函数

### load_data_file

- params
- return
  - data_types::APPData

加载 app 数据文件 config.dat，若文件不存在则创建，并写入默认数据



### save_data_to_file

- params
  - data: data_types::APPData
- return
  - bool

保存当前数据至 config.dat 文件



### handle_app_event

- params
  - _app_handle: &tauri::AppHandle\<Wry>
  - event: tauri::RunEvent
- return

处理程序自身事件，当前仅截取窗口关闭请求事件进行处理，用以区分最小化至托盘或退出程序



### create_system_tray

- params
- return
  - tauri::SystemTray

创建程序系统托盘



### handle_system_tray_event

- params
  - app: &tauri::AppHandle\<Wry>
  - event: tauri::SystemTrayEvent 
- return

处理程序托盘事件



### check_webview2_available

- params
  - scope: tauri::AppHandle
- return

检查当前系统 webview2 版本，若未发现 webview2 存在则弹窗提示并跳转至 webview2 下载界面，并退出程序



### open_folder_window

- params
  - path: &str
- return

调用系统命令行打开文件夹



### initialize_window_shadow

- params
  - window: &tauri::TauriWindow
  - is_shadow_enbale: bool
- return

为程序添加阴影边框，在 linux 上暂不可用



### check_instance

- params
  - name: &str
- return

检查程序是否为单一实例，若已存在实例则新实例将退出

::: warning

纯 Rust 环境可用，Tauri 环境目前多开实例仍将被视作单一实例，原因未知

:::
