use crate::parser::definition::FieldMeta;

#[derive(Debug, Default)]
pub struct AuthenticationDetail {
    pub time: String,
    pub event_id: u16,
    pub description: &'static str,
    pub subject_user_name: String,
    pub target_user_name: String,
    pub ip_address: String,
    pub logon_type: Option<u32>,
    pub status: String,
    pub raw_data: String,
}

pub struct AuthenticationMeta {
    pub time: FieldMeta,
    pub event_id: FieldMeta,
    pub description: FieldMeta,
    pub subject_user_name: FieldMeta,
    pub target_user_name: FieldMeta,
    pub ip_address: FieldMeta,
    pub logon_type: FieldMeta,
    pub status: FieldMeta,
    pub raw_data: FieldMeta,
}

// 解析元数据
pub static AUTH_META: AuthenticationMeta = AuthenticationMeta {
    time: FieldMeta { title: "时间" },
    event_id: FieldMeta { title: "事件ID" },
    description: FieldMeta { title: "描述" },
    subject_user_name: FieldMeta {
        title: "主体用户"
    },
    target_user_name: FieldMeta {
        title: "目标用户"
    },
    ip_address: FieldMeta { title: "来源IP" },
    logon_type: FieldMeta {
        title: "登录类型"
    },
    status: FieldMeta { title: "状态码" },
    raw_data: FieldMeta {
        title: "详细信息"
    },
};
