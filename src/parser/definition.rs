pub struct FieldMeta {
    pub title: &'static str,
    // TODO: 后续可以增加字段宽度
}

pub trait EventRecord {
    // general
    fn time(&self) -> &str;
    fn type_name(&self) -> &'static str;
    // csv
    fn csv_header(&self) -> String;
    fn to_csv_row(&self) -> String;
}
