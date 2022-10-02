# plugins.rs

## 功能

负责程序插件相关初始化



## 函数

### initialize_plugin_highlander

- params
  - event_name: &str
- return
  - tauri_plugin_highlander::Highlander\<Wry>

保证程序当前仅能有一个实例运行，尝试运行第二个实例时将发出 event_name 对应的 event

::: warning

tauri_plugin_highlander 无法在 linux 和 Macos 通过编译

:::