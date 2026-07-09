mod cfg;
mod cli;
mod models;
mod out;
mod parser;

use cli::parser::*;
use parser::flow::run_parser;

fn main() {
    let cfg = parser();

    let rt = tokio::runtime::Builder::new_multi_thread()
        // async 侧只有 collector 协程做轻量调度（channel 收发 + 批次分发），
        // 不需要更多 worker 线程。evtx 解码和 XML 解析都在 blocking 线程池，由 -j 控制。
        .worker_threads(4)
        .max_blocking_threads(cfg.jobs)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let start = std::time::Instant::now();
        run_parser(cfg).await;
        println!("总耗时: {:?}", start.elapsed());
    });
}

#[cfg(test)]
mod test;
