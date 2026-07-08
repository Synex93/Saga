mod cfg;
mod cli;
mod out;
mod parser;

use cli::parser::*;
use parser::flow::run_parser;

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .max_blocking_threads(4)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let cfg = parser();
        let start = std::time::Instant::now();
        run_parser(cfg).await;
        println!("总耗时: {:?}", start.elapsed());
    });
}

#[cfg(test)]
mod test;
