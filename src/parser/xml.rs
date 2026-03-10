use crate::cfg::event::EventId;
use quick_xml::events::Event as XmlEvent;
use quick_xml::reader::Reader;
#[derive(Debug)]
pub enum ParserResult {
    Authentication(AuthDetail),
    Session,
    AccountManagement,
}
pub trait XmlParser {
    fn parse(&self, xml: &str) -> ParserResult;
}
impl XmlParser for crate::cfg::sturct::Models {
    fn parse(&self, xml: &str) -> ParserResult {
        match self {
            Self::Authentication => ParserResult::Authentication(parse_auth(xml)),
            Self::Session => ParserResult::Session,
            Self::AccountManagement => ParserResult::AccountManagement,
        }
    }
}

#[derive(Debug, Default)]
pub struct AuthDetail {
    pub time: String,
    pub event_id: u16,
    pub description: String,
    pub subject_user_name: String,
    pub subject_domain_name: String,
    pub target_user_name: String,
    pub target_domain_name: String,
    pub ip_address: String,
    pub workstation_name: String,
    pub logon_type: Option<u32>,
    pub process_name: String,
    pub status: String,
}

pub fn parse_auth(xml: &str) -> AuthDetail {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut detail = AuthDetail::default();
    let mut buf = Vec::with_capacity(512);

    // 用静态字符串标记当前 Data Name，匹配时直接比较 &[u8]
    let mut current_data_name_bytes: &'static [u8] = b"";

    loop {
        match reader.read_event_into(&mut buf) {
            // ── TimeCreated SystemTime ──────────────────────────────────
            Ok(XmlEvent::Start(ref e)) if e.name().as_ref() == b"TimeCreated" => {
                for attr in e.attributes().flatten() {
                    if attr.key.as_ref() == b"SystemTime" {
                        if let Ok(v) = attr.unescape_value() {
                            detail.time = v.into_owned();
                        }
                    }
                }
            }

            // ── EventID ────────────────────────────────────────────────
            Ok(XmlEvent::Start(ref e)) if e.name().as_ref() == b"EventID" => {
                if let Ok(text) = reader.read_text(e.name()) {
                    if let Ok(id) = text.parse::<u16>() {
                        detail.event_id = id;
                        detail.description = EventId(id).description().to_string();
                    }
                }
            }

            // ── Data Name="..." ────────────────────────────────────────
            // 只记录我们关心的字段名，其余直接忽略，不做 String 分配
            Ok(XmlEvent::Start(ref e)) if e.name().as_ref() == b"Data" => {
                current_data_name_bytes = b""; // 先清空
                for attr in e.attributes().flatten() {
                    if attr.key.as_ref() == b"Name" {
                        // 直接在字节层面匹配，零拷贝
                        current_data_name_bytes = match attr.value.as_ref() {
                            b"SubjectUserName" => b"SubjectUserName",
                            b"SubjectDomainName" => b"SubjectDomainName",
                            b"TargetUserName" => b"TargetUserName",
                            b"TargetDomainName" => b"TargetDomainName",
                            b"IpAddress" => b"IpAddress",
                            b"WorkstationName" => b"WorkstationName",
                            b"LogonType" => b"LogonType",
                            b"ProcessName" => b"ProcessName",
                            b"Status" => b"Status",
                            b"SubStatus" => b"SubStatus",
                            _ => b"", // 不关心的字段
                        };
                        break;
                    }
                }
            }

            // ── Text 内容 ──────────────────────────────────────────────
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
                        b"SubjectDomainName" => detail.subject_domain_name = text.to_owned(),
                        b"TargetUserName" => detail.target_user_name = text.to_owned(),
                        b"TargetDomainName" => detail.target_domain_name = text.to_owned(),
                        b"IpAddress" => detail.ip_address = text.to_owned(),
                        b"WorkstationName" => detail.workstation_name = text.to_owned(),
                        b"LogonType" => detail.logon_type = text.parse().ok(),
                        b"ProcessName" => detail.process_name = text.to_owned(),
                        b"Status" => detail.status = text.to_owned(),
                        b"SubStatus" => {
                            if detail.status.is_empty() {
                                detail.status = text.to_owned();
                            }
                        }
                        _ => {}
                    }
                }
                current_data_name_bytes = b"";
            }

            // ── End ────────────────────────────────────────────────────
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
