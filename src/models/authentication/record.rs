use super::structs::{AUTH_META, AuthenticationDetail};
use crate::parser::definition::EventRecord;
use std::borrow::Cow;

impl EventRecord for AuthenticationDetail {
    fn time(&self) -> &str {
        &self.time
    }

    fn type_name(&self) -> &'static str {
        "Authentication"
    }
    fn fields(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        let m = &AUTH_META;
        vec![
            (m.time.title, Cow::Borrowed(&self.time)),
            (m.event_id.title, Cow::Owned(self.event_id.to_string())),
            (m.description.title, Cow::Borrowed(self.description)),
            (
                m.subject_user_name.title,
                Cow::Borrowed(&self.subject_user_name),
            ),
            (
                m.target_user_name.title,
                Cow::Borrowed(&self.target_user_name),
            ),
            (m.ip_address.title, Cow::Borrowed(&self.ip_address)),
            (
                m.logon_type.title,
                self.logon_type
                    .map_or(Cow::Borrowed(""), |v| Cow::Owned(v.to_string())),
            ),
            (m.status.title, Cow::Borrowed(&self.status)),
            (m.raw_data.title, Cow::Borrowed(&self.raw_data)),
        ]
    }
}
