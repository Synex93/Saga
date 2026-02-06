use clap::Subcommand;

#[derive(Debug, Clone)]
pub struct Config {
    pub path: String,
    pub model: Models,
}
pub struct ModelRule {
    pub files: Vec<&'static str>,
    pub ids: Vec<u16>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Models {
    /// 验证相关信息,EventID: 4624, 4625, 4634, 4647, 4648, 4672, 4720, 4726, 4740, 4768
    Authentication = 0,
    /// 登录相关信息,EventID: 21, 22, 23, 24, 25, 1149
    Session = 1,
}
