# Saga

**Windows 事件日志分析工具**

## 为什么重构
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
  -p, --path <PATH>  指定日志路径，默认为当前系统日志存放位置
  -o, --out <OUT>    输出模式 [default: csv] [possible values: csv]
  -h, --help         显示帮助信息
  -V, --version      显示版本信息
```

子命令支持**前缀匹配**，无需输入完整命令名：

```bash
# 以下写法等价
Saga.exe Authentication
Saga.exe authen
Saga.exe auth
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
测试环境：Intel Core i5-12500H，4 线程(当前默认限制，为避免某些环境出问题)，419 个 evtx 文件,实测数据如下：

| 指标       | 数值              |
| ---------- | ----------------- |
| 文件数     | 419 个            |
| 解析记录数 | 518,448 条        |
| 总耗时     | 6.55s             |
| 平均速度   | **~79,000 条/秒** |

构建产物体积：**~1MB**（`cargo build --release`）
相比较之前GO语言版本解析速度"单文件，50w条数据，大概需要40秒左右",现在可以说非常快了。

---

## 构建

```bash
cargo build --release
```

---

## 写在后面

目前开发重心主要在于**输出方式的完善**，现有的单一 CSV 输出较为简陋，后续计划支持更多格式与展示方式。

事件解析模板（各模型的字段提取逻辑）目前大部分由 AI 辅助生成，准确性和覆盖度有限，后续会逐步人工校验和优化，也会慢慢补全更多场景的解析模板。

如果你在使用中遇到字段缺失、解析错误或有新的场景需求，欢迎提ISSUE。