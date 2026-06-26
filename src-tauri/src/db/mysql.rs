use super::{
    DataSource, DbExecutor, Endpoint, SqlResultSet, SqlRunResult, TxnConn, resolve_endpoint,
    split_sql,
};
use serde_json::Value as JsonValue;

pub(crate) struct MysqlExecutor;

// 建连并返回连接 + 端点（端点持有 SSH 隧道，须随连接一起存活）
fn connect(source: &DataSource) -> Result<(mysql::Conn, Endpoint), String> {
    let endpoint = resolve_endpoint(source, 3306)?;
    let mut builder = mysql::OptsBuilder::new()
        .ip_or_hostname(Some(endpoint.host.clone()))
        .tcp_port(endpoint.port)
        .user(source.user.clone())
        .pass(source.password.clone())
        .db_name(source.database.clone());
    if source.ssl.unwrap_or(false) {
        builder = builder.ssl_opts(Some(
            mysql::SslOpts::default()
                .with_danger_accept_invalid_certs(true)
                .with_danger_skip_domain_validation(true),
        ));
    }
    let conn = mysql::Conn::new(builder).map_err(|e| format!("连接 MySQL 失败: {}", e))?;
    Ok((conn, endpoint))
}

/// 在已有连接上执行脚本（run 与事务会话共用）
fn run_on_conn(conn: &mut mysql::Conn, sql: &str) -> SqlRunResult {
    use mysql::prelude::Queryable;
    let mut result = SqlRunResult::new();
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
                        rows.push(vals.iter().map(value_to_json).collect());
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

struct MysqlTxn {
    conn: mysql::Conn,
    _endpoint: Endpoint,
}
impl TxnConn for MysqlTxn {
    fn exec(&mut self, sql: &str) -> SqlRunResult {
        run_on_conn(&mut self.conn, sql)
    }
    fn finish(&mut self, commit: bool) -> Result<(), String> {
        use mysql::prelude::Queryable;
        self.conn
            .query_drop(if commit { "COMMIT" } else { "ROLLBACK" })
            .map_err(|e| e.to_string())
    }
}

fn value_to_json(v: &mysql::Value) -> JsonValue {
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

impl DbExecutor for MysqlExecutor {
    fn handles(&self, kind: &str) -> bool {
        kind == "mysql"
    }

    fn run(&self, sql: &str, source: &DataSource) -> SqlRunResult {
        match connect(source) {
            // endpoint 持有隧道，run 期间保持存活
            Ok((mut conn, _endpoint)) => run_on_conn(&mut conn, sql),
            Err(e) => {
                let mut result = SqlRunResult::new();
                result.error = Some(e);
                result
            }
        }
    }

    fn begin(&self, source: &DataSource) -> Result<Box<dyn TxnConn>, String> {
        use mysql::prelude::Queryable;
        let (mut conn, endpoint) = connect(source)?;
        conn.query_drop("START TRANSACTION")
            .map_err(|e| format!("开启事务失败: {}", e))?;
        Ok(Box::new(MysqlTxn {
            conn,
            _endpoint: endpoint,
        }))
    }
}
