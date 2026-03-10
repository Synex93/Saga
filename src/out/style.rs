// use crate::parser::xml::ParserResult;
// use tabled::{Table, Tabled, settings::Style};

// #[derive(Tabled)]
// struct AuthRow {
//     #[tabled(rename = "时间")]
//     time: String,
//     #[tabled(rename = "事件ID")]
//     event_id: u16,
//     #[tabled(rename = "描述")]
//     description: String,
//     #[tabled(rename = "主体用户")]
//     subject_user_name: String,
//     #[tabled(rename = "目标用户")]
//     target_user_name: String,
//     #[tabled(rename = "来源IP")]
//     ip_address: String,
//     #[tabled(rename = "工作站")]
//     workstation_name: String,
//     #[tabled(rename = "登录类型", display = "display_logon_type")]
//     logon_type: Option<u32>,
//     #[tabled(rename = "状态码")]
//     status: String,
// }

// fn display_logon_type(v: &Option<u32>) -> String {
//     match v {
//         Some(2) => "交互式(2)".into(),
//         Some(3) => "网络(3)".into(),
//         Some(5) => "服务(5)".into(),
//         Some(10) => "RDP(10)".into(),
//         Some(n) => format!("未知({})", n),
//         None => "-".into(),
//     }
// }

// pub fn display_powershell_style(results: Vec<ParserResult>) -> String {
//     let rows: Vec<AuthRow> = results
//         .into_iter()
//         .filter_map(|r| {
//             if let ParserResult::Authentication(d) = r {
//                 Some(AuthRow {
//                     time: d
//                         .time
//                         .replace('T', " ")
//                         .split('.')
//                         .next()
//                         .unwrap_or("-")
//                         .to_string(),
//                     event_id: d.event_id,
//                     description: d.description,
//                     subject_user_name: d.subject_user_name,
//                     target_user_name: d.target_user_name,
//                     ip_address: if d.ip_address.is_empty() {
//                         "-".into()
//                     } else {
//                         d.ip_address
//                     },
//                     workstation_name: if d.workstation_name.is_empty() {
//                         "-".into()
//                     } else {
//                         d.workstation_name
//                     },
//                     logon_type: d.logon_type,
//                     status: if d.status.is_empty() {
//                         "-".into()
//                     } else {
//                         d.status
//                     },
//                 })
//             } else {
//                 None
//             }
//         })
//         .collect();

//     // println!("{}", Table::new(rows).with(Style::modern()));
//     Table::new(rows).with(Style::modern()).to_string()
// }
