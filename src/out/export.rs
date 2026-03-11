// use crate::out::tui::run_tui;
use super::csv::to_csv;

pub fn run(data: Vec<crate::parser::models::definition::ParserResult>, total: usize) {
    // 先仅支持csv
    to_csv(&data);
}
