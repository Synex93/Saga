// use crate::out::tui::run_tui;
use super::csv::to_csv;
use crate::cfg::sturct::OutFormat;
pub fn run(
    mut data: Vec<crate::parser::models::definition::ParserResult>,
    total: usize,
    of: OutFormat,
) {
    // 数据排序
    data.sort_by(|a, b| b.time().cmp(a.time()));
    match of {
        OutFormat::Csv => {
            to_csv(&data);
        }
        _ => {
            println!("暂未支持");
        }
    }
    println!("总计扫描到的事件记录数: {}", total);
}
