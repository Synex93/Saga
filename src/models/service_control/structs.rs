use crate::parser::definition::FieldMeta;

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
