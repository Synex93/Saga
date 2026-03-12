use super::definition::FieldMeta;
use crate::cfg::event::EventId;
use quick_xml::events::Event as XmlEvent;
use quick_xml::reader::Reader;

#[derive(Debug, Default)]
pub struct AccountManagementDetail {
    pub time: String,
    pub event_id: u16,
    pub description: String,
    pub subject_user_name: String,
    pub target_user_name: String,
}

pub struct AccountManagementMeta {
    pub time: FieldMeta,
    pub event_id: FieldMeta,
    pub description: FieldMeta,
    pub subject_user_name: FieldMeta,
    pub target_user_name: FieldMeta,
}

pub static ACCOUNT_MANAGEMENT_META: AccountManagementMeta = AccountManagementMeta {
    time: FieldMeta { title: "时间" },
    event_id: FieldMeta { title: "事件ID" },
    description: FieldMeta { title: "描述" },
    subject_user_name: FieldMeta { title: "操作者" },
    target_user_name: FieldMeta {
        title: "被操作账户",
    },
};

impl AccountManagementDetail {
    pub fn csv_header() -> String {
        let m = &ACCOUNT_MANAGEMENT_META;
        format!(
            "{},{},{},{},{}",
            m.time.title,
            m.event_id.title,
            m.description.title,
            m.subject_user_name.title,
            m.target_user_name.title,
        )
    }

    pub fn to_csv_row(&self) -> String {
        format!(
            "{},{},{},{},{}",
            self.time,
            self.event_id,
            self.description,
            self.subject_user_name,
            self.target_user_name,
        )
    }
}

pub fn parse_account_management(xml: &str) -> AccountManagementDetail {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut detail = AccountManagementDetail::default();
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
                            b"SubjectUserName" => b"SubjectUserName",
                            b"TargetUserName" => b"TargetUserName",
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
                        b"SubjectUserName" => detail.subject_user_name = text.to_owned(),
                        b"TargetUserName" => detail.target_user_name = text.to_owned(),
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
