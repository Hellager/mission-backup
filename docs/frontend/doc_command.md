# Command

## 功能

前后端通信时的命令

```typescript
enum TauriCommand {
    COMMAND_CLOSE_SPLASHSCREEN，                    // 关闭加载页面
    COMMAND_INITIALIZE_DATA,					    // 请求初始化数据
    COMMAND_IS_INITIALIZED,						    // 请求是否初始化
    COMMAND_INITIALIZE_PROGRAM_STATUS,				// 请求更新初始化状态
    COMMAND_EXIT_PROGRAM,							// 请求退出程序
    COMMAND_CLOSE_TO_TRAY,							// 最小化至托盘
    COMMAND_START_TIMING_SAVE_DATA,					// 开始定期保存数据
    COMMAND_TIMING_SAVE_DATA,					    // 执行定期保存数据命令
    COMMAND_CHANGE_SETTING_IS_AUTO_START,			// 请求修改开机自启动设置
    COMMAND_CHANGE_SETTING_IS_LIGHT_THEME,			// 请求修改软件主题设置
    COMMAND_CHANGE_SETTING_IS_CLOSE_TO_TRAY,		// 请求修改软件退出时是否为最小化至托盘吧
    COMMAND_CHANGE_WINDOW_STATE_BY_SYSTEM_TRAY,		// 收到来自托盘的窗口操作事件并转发处理
    COMMAND_CHANGE_SETTING_LANGUAGE,				// 请求修改软件语言设置
    COMMAND_CHANGE_SETTING_IS_PASSWORD_PROTECTED,	// 请求修改软件锁屏密码启用
    COMMAND_CHANGE_SETTING_PASSWORD,				// 请求修改软件锁屏密码
    COMMAND_CHANGE_SETTING_MONITOR_DELAY,			// 请求修改监控延时
    COMMAND_UNLOCK,									// 请求解锁屏幕
    COMMAND_CREATE_MISSION,							// 请求创建任务
    COMMAND_START_MISSION,							// 请求启动任务
    COMMAND_STOP_MISSION,							// 请求停止任务
    COMMAND_EDIT_MISSION,							// 请求修改任务
    COMMAND_DELETE_MISSION,							// 请求删除任务
    COMMAND_FORCE_DELETE_MISSION,					// 请求强制从列表删除任务
    COMMAND_CHECK_PATH_VALID,						// 请求校验路径有效性
    COMMAND_GET_DROP_PATH_INFO,						// 请求校验拖放路径信息
    COMMAND_UPDATE_LIST_INFO,						// 请求更新列表信息
    COMMAND_GET_MISSION_BACKUPS_STATUS,				// 请求任务统计情况
};
```





## 函数

### execute_rust_command

- props
  - command: Number
  - data?: any
  - additional?: any
  - additional2?: any
  - additional3?: any
- return
- type - async

执行对应命令并返回