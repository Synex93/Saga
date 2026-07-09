use super::structs::{SERVICE_CONTROL_META, ServiceControlDetail};
use crate::parser::definition::EventRecord;

impl EventRecord for ServiceControlDetail {
    fn time(&self) -> &str {
        &self.time
    }

    fn type_name(&self) -> &'static str {
        "ServiceControl"
    }
    fn csv_header(&self) -> String {
        let m = &SERVICE_CONTROL_META;
        format!(
            "{},{},{},{},{},{},{},{}",
            m.time.title,
            m.event_id.title,
            m.description.title,
            m.service_name.title,
            m.service_type.title,
            m.start_type.title,
            m.image_path.title,
            m.account_name.title,
        )
    }

    fn to_csv_row(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{}",
            self.time,
            self.event_id,
            self.description,
            self.service_name,
            self.service_type,
            self.start_type,
            self.image_path,
            self.account_name,
        )
    }
}
