use super::sturct::*;

impl Models {
    pub fn get_model_rule(&self) -> ModelRule {
        match self {
            // 验证相关
            Models::Authentication => ModelRule {
                files: vec!["Security.evtx"],
                ids: vec![4624, 4625, 4648, 4672, 4740, 4768, 4771, 4776],
            },

            // 会话生命周期
            Models::Session => ModelRule {
                files: vec![
                    "Security.evtx",
                    "Microsoft-Windows-TerminalServices-LocalSessionManager%4Operational.evtx",
                    "Microsoft-Windows-TerminalServices-RemoteConnectionManager%4Operational.evtx",
                ],
                ids: vec![4634, 4647, 21, 22, 23, 24, 25, 40, 1149],
            },

            // 账户管理
            Models::AccountManagement => ModelRule {
                files: vec!["Security.evtx"],
                ids: vec![4720, 4726],
            },
        }
    }
}
