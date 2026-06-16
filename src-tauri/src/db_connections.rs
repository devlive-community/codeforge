//! 数据库连接的独立存储表（codeforge.sqlite 中的 db_connections）。
//! 从 KV 的 sql-connections JSON blob 抽出，便于将来按需扩展/分页/检索。

use crate::execution::get_codeforge_db_path;
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};
use std::sync::Mutex as StdMutex;
use tauri::State;

#[derive(Serialize, Deserialize, Clone)]
pub struct DbConnection {
    pub id: String,
    pub name: String,
    pub kind: String,
    #[serde(default)]
    pub file: Option<String>,
    #[serde(default)]
    pub host: Option<String>,
    #[serde(default)]
    pub port: Option<u16>,
    #[serde(default)]
    pub user: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default)]
    pub database: Option<String>,
}

pub struct DbConnStore {
    conn: StdMutex<Connection>,
}

impl DbConnStore {
    pub fn new() -> Result<Self, String> {
        let db_path = get_codeforge_db_path()?;
        let conn = Connection::open(&db_path).map_err(|e| format!("打开数据库失败: {}", e))?;
        let _ = conn.pragma_update(None, "journal_mode", "WAL");
        let _ = conn.pragma_update(None, "synchronous", "NORMAL");
        let _ = conn.busy_timeout(std::time::Duration::from_secs(5));
        conn.execute(
            "CREATE TABLE IF NOT EXISTS db_connections (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                kind TEXT NOT NULL,
                file TEXT,
                host TEXT,
                port INTEGER,
                user TEXT,
                password TEXT,
                database TEXT,
                sort_order INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )
        .map_err(|e| format!("初始化连接表失败: {}", e))?;
        Ok(Self {
            conn: StdMutex::new(conn),
        })
    }
}

/// 列出全部连接（按 sort_order, name）
#[tauri::command]
pub async fn db_connections_list(
    state: State<'_, DbConnStore>,
) -> Result<Vec<DbConnection>, String> {
    let conn = state.conn.lock().map_err(|_| "数据库锁错误".to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, name, kind, file, host, port, user, password, database
             FROM db_connections ORDER BY sort_order, name",
        )
        .map_err(|e| format!("查询连接失败: {}", e))?;
    let rows = stmt
        .query_map([], |row| {
            Ok(DbConnection {
                id: row.get(0)?,
                name: row.get(1)?,
                kind: row.get(2)?,
                file: row.get(3)?,
                host: row.get(4)?,
                port: row.get::<_, Option<i64>>(5)?.map(|v| v as u16),
                user: row.get(6)?,
                password: row.get(7)?,
                database: row.get(8)?,
            })
        })
        .map_err(|e| format!("读取连接失败: {}", e))?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| format!("读取连接失败: {}", e))?);
    }
    Ok(out)
}

/// 新增或更新一个连接（按 id upsert，更新时保留原有排序）
#[tauri::command]
pub async fn db_connection_save(
    c: DbConnection,
    state: State<'_, DbConnStore>,
) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|_| "数据库锁错误".to_string())?;
    let exists = conn
        .query_row(
            "SELECT 1 FROM db_connections WHERE id = ?1",
            params![c.id],
            |_| Ok(()),
        )
        .optional()
        .map_err(|e| format!("查询连接失败: {}", e))?
        .is_some();
    if exists {
        conn.execute(
            "UPDATE db_connections SET name=?2, kind=?3, file=?4, host=?5, port=?6, user=?7, password=?8, database=?9 WHERE id=?1",
            params![c.id, c.name, c.kind, c.file, c.host, c.port, c.user, c.password, c.database],
        )
        .map_err(|e| format!("更新连接失败: {}", e))?;
    } else {
        let next: i64 = conn
            .query_row(
                "SELECT COALESCE(MAX(sort_order), 0) + 1 FROM db_connections",
                [],
                |r| r.get(0),
            )
            .unwrap_or(1);
        conn.execute(
            "INSERT INTO db_connections (id, name, kind, file, host, port, user, password, database, sort_order)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![c.id, c.name, c.kind, c.file, c.host, c.port, c.user, c.password, c.database, next],
        )
        .map_err(|e| format!("保存连接失败: {}", e))?;
    }
    Ok(())
}

/// 删除一个连接
#[tauri::command]
pub async fn db_connection_delete(id: String, state: State<'_, DbConnStore>) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|_| "数据库锁错误".to_string())?;
    conn.execute("DELETE FROM db_connections WHERE id = ?1", params![id])
        .map_err(|e| format!("删除连接失败: {}", e))?;
    Ok(())
}
