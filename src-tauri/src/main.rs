// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::utils::privileges::{check_admin_privileges, request_admin_privileges};
use cursor_pool_lib::*;
use std::env;

fn main() {
    // Windows 平台下检查管理员权限
    #[cfg(target_os = "windows")]
    {
        if let Ok(false) = check_admin_privileges() {
            let exe_path = env::current_exe()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            if let Ok(true) = request_admin_privileges(&exe_path) {
                std::process::exit(0);
            }
        }
    }

    cursor_pool_lib::run()
}
