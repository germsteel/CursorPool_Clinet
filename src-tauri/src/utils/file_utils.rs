use std::fs;
use std::io::Error;
use std::path::Path;
use std::process::Command;

/// 检查文件是否是只读
pub fn is_read_only(path: &Path) -> Result<bool, Error> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.permissions().readonly())
}

/// 设置文件为只读
pub fn set_read_only(path: &Path) -> Result<(), Error> {
    if cfg!(target_os = "macos") {
        // 使用 osascript 获取 root 权限设置只读
        let script = format!(
            "do shell script \"chmod a-w '{}'\" with prompt \"Cursor-Pool 需要获取权限来修改文件\"",
            path.to_string_lossy()
        );

        let output = Command::new("osascript")
            .arg("-e")
            .arg(script)
            .output()
            .map_err(|e| Error::new(std::io::ErrorKind::Other, e.to_string()))?;

        if !output.status.success() {
            return Err(Error::new(
                std::io::ErrorKind::PermissionDenied,
                format!("设置只读失败: {}", String::from_utf8_lossy(&output.stderr)),
            ));
        }
    } else {
        let mut permissions = fs::metadata(path)?.permissions();
        permissions.set_readonly(true);
        fs::set_permissions(path, permissions)?;
    }
    Ok(())
}

/// 取消文件只读属性
pub fn unset_read_only(path: &Path) -> Result<(), Error> {
    if cfg!(target_os = "macos") {
        // 使用 osascript 获取 root 权限取消只读
        let script = format!(
            "do shell script \"chmod u+w '{}'\" with prompt \"Cursor-Pool 需要获取权限来修改文件\"",
            path.to_string_lossy()
        );

        let output = Command::new("osascript")
            .arg("-e")
            .arg(script)
            .output()
            .map_err(|e| Error::new(std::io::ErrorKind::Other, e.to_string()))?;

        if !output.status.success() {
            return Err(Error::new(
                std::io::ErrorKind::PermissionDenied,
                format!("取消只读失败: {}", String::from_utf8_lossy(&output.stderr)),
            ));
        }
    } else {
        let mut permissions = fs::metadata(path)?.permissions();
        permissions.set_readonly(false);
        fs::set_permissions(path, permissions)?;
    }
    Ok(())
}

/// 安全写入文件, 自动处理只读属性
pub fn safe_write(path: &Path, content: &str) -> Result<(), Error> {
    // 检查文件是否存在
    if path.exists() {
        // 如果是只读文件, 先取消只读属性
        if is_read_only(path)? {
            unset_read_only(path)?;
        }
    }

    // 写入文件
    fs::write(path, content)?;

    // 设置文件为只读
    // set_read_only(path)?;

    Ok(())
}
