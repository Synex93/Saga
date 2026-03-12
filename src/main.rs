mod cfg;
mod cli;
mod out;
mod parser;

use cli::parser::*;
use parser::flow::run_parser;
use std::time::Instant;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let cfg = parser();

    let start = Instant::now();

    run_parser(cfg).await;

    let duration = start.elapsed();

    println!("总耗时: {:?}", duration);
}
#[cfg(test)]
mod test;
