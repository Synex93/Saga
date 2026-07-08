use super::definition::FieldMeta;
use crate::cfg::event::EventId;
use quick_xml::events::Event as XmlEvent;
use quick_xml::reader::Reader;

#[derive(Debug, Default)]
pub struct ServiceControlDetail {
    pub time: String,
    pub event_id: u16,
    pub description: &'static str,
    pub service_name: String,
    pub service_type: String,
    pub start_type: String,
    pub image_path: String,
    pub account_name: String,
}

pub struct ServiceControlMeta {
    pub time: FieldMeta,
    pub event_id: FieldMeta,
    pub description: FieldMeta,
    pub service_name: FieldMeta,
    pub service_type: FieldMeta,
    pub start_type: FieldMeta,
    pub image_path: FieldMeta,
    pub account_name: FieldMeta,
}

pub static SERVICE_CONTROL_META: ServiceControlMeta = ServiceControlMeta {
    time: FieldMeta { title: "时间" },
    event_id: FieldMeta { title: "事件ID" },
    description: FieldMeta { title: "描述" },
    service_name: FieldMeta {
        title: "服务名称"
    },
    service_type: FieldMeta {
        title: "服务类型"
    },
    start_type: FieldMeta {
        title: "启动类型"
    },
    image_path: FieldMeta {
        title: "可执行路径",
    },
    account_name: FieldMeta {
        title: "运行账户"
    },
};

impl ServiceControlDetail {
    pub fn csv_header() -> String {
        let m = &SERVICE_CONTROL_META;
        format!(
            "{},{},{},{},{},{},{},{}",
            m.time.title,
            m.event_id.title,
            m.description.title,
            m.service_name.title,
            m.service_type.title,
            m.start_type.title,
            m.image_path.title,
            m.account_name.title,
        )
    }

    pub fn to_csv_row(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{}",
            self.time,
            self.event_id,
            self.description,
            self.service_name,
            self.service_type,
            self.start_type,
            self.image_path,
            self.account_name,
        )
    }
}

pub fn parse_service_control(xml: &str) -> ServiceControlDetail {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut detail = ServiceControlDetail::default();
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
                            b"ServiceName" => b"ServiceName",
                            b"ServiceType" => b"ServiceType",
                            b"StartType" => b"StartType",
                            b"ImagePath" => b"ImagePath",
                            b"AccountName" => b"AccountName",
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
                        b"ServiceName" => detail.service_name = text.to_owned(),
                        b"ServiceType" => detail.service_type = text.to_owned(),
                        b"StartType" => detail.start_type = text.to_owned(),
                        b"ImagePath" => detail.image_path = text.to_owned(),
                        b"AccountName" => detail.account_name = text.to_owned(),
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
