use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

const MAX_ATTEMPTS: i32 = 2;
const RETRY_DELAY: Duration = Duration::from_secs(1);
const CURSOR_POOL_NAME: &str = "cursor-pool";

pub struct ProcessManager;

impl ProcessManager {
    pub fn new() -> Self {
        ProcessManager
    }

    /// 检查Cursor进程是否正在运行
    pub fn is_cursor_running(&self) -> bool {
        if let Ok(processes) = self.get_cursor_processes() {
            !processes.is_empty()
        } else {
            false
        }
    }

    /// 终止所有Cursor进程
    pub fn kill_cursor_processes(&self) -> Result<(), String> {
        for attempt in 1..=MAX_ATTEMPTS {
            let processes = self
                .get_cursor_processes()
                .map_err(|e| format!("获取进程列表失败: {}", e))?;

            if processes.is_empty() {
                thread::sleep(Duration::from_secs(1));
                return Ok(());
            }

            // 直接强制结束进程
            for pid in &processes {
                let _ = self.kill_process(pid);
                thread::sleep(Duration::from_millis(200));
            }

            thread::sleep(RETRY_DELAY);

            // 检查是否还有进程存在
            if let Ok(remaining) = self.get_cursor_processes() {
                if remaining.is_empty() {
                    thread::sleep(Duration::from_secs(1));
                    return Ok(());
                }
            }

            if attempt == MAX_ATTEMPTS {
                return Err("达到最大重试次数,仍无法终止所有Cursor进程".to_string());
            }
        }

        Ok(())
    }

    /// 检查是否有其他 Cursor Pool 实例在运行
    pub fn is_other_cursor_pool_running(&self) -> bool {
        if let Ok(processes) = self.get_cursor_pool_processes() {
            processes.len() > 1 // 大于1说明有其他实例
        } else {
            false
        }
    }

    /// 终止其他所有 Cursor Pool 实例
    pub fn kill_other_cursor_pool_processes(&self) -> Result<(), String> {
        let current_pid = std::process::id().to_string();

        if let Ok(processes) = self.get_cursor_pool_processes() {
            for pid in processes {
                // 跳过当前进程
                if pid == current_pid {
                    continue;
                }

                if let Err(e) = self.kill_process(&pid) {
                    eprintln!("终止进程 {} 失败: {}", pid, e);
                }
                thread::sleep(Duration::from_millis(200));
            }
        }

        Ok(())
    }

    /// 获取所有 Cursor Pool 进程的 PID
    fn get_cursor_pool_processes(&self) -> Result<Vec<String>, String> {
        let (cmd, args) = match std::env::consts::OS {
            "windows" => ("tasklist", vec!["/FO", "CSV", "/NH"]),
            "macos" => ("ps", vec!["-ax"]),
            "linux" => ("ps", vec!["-A"]),
            _ => return Err("不支持的操作系统".to_string()),
        };

        let mut command = self.create_hidden_command(cmd);
        let output = command
            .args(&args)
            .output()
            .map_err(|e| format!("执行命令失败: {}", e))?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        Ok(self.parse_cursor_pool_processes(&output_str))
    }

    /// 解析 Cursor Pool 进程列表
    fn parse_cursor_pool_processes(&self, output: &str) -> Vec<String> {
        let mut processes = Vec::new();

        for line in output.lines() {
            let lower_line = line.to_lowercase();

            // 检查是否为 Cursor Pool 进程
            if lower_line.contains(CURSOR_POOL_NAME) {
                if let Some(pid) = self.extract_pid(line) {
                    processes.push(pid);
                }
            }
        }

        processes
    }

    /// 在Windows平台上，创建不显示窗口的命令
    fn create_hidden_command(&self, cmd: &str) -> Command {
        let mut command = Command::new(cmd);

        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            command.creation_flags(CREATE_NO_WINDOW);
        }

