#[cfg(windows)]
use std::ffi::{OsStr, OsString};
use std::io;
#[cfg(windows)]
use std::iter;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;

pub struct ElevatedProcessResult {
    pub exit_code: u32,
    pub message: String,
}

/// 以管理员权限重新启动当前程序，等待其完成并读取回传的摘要。
///
/// 使用 ShellExecuteExW 请求 UAC。管理员子进程不能可靠地继承原 CMD 的标准输出，
/// 因此通过仅内部使用的结果文件，将扫描统计或错误信息显示回原始终端。
#[cfg(windows)]
pub fn restart_as_administrator() -> io::Result<ElevatedProcessResult> {
    use windows_sys::Win32::Foundation::CloseHandle;
    use windows_sys::Win32::System::Threading::{
        GetExitCodeProcess, INFINITE, WaitForSingleObject,
    };
    use windows_sys::Win32::UI::Shell::{
        SEE_MASK_NOCLOSEPROCESS, SHELLEXECUTEINFOW, ShellExecuteExW,
    };

    let executable = std::env::current_exe()?;
    let report_path = std::env::temp_dir().join(format!(
        "Saga-elevated-result-{}-{}.txt",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));

    let mut parameters = std::env::args_os()
        .skip(1)
        .map(|argument| quote_windows_argument(&argument))
        .collect::<Vec<_>>();
    parameters.push("--elevated-retry".to_owned());
    parameters.push("--elevated-report".to_owned());
    parameters.push(quote_windows_argument(
        &report_path.clone().into_os_string(),
    ));

    let operation = wide_null(OsStr::new("runas"));
    let executable = wide_null(executable.as_os_str());
    let parameters = wide_null(OsStr::new(&parameters.join(" ")));

    let mut execute_info: SHELLEXECUTEINFOW = unsafe { std::mem::zeroed() };
    execute_info.cbSize = std::mem::size_of::<SHELLEXECUTEINFOW>() as u32;
    execute_info.fMask = SEE_MASK_NOCLOSEPROCESS;
    execute_info.lpVerb = operation.as_ptr();
    execute_info.lpFile = executable.as_ptr();
    execute_info.lpParameters = parameters.as_ptr();
    execute_info.nShow = 1;

    // ShellExecuteExW 返回 0 表示失败（包括用户取消 UAC）。
    if unsafe { ShellExecuteExW(&mut execute_info) } == 0 {
        return Err(io::Error::last_os_error());
    }
    if execute_info.hProcess.is_null() {
        return Err(io::Error::other("管理员进程未返回可等待的进程句柄"));
    }

    unsafe {
        WaitForSingleObject(execute_info.hProcess, INFINITE);
    }

    let mut exit_code = 1;
    let get_exit_code_ok =
        unsafe { GetExitCodeProcess(execute_info.hProcess, &mut exit_code) } != 0;
    unsafe {
        CloseHandle(execute_info.hProcess);
    }
    if !get_exit_code_ok {
        return Err(io::Error::last_os_error());
    }

    let message = std::fs::read_to_string(&report_path).unwrap_or_else(|_| {
        "管理员进程已结束，但未能收到执行结果。请确认临时目录可写。".to_owned()
    });
    let _ = std::fs::remove_file(report_path);

    Ok(ElevatedProcessResult { exit_code, message })
}

#[cfg(windows)]
fn wide_null(value: &OsStr) -> Vec<u16> {
    value.encode_wide().chain(iter::once(0)).collect()
}

/// 按 Windows CommandLineToArgvW 规则引用一个参数，保留空格、反斜杠和双引号。
#[cfg(windows)]
fn quote_windows_argument(argument: &OsString) -> String {
    let argument = argument.to_string_lossy();
    let mut result = String::with_capacity(argument.len() + 2);
    result.push('"');

    let mut backslashes = 0;
    for character in argument.chars() {
        match character {
            '\\' => backslashes += 1,
            '"' => {
                result.extend(std::iter::repeat_n('\\', backslashes * 2 + 1));
                result.push('"');
                backslashes = 0;
            }
            _ => {
                result.extend(std::iter::repeat_n('\\', backslashes));
                result.push(character);
                backslashes = 0;
            }
        }
    }

    result.extend(std::iter::repeat_n('\\', backslashes * 2));
    result.push('"');
    result
}

#[cfg(not(windows))]
pub fn restart_as_administrator() -> io::Result<ElevatedProcessResult> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "自动请求管理员权限仅支持 Windows",
    ))
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;

    #[test]
    fn quotes_arguments_for_windows_command_line() {
        assert_eq!(quote_windows_argument(&OsString::from("Auth")), "\"Auth\"");
        assert_eq!(
            quote_windows_argument(&OsString::from(r"D:\Case Files")),
            r#""D:\Case Files""#
        );
    }
}
