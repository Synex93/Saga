use super::structs::{ACCOUNT_MANAGEMENT_META, AccountManagementDetail};
use crate::parser::definition::EventRecord;

impl EventRecord for AccountManagementDetail {
    fn time(&self) -> &str {
        &self.time
    }

    fn type_name(&self) -> &'static str {
        "AccountManagement"
    }

    fn csv_header(&self) -> String {
        let m = &ACCOUNT_MANAGEMENT_META;
        format!(
            "{},{},{},{},{}",
            m.time.title,
            m.event_id.title,
            m.description.title,
            m.subject_user_name.title,
            m.target_user_name.title,
        )
    }

    fn to_csv_row(&self) -> String {
        format!(
            "{},{},{},{},{}",
            self.time,
            self.event_id,
            self.description,
            self.subject_user_name,
            self.target_user_name,
        )
    }
}
