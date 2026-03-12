use super::utils::{filter_event_by_name, parse_event_id};
use crate::cfg::sturct::Config;
use crate::parser::xml::XmlParser;
use evtx::EvtxParser;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::task;

pub async fn run_parser(cfg: Config) {
    let rule = &cfg.model.get_model_rule();
    let all_files = list_evtx(&cfg.path);
    let files = filter_event_by_name(all_files, &rule.files);

    // 提高匹配效率
    let target_ids: Arc<HashSet<u16>> = Arc::new(rule.ids.clone().into_iter().collect());

    // 通道
    let (tx, mut rx) = mpsc::channel(256);

    // Arc+Mutex 安全共享计数器
    let total_num = Arc::new(Mutex::new(0u64));
    let mut handles = vec![];

    for fp in files {
        let total_counter = Arc::clone(&total_num);
        let ids = Arc::clone(&target_ids);

        let tx_clone = tx.clone();

        let handle = task::spawn_blocking(move || {
            let mut total_num = 0;
            if let Ok(mut parser) = EvtxParser::from_path(fp) {
                for record in parser.records() {
                    if let Ok(r) = record {
                        let xml = &r.data;

                        if let Some(id) = parse_event_id(xml) {
                            if ids.contains(&id) {
                                let _ = tx_clone.blocking_send(xml.clone());
                            }
                        }
                    }
                    total_num += 1;
                }
            }
            let mut num = total_counter.lock().unwrap();
            *num += total_num;
        });

        handles.push(handle);
    }

    // 单开spawn收集结果解析
    drop(tx);
    let collect_handle = tokio::spawn(async move {
        let mut results = Vec::new();
        while let Some(item) = rx.recv().await {
            results.push(cfg.model.parse(&item));
        }
        results
    });

    for handle in handles {
        let _ = handle.await;
    }

    crate::out::export::run(
        collect_handle.await.unwrap_or_default(),
        *total_num.lock().unwrap() as usize,
        cfg.format,
    );
    println!("总计扫描到的事件记录数: {}", *total_num.lock().unwrap());
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
