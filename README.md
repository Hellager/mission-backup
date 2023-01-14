<p align="center"><img src="https://github.com/Hellager/mission-backup/blob/main/docs/.vitepress/icon.svg" alt="Logo" width="100"/></p>

<h2 align="center">有备 - 简单好用的备份软件</h2>

<p align="center">
<a href=""><img src="https://img.shields.io/github/license/hellager/mission-backup"></a>
<a href="https://youbei.hellagur.com/"><img src="https://img.shields.io/badge/docs-passing-brightgreen"></a>
<a href=""><img src="https://img.shields.io/github/actions/workflow/status/Hellager/mission-backup/release.yml"></a>
<a href="https://github.com/Hellager/mission-backup/releases"><img src="https://img.shields.io/github/v/release/Hellager/mission-backup"></a>
<a href=""><img src="https://img.shields.io/github/downloads/hellager/mission-backup/total"></a>
</p>

## 特性

- 基于 Tauri，兼顾了 web 前端和 rust 后端
- 5MB 的安装包，15MB 的程序，多平台可用
- 基于 cron 和 notify 实现对目标定时/监控备份
- 可自定义备份保存时长和空间
- 通过图表直观展示备份情况
- 完善的文档支持

## 应用 

### 让我康康今天改了多少版

开启监控备份，设定合适的监控延时时间和备份保存时间

在忙碌完了一天的工作以后，查看统计界面，直观感受今天改了多少版本

真是 ~~摸鱼~~ 努力的一天呢

### 可能，也许，还是第一版好？

开启定时/监控备份，设置忽略选项，不限制备份保存时长和体积

保证每一个版本都在本地有一个备份，随时找到你所需要的历史版本

### 我不接受这样的结局！

开启定时/监控备份，选择存档目录

Minecraft, 新手新档，纯净生存，奋战多日，拼出一片天地

觅得深矿，着全身铁套而入，后遇小白、古力帕围攻，惨死

离家甚远，赶路不便，呜呼哀哉

幸有备而来，提最新存档，再探！

## 安装

### 下载界面

前往文档 [下载界面](https://youbei.hellagur.com/general/download.html) 选择对应平台进行下载安装

### Github Release

前往 [Github Release](https://github.com/Hellager/mission-backup/releases) 选择对应格式进行下载安装

### 自编译

```shell
$ git clone https://github.com/Hellager/mission-backup.git
$ cd ./mission-backup
$ pnpm install
$ pnpm tauri build
```

## 使用

前往文档 [使用介绍](https://youbei.hellagur.com/general/usage.html) 查看详细使用指南

## 致谢

- [Tauri](https://github.com/tauri-apps/tauri) 项目框架
- [Element Plus](https://github.com/element-plus/element-plus) 界面 UI
- [hotwatch](https://github.com/francesca64/hotwatch) 监控功能
- [tokio-cron-scheduler](https://github.com/mvniekerk/tokio-cron-scheduler) 定时功能

## License

[Apache](https://github.com/Hellager/mission-backup/blob/main/LICENSE) © Steins

