use super::definition::FieldMeta;
use crate::cfg::event::EventId;
use quick_xml::events::Event as XmlEvent;
use quick_xml::reader::Reader;

#[derive(Debug, Default)]
pub struct SessionDetail {
    pub time: String,
    pub event_id: u16,
    pub description: &'static str,
    pub user_name: String,
    pub session_id: String,
    pub remote_host: String,
    pub reason: String,
}

pub struct SessionMeta {
    pub time: FieldMeta,
    pub event_id: FieldMeta,
    pub description: FieldMeta,
    pub user_name: FieldMeta,
    pub session_id: FieldMeta,
    pub remote_host: FieldMeta,
    pub reason: FieldMeta,
}

pub static SESSION_META: SessionMeta = SessionMeta {
    time: FieldMeta { title: "时间" },
    event_id: FieldMeta { title: "事件ID" },
    description: FieldMeta { title: "描述" },
    user_name: FieldMeta { title: "用户" },
    session_id: FieldMeta { title: "会话ID" },
    remote_host: FieldMeta {
        title: "来源主机"
    },
    reason: FieldMeta { title: "原因码" },
};

impl SessionDetail {
    pub fn csv_header() -> String {
        let m = &SESSION_META;
        format!(
            "{},{},{},{},{},{},{}",
            m.time.title,
            m.event_id.title,
            m.description.title,
            m.user_name.title,
            m.session_id.title,
            m.remote_host.title,
            m.reason.title,
        )
    }

    pub fn to_csv_row(&self) -> String {
        format!(
            "{},{},{},{},{},{},{}",
            self.time,
            self.event_id,
            self.description,
            self.user_name,
            self.session_id,
            self.remote_host,
            self.reason,
        )
    }
}

pub fn parse_session(xml: &str) -> SessionDetail {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut detail = SessionDetail::default();
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
                        detail.description = EventId(id).description();
                    }
                }
            }

            Ok(XmlEvent::Start(ref e)) if e.name().as_ref() == b"Data" => {
                current_data_name_bytes = b"";
                for attr in e.attributes().flatten() {
                    if attr.key.as_ref() == b"Name" {
                        current_data_name_bytes = match attr.value.as_ref() {
                            b"User" => b"User",
                            b"SessionID" => b"SessionID",
                            b"Address" => b"Address", // RDP 来源IP (21/24/25)
                            b"ClientName" => b"ClientName", // RDP 客户端主机名 (1149)
                            b"Reason" => b"Reason",   // 断连原因码 (40)
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
                        b"User" => detail.user_name = text.to_owned(),
                        b"SessionID" => detail.session_id = text.to_owned(),
                        b"Address" => detail.remote_host = text.to_owned(),
                        b"ClientName" => {
                            if detail.remote_host.is_empty() {
                                detail.remote_host = text.to_owned();
                            }
                        }
                        b"Reason" => detail.reason = text.to_owned(),
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
