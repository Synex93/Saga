use crate::cfg::sturct::Config;
use evtx::EvtxParser;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::task;

pub async fn run_parser(cfg: Config) {
    let rule = &cfg.model.get_model_rule();
    let all_files = list_evtx(&cfg.path);
    let files = get_matched_files(all_files, &rule.files);

    // 提高匹配效率
    let target_ids: Arc<HashSet<u16>> = Arc::new(rule.ids.clone().into_iter().collect());

    // Arc+Mutex 安全共享计数器
    let total_num = Arc::new(Mutex::new(0u64));
    let match_num = Arc::new(Mutex::new(0u64));
    let mut handles = vec![];

    for fp in files {
        let total_counter = Arc::clone(&total_num);
        let match_counter = Arc::clone(&match_num);
        let ids = Arc::clone(&target_ids);

        let handle = task::spawn_blocking(move || {
            let mut cache_total_num = 0;
            let mut cache_match_num = 0;
            if let Ok(mut parser) = EvtxParser::from_path(fp) {
                for record in parser.records() {
                    if let Ok(r) = record {
                        let xml = &r.data;

                        if let Some(id) = extract_event_id(xml) {
                            if ids.contains(&id) {
                                // println!("{xml:?}");
                                cache_match_num += 1;
                            }
                        }
                    }
                    cache_total_num += 1;
                }
            }
            let mut tc_num = total_counter.lock().unwrap();
            let mut mc_num = match_counter.lock().unwrap();
            *tc_num += cache_total_num;
            *mc_num += cache_match_num;
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    println!("总计扫描到的事件记录数: {}", *total_num.lock().unwrap());
    println!("总计匹配到的时间记录数: {}", *match_num.lock().unwrap());
}

fn list_evtx(dir: &str) -> Vec<PathBuf> {
    let ext = "evtx".to_string();
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_file() {
                if let Some(e) = path.extension() {
                    if e.eq_ignore_ascii_case(&ext) {
                        files.push(path);
                    }
                }
            }
        }
    }
    files
}

fn get_matched_files(all_files: Vec<PathBuf>, files: &[&str]) -> Vec<PathBuf> {
    // 提高匹配效率
    let allowed_files: HashSet<&str> = files.iter().cloned().collect();

    all_files
        .into_iter()
        .filter(|path| {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                allowed_files.contains(file_name)
            } else {
                false
            }
        })
        .collect()
}

fn extract_event_id(xml: &str) -> Option<u16> {
    let start_tag = "<EventID>";
    let end_tag = "</EventID>";

    let s_idx = xml.find(start_tag)? + start_tag.len();
    let e_idx = xml[s_idx..].find(end_tag)?;

    xml[s_idx..s_idx + e_idx].trim().parse::<u16>().ok()
}
