use super::structs::{SCHEDULED_TASK_META, ScheduledTaskDetail};
use crate::parser::definition::EventRecord;
use std::borrow::Cow;

impl EventRecord for ScheduledTaskDetail {
    fn time(&self) -> &str {
        &self.time
    }

    fn type_name(&self) -> &'static str {
        "ScheduledTask"
    }
    fn fields(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        let m = &SCHEDULED_TASK_META;
        vec![
            (m.time.title, Cow::Borrowed(&self.time)),
            (m.event_id.title, Cow::Owned(self.event_id.to_string())),
            (m.description.title, Cow::Borrowed(self.description)),
            (m.task_name.title, Cow::Borrowed(&self.task_name)),
            (
                m.subject_user_name.title,
                Cow::Borrowed(&self.subject_user_name),
            ),
            (m.action.title, Cow::Borrowed(&self.action)),
            (m.result_code.title, Cow::Borrowed(&self.result_code)),
            (m.raw_data.title, Cow::Borrowed(&self.raw_data)),
        ]
    }
}
