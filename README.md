# Saga - Windows 事件日志分析工具

![Saga](https://socialify.git.ci/Synex93/Saga/image?custom_language=Rust&font=Inter&forks=1&issues=1&language=1&name=1&owner=1&pulls=1&stargazers=1&theme=Light)


## 关于重构
原 Go 版本存在以下问题：
- 解析速度过于缓慢，未找到合适的替换方案
- 由于解析缓慢，CLI 方案需要缓存机制，同样未找到合适方案
---

## 使用方法

```
Usage: Saga.exe [OPTIONS] [COMMAND]

Commands:
  Authentication     验证相关信息, EventID: 4624, 4625, 4648, 4672, 4740, 4768, 4771, 4776
  Session            会话生命周期, EventID: 4634, 4647, 21, 22, 23, 24, 25, 40, 1149
  AccountManagement  账户管理, EventID: 4720, 4722, 4723, 4724, 4725, 4726, 4738, 4740, 4767
  ServiceControl     服务管理, EventID: 7034, 7035, 7036, 7040, 7045
  ScheduledTask      计划任务, EventID: 4698, 4699, 4700, 4701, 4702, 106, 140, 141, 200, 201
  PowerShell         PowerShell 执行日志, EventID: 4103, 4104, 4105, 4106, 400, 403, 600

Options:
  -p, --path <PATH>  指定日志路径,默认为当前系统日志存放位置
  -o, --out <OUT>    输出模式 [default: excel] [possible values: csv, excel]
  -j, --jobs <JOBS>  并发线程数，默认 4 (生产环境安全值) [default: 4]
  -h, --help         显示帮助信息
  -V, --version      显示版本信息
```

子命令支持**前缀匹配**，无需输入完整命令名：

```bash
# 以下写法等价
Saga.exe Authentication
Saga.exe Authen
Saga.exe Auth
```

---

## 示例

```bash
# 分析当前系统的登录验证日志
Saga.exe Authentication

# 指定日志路径
Saga.exe -p C:\Windows\System32\winevt\Logs Authentication

# 分析 PowerShell 执行记录
Saga.exe PowerShell
```

---

## 支持的日志模型

| 模型                | 日志文件                               | 说明                                 |
| ------------------- | -------------------------------------- | ------------------------------------ |
| `Authentication`    | Security.evtx                          | 登录成功/失败、NTLM/Kerberos 验证    |
| `Session`           | Security.evtx, TerminalServices\*.evtx | RDP 会话创建、断连、重连             |
| `AccountManagement` | Security.evtx                          | 账户创建、删除、启用、禁用、密码修改 |
| `ServiceControl`    | System.evtx                            | 服务安装、启动、停止、崩溃           |
| `ScheduledTask`     | Security.evtx, TaskScheduler\*.evtx    | 计划任务创建、修改、执行             |
| `PowerShell`        | PowerShell\*.evtx                      | 脚本块内容、命令执行记录             |

---

## 关于性能
测试环境：Intel Core Ultra 5 338H（12 核 12 线程），默认 4 线程（生产环境安全值），实测数据如下：

| 模型            | 输出格式 | 解析记录数   | 总耗时  | 平均速度           |
| --------------- | -------- | ------------ | ------- | ------------------ |
| Authentication  | Excel    | 2,738,874 条 | ~18.5s  | **~148,000 条/秒** |
| Authentication  | CSV      | 2,738,874 条 | ~13.7s  | **~200,000 条/秒** |
| Session         | Excel    | 2,742,760 条 | ~14.5s  | **~189,000 条/秒** |
| ScheduledTask   | Excel    | 2,756,432 条 | ~11.3s  | **~244,000 条/秒** |

默认 4 线程适用于直接在生产服务器上运行，避免抢占业务 CPU。实测 `-j 12` 相比默认值在所有模型上均无显著提升（差距 <0.2s），当前瓶颈在 evtx 二进制解码阶段——每个 evtx 文件由单线程解码，XML 解析已并行化但并非瓶颈。各模型均以 `Security.evtx`（270 万+ 条记录）为主导，多文件并未带来更多并行解码机会。`-j` 参数预留给后续解码层优化后使用。

构建产物体积：**~1MB**（`cargo build --release`）
相比较之前GO语言版本解析速度"单文件，50w条数据，大概需要40秒左右",现在可以说非常快了。

---

## 构建

```bash
cargo build --release
```

---

## 写在后面

输出方面已支持 CSV 和 Excel 两种格式，数值字段（如事件 ID、登录类型）以原生数字类型写入 Excel，可直接用于排序和筛选。

事件解析模板（各模型的字段提取逻辑）目前大部分由 AI 辅助生成，准确性和覆盖度有限，后续重心将转向人工校验解析准确性和补全更多场景的模板。当前性能瓶颈在 evtx 二进制解码（单文件单线程），后续也会探索解码层并行化。

如果你在使用中遇到字段缺失、解析错误或有新的场景需求，欢迎提ISSUE。
