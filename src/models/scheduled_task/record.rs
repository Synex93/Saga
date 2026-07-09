use super::structs::{SCHEDULED_TASK_META, ScheduledTaskDetail};
use crate::parser::definition::EventRecord;

impl EventRecord for ScheduledTaskDetail {
    fn time(&self) -> &str {
        &self.time
    }

    fn type_name(&self) -> &'static str {
        "ScheduledTask"
    }
    fn csv_header(&self) -> String {
        let m = &SCHEDULED_TASK_META;
        format!(
            "{},{},{},{},{},{},{}",
            m.time.title,
            m.event_id.title,
            m.description.title,
            m.task_name.title,
            m.subject_user_name.title,
            m.action.title,
            m.result_code.title,
        )
    }

    fn to_csv_row(&self) -> String {
        format!(
            "{},{},{},{},{},{},{}",
            self.time,
            self.event_id,
            self.description,
            self.task_name,
            self.subject_user_name,
            self.action,
            self.result_code,
        )
    }
}
