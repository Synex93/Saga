// use crate::out::tui::run_tui;
use super::csv::to_csv;
use crate::cfg::sturct::OutFormat;

pub fn run(
    data: Vec<crate::parser::models::definition::ParserResult>,
    total: usize,
    of: OutFormat,
) {
    match of {
        // 先仅支持csv
        OutFormat::Csv => {
            to_csv(&data);
        }
        _ => {
            println!("暂未支持");
        }
    }
}
