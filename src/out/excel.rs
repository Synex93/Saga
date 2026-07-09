// use crate::parser::definition::EventRecord;
// use chrono::Local;
// use std::path::Path;

// pub fn to_excel(data: &[Box<dyn EventRecord + Send>]) {
//     if data.is_empty() {
//         return;
//     }

//     let mut book = umya_spreadsheet::new_file();
//     let sheet = book.sheet_mut(0).unwrap();

//     for (col, (title, _)) in data[0].fields().iter().enumerate() {
//         sheet.cell_mut(((col as u32) + 1, 1)).set_value(*title);
//     }

//     for (row_idx, rec) in data.iter().enumerate() {
//         let row = (row_idx as u32) + 2;
//         for (col, (_, val)) in rec.fields().iter().enumerate() {
//             sheet
//                 .cell_mut(((col as u32) + 1, row))
//                 .set_value(val.as_ref());
//         }
//     }

//     let now = Local::now().format("%Y%m%d_%H%M%S");
//     let filename = format!("{}_{}.xlsx", data[0].type_name(), now);
//     umya_spreadsheet::writer::xlsx::write(&book, Path::new(&filename)).expect("写入 Excel 失败");
// }
use crate::parser::definition::EventRecord;
use chrono::Local;
use rust_xlsxwriter::Workbook;

pub fn to_excel(data: &[Box<dyn EventRecord + Send>]) {
    if data.is_empty() {
        return;
    }

    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

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
