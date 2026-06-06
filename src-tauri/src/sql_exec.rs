use rusqlite::Connection;
use serde::Serialize;
use serde_json::Value as JsonValue;

#[derive(Serialize)]
pub struct SqlResultSet {
    columns: Vec<String>,
    rows: Vec<Vec<JsonValue>>,
}

#[derive(Serialize)]
pub struct SqlRunResult {
    result_sets: Vec<SqlResultSet>,
    messages: Vec<String>,
    error: Option<String>,
    elapsed_ms: u128,
}

fn value_to_json(v: rusqlite::types::Value) -> JsonValue {
    use rusqlite::types::Value::*;
    match v {
        Null => JsonValue::Null,
        Integer(i) => JsonValue::from(i),
        Real(f) => JsonValue::from(f),
        Text(s) => JsonValue::from(s),
        Blob(b) => JsonValue::from(format!("<blob {} bytes>", b.len())),
    }
}

/// 把脚本按分号切成多条语句（处理字符串字面量与注释，UTF-8 安全）
fn split_sql(sql: &str) -> Vec<String> {
    #[derive(PartialEq)]
    enum S {
        Normal,
        Single,
        Double,
        Line,
        Block,
    }
    let chars: Vec<char> = sql.chars().collect();
    let n = chars.len();
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut state = S::Normal;
    let mut i = 0;
    while i < n {
        let c = chars[i];
        let next = if i + 1 < n { Some(chars[i + 1]) } else { None };
        match state {
            S::Normal => {
                if c == '\'' {
                    state = S::Single;
                    cur.push(c);
                } else if c == '"' {
                    state = S::Double;
                    cur.push(c);
                } else if c == '-' && next == Some('-') {
                    state = S::Line;
                    cur.push(c);
                } else if c == '/' && next == Some('*') {
                    state = S::Block;
                    cur.push(c);
                } else if c == ';' {
                    let t = cur.trim().to_string();
                    if !t.is_empty() {
                        out.push(t);
                    }
                    cur.clear();
                } else {
                    cur.push(c);
                }
            }
            S::Single => {
                cur.push(c);
                if c == '\'' {
                    state = S::Normal;
                }
            }
            S::Double => {
                cur.push(c);
                if c == '"' {
                    state = S::Normal;
                }
            }
            S::Line => {
                cur.push(c);
                if c == '\n' {
                    state = S::Normal;
                }
            }
            S::Block => {
                cur.push(c);
                if c == '*' && next == Some('/') {
                    cur.push('/');
                    i += 1;
                    state = S::Normal;
                }
            }
        }
        i += 1;
    }
    let t = cur.trim().to_string();
    if !t.is_empty() {
        out.push(t);
    }
    out
}

/// 执行 SQL 脚本。db_path 为空则使用内存数据库；逐条执行，查询返回结果集，失败返回错误信息。
#[tauri::command]
pub async fn run_sql(sql: String, db_path: Option<String>) -> Result<SqlRunResult, String> {
    tokio::task::spawn_blocking(move || {
        let start = std::time::Instant::now();
        let conn = match db_path.as_deref() {
            Some(p) if !p.trim().is_empty() => Connection::open(p),
            _ => Connection::open_in_memory(),
        }
        .map_err(|e| format!("打开数据库失败: {}", e))?;

        let mut result = SqlRunResult {
            result_sets: Vec::new(),
            messages: Vec::new(),
            error: None,
            elapsed_ms: 0,
        };

        'stmts: for stmt_sql in split_sql(&sql) {
            let mut stmt = match conn.prepare(&stmt_sql) {
                Ok(s) => s,
                Err(e) => {
                    result.error = Some(e.to_string());
                    break 'stmts;
                }
            };
            let ncol = stmt.column_count();
            if ncol > 0 {
                let columns: Vec<String> =
                    stmt.column_names().iter().map(|s| s.to_string()).collect();
                let rows_iter = stmt.query_map([], |row| {
                    let mut v = Vec::with_capacity(ncol);
                    for idx in 0..ncol {
                        let val: rusqlite::types::Value = row.get(idx)?;
                        v.push(value_to_json(val));
                    }
                    Ok(v)
                });
                match rows_iter {
                    Ok(it) => {
                        let mut rows = Vec::new();
                        for r in it {
                            match r {
                                Ok(rw) => rows.push(rw),
                                Err(e) => {
                                    result.error = Some(e.to_string());
                                    break 'stmts;
                                }
                            }
                        }
                        result.result_sets.push(SqlResultSet { columns, rows });
                    }
                    Err(e) => {
                        result.error = Some(e.to_string());
                        break 'stmts;
                    }
                }
            } else {
                match stmt.execute([]) {
                    Ok(affected) => result.messages.push(format!("OK，影响 {} 行", affected)),
                    Err(e) => {
                        result.error = Some(e.to_string());
                        break 'stmts;
                    }
                }
            }
        }

        result.elapsed_ms = start.elapsed().as_millis();
        Ok(result)
    })
    .await
    .map_err(|e| format!("SQL 任务失败: {}", e))?
}
