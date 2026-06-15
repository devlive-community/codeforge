use super::{split_sql, DataSource, DbExecutor, SqlResultSet, SqlRunResult};
use duckdb::Connection;
use serde_json::Value as JsonValue;

pub(crate) struct DuckdbExecutor;

fn value_to_json(v: duckdb::types::Value) -> JsonValue {
    use duckdb::types::Value::*;
    match v {
        Null => JsonValue::Null,
        Boolean(b) => JsonValue::from(b),
        TinyInt(i) => JsonValue::from(i),
        SmallInt(i) => JsonValue::from(i),
        Int(i) => JsonValue::from(i),
        BigInt(i) => JsonValue::from(i),
        UTinyInt(i) => JsonValue::from(i),
        USmallInt(i) => JsonValue::from(i),
        UInt(i) => JsonValue::from(i),
        UBigInt(i) => JsonValue::from(i),
        HugeInt(i) => JsonValue::from(i.to_string()),
        Float(f) => JsonValue::from(f as f64),
        Double(f) => JsonValue::from(f),
        Text(s) => JsonValue::from(s),
        Blob(b) => JsonValue::from(format!("<blob {} bytes>", b.len())),
        // 时间/小数/嵌套等复杂类型统一以可读字符串呈现
        other => JsonValue::from(format!("{:?}", other)),
    }
}

impl DbExecutor for DuckdbExecutor {
    fn handles(&self, kind: &str) -> bool {
        kind == "duckdb"
    }

    fn run(&self, sql: &str, source: &DataSource) -> SqlRunResult {
        let mut result = SqlRunResult::new();
        let conn = match source.file.as_deref() {
            Some(p) if !p.trim().is_empty() => Connection::open(p),
            _ => Connection::open_in_memory(),
        };
        let conn = match conn {
            Ok(c) => c,
            Err(e) => {
                result.error = Some(format!("打开 DuckDB 失败: {}", e));
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
                let columns: Vec<String> =
                    stmt.column_names().iter().map(|s| s.to_string()).collect();
                let rows_iter = stmt.query_map([], |row| {
                    let mut v = Vec::with_capacity(ncol);
                    for idx in 0..ncol {
                        v.push(value_to_json(row.get(idx)?));
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
}