        // 重定向标准输入输出，进一步确保不显示窗口
        command
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());

        command
    }

    /// 获取所有Cursor进程的PID
    fn get_cursor_processes(&self) -> Result<Vec<String>, String> {
        let (cmd, args) = match std::env::consts::OS {
            "windows" => ("tasklist", vec!["/FO", "CSV", "/NH"]),
            "macos" => ("ps", vec!["-ax"]),
            "linux" => ("ps", vec!["-A"]),
            _ => return Err("不支持的操作系统".to_string()),
        };

        let mut command = self.create_hidden_command(cmd);
        let output = command
            .args(&args)
            .output()
            .map_err(|e| format!("执行命令失败: {}", e))?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        Ok(self.parse_process_list(&output_str))
    }

    /// 解析进程列表输出
    fn parse_process_list(&self, output: &str) -> Vec<String> {
        let mut processes = Vec::new();

        for line in output.lines() {
            let lower_line = line.to_lowercase();

            // 跳过自身进程
            if lower_line.contains("cursor-pool") {
                continue;
            }

            // 检查是否为Cursor进程
            if let Some(pid) = self.find_cursor_process(line, &lower_line) {
                processes.push(pid);
            }
        }

        processes
    }

    /// 查找Cursor进程并返回PID
    fn find_cursor_process(&self, line: &str, lower_line: &str) -> Option<String> {
        let patterns = ["cursor.exe", "cursor ", "cursor", "*cursor*"];

        for pattern in &patterns {
            if self.match_pattern(lower_line, &pattern.to_lowercase()) {
                return self.extract_pid(line);
            }
        }
        None
    }

    /// 模式匹配
    fn match_pattern(&self, line: &str, pattern: &str) -> bool {
        if pattern.starts_with('*') && pattern.ends_with('*') {
            let search = &pattern[1..pattern.len() - 1];
            line.contains(search)
        } else if let Some(stripped) = pattern.strip_prefix('*') {
            line.ends_with(stripped)
        } else if let Some(stripped) = pattern.strip_suffix('*') {
            line.starts_with(stripped)
        } else {
            line == pattern
        }
    }

    /// 提取PID
    fn extract_pid(&self, line: &str) -> Option<String> {
        match std::env::consts::OS {
            "windows" => {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 2 {
                    Some(parts[1].trim_matches('"').to_string())
                } else {
                    None
                }
            }
            "macos" | "linux" => {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if !parts.is_empty() {
                    Some(parts[0].to_string())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// 终止进程
    fn kill_process(&self, pid: &str) -> Result<(), String> {
        let (cmd, args) = match std::env::consts::OS {
            "windows" => ("taskkill", vec!["/F", "/PID", pid]),
            "macos" | "linux" => ("kill", vec!["-9", pid]),
            _ => return Err("不支持的操作系统".to_string()),
        };

        let mut command = self.create_hidden_command(cmd);
        command
            .args(&args)
            .output()
            .map_err(|e| format!("终止进程失败: {}", e))?;

        Ok(())
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_manager() {
        let manager = ProcessManager::new();

        // 测试检测Cursor进程
        println!("Cursor是否运行: {}", manager.is_cursor_running());

        // 测试获取进程列表
        if let Ok(processes) = manager.get_cursor_processes() {
            println!("找到的Cursor进程: {:?}", processes);
        }

        // 测试终止进程
        match manager.kill_cursor_processes() {
            Ok(_) => println!("成功终止所有Cursor进程"),
            Err(e) => println!("终止进程失败: {}", e),
        }
    }

    #[test]
    fn test_pattern_matching() {
        let manager = ProcessManager::new();

        assert!(manager.match_pattern("cursor", "cursor"));
        assert!(manager.match_pattern("cursor.exe", "*cursor*"));
        assert!(manager.match_pattern("cursor process", "*cursor*"));
        assert!(manager.match_pattern("cursor.exe", "cursor*"));
        assert!(!manager.match_pattern("other process", "*cursor*"));
    }
}
