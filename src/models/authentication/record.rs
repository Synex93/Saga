use super::structs::{AUTH_META, AuthenticationDetail};
use crate::parser::definition::EventRecord;

impl EventRecord for AuthenticationDetail {
    fn time(&self) -> &str {
        &self.time
    }

    fn type_name(&self) -> &'static str {
        "Authentication"
    }

    fn csv_header(&self) -> String {
        let m = &AUTH_META;
        format!(
            "{},{},{},{},{},{},{},{}",
            m.time.title,
            m.event_id.title,
            m.description.title,
            m.subject_user_name.title,
            m.target_user_name.title,
            m.ip_address.title,
            m.logon_type.title,
            m.status.title
        )
    }

    fn to_csv_row(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{}",
            self.time,
            self.event_id,
            self.description,
            self.subject_user_name,
            self.target_user_name,
            self.ip_address,
            self.logon_type.map_or(String::new(), |v| v.to_string()),
            self.status,
        )
    }
}
