use super::structs::{SESSION_META, SessionDetail};
use crate::parser::definition::EventRecord;
use std::borrow::Cow;

impl EventRecord for SessionDetail {
    fn time(&self) -> &str {
        &self.time
    }

    fn type_name(&self) -> &'static str {
        "Session"
    }
    fn fields(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        let m = &SESSION_META;
        vec![
            (m.time.title, Cow::Borrowed(&self.time)),
            (m.event_id.title, Cow::Owned(self.event_id.to_string())),
            (m.description.title, Cow::Borrowed(self.description)),
            (m.user_name.title, Cow::Borrowed(&self.user_name)),
            (m.session_id.title, Cow::Borrowed(&self.session_id)),
            (m.remote_host.title, Cow::Borrowed(&self.remote_host)),
            (m.reason.title, Cow::Borrowed(&self.reason)),
        ]
    }
}
