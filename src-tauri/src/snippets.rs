use crate::execution::get_codeforge_db_path;
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::sync::Mutex as StdMutex;
use tauri::State;

/// 用户代码片段，存于同一个 codeforge.sqlite 库。
pub struct Snippets {
    conn: StdMutex<Connection>,
}

#[derive(Serialize, Deserialize)]
pub struct Snippet {
    pub id: String,
    pub prefix: String,
    pub body: String,
    #[serde(default)]
    pub description: String,
    /// 适用语言；'*' 或空表示所有语言
    #[serde(default)]
    pub language: String,
}

impl Snippets {
    pub fn new() -> Result<Self, String> {
        let db_path = get_codeforge_db_path()?;
        let conn = Connection::open(&db_path).map_err(|e| format!("打开数据库失败: {}", e))?;
        let _ = conn.pragma_update(None, "journal_mode", "WAL");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS snippets (
                id TEXT PRIMARY KEY,
                prefix TEXT NOT NULL,
                body TEXT NOT NULL,
                description TEXT NOT NULL DEFAULT '',
                language TEXT NOT NULL DEFAULT '*',
                updated_at INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )
        .map_err(|e| format!("初始化代码片段表失败: {}", e))?;

        Ok(Self {
            conn: StdMutex::new(conn),
        })
    }
}

/// 列出所有代码片段（按前缀排序）
#[tauri::command]
pub async fn get_snippets(state: State<'_, Snippets>) -> Result<Vec<Snippet>, String> {
    let conn = state.conn.lock().map_err(|_| "数据库锁错误".to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, prefix, body, description, language FROM snippets ORDER BY prefix")
        .map_err(|e| format!("查询代码片段失败: {}", e))?;
    let rows = stmt
        .query_map([], |row| {
            Ok(Snippet {
                id: row.get(0)?,
                prefix: row.get(1)?,
                body: row.get(2)?,
                description: row.get(3)?,
                language: row.get(4)?,
            })
        })
        .map_err(|e| format!("读取代码片段失败: {}", e))?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| format!("读取代码片段失败: {}", e))?);
    }
    Ok(out)
}

/// 新增/更新一条代码片段（按 id upsert）
#[tauri::command]
pub async fn save_snippet(snippet: Snippet, state: State<'_, Snippets>) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|_| "数据库锁错误".to_string())?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0);
    conn.execute(
        "INSERT INTO snippets (id, prefix, body, description, language, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(id) DO UPDATE SET prefix=?2, body=?3, description=?4, language=?5, updated_at=?6",
        params![
            snippet.id,
            snippet.prefix,
            snippet.body,
            snippet.description,
            snippet.language,
            now
        ],
    )
    .map_err(|e| format!("保存代码片段失败: {}", e))?;
    Ok(())
}

/// 删除一条代码片段
#[tauri::command]
pub async fn delete_snippet(id: String, state: State<'_, Snippets>) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|_| "数据库锁错误".to_string())?;
    conn.execute("DELETE FROM snippets WHERE id = ?1", params![id])
        .map_err(|e| format!("删除代码片段失败: {}", e))?;
    Ok(())
}
