use crate::cfg::sturct::*;
use clap::Parser;

#[derive(Parser)]
#[command(name = "Saga", about = "Saga - Windows事件日志分析工具")]
#[command(
    version,
    arg_required_else_help = true,
    disable_help_flag = true,
    disable_version_flag = true,
    disable_help_subcommand = true
)]
struct Args {
    /// 指定日志路径,默认为当前系统日志存放位置
    #[arg(short, long)]
    path: Option<String>,

    /// 模块选择
    #[command(subcommand)]
    models: Option<Models>,

    /// 输出模式
    #[arg(short, long, value_enum, default_value_t = OutFormat::Csv)]
    out: OutFormat,
    /// 显示帮助信息
    #[arg(short, long, action = clap::ArgAction::Help)]
    pub help: Option<bool>,

    /// 显示版本信息
    #[arg(short = 'V', long, action = clap::ArgAction::Version)]
    pub version: Option<bool>,
}

pub fn parser() -> Config {
    let args = Args::parse();

    let default_path = String::from("C:\\Windows\\System32\\winevt\\Logs");
    let final_path = args.path.unwrap_or(default_path);
    let out_fromat = args.out;

    match args.models {
        Some(m) => Config {
            path: final_path,
            model: m,
            format: out_fromat,
        },
        None => {
            eprintln!("模块参数为空，请使用-h查看帮助");
            std::process::exit(1);
        }
    }
}
