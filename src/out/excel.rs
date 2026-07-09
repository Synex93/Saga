use crate::parser::definition::EventRecord;
use chrono::Local;
use rust_xlsxwriter::Workbook;

pub fn to_excel(data: &[Box<dyn EventRecord + Send>]) {
    if data.is_empty() {
        return;
    }

    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet_with_constant_memory();

    for (col, (title, _)) in data[0].fields().iter().enumerate() {
        worksheet
            .write_string(0, col as u16, *title)
            .expect("写入表头失败");
    }

    for (row_idx, rec) in data.iter().enumerate() {
        for (col, (_, val)) in rec.fields().iter().enumerate() {
            worksheet
                .write_string((row_idx as u32) + 1, col as u16, val.as_ref())
                .expect("写入数据失败");
        }
    }

    let now = Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("{}_{}.xlsx", data[0].type_name(), now);
    workbook.save(filename).expect("写入 Excel 失败");
}
