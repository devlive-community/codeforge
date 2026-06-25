use super::{
    DataSource, DbExecutor, Endpoint, SqlResultSet, SqlRunResult, TxnConn, resolve_endpoint,
};
use postgres::config::SslMode;
use postgres::{Client, Config, NoTls, SimpleQueryMessage};
use serde_json::Value as JsonValue;

pub(crate) struct PostgresExecutor;

// 建连并返回客户端 + 端点（端点持有 SSH 隧道，须随客户端一起存活）
fn connect(source: &DataSource) -> Result<(Client, Endpoint), String> {
    let endpoint = resolve_endpoint(source, 5432)?;
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
    let client = if use_ssl {
        let connector = native_tls::TlsConnector::builder()
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
            .build()
            .map_err(|e| format!("初始化 TLS 失败: {}", e))?;
        let tls = postgres_native_tls::MakeTlsConnector::new(connector);
        cfg.connect(tls)
            .map_err(|e| format!("连接 PostgreSQL 失败: {}", e))?
    } else {
        cfg.connect(NoTls)
            .map_err(|e| format!("连接 PostgreSQL 失败: {}", e))?
    };
    Ok((client, endpoint))
}

/// 在已有客户端上执行脚本（run 与事务会话共用）。simple_query 一次执行整段，按消息流切分结果集。
fn run_on_client(client: &mut Client, sql: &str) -> SqlRunResult {
    let mut result = SqlRunResult::new();
    let messages = match client.simple_query(sql) {
        Ok(m) => m,
        Err(e) => {
            result.error = Some(e.to_string());
            return result;
        }
    };
    let mut columns: Vec<String> = Vec::new();
    let mut rows: Vec<Vec<JsonValue>> = Vec::new();
    let flush =
        |columns: &mut Vec<String>, rows: &mut Vec<Vec<JsonValue>>, result: &mut SqlRunResult| {
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

struct PostgresTxn {
    client: Client,
    _endpoint: Endpoint,
}
impl TxnConn for PostgresTxn {
    fn exec(&mut self, sql: &str) -> SqlRunResult {
        run_on_client(&mut self.client, sql)
    }
    fn finish(&mut self, commit: bool) -> Result<(), String> {
        self.client
            .simple_query(if commit { "COMMIT" } else { "ROLLBACK" })
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}

impl DbExecutor for PostgresExecutor {
    fn handles(&self, kind: &str) -> bool {
        kind == "postgres" || kind == "postgresql"
    }

    fn run(&self, sql: &str, source: &DataSource) -> SqlRunResult {
        match connect(source) {
            Ok((mut client, _endpoint)) => run_on_client(&mut client, sql),
            Err(e) => {
                let mut result = SqlRunResult::new();
                result.error = Some(e);
                result
            }
        }
    }

    fn begin(&self, source: &DataSource) -> Result<Box<dyn TxnConn>, String> {
        let (mut client, endpoint) = connect(source)?;
        client
            .simple_query("BEGIN")
            .map_err(|e| format!("开启事务失败: {}", e))?;
        Ok(Box::new(PostgresTxn {
            client,
            _endpoint: endpoint,
        }))
    }
}
