use std::process::Command;

#[cfg(target_os = "windows")]
pub fn check_admin_privileges() -> Result<bool, String> {
    use windows::Win32::UI::Shell::IsUserAnAdmin;

    // 使用 Windows API 检查管理员权限
    unsafe { Ok(IsUserAnAdmin().as_bool()) }
}

#[cfg(target_os = "windows")]
pub fn request_admin_privileges(exe_path: &str) -> Result<bool, String> {
    use std::env;
    use std::os::windows::process::CommandExt;

    const CREATE_NO_WINDOW: u32 = 0x08000000;

    // 设置环境变量标记自动模式
    env::set_var("AUTOMATED_MODE", "1");

    let current_dir = env::current_dir().map_err(|e| format!("获取当前目录失败: {}", e))?;

    // 使用  Start-Process
    let result = Command::new("powershell")
        .creation_flags(CREATE_NO_WINDOW)
        .args(["Start-Process", exe_path, "-Verb", "runas"])
        .current_dir(current_dir)
        .spawn();

    match result {
        Ok(_) => {
            // println!("已请求管理员权限,等待 UAC 确认");
            Ok(true)
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                // println!("用户取消了 UAC 请求");
                Ok(false)
            } else {
                Err(format!("请求管理员权限失败: {}", e))
            }
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
pub fn check_admin_privileges() -> Result<bool, String> {
    let output = Command::new("id")
        .arg("-u")
        .output()
        .map_err(|e| format!("执行命令失败: {}", e))?;

    let uid = String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<u32>()
        .map_err(|e| format!("解析 UID 失败: {}", e))?;

    Ok(uid == 0)
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
pub fn request_admin_privileges(exe_path: &str) -> Result<bool, String> {
    use std::env;

    // 设置环境变量标记自动模式
    env::set_var("AUTOMATED_MODE", "1");

    let args: Vec<String> = env::args().skip(1).collect();

    let mut command = if cfg!(target_os = "macos") {
        let mut cmd = Command::new("osascript");
        cmd.arg("-e").arg(format!(
            "do shell script \"\\\"{}\\\" {}\" with prompt \"Cursor-Pool 需要获取权限来修改文件\"",
            exe_path,
            args.join(" ")
        ));
        cmd
    } else {
        let mut cmd = Command::new("pkexec");
        cmd.arg(exe_path);
        cmd.args(args);
        cmd
    };

    match command.spawn() {
        Ok(_) => Ok(true),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                Ok(false)
            } else {
                Err(format!("请求管理员权限失败: {}", e))
            }
        }
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
pub fn check_admin_privileges() -> Result<bool, String> {
    Err(format!("不支持的操作系统: {}", std::env::consts::OS))
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
pub fn request_admin_privileges(_exe_path: &str) -> Result<bool, String> {
    Err(format!("不支持的操作系统: {}", std::env::consts::OS))
}

// 添加新的函数来检测系统是否为 Windows
pub fn is_windows() -> bool {
    cfg!(target_os = "windows")
}
