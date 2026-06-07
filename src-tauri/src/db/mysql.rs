use super::{DataSource, DbExecutor, SqlResultSet, SqlRunResult, split_sql};
use serde_json::Value as JsonValue;

pub(crate) struct MysqlExecutor;

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
        use mysql::prelude::Queryable;
        let mut result = SqlRunResult::new();

        let opts = mysql::OptsBuilder::new()
            .ip_or_hostname(
                source
                    .host
                    .clone()
                    .or_else(|| Some("127.0.0.1".to_string())),
            )
            .tcp_port(source.port.unwrap_or(3306))
            .user(source.user.clone())
            .pass(source.password.clone())
            .db_name(source.database.clone());

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
}
