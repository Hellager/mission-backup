# explorer.rs

## 功能

文件/文件夹的拷贝及删除等操作



## 函数

### remove_target

- params
  - target: &str
- return
  - bool

根据所给路径删除目标



### copy_target_with_no_ignores

- params
  - from: &str
  - to: &str
- return
  - bool

无忽略拷贝目标至保存目录

### copy_target_with_ignores

- params
  - from: &str
  - to: &str
  - ignore: &Vec\<String>
- return
  - bool

使用自定义忽略内容，拷贝目标至保存目录

### copy_folder_with_ignore_file

- params
  - from: &str
  - to: &str
- return
  - bool

根据 .gitignore 忽略，拷贝目标至保存目录

::: warning

若目标文件夹仅有 .gitignore 文件，没有 .git 目录，将会全量拷贝而不是根据 .gitignore 忽略

:::

### copy_folder_with_custom_ignores

- params
  - from: &str
  - to: &str
  - ignore: &Vec\<String>
- return
  - bool

使用自定义忽略内容，拷贝文件夹至保存目录

### restrict_save_path_day_cout

- params
  - path: &str
  - count: u64
- return 
  - bool

限制目录文件/文件夹数量

### restrict_save_path_size

- params
  - path: &str
  - size: u64
- return
  - bool

限制目录空间大小