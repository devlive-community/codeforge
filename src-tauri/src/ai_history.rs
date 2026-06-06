use crate::execution::get_codeforge_db_path;
use rusqlite::{Connection, params};
use std::sync::Mutex as StdMutex;
use tauri::State;

/// AI 对话历史，绑定到某次执行记录（execution_id），存于同一个 codeforge.sqlite 库。
/// 未关联执行的对话属临时会话，不落库。
pub struct AiHistory {
    conn: StdMutex<Connection>,
}

impl AiHistory {
    pub fn new() -> Result<Self, String> {
        let db_path = get_codeforge_db_path()?;
        let conn = Connection::open(&db_path).map_err(|e| format!("打开数据库失败: {}", e))?;
        let _ = conn.pragma_update(None, "journal_mode", "WAL");
        let _ = conn.pragma_update(None, "synchronous", "NORMAL");
        let _ = conn.busy_timeout(std::time::Duration::from_secs(5));
        // 清理早期错误结构的旧表
        let _ = conn.execute("DROP TABLE IF EXISTS ai_conversations", []);
        conn.execute(
            "CREATE TABLE IF NOT EXISTS ai_execution_chats (
                execution_id INTEGER PRIMARY KEY,
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

/// 保存/更新某次执行对应的 AI 对话
#[tauri::command]
pub async fn save_ai_conversation(
    execution_id: i64,
    messages: String,
    updated_at: i64,
    history: State<'_, AiHistory>,
) -> Result<(), String> {
    let conn = history
        .conn
        .lock()
        .map_err(|_| "数据库锁错误".to_string())?;
    conn.execute(
        "INSERT INTO ai_execution_chats (execution_id, messages, updated_at)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(execution_id) DO UPDATE SET messages=?2, updated_at=?3",
        params![execution_id, messages, updated_at],
    )
    .map_err(|e| format!("保存 AI 对话失败: {}", e))?;
    Ok(())
}

/// 读取某次执行的 AI 对话（messages JSON）；无则返回空串
#[tauri::command]
pub async fn get_ai_conversation(
    execution_id: i64,
    history: State<'_, AiHistory>,
) -> Result<String, String> {
    let conn = history
        .conn
        .lock()
        .map_err(|_| "数据库锁错误".to_string())?;
    let result = conn.query_row(
        "SELECT messages FROM ai_execution_chats WHERE execution_id = ?1",
        params![execution_id],
        |row| row.get::<_, String>(0),
    );
    match result {
        Ok(s) => Ok(s),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(String::new()),
        Err(e) => Err(format!("读取 AI 对话失败: {}", e)),
    }
}

/// 返回所有有 AI 对话的执行 id（供历史面板标记）
#[tauri::command]
pub async fn list_ai_conversation_ids(history: State<'_, AiHistory>) -> Result<Vec<i64>, String> {
    let conn = history
        .conn
        .lock()
        .map_err(|_| "数据库锁错误".to_string())?;
    let mut stmt = conn
        .prepare("SELECT execution_id FROM ai_execution_chats")
        .map_err(|e| format!("读取失败: {}", e))?;
    let rows = stmt
        .query_map([], |row| row.get::<_, i64>(0))
        .map_err(|e| format!("读取失败: {}", e))?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("读取失败: {}", e))
}

#[tauri::command]
pub async fn delete_ai_conversation(
    execution_id: i64,
    history: State<'_, AiHistory>,
) -> Result<(), String> {
    let conn = history
        .conn
        .lock()
        .map_err(|_| "数据库锁错误".to_string())?;
    conn.execute(
        "DELETE FROM ai_execution_chats WHERE execution_id = ?1",
        params![execution_id],
    )
    .map_err(|e| format!("删除 AI 对话失败: {}", e))?;
    Ok(())
}
