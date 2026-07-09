use super::structs::{ACCOUNT_MANAGEMENT_META, AccountManagementDetail};
use crate::parser::definition::EventRecord;
use std::borrow::Cow;

impl EventRecord for AccountManagementDetail {
    fn time(&self) -> &str {
        &self.time
    }

    fn type_name(&self) -> &'static str {
        "AccountManagement"
    }

    fn fields(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        let m = &ACCOUNT_MANAGEMENT_META;
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
        ]
    }
}
