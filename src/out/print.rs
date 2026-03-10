// pub fn run(data: Vec<crate::parser::xml::ParserResult>) {
//     let len = data.len();
//     display_powershell_style(data);
//     // for item in data {
//     // display_results(item);
//     // }
//     println!("匹配事件总数：{}", len)
// }

// use super::style::display_powershell_style;
// use minus::Pager;

// pub fn run(data: Vec<crate::parser::xml::ParserResult>) {
//     let len = data.len();
//     let output = display_powershell_style(data);

//     // let mut pager = Pager::new();
//     println!("{}", output);

//     // pager.push_str(&output).unwrap();
//     // pager.push_str(&format!("\n匹配事件总数：{}", len)).unwrap();
//     // minus::page_all(pager).unwrap();
// }

use crate::out::tui::run_tui;

pub fn run(data: Vec<crate::parser::xml::ParserResult>, total: usize) {
    run_tui(data, total).unwrap();
}
