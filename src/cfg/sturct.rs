use clap::{Subcommand, ValueEnum};
#[derive(Debug, Clone)]
pub struct Config {
    pub path: String,
    pub model: Models,
    pub format: OutFormat,
}

pub struct ModelRule {
    pub files: Vec<&'static str>,
    pub ids: Vec<u16>,
}

#[derive(Subcommand, Debug, Clone)]
#[command(rename_all = "verbatim")]
pub enum Models {
    /// 验证相关信息, EventID: 4624, 4625, 4648, 4672, 4740, 4768, 4771, 4776
    Authentication,

    /// 会话生命周期, EventID: 4634, 4647, 21, 22, 23, 24, 25, 40, 1149
    Session,

    /// 账户管理, EventID: 4720, 4722, 4723, 4724, 4725, 4726, 4738, 4740, 4767
    AccountManagement,

    /// 服务管理, EventID: 7034, 7035, 7036, 7040, 7045
    ServiceControl,

    /// 计划任务, EventID: 4698, 4699, 4700, 4701, 4702, 106, 140, 141, 200, 201
    ScheduledTask,

    /// PowerShell 执行日志, EventID: 4103, 4104, 4105, 4106, 400, 403, 600
    PowerShell,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum OutFormat {
    Csv,
}
