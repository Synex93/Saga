use crate::parser::definition::FieldMeta;

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
