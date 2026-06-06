use rusqlite::Connection;
use serde::{Deserialize, Serialize};
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

impl SqlRunResult {
    fn new() -> Self {
        Self {
            result_sets: Vec::new(),
            messages: Vec::new(),
            error: None,
            elapsed_ms: 0,
        }
    }
}

/// 数据源描述：内存 / SQLite 文件 / MySQL
#[derive(Deserialize)]
pub struct DataSource {
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

/// 把脚本按分号切成多条语句（处理字符串字面量与注释，UTF-8 安全）
fn split_sql(sql: &str) -> Vec<String> {
    #[derive(PartialEq)]
    enum S {
        Normal,
        Single,
        Double,
        Backtick,
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
            S::Normal => match c {
                '\'' => {
                    state = S::Single;
                    cur.push(c);
                }
                '"' => {
                    state = S::Double;
                    cur.push(c);
                }
                '`' => {
                    state = S::Backtick;
                    cur.push(c);
                }
                '-' if next == Some('-') => {
                    state = S::Line;
                    cur.push(c);
                }
                '/' if next == Some('*') => {
                    state = S::Block;
                    cur.push(c);
                }
                ';' => {
                    let t = cur.trim().to_string();
                    if !t.is_empty() {
                        out.push(t);
                    }
                    cur.clear();
                }
                _ => cur.push(c),
            },
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
            S::Backtick => {
                cur.push(c);
                if c == '`' {
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

// ===== SQLite =====
fn sqlite_value_to_json(v: rusqlite::types::Value) -> JsonValue {
    use rusqlite::types::Value::*;
    match v {
        Null => JsonValue::Null,
        Integer(i) => JsonValue::from(i),
        Real(f) => JsonValue::from(f),
        Text(s) => JsonValue::from(s),
        Blob(b) => JsonValue::from(format!("<blob {} bytes>", b.len())),
    }
}

fn run_sqlite(sql: &str, file: Option<&str>) -> SqlRunResult {
    let mut result = SqlRunResult::new();
    let conn = match file {
        Some(p) if !p.trim().is_empty() => Connection::open(p),
        _ => Connection::open_in_memory(),
    };
    let conn = match conn {
        Ok(c) => c,
        Err(e) => {
            result.error = Some(format!("打开数据库失败: {}", e));
            return result;
        }
    };

    'stmts: for stmt_sql in split_sql(sql) {
        let mut stmt = match conn.prepare(&stmt_sql) {
            Ok(s) => s,
            Err(e) => {
                result.error = Some(e.to_string());
                break 'stmts;
            }
        };
        let ncol = stmt.column_count();
        if ncol > 0 {
            let columns: Vec<String> = stmt.column_names().iter().map(|s| s.to_string()).collect();
            let rows_iter = stmt.query_map([], |row| {
                let mut v = Vec::with_capacity(ncol);
                for idx in 0..ncol {
                    v.push(sqlite_value_to_json(row.get(idx)?));
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
    result
}

// ===== MySQL =====
fn mysql_value_to_json(v: &mysql::Value) -> JsonValue {
    use mysql::Value::*;
    match v {
        NULL => JsonValue::Null,
        Int(i) => JsonValue::from(*i),
        UInt(u) => JsonValue::from(*u),
        Float(f) => JsonValue::from(*f as f64),
        Double(d) => JsonValue::from(*d),
        Bytes(b) => match std::str::from_utf8(b) {
            Ok(s) => JsonValue::from(s.to_string()),
            Err(_) => JsonValue::from(format!("<{} bytes>", b.len())),
        },
        Date(y, mo, d, h, mi, s, _us) => JsonValue::from(format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            y, mo, d, h, mi, s
        )),
        Time(neg, d, h, mi, s, _us) => JsonValue::from(format!(
            "{}{} {:02}:{:02}:{:02}",
            if *neg { "-" } else { "" },
            d,
            h,
            mi,
            s
        )),
    }
}

fn run_mysql(sql: &str, src: &DataSource) -> SqlRunResult {
    use mysql::prelude::Queryable;
    let mut result = SqlRunResult::new();

    let opts = mysql::OptsBuilder::new()
        .ip_or_hostname(src.host.clone().or_else(|| Some("127.0.0.1".to_string())))
        .tcp_port(src.port.unwrap_or(3306))
        .user(src.user.clone())
        .pass(src.password.clone())
        .db_name(src.database.clone());

    let mut conn = match mysql::Conn::new(opts) {
        Ok(c) => c,
        Err(e) => {
            result.error = Some(format!("连接 MySQL 失败: {}", e));
            return result;
        }
    };

    'stmts: for stmt_sql in split_sql(sql) {
        let mut qr = match conn.query_iter(&stmt_sql) {
            Ok(q) => q,
            Err(e) => {
                result.error = Some(e.to_string());
                break 'stmts;
            }
        };
        let columns: Vec<String> = qr
            .columns()
            .as_ref()
            .iter()
            .map(|c| c.name_str().to_string())
            .collect();
        if columns.is_empty() {
            let affected = qr.affected_rows();
            result.messages.push(format!("OK，影响 {} 行", affected));
        } else {
            let mut rows = Vec::new();
            for r in qr.by_ref() {
                match r {
                    Ok(row) => {
                        let vals = row.unwrap();
                        rows.push(vals.iter().map(mysql_value_to_json).collect());
                    }
                    Err(e) => {
                        result.error = Some(e.to_string());
                        break 'stmts;
                    }
                }
            }
            result.result_sets.push(SqlResultSet { columns, rows });
        }
    }
    result
}

/// 执行 SQL 脚本。根据 source.kind 选择数据源（memory/sqlite/mysql）。
#[tauri::command]
pub async fn run_sql(sql: String, source: DataSource) -> Result<SqlRunResult, String> {
    tokio::task::spawn_blocking(move || {
        let start = std::time::Instant::now();
        let mut result = match source.kind.as_str() {
            "mysql" => run_mysql(&sql, &source),
            "sqlite" => run_sqlite(&sql, source.file.as_deref()),
            _ => run_sqlite(&sql, None),
        };
        result.elapsed_ms = start.elapsed().as_millis();
        Ok(result)
    })
    .await
    .map_err(|e| format!("SQL 任务失败: {}", e))?
}
