use crate::execution::get_codeforge_db_path;
use rusqlite::{Connection, params};
use serde::Serialize;
use std::sync::Mutex as StdMutex;
use tauri::State;

#[derive(Serialize)]
pub struct AiConversationMeta {
    pub id: String,
    pub title: String,
    pub updated_at: i64,
}

/// AI 对话历史，存于与执行历史相同的 codeforge.sqlite 库
pub struct AiHistory {
    conn: StdMutex<Connection>,
}

impl AiHistory {
    pub fn new() -> Result<Self, String> {
        let db_path = get_codeforge_db_path()?;
        let conn = Connection::open(&db_path).map_err(|e| format!("打开数据库失败: {}", e))?;
        // 并发读写更稳
        let _ = conn.pragma_update(None, "journal_mode", "WAL");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS ai_conversations (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                messages TEXT NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )
        .map_err(|e| format!("初始化 AI 对话表失败: {}", e))?;

        Ok(Self {
            conn: StdMutex::new(conn),
        })
    }
}

#[tauri::command]
pub async fn save_ai_conversation(
    id: String,
    title: String,
    messages: String,
    updated_at: i64,
    history: State<'_, AiHistory>,
) -> Result<(), String> {
    let conn = history
        .conn
        .lock()
        .map_err(|_| "数据库锁错误".to_string())?;
    conn.execute(
        "INSERT INTO ai_conversations (id, title, messages, updated_at)
         VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(id) DO UPDATE SET title=?2, messages=?3, updated_at=?4",
        params![id, title, messages, updated_at],
    )
    .map_err(|e| format!("保存 AI 对话失败: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn list_ai_conversations(
    history: State<'_, AiHistory>,
) -> Result<Vec<AiConversationMeta>, String> {
    let conn = history
        .conn
        .lock()
        .map_err(|_| "数据库锁错误".to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, title, updated_at FROM ai_conversations ORDER BY updated_at DESC")
        .map_err(|e| format!("读取 AI 对话失败: {}", e))?;
    let rows = stmt
        .query_map([], |row| {
            Ok(AiConversationMeta {
                id: row.get(0)?,
                title: row.get(1)?,
                updated_at: row.get(2)?,
            })
        })
        .map_err(|e| format!("读取 AI 对话失败: {}", e))?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("读取 AI 对话失败: {}", e))
}

/// 返回该会话的 messages JSON 字符串
#[tauri::command]
pub async fn get_ai_conversation(
    id: String,
    history: State<'_, AiHistory>,
) -> Result<String, String> {
    let conn = history
        .conn
        .lock()
        .map_err(|_| "数据库锁错误".to_string())?;
    conn.query_row(
        "SELECT messages FROM ai_conversations WHERE id = ?1",
        params![id],
        |row| row.get::<_, String>(0),
    )
    .map_err(|e| format!("读取 AI 对话失败: {}", e))
}

#[tauri::command]
pub async fn delete_ai_conversation(
    id: String,
    history: State<'_, AiHistory>,
) -> Result<(), String> {
    let conn = history
        .conn
        .lock()
        .map_err(|_| "数据库锁错误".to_string())?;
    conn.execute("DELETE FROM ai_conversations WHERE id = ?1", params![id])
        .map_err(|e| format!("删除 AI 对话失败: {}", e))?;
    Ok(())
}
