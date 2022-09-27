# logger.rs

## 功能

初始化日志功能，同时在终端和文件进行输出



## 函数

## initialize_logger

- params 
  - log_file_path: &str
- return

通过参数指定输出日志文件路径，终端输出等级为 Debug，文件输出等级为 Info， Warn 以上单列文件输出，滚动式更新并限制单一日志文件体积不大于 6Mb