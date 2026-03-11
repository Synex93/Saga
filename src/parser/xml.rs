use crate::parser::models::{authentication::parse_authentication, definition::ParserResult};

pub trait XmlParser {
    fn parse(&self, xml: &str) -> ParserResult;
}
impl XmlParser for crate::cfg::sturct::Models {
    fn parse(&self, xml: &str) -> ParserResult {
        match self {
            Self::Authentication => ParserResult::Authentication(parse_authentication(xml)),
            Self::Session => ParserResult::Session,
            Self::AccountManagement => ParserResult::AccountManagement,
        }
    }
}
