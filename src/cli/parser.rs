use crate::cfg::sturct::*;
use clap::Parser;

#[derive(Parser)]
#[command(version, arg_required_else_help = true)]
struct Args {
    /// 日志路径
    #[arg(short, long)]
    path: Option<String>,

    /// 模块选择
    #[command(subcommand)]
    models: Option<Models>,
}

pub fn parser() -> Config {
    let args = Args::parse();

    let default_path = String::from("C:\\Windows\\System32\\winevt\\Logs");
    let final_path = args.path.unwrap_or(default_path);

    match args.models {
        Some(m) => Config {
            path: final_path,
            model: m,
        },
        None => {
            eprintln!("模块参数为空，请使用-h查看帮助");
            std::process::exit(1);
        }
    }
}
