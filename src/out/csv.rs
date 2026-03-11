// 数据导出CSV文件
use crate::parser::models::definition::{ParserResult, ToCsv};
use chrono::Local;
use std::fs;

pub fn to_csv(data: &[ParserResult]) {
    if data.is_empty() {
        return;
    }

    let now = Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("{}_{}.csv", data[0].type_name(), now);

    let mut csv = data[0].csv_header();
    csv.push('\n');
    for r in data {
        csv.push_str(&r.to_csv_row());
        csv.push('\n');
    }
    fs::write(&filename, csv).expect("写入 CSV 失败");
}
