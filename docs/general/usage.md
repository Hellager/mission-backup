# 使用

## 主界面

![主界面.gif](https://s2.loli.net/2022/09/30/X3rR4WkMbizJwta.gif)

主界面展示当前所有储存的任务及实现功能页面跳转

任务将以表格形式展示，可查看其状态或执行相关操作如暂停，编辑或删除

下方系统托盘从左到右依次实现锁屏，编辑当前任务，添加任务，软件设置及备份统计功能

### 锁屏

![锁屏.gif](https://s2.loli.net/2022/09/30/IRh1kecduPYEifL.gif)

通过锁屏限制所有操作，仅正确输入对应密码后可进入软件

默认密码为 2022

### 添加任务
![添加任务.gif](https://s2.loli.net/2022/09/30/RJgn1Q4bTk6jzAV.gif)

点击托盘➕按钮即可跳转至添加任务界面

或拖放文件/文件夹至软件界面，将自动跳转至添加任务界面

### 编辑任务

点击托盘🖊按钮

或双击想编辑的任务对应的一行

或单击想编辑的任务对应的一行，再单击托盘🖊按钮

### 删除任务

在主界面任务操作一列点击🗑按钮即可删除

若任务为正在执行状态则不可删除

### 跳转至设置界面

点击托盘⚙按钮即可跳转至设置界面

### 跳转至统计界面

点击托盘📈按钮即可跳转至统计界面

## 设置界面

![设置界面.png](https://s2.loli.net/2022/09/30/8KBIsag3xJqo47P.png)

| 设置项       | 描述                                             |
| ------------ | ------------------------------------------------ |
| 开机启动     | 是否开机自启动                                   |
| 主题切换     | 切换主题为明亮/黑暗                              |
| 最小化至托盘 | 改变点击关闭按钮时的行为                         |
| 语言切换     | 选择程序语言，目前可选 中/英                     |
| 启用锁屏密码 | 是否启用锁屏功能                                 |
| 重置密码     | 重置锁屏密码                                     |
| 重置监控延时 | 设置任务监控延时时间避免重复操作，重启程序后有效 |
| 版本         | 显示当前软件版本                                 |

## 任务界面

![任务界面.png](https://s2.loli.net/2022/09/30/SvBEPZL4mcXAO5b.png)

| 项目名称     | 是否必需         | 值类型 | 备注                                      |
| ------------ | ---------------- | ------ | ----------------------------------------- |
| 任务名称     | 是               | String |                                           |
| 目标类型     | 是               | String |                                           |
| 源路径       | 是               | String |                                           |
| 保存路径     | 是               | String |                                           |
| 启用忽略     | 否               | Bool   |                                           |
| 忽略策略     | 否               | String | 可自定义或使用 .gitignore                 |
| 启用压缩     | 否               | Bool   |                                           |
| 压缩格式     | 否               | String | 支持 zip / tar /tar.gz / tar.bz2 / tar.xz |
| 启用定时     | 是，与监控二选一 | Bool   |                                           |
| cron 表达式  | 否               | String |                                           |
| 启用监控     | 是，与定时二选一 | Bool   |                                           |
| 保存时长限制 | 否               | Number | 单位为 天                                 |
| 保存空间限制 | 否               | Number | 单位为 MB                                 |

## 统计界面

![统计界面.png](https://s2.loli.net/2022/09/30/NlqF46s9wJD1Edc.png)

### 选择任务

选择想查看的任务

### 选择时间

可以以 天 / 周 / 月 / 年 为单位查看备份统计情况

