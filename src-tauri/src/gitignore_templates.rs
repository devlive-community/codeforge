use crate::execution::get_codeforge_db_path;
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::sync::Mutex as StdMutex;
use tauri::State;

/// 用户自定义 .gitignore 模板，存于同一个 codeforge.sqlite 库的独立表。
pub struct GitignoreStore {
    conn: StdMutex<Connection>,
}

#[derive(Serialize, Deserialize)]
pub struct GitignoreTemplate {
    pub id: String,
    pub label: String,
    pub content: String,
}

impl GitignoreStore {
    pub fn new() -> Result<Self, String> {
        let db_path = get_codeforge_db_path()?;
        let conn = Connection::open(&db_path).map_err(|e| format!("打开数据库失败: {}", e))?;
        let _ = conn.pragma_update(None, "journal_mode", "WAL");
        let _ = conn.pragma_update(None, "synchronous", "NORMAL");
        let _ = conn.busy_timeout(std::time::Duration::from_secs(5));
        conn.execute(
            "CREATE TABLE IF NOT EXISTS gitignore_templates (
                id TEXT PRIMARY KEY,
                label TEXT NOT NULL,
                content TEXT NOT NULL,
                updated_at INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )
        .map_err(|e| format!("初始化 .gitignore 模板表失败: {}", e))?;

        Ok(Self {
            conn: StdMutex::new(conn),
        })
    }
}

/// 列出自定义 .gitignore 模板（按名称排序）
#[tauri::command]
pub async fn gitignore_templates_list(
    state: State<'_, GitignoreStore>,
) -> Result<Vec<GitignoreTemplate>, String> {
    let conn = state.conn.lock().map_err(|_| "数据库锁错误".to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, label, content FROM gitignore_templates ORDER BY label")
        .map_err(|e| format!("查询模板失败: {}", e))?;
    let rows = stmt
        .query_map([], |row| {
            Ok(GitignoreTemplate {
                id: row.get(0)?,
                label: row.get(1)?,
                content: row.get(2)?,
            })
        })
        .map_err(|e| format!("读取模板失败: {}", e))?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| format!("读取模板失败: {}", e))?);
    }
    Ok(out)
}

/// 新增/更新一条自定义模板（按 id upsert）
#[tauri::command]
pub async fn gitignore_template_save(
    template: GitignoreTemplate,
    state: State<'_, GitignoreStore>,
) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|_| "数据库锁错误".to_string())?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0);
    conn.execute(
        "INSERT INTO gitignore_templates (id, label, content, updated_at)
         VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(id) DO UPDATE SET label=?2, content=?3, updated_at=?4",
        params![template.id, template.label, template.content, now],
    )
    .map_err(|e| format!("保存模板失败: {}", e))?;
    Ok(())
}

/// 删除一条自定义模板
#[tauri::command]
pub async fn gitignore_template_delete(
    id: String,
    state: State<'_, GitignoreStore>,
) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|_| "数据库锁错误".to_string())?;
    conn.execute("DELETE FROM gitignore_templates WHERE id = ?1", params![id])
        .map_err(|e| format!("删除模板失败: {}", e))?;
    Ok(())
}
