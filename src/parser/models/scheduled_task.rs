use super::definition::FieldMeta;
use crate::cfg::event::EventId;
use quick_xml::events::Event as XmlEvent;
use quick_xml::reader::Reader;

#[derive(Debug, Default)]
pub struct ScheduledTaskDetail {
    pub time: String,
    pub event_id: u16,
    pub description: String,
    pub task_name: String,
    pub subject_user_name: String,
    pub action: String,
    pub result_code: String,
}

pub struct ScheduledTaskMeta {
    pub time: FieldMeta,
    pub event_id: FieldMeta,
    pub description: FieldMeta,
    pub task_name: FieldMeta,
    pub subject_user_name: FieldMeta,
    pub action: FieldMeta,
    pub result_code: FieldMeta,
}

pub static SCHEDULED_TASK_META: ScheduledTaskMeta = ScheduledTaskMeta {
    time: FieldMeta { title: "时间" },
    event_id: FieldMeta { title: "事件ID" },
    description: FieldMeta { title: "描述" },
    task_name: FieldMeta {
        title: "任务名称"
    },
    subject_user_name: FieldMeta { title: "操作者" },
    action: FieldMeta {
        title: "执行动作"
    },
    result_code: FieldMeta { title: "结果码" },
};

impl ScheduledTaskDetail {
    pub fn csv_header() -> String {
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

    pub fn to_csv_row(&self) -> String {
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

pub fn parse_scheduled_task(xml: &str) -> ScheduledTaskDetail {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut detail = ScheduledTaskDetail::default();
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
                            // Security.evtx (4698-4702)
                            b"TaskName" => b"TaskName",
                            b"SubjectUserName" => b"SubjectUserName",
                            b"TaskContent" => b"TaskContent", // 任务 XML 里含 action
                            // TaskScheduler/Operational (200/201)
                            b"ActionName" => b"ActionName", // 实际执行的命令
                            b"ResultCode" => b"ResultCode", // 201 执行结果
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
                        b"TaskName" => detail.task_name = text.to_owned(),
                        b"SubjectUserName" => detail.subject_user_name = text.to_owned(),
                        b"ActionName" => detail.action = text.to_owned(),
                        b"TaskContent" => {
                            // TaskContent 是完整 XML，仅在 action 为空时作兜底
                            if detail.action.is_empty() {
                                detail.action = text.to_owned();
                            }
                        }
                        b"ResultCode" => detail.result_code = text.to_owned(),
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
