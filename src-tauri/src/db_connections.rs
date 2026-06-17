//! 数据库连接的独立存储表（codeforge.sqlite 中的 db_connections）。
//! 从 KV 的 sql-connections JSON blob 抽出，便于将来按需扩展/分页/检索。

use crate::execution::get_codeforge_db_path;
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};
use std::sync::Mutex as StdMutex;
use tauri::State;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
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
    // 加密连接：直连 TLS
    #[serde(default)]
    pub ssl: Option<bool>,
    // SSH 隧道
    #[serde(default)]
    pub ssh_enabled: Option<bool>,
    #[serde(default)]
    pub ssh_host: Option<String>,
    #[serde(default)]
    pub ssh_port: Option<u16>,
    #[serde(default)]
    pub ssh_user: Option<String>,
    #[serde(default)]
    pub ssh_password: Option<String>,
    #[serde(default)]
    pub ssh_key_file: Option<String>,
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
                ssl INTEGER,
                ssh_enabled INTEGER,
                ssh_host TEXT,
                ssh_port INTEGER,
                ssh_user TEXT,
                ssh_password TEXT,
                ssh_key_file TEXT,
                sort_order INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )
        .map_err(|e| format!("初始化连接表失败: {}", e))?;

        // 旧表懒迁移：逐列尝试新增，已存在则忽略错误
        for col in [
            "ssl INTEGER",
            "ssh_enabled INTEGER",
            "ssh_host TEXT",
            "ssh_port INTEGER",
            "ssh_user TEXT",
            "ssh_password TEXT",
            "ssh_key_file TEXT",
        ] {
            let _ = conn.execute(
                &format!("ALTER TABLE db_connections ADD COLUMN {}", col),
                [],
            );
        }

        Ok(Self {
            conn: StdMutex::new(conn),
        })
    }
}

const COLUMNS: &str = "id, name, kind, file, host, port, user, password, database, \
     ssl, ssh_enabled, ssh_host, ssh_port, ssh_user, ssh_password, ssh_key_file";

fn row_to_conn(row: &rusqlite::Row) -> rusqlite::Result<DbConnection> {
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
        ssl: row.get::<_, Option<bool>>(9)?,
        ssh_enabled: row.get::<_, Option<bool>>(10)?,
        ssh_host: row.get(11)?,
        ssh_port: row.get::<_, Option<i64>>(12)?.map(|v| v as u16),
        ssh_user: row.get(13)?,
        ssh_password: row.get(14)?,
        ssh_key_file: row.get(15)?,
    })
}

/// 列出全部连接（按 sort_order, name）
#[tauri::command]
pub async fn db_connections_list(
    state: State<'_, DbConnStore>,
) -> Result<Vec<DbConnection>, String> {
    let conn = state.conn.lock().map_err(|_| "数据库锁错误".to_string())?;
    let mut stmt = conn
        .prepare(&format!(
            "SELECT {} FROM db_connections ORDER BY sort_order, name",
            COLUMNS
        ))
        .map_err(|e| format!("查询连接失败: {}", e))?;
    let rows = stmt
        .query_map([], row_to_conn)
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
            "UPDATE db_connections SET name=?2, kind=?3, file=?4, host=?5, port=?6, user=?7, \
             password=?8, database=?9, ssl=?10, ssh_enabled=?11, ssh_host=?12, ssh_port=?13, \
             ssh_user=?14, ssh_password=?15, ssh_key_file=?16 WHERE id=?1",
            params![
                c.id,
                c.name,
                c.kind,
                c.file,
                c.host,
                c.port,
                c.user,
                c.password,
                c.database,
                c.ssl,
                c.ssh_enabled,
                c.ssh_host,
                c.ssh_port,
                c.ssh_user,
                c.ssh_password,
                c.ssh_key_file
            ],
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
            "INSERT INTO db_connections (id, name, kind, file, host, port, user, password, \
             database, ssl, ssh_enabled, ssh_host, ssh_port, ssh_user, ssh_password, \
             ssh_key_file, sort_order)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
            params![
                c.id,
                c.name,
                c.kind,
                c.file,
                c.host,
                c.port,
                c.user,
                c.password,
                c.database,
                c.ssl,
                c.ssh_enabled,
                c.ssh_host,
                c.ssh_port,
                c.ssh_user,
                c.ssh_password,
                c.ssh_key_file,
                next
            ],
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
