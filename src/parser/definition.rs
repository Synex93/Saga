use std::borrow::Cow;

pub struct FieldMeta {
    pub title: &'static str,
    // TODO: 后续可以增加字段宽度
}

pub trait EventRecord {
    // general
    fn time(&self) -> &str;
    fn type_name(&self) -> &'static str;

    fn fields(&self) -> Vec<(&'static str, Cow<'_, str>)>;

    fn csv_header(&self) -> String {
        self.fields()
            .iter()
            .map(|(t, _)| *t)
            .collect::<Vec<_>>()
            .join(",")
    }
    fn to_csv_row(&self) -> String {
        self.fields()
            .iter()
            .map(|(_, v)| {
                let v = v.as_ref();
                if v.contains(',') || v.contains('"') || v.contains('\n') || v.contains('\r') {
                    format!("\"{}\"", v.replace('"', "\"\""))
                } else {
                    v.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(",")
    }
}
