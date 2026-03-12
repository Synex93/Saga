use crate::parser::models::{
    account_management::parse_account_management, authentication::parse_authentication,
    definition::ParserResult, powershell::parse_powershell, scheduled_task::parse_scheduled_task,
    service_control::parse_service_control, session::parse_session,
};

pub trait XmlParser {
    fn parse(&self, xml: &str) -> ParserResult;
}

impl XmlParser for crate::cfg::sturct::Models {
    fn parse(&self, xml: &str) -> ParserResult {
        match self {
            Self::Authentication => ParserResult::Authentication(parse_authentication(xml)),
            Self::Session => ParserResult::Session(parse_session(xml)),
            Self::AccountManagement => {
                ParserResult::AccountManagement(parse_account_management(xml))
            }
            Self::ServiceControl => ParserResult::ServiceControl(parse_service_control(xml)),
            Self::ScheduledTask => ParserResult::ScheduledTask(parse_scheduled_task(xml)),
            Self::PowerShell => ParserResult::PowerShell(parse_powershell(xml)),
        }
    }
}
