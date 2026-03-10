use std::collections::HashSet;
use std::path::PathBuf;

// 获取EventID
pub fn parse_event_id(xml: &str) -> Option<u16> {
    let start_tag = "<EventID>";
    let end_tag = "</EventID>";

    let s_idx = xml.find(start_tag)? + start_tag.len();
    let e_idx = xml[s_idx..].find(end_tag)?;

    xml[s_idx..s_idx + e_idx].trim().parse::<u16>().ok()
}

// 过滤文件名
pub fn filter_event_by_name(all_paths: Vec<PathBuf>, allowed_names: &[&str]) -> Vec<PathBuf> {
    let name_set: HashSet<&str> = allowed_names.iter().cloned().collect();

    all_paths
        .into_iter()
        .filter(|path| {
            path.file_name()
                .and_then(|n| n.to_str())
                .map_or(false, |name| name_set.contains(name))
        })
        .collect()
}
