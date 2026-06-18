use super::{DataSource, DbExecutor, SqlResultSet, SqlRunResult, resolve_endpoint};
use postgres::config::SslMode;
use postgres::{Config, NoTls, SimpleQueryMessage};
use serde_json::Value as JsonValue;

pub(crate) struct PostgresExecutor;

impl DbExecutor for PostgresExecutor {
    fn handles(&self, kind: &str) -> bool {
        kind == "postgres" || kind == "postgresql"
    }

    fn run(&self, sql: &str, source: &DataSource) -> SqlRunResult {
        let mut result = SqlRunResult::new();

        // 解析端点：启用 SSH 时隧道转发到本地端口（隧道随 endpoint 在本函数结束时关闭）
        let endpoint = match resolve_endpoint(source, 5432) {
            Ok(e) => e,
            Err(e) => {
                result.error = Some(e);
                return result;
            }
        };

        let use_ssl = source.ssl.unwrap_or(false);
        let mut cfg = Config::new();
        cfg.host(&endpoint.host)
            .port(endpoint.port)
            .user(source.user.as_deref().unwrap_or("postgres"))
            .ssl_mode(if use_ssl {
                SslMode::Require
            } else {
                SslMode::Disable
            });
        if let Some(pwd) = source.password.as_deref() {
            cfg.password(pwd);
        }
        if let Some(db) = source.database.as_deref() {
            cfg.dbname(db);
        }

        // 启用 SSL：native-tls 加密连接（开发场景放宽证书校验）；否则 NoTls 明文
        let mut client = if use_ssl {
            let connector = match native_tls::TlsConnector::builder()
                .danger_accept_invalid_certs(true)
                .danger_accept_invalid_hostnames(true)
                .build()
            {
                Ok(c) => c,
                Err(e) => {
                    result.error = Some(format!("初始化 TLS 失败: {}", e));
                    return result;
                }
            };
            let tls = postgres_native_tls::MakeTlsConnector::new(connector);
            match cfg.connect(tls) {
                Ok(c) => c,
                Err(e) => {
                    result.error = Some(format!("连接 PostgreSQL 失败: {}", e));
                    return result;
                }
            }
        } else {
            match cfg.connect(NoTls) {
                Ok(c) => c,
                Err(e) => {
                    result.error = Some(format!("连接 PostgreSQL 失败: {}", e));
                    return result;
                }
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
        let flush = |columns: &mut Vec<String>,
                     rows: &mut Vec<Vec<JsonValue>>,
                     result: &mut SqlRunResult| {
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
