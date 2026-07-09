use crate::parser::definition::FieldMeta;

#[derive(Debug, Default)]
pub struct AccountManagementDetail {
    pub time: String,
    pub event_id: u16,
    pub description: &'static str,
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
