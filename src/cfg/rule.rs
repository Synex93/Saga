use super::sturct::*;

impl Models {
    pub fn get_model_rule(&self) -> ModelRule {
        match self {
            // 验证相关
            Models::Authentication => ModelRule {
                files: vec!["Security.evtx"],
                ids: vec![4624, 4625, 4634, 4647, 4648, 4672, 4720, 4726, 4740, 4768],
            },

            // RDP
            Models::Session => ModelRule {
                files: vec![
                    "Microsoft-Windows-TerminalServices-LocalSessionManager%4Operational.evtx",
                    "Microsoft-Windows-TerminalServices-RemoteConnectionManager%4Operational.evtx",
                ],
                ids: vec![21, 22, 23, 24, 25, 1149],
            },
        }
    }
}
