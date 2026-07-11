use crate::cfg::sturct::*;
use clap::Parser;
use std::ffi::OsString;
use std::path::PathBuf;

const DEFAULT_LOG_PATH: &str = r"C:\Windows\System32\winevt\Logs";

#[derive(Parser)]
#[command(name = "Saga", about = "Saga - Windows事件日志分析工具")]
#[command(
    version,
    arg_required_else_help = true,
    disable_help_flag = true,
    disable_version_flag = true,
    disable_help_subcommand = true,
    infer_subcommands = true
)]
struct Args {
    /// 指定日志路径，默认为当前系统日志存放位置
    #[arg(short, long, global = true, value_parser = parse_path_argument)]
    path: Option<PathBuf>,

    /// 模块选择
    #[command(subcommand)]
    models: Option<Models>,

    /// 输出模式
    #[arg(short, long, global = true, value_enum, default_value_t = OutFormat::Excel)]
    out: OutFormat,
    /// 并发线程数，默认 4 (生产环境安全值)
    #[arg(short = 'j', long, global = true, default_value_t = 4)]
    jobs: usize,
    /// 仅供自动提权重启使用，防止权限不足时重复请求 UAC。
    #[arg(long, hide = true, global = true)]
    elevated_retry: bool,
    /// 管理员子进程回传扫描结果的临时文件，仅供自动提权流程使用。
    #[arg(long, hide = true, global = true)]
    elevated_report: Option<PathBuf>,
    /// 显示帮助信息
    #[arg(short, long, action = clap::ArgAction::Help)]
    pub help: Option<bool>,

    /// 显示版本信息
    #[arg(short = 'V', long, action = clap::ArgAction::Version)]
    pub version: Option<bool>,
}

#[derive(Debug)]
pub struct ParsedArgs {
    pub config: Config,
    pub elevated_retry: bool,
    pub elevated_report: Option<PathBuf>,
}

pub fn parser() -> ParsedArgs {
    into_parsed_args(Args::parse_from(normalize_path_arguments(
        std::env::args_os(),
    )))
}

fn into_parsed_args(args: Args) -> ParsedArgs {
    let path = args.path.unwrap_or_else(|| PathBuf::from(DEFAULT_LOG_PATH));

    match args.models {
        Some(model) => ParsedArgs {
            config: Config {
                path,
                model,
                format: args.out,
                jobs: args.jobs,
            },
            elevated_retry: args.elevated_retry,
            elevated_report: args.elevated_report,
        },
        None => {
            eprintln!("模块参数为空，请使用 -h 查看帮助");
            std::process::exit(1);
        }
    }
}

fn parse_path_argument(value: &str) -> Result<PathBuf, String> {
    let path = strip_wrapping_quotes(value.trim());
    if path.is_empty() {
        return Err("日志路径不能为空".to_owned());
    }
    Ok(PathBuf::from(path))
}

/// 将全角/弯引号包围、但被终端拆分的 `-p` 路径重新合并为一个参数。
/// 半角双引号仍是 PowerShell/CMD 中推荐的标准写法。
fn normalize_path_arguments(arguments: impl IntoIterator<Item = OsString>) -> Vec<OsString> {
    let mut result = Vec::new();
    let mut arguments = arguments.into_iter().peekable();

    while let Some(argument) = arguments.next() {
        let is_path_option = matches!(argument.to_str(), Some("-p") | Some("--path"));
        result.push(argument);

        if !is_path_option {
            continue;
        }

        let Some(value) = arguments.next() else {
            break;
        };
        let Some((open, close)) = wrapping_quote_pair(value.to_string_lossy().as_ref()) else {
            result.push(value);
            continue;
        };

        let mut combined = value;
        while !combined.to_string_lossy().ends_with(close) {
            let Some(next) = arguments.next() else {
                break;
            };
            combined.push(" ");
            combined.push(next);
        }

        // 仅当开头确实是引号时才合并；未闭合的引号仍交给 Clap 正常报错。
        if combined.to_string_lossy().starts_with(open) {
            result.push(combined);
        } else {
            unreachable!("路径参数必须以已检测到的引号开头");
        }
    }

    result
}

fn wrapping_quote_pair(value: &str) -> Option<(char, char)> {
    match value.chars().next()? {
        '"' => Some(('"', '"')),
        '\'' => Some(('\'', '\'')),
        '“' => Some(('“', '”')),
        '‘' => Some(('‘', '’')),
        _ => None,
    }
}

/// 兼容某些终端或复制粘贴后被原样传入的中英文引号。
fn strip_wrapping_quotes(value: &str) -> &str {
    let Some((open, close)) = wrapping_quote_pair(value) else {
        return value;
    };
    let Some(last) = value.chars().next_back() else {
        return value;
    };

    if last == close && value.len() >= open.len_utf8() + close.len_utf8() {
        &value[open.len_utf8()..value.len() - close.len_utf8()]
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_global_options_after_subcommand() {
        let args = Args::try_parse_from([
            "Saga.exe",
            "Auth",
            "-p",
            r"D:\Case Files\Windows 日志",
            "-o",
            "csv",
            "-j",
            "8",
        ])
        .expect("模块后的全局参数应能解析");
        let parsed = into_parsed_args(args);

        assert_eq!(parsed.config.model.command_name(), "Authentication");
        assert_eq!(
            parsed.config.path,
            PathBuf::from(r"D:\Case Files\Windows 日志")
        );
        assert_eq!(parsed.config.format, OutFormat::Csv);
        assert_eq!(parsed.config.jobs, 8);
    }

    #[test]
    fn accepts_global_options_before_subcommand() {
        let args = Args::try_parse_from(["Saga.exe", "-p", r"D:\日志", "Authentication"])
            .expect("模块前的全局参数应能解析");
        let parsed = into_parsed_args(args);

        assert_eq!(parsed.config.path, PathBuf::from(r"D:\日志"));
    }

    #[test]
    fn joins_full_width_quoted_path_split_by_terminal() {
        let arguments = normalize_path_arguments([
            OsString::from("Saga.exe"),
            OsString::from("Auth"),
            OsString::from("-p"),
            OsString::from("“D:\\Case"),
            OsString::from("Files\\事件日志”"),
        ]);

        assert_eq!(arguments[3], OsString::from("“D:\\Case Files\\事件日志”"));
        let args = Args::try_parse_from(arguments).expect("全角引号路径应能解析");
        assert_eq!(
            into_parsed_args(args).config.path,
            PathBuf::from(r"D:\Case Files\事件日志")
        );
    }

    #[test]
    fn removes_wrapping_english_and_chinese_quotes() {
        assert_eq!(
            strip_wrapping_quotes(r#""D:\Case Files""#),
            r"D:\Case Files"
        );
        assert_eq!(
            strip_wrapping_quotes("“D:\\案件\\事件日志”"),
            r"D:\案件\事件日志"
        );
        assert_eq!(strip_wrapping_quotes("‘D:\\日志’"), r"D:\日志");
    }
}
