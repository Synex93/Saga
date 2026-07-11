#[cfg(test)]
mod tests {
    use crate::parser::flow::list_evtx;
    use evtx::EvtxParser;
    use std::sync::{Arc, Mutex};
    use std::time::Instant;
    use tokio::task;

    /// cargo test bench_parse -- --ignored --nocapture
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    #[ignore = "需要本机 Windows 系统日志及读取权限；手动执行 cargo test bench_parse -- --ignored --nocapture"]
    async fn bench_parse() {
        let log_path = r"C:\Windows\System32\winevt\Logs";
        let files = list_evtx(std::path::Path::new(log_path)).expect("读取日志目录失败");

        println!("找到 evtx 文件数: {}", files.len());

        let total = Arc::new(Mutex::new(0u64));
        let failed = Arc::new(Mutex::new(0u64));
        let mut handles = vec![];

        let start = Instant::now();

        for fp in files {
            let total = Arc::clone(&total);
            let failed = Arc::clone(&failed);

            let handle = task::spawn_blocking(move || {
                let mut t = 0u64;
                let mut f = 0u64;
                if let Ok(mut parser) = EvtxParser::from_path(fp) {
                    for record in parser.records() {
                        match record {
                            Ok(_) => t += 1,
                            Err(_) => f += 1,
                        }
                    }
                }
                *total.lock().unwrap() += t;
                *failed.lock().unwrap() += f;
            });

            handles.push(handle);
        }

        for h in handles {
            let _ = h.await;
        }

        let elapsed = start.elapsed();
        let total = *total.lock().unwrap();

        println!("─────────────────────────────────");
        println!("  成功记录数 : {}", total);
        println!("  失败记录数 : {}", *failed.lock().unwrap());
        println!("  总耗时     : {:.3}s", elapsed.as_secs_f64());
        println!(
            "  平均速度   : {:.0} 条/秒",
            total as f64 / elapsed.as_secs_f64()
        );
        println!("─────────────────────────────────");
    }
}
