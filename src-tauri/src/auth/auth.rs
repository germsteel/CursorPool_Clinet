use crate::utils::{update_sqlite_db, AppPaths};
use serde_json::json;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
pub struct AuthInfo {
    pub email: String,
    pub token: String,
}

pub fn update_auth(paths: &AppPaths, auth: &AuthInfo) -> Result<(), String> {
    // 更新 auth.json
    if let Some(parent) = paths.auth.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
    }

    let auth_data = json!({
        "email": auth.email,
        "access_token": auth.token
    });

    fs::write(
        &paths.auth,
        serde_json::to_string_pretty(&auth_data).map_err(|e| format!("序列化JSON失败: {}", e))?,
    )
    .map_err(|e| format!("写入auth.json失败: {}", e))?;

    // 更新数据库中的认证信息
    let mut auth_updates = HashMap::new();
    auth_updates.insert("cursorAuth/refreshToken".to_string(), auth.token.clone());
    auth_updates.insert("cursorAuth/accessToken".to_string(), auth.token.clone());
    auth_updates.insert("cursorAuth/cachedEmail".to_string(), auth.email.clone());

    // 可选: 根据Cursor版本添加额外的字段
    auth_updates.insert("cursor.email".to_string(), auth.email.clone());
    auth_updates.insert("cursor.accessToken".to_string(), auth.token.clone());

    update_sqlite_db(&paths.db, &auth_updates)?;

    Ok(())
}
