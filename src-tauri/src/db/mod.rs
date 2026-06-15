//! 数据库执行器：插件式架构。
//! 每种数据库类型实现 `DbExecutor` 并在 `executors()` 中注册一行，新增类型互不影响。

mod clickhouse;
mod duckdb;
mod mysql;
mod postgres;
mod sqlite;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Serialize)]
pub(crate) struct SqlResultSet {
    pub(crate) columns: Vec<String>,
    pub(crate) rows: Vec<Vec<JsonValue>>,
}

#[derive(Serialize)]
pub struct SqlRunResult {
    pub(crate) result_sets: Vec<SqlResultSet>,
    pub(crate) messages: Vec<String>,
    pub(crate) error: Option<String>,
    pub(crate) elapsed_ms: u128,
}

impl SqlRunResult {
    pub(crate) fn new() -> Self {
        Self {
            result_sets: Vec::new(),
            messages: Vec::new(),
            error: None,
            elapsed_ms: 0,
        }
    }
}

/// 数据源描述：内存 / SQLite 文件 / MySQL（后续可扩展更多字段）
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

/// 数据库执行器接口：新增数据库类型只需实现本 trait 并在 executors() 注册。
pub(crate) trait DbExecutor: Send + Sync {
    /// 是否处理该数据源类型（如 sqlite 同时处理 "sqlite" 与 "memory"）
    fn handles(&self, kind: &str) -> bool;
    /// 执行脚本，返回结构化结果（错误写入 result.error，不以 Err 形式返回）
    fn run(&self, sql: &str, source: &DataSource) -> SqlRunResult;
}

/// 已注册的执行器。新增数据库类型：在此加一行。
fn executors() -> Vec<Box<dyn DbExecutor>> {
    vec![
        Box::new(sqlite::SqliteExecutor),
        Box::new(mysql::MysqlExecutor),
        Box::new(postgres::PostgresExecutor),
        Box::new(clickhouse::ClickhouseExecutor),
        Box::new(duckdb::DuckdbExecutor),
    ]
}

/// 把脚本按分号切成多条语句（处理字符串字面量、反引号与注释，UTF-8 安全）
pub(crate) fn split_sql(sql: &str) -> Vec<String> {
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

/// 执行 SQL 脚本，按 source.kind 派发到对应执行器，并写入执行历史。
#[tauri::command]
pub async fn run_sql(
    sql: String,
    source: DataSource,
    history: tauri::State<'_, crate::execution::ExecutionHistory>,
) -> Result<SqlRunResult, String> {
    let sql_for_record = sql.clone();
    let result = tokio::task::spawn_blocking(move || {
        let start = std::time::Instant::now();
        let execs = executors();
        let mut result = match execs.iter().find(|e| e.handles(&source.kind)) {
            Some(exec) => exec.run(&sql, &source),
            None => {
                let mut r = SqlRunResult::new();
                r.error = Some(format!("不支持的数据源类型: {}", source.kind));
                r
            }
        };
        result.elapsed_ms = start.elapsed().as_millis();
        result
    })
    .await
    .map_err(|e| format!("SQL 任务失败: {}", e))?;

    // 与其它语言一致：记录到执行历史
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    // stdout 存完整结果 JSON，历史详情用 SqlTableView 渲染，与实时运行一致
    let record = crate::plugins::ExecutionResult {
        id: None,
        success: result.error.is_none(),
        code: sql_for_record,
        stdout: serde_json::to_string(&result).unwrap_or_default(),
        stderr: result.error.clone().unwrap_or_default(),
        execution_time: result.elapsed_ms,
        timestamp,
        language: "sql".to_string(),
    };
    let _ = history.insert(&record);

    Ok(result)
}

/// 分页执行单条查询：把语句包成子查询加 LIMIT/OFFSET，按需拉取一页，避免一次性取全量。
/// 仅供前端对单条 SELECT/WITH 调用；record=true 时按原始 SQL 记入历史（首页传 true，翻页传 false）。
#[tauri::command]
pub async fn run_sql_paged(
    sql: String,
    source: DataSource,
    limit: u32,
    offset: u32,
    record: bool,
    history: tauri::State<'_, crate::execution::ExecutionHistory>,
) -> Result<SqlRunResult, String> {
    let original = sql.clone();
    let result = tokio::task::spawn_blocking(move || {
        let inner = sql.trim().trim_end_matches(';').trim();
        let wrapped = format!(
            "SELECT * FROM (\n{}\n) AS __cf_page LIMIT {} OFFSET {}",
            inner, limit, offset
        );
        let start = std::time::Instant::now();
        let execs = executors();
        let mut result = match execs.iter().find(|e| e.handles(&source.kind)) {
            Some(exec) => exec.run(&wrapped, &source),
            None => {
                let mut r = SqlRunResult::new();
                r.error = Some(format!("不支持的数据源类型: {}", source.kind));
                r
            }
        };
        result.elapsed_ms = start.elapsed().as_millis();
        result
    })
    .await
    .map_err(|e| format!("SQL 任务失败: {}", e))?;

    if record {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let rec = crate::plugins::ExecutionResult {
            id: None,
            success: result.error.is_none(),
            code: original,
            stdout: serde_json::to_string(&result).unwrap_or_default(),
            stderr: result.error.clone().unwrap_or_default(),
            execution_time: result.elapsed_ms,
            timestamp,
            language: "sql".to_string(),
        };
        let _ = history.insert(&rec);
    }
    Ok(result)
}
