use crate::utils::paths::AppPaths;
use rusqlite::{Connection, Result as SqliteResult};

pub struct CursorVersion;

impl CursorVersion {
    /// 从 SQLite 数据库中获取 Cursor 版本号
    pub fn get_version() -> Result<String, String> {
        let paths = AppPaths::new()?;

        // 检查数据库文件是否存在
        if !paths.db.exists() {
            return Ok("Unknown".to_string());
        }

        // 尝试连接数据库
        let conn = Connection::open(&paths.db).map_err(|e| format!("无法打开数据库: {}", e))?;

        // 首先尝试从 lastVersion 获取版本号
        match Self::get_last_version(&conn) {
            Ok(Some(version)) => return Ok(version),
            Ok(None) => {}
            Err(e) => return Err(format!("查询 lastVersion 失败: {}", e)),
        }

        // 如果 lastVersion 不存在，尝试从 releaseNotes 获取版本号
        match Self::get_version_from_release_notes(&conn) {
            Ok(Some(version)) => Ok(version),
            Ok(None) => Ok("Unknown".to_string()),
            Err(e) => Err(format!("查询 releaseNotes 失败: {}", e)),
        }
    }

    /// 从 lastVersion 字段获取版本号
    fn get_last_version(conn: &Connection) -> SqliteResult<Option<String>> {
        let mut stmt = conn.prepare("SELECT value FROM ItemTable WHERE key = 'lastVersion'")?;
        let mut rows = stmt.query([])?;

        if let Some(row) = rows.next()? {
            let version: String = row.get(0)?;
            Ok(Some(version))
        } else {
            Ok(None)
        }
    }

    /// 从 releaseNotes 字段解析版本号
    fn get_version_from_release_notes(conn: &Connection) -> SqliteResult<Option<String>> {
        let mut stmt = conn.prepare("SELECT value FROM ItemTable WHERE key = 'releaseNotes'")?;
        let mut rows = stmt.query([])?;

        if let Some(row) = rows.next()? {
            let notes: String = row.get(0)?;

            // 尝试从 releaseNotes 中提取版本号
            // 通常格式为 "Cursor v0.X.Y" 或类似格式
            if let Some(version) = Self::extract_version_from_notes(&notes) {
                Ok(Some(version))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// 从发布说明中提取版本号
    fn extract_version_from_notes(notes: &str) -> Option<String> {
        // 尝试匹配常见的版本号格式
        let patterns = ["Cursor v", "Version ", "v"];

        for pattern in patterns {
            if let Some(pos) = notes.find(pattern) {
                let start = pos + pattern.len();
                let end = notes[start..]
                    .find(|c: char| !c.is_digit(10) && c != '.')
                    .map_or(notes.len(), |p| start + p);

                if start < end {
                    let version = notes[start..end].trim();
                    if !version.is_empty() {
                        return Some(version.to_string());
                    }
                }
            }
        }

        None
    }
}
