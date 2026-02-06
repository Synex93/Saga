mod cfg;
mod cli;
mod control;

use cli::parser::*;
use control::evtx::run_parser;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let cfg = parser();

    let start = Instant::now();
    run_parser(cfg).await;
    let duration = start.elapsed();

    println!("总耗时: {:?}", duration);
}
