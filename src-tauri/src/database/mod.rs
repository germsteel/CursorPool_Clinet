use rusqlite::{params, Connection, Result as SqliteResult};
use std::sync::{Arc, Mutex};
use tauri::AppHandle;
use tauri::Manager;
use tracing::error;

#[derive(Debug, Clone)]
pub struct Database {
    connection: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new(app_handle: &AppHandle) -> SqliteResult<Self> {
        // 获取应用数据目录
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .expect("无法获取应用数据目录");

        // 确保目录存在
        std::fs::create_dir_all(&app_dir).expect("无法创建数据目录");

        // 数据库文件路径
        let db_path = app_dir.join("cursor_pool.db");

        // 创建或打开数据库连接
        let connection = Connection::open(&db_path).map_err(|e| {
            error!(target: "database", "打开数据库连接失败 - 路径: {:?}, 错误: {}", db_path, e);
            e
        })?;

        // 初始化表结构
        connection.execute(
            "CREATE TABLE IF NOT EXISTS item (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        ).map_err(|e| {
            error!(target: "database", "创建item表失败: {}", e);
            e
        })?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS account (
                account TEXT PRIMARY KEY,
                userId TEXT NOT NULL,
                cursorToken TEXT NOT NULL
            )",
            [],
        ).map_err(|e| {
            error!(target: "database", "创建account表失败: {}", e);
            e
        })?;

        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    // item表操作

    pub fn set_item(&self, key: &str, value: &str) -> SqliteResult<()> {
        let conn = self.connection.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO item (key, value) VALUES (?, ?)",
            params![key, value],
        ).map_err(|e| {
            error!(target: "database", "设置数据项失败 - 键: {}, 错误: {}", key, e);
            e
        })?;
        Ok(())
    }

    pub fn get_item(&self, key: &str) -> SqliteResult<Option<String>> {
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM item WHERE key = ?").map_err(|e| {
            error!(target: "database", "准备查询语句失败 - 键: {}, 错误: {}", key, e);
            e
        })?;
        
        let mut rows = stmt.query(params![key]).map_err(|e| {
            error!(target: "database", "执行查询失败 - 键: {}, 错误: {}", key, e);
            e
        })?;

        match rows.next().map_err(|e| {
            error!(target: "database", "获取查询结果失败 - 键: {}, 错误: {}", key, e);
            e
        })? {
            Some(row) => row.get(0).map(Some).map_err(|e| {
                error!(target: "database", "读取结果值失败 - 键: {}, 错误: {}", key, e);
                e
            }),
            None => Ok(None),
        }
    }

    pub fn delete_item(&self, key: &str) -> SqliteResult<()> {
        let conn = self.connection.lock().unwrap();
        conn.execute("DELETE FROM item WHERE key = ?", params![key]).map_err(|e| {
            error!(target: "database", "删除数据项失败 - 键: {}, 错误: {}", key, e);
            e
        })?;
        Ok(())
    }

    pub fn get_all_items(&self) -> SqliteResult<Vec<(String, String)>> {
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare("SELECT key, value FROM item").map_err(|e| {
            error!(target: "database", "准备获取所有数据项查询失败: {}", e);
            e
        })?;
        
        let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?))).map_err(|e| {
            error!(target: "database", "执行获取所有数据项查询失败: {}", e);
            e
        })?;

        let mut items = Vec::new();
        for item in rows {
            match item {
                Ok(i) => items.push(i),
                Err(e) => {
                    error!(target: "database", "获取数据项失败: {}", e);
                    return Err(e);
                }
            }
        }

        Ok(items)
    }

    // account表操作

    pub fn add_account(
        &self,
        account: &str,
        user_id: &str,
        cursor_token: &str,
    ) -> SqliteResult<()> {
        let conn = self.connection.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO account (account, userId, cursorToken) VALUES (?, ?, ?)",
            params![account, user_id, cursor_token],
        ).map_err(|e| {
            error!(target: "database", "添加账户失败 - 账户: {}, 错误: {}", account, e);
            e
        })?;
        Ok(())
    }

    pub fn get_account(&self, account: &str) -> SqliteResult<Option<(String, String, String)>> {
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare("SELECT account, userId, cursorToken FROM account WHERE account = ?").map_err(|e| {
            error!(target: "database", "准备查询账户语句失败 - 账户: {}, 错误: {}", account, e);
            e
        })?;
        
        let mut rows = stmt.query(params![account]).map_err(|e| {
            error!(target: "database", "执行查询账户失败 - 账户: {}, 错误: {}", account, e);
            e
        })?;

        match rows.next().map_err(|e| {
            error!(target: "database", "获取账户查询结果失败 - 账户: {}, 错误: {}", account, e);
            e
        })? {
            Some(row) => {
                let res = (row.get(0)?, row.get(1)?, row.get(2)?);
                Ok(Some(res))
            },
            None => Ok(None),
        }
    }

    pub fn delete_account(&self, account: &str) -> SqliteResult<()> {
        let conn = self.connection.lock().unwrap();
        conn.execute("DELETE FROM account WHERE account = ?", params![account]).map_err(|e| {
            error!(target: "database", "删除账户失败 - 账户: {}, 错误: {}", account, e);
            e
        })?;
        Ok(())
    }

    pub fn get_all_accounts(&self) -> SqliteResult<Vec<(String, String, String)>> {
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare("SELECT account, userId, cursorToken FROM account").map_err(|e| {
            error!(target: "database", "准备获取所有账户查询失败: {}", e);
            e
        })?;
        
        let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?))).map_err(|e| {
            error!(target: "database", "执行获取所有账户查询失败: {}", e);
            e
        })?;

        let mut accounts = Vec::new();
        for account in rows {
            match account {
                Ok(a) => accounts.push(a),
                Err(e) => {
                    error!(target: "database", "获取账户信息失败: {}", e);
                    return Err(e);
                }
            }
        }

        Ok(accounts)
    }

    pub fn update_account_token(&self, account: &str, cursor_token: &str) -> SqliteResult<()> {
        let conn = self.connection.lock().unwrap();
        conn.execute(
            "UPDATE account SET cursorToken = ? WHERE account = ?",
            params![cursor_token, account],
        ).map_err(|e| {
            error!(target: "database", "更新账户令牌失败 - 账户: {}, 错误: {}", account, e);
            e
        })?;
        Ok(())
    }
}
