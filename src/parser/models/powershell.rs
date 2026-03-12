use super::definition::FieldMeta;
use crate::cfg::event::EventId;
use quick_xml::events::Event as XmlEvent;
use quick_xml::reader::Reader;

#[derive(Debug, Default)]
pub struct PowerShellDetail {
    pub time: String,
    pub event_id: u16,
    pub description: String,
    pub user_name: String,
    pub host_name: String,
    pub script_block: String,
    pub command_line: String,
    pub sequence_id: String,
}

pub struct PowerShellMeta {
    pub time: FieldMeta,
    pub event_id: FieldMeta,
    pub description: FieldMeta,
    pub user_name: FieldMeta,
    pub host_name: FieldMeta,
    pub script_block: FieldMeta,
    pub command_line: FieldMeta,
    pub sequence_id: FieldMeta,
}

pub static POWER_SHELL_META: PowerShellMeta = PowerShellMeta {
    time: FieldMeta { title: "时间" },
    event_id: FieldMeta { title: "事件ID" },
    description: FieldMeta { title: "描述" },
    user_name: FieldMeta { title: "用户" },
    host_name: FieldMeta { title: "PS宿主" },
    script_block: FieldMeta {
        title: "脚本内容"
    },
    command_line: FieldMeta { title: "命令行" },
    sequence_id: FieldMeta {
        title: "分片序号"
    },
};

impl PowerShellDetail {
    pub fn csv_header() -> String {
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

    pub fn to_csv_row(&self) -> String {
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

pub fn parse_powershell(xml: &str) -> PowerShellDetail {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut detail = PowerShellDetail::default();
    let mut buf = Vec::with_capacity(512);
    let mut current_data_name_bytes: &'static [u8] = b"";

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(ref e)) if e.name().as_ref() == b"TimeCreated" => {
                for attr in e.attributes().flatten() {
                    if attr.key.as_ref() == b"SystemTime" {
                        if let Ok(v) = attr.unescape_value() {
                            detail.time = v.into_owned();
                        }
                    }
                }
            }

            Ok(XmlEvent::Start(ref e)) if e.name().as_ref() == b"EventID" => {
                if let Ok(text) = reader.read_text(e.name()) {
                    if let Ok(id) = text.parse::<u16>() {
                        detail.event_id = id;
                        detail.description = EventId(id).description().to_string();
                    }
                }
            }

            Ok(XmlEvent::Start(ref e)) if e.name().as_ref() == b"Data" => {
                current_data_name_bytes = b"";
                for attr in e.attributes().flatten() {
                    if attr.key.as_ref() == b"Name" {
                        current_data_name_bytes = match attr.value.as_ref() {
                            // Operational.evtx (4103/4104)
                            b"UserId" => b"UserId",
                            b"HostName" => b"HostName",
                            b"ScriptBlockText" => b"ScriptBlockText", // 4104 核心
                            b"MessageNumber" => b"MessageNumber",     // 4104 分片序号
                            // 4103 命令管道
                            b"ContextInfo" => b"ContextInfo",
                            // Windows PowerShell.evtx (400/403/600)
                            b"HostApplication" => b"HostApplication", // 启动命令行
                            _ => b"",
                        };
                        break;
                    }
                }
            }

            Ok(XmlEvent::Text(ref e)) => {
                if current_data_name_bytes.is_empty() {
                    buf.clear();
                    continue;
                }
                if let Ok(cow) = e.decode() {
                    let text = cow.as_ref();
                    if text == "-" || text.is_empty() {
                        current_data_name_bytes = b"";
                        buf.clear();
                        continue;
                    }
                    match current_data_name_bytes {
                        b"UserId" => detail.user_name = text.to_owned(),
                        b"HostName" => detail.host_name = text.to_owned(),
                        b"ScriptBlockText" => detail.script_block = text.to_owned(),
                        b"MessageNumber" => detail.sequence_id = text.to_owned(),
                        b"ContextInfo" => {
                            if detail.command_line.is_empty() {
                                detail.command_line = text.to_owned();
                            }
                        }
                        b"HostApplication" => {
                            if detail.command_line.is_empty() {
                                detail.command_line = text.to_owned();
                            }
                        }
                        _ => {}
                    }
                }
                current_data_name_bytes = b"";
            }

            Ok(XmlEvent::End(ref e)) => match e.name().as_ref() {
                b"Data" => current_data_name_bytes = b"",
                b"Event" => break,
                _ => {}
            },

            Ok(XmlEvent::Eof) | Err(_) => break,
            _ => {}
        }
        buf.clear();
    }

    detail
}
