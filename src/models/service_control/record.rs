use super::structs::{SERVICE_CONTROL_META, ServiceControlDetail};
use crate::parser::definition::{CellValue, EventRecord};

impl EventRecord for ServiceControlDetail {
    fn time(&self) -> &str {
        &self.time
    }

    fn type_name(&self) -> &'static str {
        "ServiceControl"
    }
    fn fields(&self) -> Vec<(&'static str, CellValue<'_>)> {
        let m = &SERVICE_CONTROL_META;
        vec![
            (m.time.title, CellValue::text(&self.time)),
            (m.event_id.title, CellValue::num(self.event_id)),
            (m.description.title, CellValue::text(self.description)),
            (m.service_name.title, CellValue::text(&self.service_name)),
            (m.service_type.title, CellValue::text(&self.service_type)),
            (m.start_type.title, CellValue::text(&self.start_type)),
            (m.image_path.title, CellValue::text(&self.image_path)),
            (m.account_name.title, CellValue::text(&self.account_name)),
            (m.raw_data.title, CellValue::text(&self.raw_data)),
        ]
    }
}
