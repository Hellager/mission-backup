# 备份计划

#### 项目介绍
通过程序对文件夹进行自动备份 <br>
目前支持 **定时自动备份** 及 **监控自动备份** <br>
目前支持 **webdav同步保存备份** 及 **samba同步保存备份**<br>
目前支持以下压缩格式 **zip, tar.gz, tar.bz2, tar.xz, 7z** <br>
目前支持以下格式加密压缩 **zip, 7z**<br>
<strike>绝不是因为不会用FreeFileSync GoodSync</strike><br>

#### 项目更新
更新内容(v1.0.1):
1. &nbsp; 代码重构 降低耦合度
2. &nbsp; 添加组件单元测试
3. &nbsp; 配置加密保存
4. &nbsp; 添加最小化到托盘
5. &nbsp; 添加 zip, 7z 格式的加密压缩
6. &nbsp; 添加 webdav 服务支持
7. &nbsp; 添加 samba 服务支持
8. &nbsp; 添加场景切换
9. &nbsp; 修复部分已知错误

#### 安装教程
下载压缩包，解压后直接运行即可 <br>
[Backup-Schedule v1.0.1](https://pan.baidu.com/s/1lp-ar6vcfxS4mFqSZ-P7gA) 提取码: 63m1

#### 使用说明

1. &nbsp; **主界面操作** <br>
运行 main.exe 进入主界面 数据以表格形式进行展示<br>
**右键打开功能菜单**可以实现计划的增删改查及远程服务设置 右上角下拉菜单可以实现场景切换<br>
[![ocMAa9.png](https://s4.ax1x.com/2021/12/07/ocMAa9.png)](https://imgtu.com/i/ocMAa9)
2.  &nbsp; **管理计划** <br>
功能菜单中选择**新建**进入新建计划界面 根据需要填写相关参数<br>
在主界面中选中某一行 此时可以通过右键功能菜单 实现计划配置的修改及删除<br>
在主界面中鼠标双击某一行可直接进入对于计划配置修改界面<br>
[![ocMtRP.png](https://s4.ax1x.com/2021/12/07/ocMtRP.png)](https://imgtu.com/i/ocMtRP)
3.  &nbsp; **远程同步** <br>
功能菜单中选择 **远程** 进入远程服务设置界面 勾选对应选择框并填写相关参数后确认即可<br><br>
**Webdav服务**以 **坚果云** 为例 需填写 服务器地址 保存根目录 用户名 以及 用户密码<br>
在坚果云网页端找到 **账户信息** -> **安全选项** -> **添加应用** -> **得到密码**<br>
填写示例:<br>
host config: https://dav.jianguoyun.com/dav/;/Target<br>
username: 123456789@abc.com<br>
password: 123456<br>
**Samba服务** 需填写 服务器地址 端口 远程文件夹名称 保存根目录 用户名 以及用户密码<br>
填写示例:<br>
host config: 192.168.1.6;445;remote_folder;/Target<br>
username: abcde<br>
password: 123456<br><br>
当远程服务成功添加后 程序将会在指定根目录下创建 Backup-Schedule 目录 并根据计划名称创建子文件夹<br>
子文件夹结构与本地备份保存目录结构一致<br>
[![ocM4Z4.png](https://s4.ax1x.com/2021/12/07/ocM4Z4.png)](https://imgtu.com/i/ocM4Z4)
4.  &nbsp; **计划信息提示** <br>
主界面表格对应位置悬停可查看相应信息<br>
状态 -> 下次执行时间/上次修改时间<br>
源目录/保存目录 -> 完整目录路径<br>
备份数/备份大小 -> 本地 Webdav目录 Samba目录下存在的备份数量及其大小<br>
[![ocMyin.png](https://s4.ax1x.com/2021/12/07/ocMyin.png)](https://imgtu.com/i/ocMyin)
6.  &nbsp; **自动执行计划** <br>
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

4. &nbsp; **远程同步备份不受限制** 当计划存在限制条件时 远程的备份不会自动同步进行限制即不会限制其数量/大小 <br>

4. &nbsp; 理论上支持多平台 暂未测试 
