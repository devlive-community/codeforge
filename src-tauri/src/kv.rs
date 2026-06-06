use crate::execution::get_codeforge_db_path;
use rusqlite::{Connection, params};
use std::collections::HashMap;
use std::sync::Mutex as StdMutex;
use tauri::State;

/// 通用键值存储，存于同一个 codeforge.sqlite 库。
/// 用于替代前端 localStorage，集中持久化所有应用配置/状态。
pub struct KvStore {
    conn: StdMutex<Connection>,
}

impl KvStore {
    pub fn new() -> Result<Self, String> {
        let db_path = get_codeforge_db_path()?;
        let conn = Connection::open(&db_path).map_err(|e| format!("打开数据库失败: {}", e))?;
        let _ = conn.pragma_update(None, "journal_mode", "WAL");
        let _ = conn.pragma_update(None, "synchronous", "NORMAL");
        let _ = conn.busy_timeout(std::time::Duration::from_secs(5));
        conn.execute(
            "CREATE TABLE IF NOT EXISTS kv_store (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )
        .map_err(|e| format!("初始化键值表失败: {}", e))?;

        Ok(Self {
            conn: StdMutex::new(conn),
        })
    }
}

/// 读取所有键值（启动时一次性载入到前端缓存）
#[tauri::command]
pub async fn kv_get_all(state: State<'_, KvStore>) -> Result<HashMap<String, String>, String> {
    let conn = state.conn.lock().map_err(|_| "数据库锁错误".to_string())?;
    let mut stmt = conn
        .prepare("SELECT key, value FROM kv_store")
        .map_err(|e| format!("查询键值失败: {}", e))?;
    let rows = stmt
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|e| format!("读取键值失败: {}", e))?;
    let mut map = HashMap::new();
    for r in rows {
        let (k, v) = r.map_err(|e| format!("读取键值失败: {}", e))?;
        map.insert(k, v);
    }
    Ok(map)
}

/// 写入一个键值
#[tauri::command]
pub async fn kv_set(key: String, value: String, state: State<'_, KvStore>) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|_| "数据库锁错误".to_string())?;
    conn.execute(
        "INSERT INTO kv_store (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value=?2",
        params![key, value],
    )
    .map_err(|e| format!("保存键值失败: {}", e))?;
    Ok(())
}

/// 删除一个键
#[tauri::command]
pub async fn kv_delete(key: String, state: State<'_, KvStore>) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|_| "数据库锁错误".to_string())?;
    conn.execute("DELETE FROM kv_store WHERE key = ?1", params![key])
        .map_err(|e| format!("删除键值失败: {}", e))?;
    Ok(())
}
