<p align="center"><img src="https://github.com/Hellager/MissionBackup/blob/main/docs/.vitepress/icon.svg" alt="Logo" width="100"/></p>

<h2 align="center">有备 - 简单好用的备份软件</h2>

<p align="center">
<a href=""><img src="https://img.shields.io/github/license/hellager/MissionBackup"></a>
<a href="https://youbei.hellagur.com/"><img src="https://img.shields.io/badge/docs-passing-brightgreen"></a>
<a href=""><img src="https://img.shields.io/github/workflow/status/Hellager/MissionBackup/Deploy"></a>
<a href="https://github.com/Hellager/MissionBackup/releases"><img src="https://img.shields.io/github/v/release/hellager/MissionBackup"></a>
<a href=""><img src="https://img.shields.io/github/downloads/hellager/MissionBackup/total"></a>
</p>

## 特性

- 基于 Tauri，兼顾了 web 前端和 rust 后端
- 5MB 的安装包，15MB 的程序，多平台可用
- 基于 cron 和 notify 实现对目标定时/监控备份
- 可自定义备份保存时长和空间
- 通过图表直观展示备份情况
- 完善的文档支持

## 安装

### 下载界面

前往文档的下载界面，选择渠道进行下载

### Github Release

前往 Github Release 界面，选择版本进行下载

### 自编译

```shell
$ git clone https://github.com/Hellager/MissionBackup.git
$ cd ./MissionBackup
$ pnpm install
$ pnpm tauri build
```

## 使用

前往文档 使用界面 查看

## 鸣谢

[Tauri 框架](https://github.com/tauri-apps/tauri)

[Element Plus UI](https://github.com/element-plus/element-plus)

[hotwatch 监控文件](https://github.com/francesca64/hotwatch)

[tokio-cron-scheduler cron 任务控制器](https://github.com/mvniekerk/tokio-cron-scheduler)

## License

[Apache](https://github.com/Hellager/youbei_template/blob/main/LICENSE) © Steins

