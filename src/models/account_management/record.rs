use super::structs::{ACCOUNT_MANAGEMENT_META, AccountManagementDetail};
use crate::parser::definition::{CellValue, EventRecord};

impl EventRecord for AccountManagementDetail {
    fn time(&self) -> &str {
        &self.time
    }

    fn type_name(&self) -> &'static str {
        "AccountManagement"
    }

    fn fields(&self) -> Vec<(&'static str, CellValue<'_>)> {
        let m = &ACCOUNT_MANAGEMENT_META;
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
            (m.raw_data.title, CellValue::text(&self.raw_data)),
        ]
    }
}
