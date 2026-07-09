use super::structs::{SESSION_META, SessionDetail};
use crate::parser::definition::EventRecord;

impl EventRecord for SessionDetail {
    fn time(&self) -> &str {
        &self.time
    }

    fn type_name(&self) -> &'static str {
        "Session"
    }
    fn csv_header(&self) -> String {
        let m = &SESSION_META;
        format!(
            "{},{},{},{},{},{},{}",
            m.time.title,
            m.event_id.title,
            m.description.title,
            m.user_name.title,
            m.session_id.title,
            m.remote_host.title,
            m.reason.title,
        )
    }

    fn to_csv_row(&self) -> String {
        format!(
            "{},{},{},{},{},{},{}",
            self.time,
            self.event_id,
            self.description,
            self.user_name,
            self.session_id,
            self.remote_host,
            self.reason,
        )
    }
}
