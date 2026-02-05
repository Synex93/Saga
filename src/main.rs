mod cfg;
mod cli;
use cli::parser::*;
fn main() {
    let cfg = parser();
    println!("{cfg:?}");
}
