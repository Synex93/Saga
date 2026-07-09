use super::structs::{POWER_SHELL_META, PowerShellDetail};
use crate::parser::definition::EventRecord;

impl EventRecord for PowerShellDetail {
    fn time(&self) -> &str {
        &self.time
    }

    fn type_name(&self) -> &'static str {
        "PowerShell"
    }
    fn csv_header(&self) -> String {
        let m = &POWER_SHELL_META;
        format!(
            "{},{},{},{},{},{},{},{}",
            m.time.title,
            m.event_id.title,
            m.description.title,
            m.user_name.title,
            m.host_name.title,
            m.script_block.title,
            m.command_line.title,
            m.sequence_id.title,
        )
    }

    fn to_csv_row(&self) -> String {
        // script_block 可能含换行和逗号，用双引号包裹
        format!(
            "{},{},{},{},{},\"{}\",{},{}",
            self.time,
            self.event_id,
            self.description,
            self.user_name,
            self.host_name,
            self.script_block.replace('"', "\"\""),
            self.command_line,
            self.sequence_id,
        )
    }
}
