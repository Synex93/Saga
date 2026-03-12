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
pub enum Models {
    /// 验证相关信息, EventID: 4624, 4625, 4648, 4672, 4740, 4768, 4771, 4776
    Authentication = 0,
    /// 会话生命周期, EventID: 4634, 4647, 21, 22, 23, 24, 25, 40, 1149
    Session = 1,
    /// 账户管理, EventID: 4720, 4726
    AccountManagement = 2,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum OutFormat {
    Csv,
}
