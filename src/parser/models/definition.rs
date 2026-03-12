use super::account_management::AccountManagementDetail;
use super::authentication::AuthenticationDetail;
use super::powershell::PowerShellDetail;
use super::scheduled_task::ScheduledTaskDetail;
use super::service_control::ServiceControlDetail;
use super::session::SessionDetail;

#[derive(Debug)]
pub enum ParserResult {
    Authentication(AuthenticationDetail),
    Session(SessionDetail),
    AccountManagement(AccountManagementDetail),
    ServiceControl(ServiceControlDetail),
    ScheduledTask(ScheduledTaskDetail),
    PowerShell(PowerShellDetail),
}

pub struct FieldMeta {
    pub title: &'static str,
    // TODO: 后续可以增加字段宽度
}

pub trait ToCsv {
    fn type_name(&self) -> &'static str;
    fn csv_header(&self) -> String;
    fn to_csv_row(&self) -> String;
}

impl ToCsv for ParserResult {
    fn type_name(&self) -> &'static str {
        match self {
            ParserResult::Authentication(_) => "Authentication",
            ParserResult::Session(_) => "Session",
            ParserResult::AccountManagement(_) => "AccountManagement",
            ParserResult::ServiceControl(_) => "ServiceControl",
            ParserResult::ScheduledTask(_) => "ScheduledTask",
            ParserResult::PowerShell(_) => "PowerShell",
        }
    }

    fn csv_header(&self) -> String {
        match self {
            ParserResult::Authentication(_) => AuthenticationDetail::csv_header(),
            ParserResult::Session(_) => SessionDetail::csv_header(),
            ParserResult::AccountManagement(_) => AccountManagementDetail::csv_header(),
            ParserResult::ServiceControl(_) => ServiceControlDetail::csv_header(),
            ParserResult::ScheduledTask(_) => ScheduledTaskDetail::csv_header(),
            ParserResult::PowerShell(_) => PowerShellDetail::csv_header(),
        }
    }

    fn to_csv_row(&self) -> String {
        match self {
            ParserResult::Authentication(detail) => detail.to_csv_row(),
            ParserResult::Session(detail) => detail.to_csv_row(),
            ParserResult::AccountManagement(detail) => detail.to_csv_row(),
            ParserResult::ServiceControl(detail) => detail.to_csv_row(),
            ParserResult::ScheduledTask(detail) => detail.to_csv_row(),
            ParserResult::PowerShell(detail) => detail.to_csv_row(),
        }
    }
}
