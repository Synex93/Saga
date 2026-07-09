use super::structs::{SERVICE_CONTROL_META, ServiceControlDetail};
use crate::parser::definition::EventRecord;
use std::borrow::Cow;

impl EventRecord for ServiceControlDetail {
    fn time(&self) -> &str {
        &self.time
    }

    fn type_name(&self) -> &'static str {
        "ServiceControl"
    }
    fn fields(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        let m = &SERVICE_CONTROL_META;
        vec![
            (m.time.title, Cow::Borrowed(&self.time)),
            (m.event_id.title, Cow::Owned(self.event_id.to_string())),
            (m.description.title, Cow::Borrowed(self.description)),
            (m.service_name.title, Cow::Borrowed(&self.service_name)),
            (m.service_type.title, Cow::Borrowed(&self.service_type)),
            (m.start_type.title, Cow::Borrowed(&self.start_type)),
            (m.image_path.title, Cow::Borrowed(&self.image_path)),
            (m.account_name.title, Cow::Borrowed(&self.account_name)),
        ]
    }
}
