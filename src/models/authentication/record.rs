use super::structs::{AUTH_META, AuthenticationDetail};
use crate::parser::definition::{CellValue, EventRecord};

impl EventRecord for AuthenticationDetail {
    fn time(&self) -> &str {
        &self.time
    }

    fn type_name(&self) -> &'static str {
        "Authentication"
    }
    fn fields(&self) -> Vec<(&'static str, CellValue<'_>)> {
        let m = &AUTH_META;
        vec![
            (m.time.title, CellValue::text(&self.time)),
            (m.event_id.title, CellValue::num(self.event_id)),
            (m.description.title, CellValue::text(self.description)),
            (
                m.subject_user_name.title,
                CellValue::text(&self.subject_user_name),
            ),
            (
                m.target_user_name.title,
                CellValue::text(&self.target_user_name),
            ),
            (m.ip_address.title, CellValue::text(&self.ip_address)),
            (
                m.logon_type.title,
                match self.logon_type {
                    Some(v) => CellValue::num(v),
                    None => CellValue::text(""),
                },
            ),
            (m.status.title, CellValue::text(&self.status)),
            (m.raw_data.title, CellValue::text(&self.raw_data)),
        ]
    }
}
