use super::{DataSource, DbExecutor, SqlResultSet, SqlRunResult};
use postgres::{Config, NoTls, SimpleQueryMessage};
use serde_json::Value as JsonValue;

pub(crate) struct PostgresExecutor;

impl DbExecutor for PostgresExecutor {
    fn handles(&self, kind: &str) -> bool {
        kind == "postgres" || kind == "postgresql"
    }

    fn run(&self, sql: &str, source: &DataSource) -> SqlRunResult {
        let mut result = SqlRunResult::new();

        let mut cfg = Config::new();
        cfg.host(source.host.as_deref().unwrap_or("127.0.0.1"))
            .port(source.port.unwrap_or(5432))
            .user(source.user.as_deref().unwrap_or("postgres"));
        if let Some(pwd) = source.password.as_deref() {
            cfg.password(pwd);
        }
        if let Some(db) = source.database.as_deref() {
            cfg.dbname(db);
        }

        // SSL/SSH 隧道为独立议题(#93)，此处暂用 NoTls
        let mut client = match cfg.connect(NoTls) {
            Ok(c) => c,
            Err(e) => {
                result.error = Some(format!("连接 PostgreSQL 失败: {}", e));
                return result;
            }
        };

        // simple_query 一次执行整段脚本，按消息流切分结果集与影响行数
        let messages = match client.simple_query(sql) {
            Ok(m) => m,
            Err(e) => {
                result.error = Some(e.to_string());
                return result;
            }
        };

        let mut columns: Vec<String> = Vec::new();
        let mut rows: Vec<Vec<JsonValue>> = Vec::new();
        let flush = |columns: &mut Vec<String>, rows: &mut Vec<Vec<JsonValue>>, result: &mut SqlRunResult| {
            if !columns.is_empty() {
                result.result_sets.push(SqlResultSet {
                    columns: std::mem::take(columns),
                    rows: std::mem::take(rows),
                });
            }
        };

        for msg in messages {
            match msg {
                SimpleQueryMessage::Row(row) => {
                    if columns.is_empty() {
                        columns = row.columns().iter().map(|c| c.name().to_string()).collect();
                    }
                    let vals = (0..row.len())
                        .map(|i| match row.get(i) {
                            Some(s) => JsonValue::from(s.to_string()),
                            None => JsonValue::Null,
                        })
                        .collect();
                    rows.push(vals);
                }
                SimpleQueryMessage::CommandComplete(n) => {
                    if columns.is_empty() {
                        result.messages.push(format!("OK，影响 {} 行", n));
                    } else {
                        flush(&mut columns, &mut rows, &mut result);
                    }
                }
                _ => {}
            }
        }
        flush(&mut columns, &mut rows, &mut result);
        result
    }
}
