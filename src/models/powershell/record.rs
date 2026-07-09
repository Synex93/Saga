use super::structs::{POWER_SHELL_META, PowerShellDetail};
use crate::parser::definition::EventRecord;
use std::borrow::Cow;

impl EventRecord for PowerShellDetail {
    fn time(&self) -> &str {
        &self.time
    }

    fn type_name(&self) -> &'static str {
        "PowerShell"
    }
    fn fields(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        let m = &POWER_SHELL_META;
        vec![
            (m.time.title, Cow::Borrowed(&self.time)),
            (m.event_id.title, Cow::Owned(self.event_id.to_string())),
            (m.description.title, Cow::Borrowed(self.description)),
            (m.user_name.title, Cow::Borrowed(&self.user_name)),
            (m.host_name.title, Cow::Borrowed(&self.host_name)),
            (m.script_block.title, Cow::Borrowed(&self.script_block)),
            (m.command_line.title, Cow::Borrowed(&self.command_line)),
            (m.sequence_id.title, Cow::Borrowed(&self.sequence_id)),
            (m.raw_data.title, Cow::Borrowed(&self.raw_data)),
        ]
    }
}
