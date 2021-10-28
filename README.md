# 备份计划

#### 项目介绍
通过程序对文件夹进行自动备份 <br>
目前支持 **定时自动备份** 及 **监控自动备份** <br>
目前支持以下压缩格式 **zip, tar.gz, tar.bz2, tar.xz, 7z** <br>
<strike>绝不是因为不会用FreeFileSync</strike><br>

#### 安装教程
下载压缩包，解压后直接运行即可 <br>
[Backup-Schedule v1.0.0](https://pan.baidu.com/s/19LJMwAJ8JeGNvst5d5Mqsg) 提取码: 5eoi

#### 使用说明

1. &nbsp; **添加新计划** <br>
运行 main.exe 点击 **新建计划** 进入设置界面 <br>
根据需求填写相关参数 点击 **确认** 新建计划 <br>
[![5q22ZQ.jpg](https://z3.ax1x.com/2021/10/28/5q22ZQ.jpg)](https://imgtu.com/i/5q22ZQ)
2.  &nbsp; **管理计划** <br>
在主界面可以查看计划相关信息 鼠标悬停可以查看完整路径 <br>
[![5qfOoQ.jpg](https://z3.ax1x.com/2021/10/28/5qfOoQ.jpg)](https://imgtu.com/i/5qfOoQ)
3.  &nbsp; **自动执行计划** <br>
当计划检测到 **触发条件** 即 **到达定时时间** 或 **文件夹内容存在修改** 将自动执行备份操作 <br>
备份文件将按照规定格式及计划指定的目录进行保存<br>
[![5q2Raj.jpg](https://z3.ax1x.com/2021/10/28/5q2Raj.jpg)](https://imgtu.com/i/5q2Raj)

#### 注意事项
1. &nbsp; 为避免占用过多系统资源 限制同时最多存在 十个定时任务 <br>
&nbsp;如果确有需求 修改如下位置代码后自行编译运行即可
```python
//utils\runner.py line 23
  executors = {
      'default': ThreadPoolExecutor(10)
  }
```

2. &nbsp; 如果你想自动备份的文件夹是 **需要编译的工程** 比如 KEIL工程等<br>
&nbsp;**不建议使用监控任务** 的触发方式
&nbsp;如果确有需求 修改如下位置代码后自行编译运行即可 请确保定时器时间大于或等于编译时间
```python
//utils\monitor.py line 38
self.timer = threading.Timer(3, self.check_snap_shot)
```

3. &nbsp; **监控触发方式** 将监控 **指定文件夹下的所有文件** 包括其子文件夹及子文件夹下所有文件 <br>
&nbsp;请避免同时需要监控的文件过多占用大量系统资源

4. &nbsp; 理论上支持多平台 暂未测试 
