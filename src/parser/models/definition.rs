use super::authentication::AuthenticationDetail;

#[derive(Debug)]
pub enum ParserResult {
    Authentication(AuthenticationDetail),
    Session,
    AccountManagement,
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
            ParserResult::Session => "Session",
            ParserResult::AccountManagement => "AccountManagement",
        }
    }
    fn csv_header(&self) -> String {
        match self {
            ParserResult::Authentication(_) => AuthenticationDetail::csv_header(),
            ParserResult::Session => todo!(),
            ParserResult::AccountManagement => todo!(),
        }
    }

    fn to_csv_row(&self) -> String {
        match self {
            ParserResult::Authentication(detail) => detail.to_csv_row(),
            ParserResult::Session => todo!(),
            ParserResult::AccountManagement => todo!(),
        }
    }
}
