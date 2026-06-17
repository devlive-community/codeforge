use super::{DataSource, DbExecutor, SqlResultSet, SqlRunResult, resolve_endpoint, split_sql};
use serde_json::Value as JsonValue;

pub(crate) struct ClickhouseExecutor;

impl DbExecutor for ClickhouseExecutor {
    fn handles(&self, kind: &str) -> bool {
        kind == "clickhouse"
    }

    fn run(&self, sql: &str, source: &DataSource) -> SqlRunResult {
        let mut result = SqlRunResult::new();

        // 解析端点：启用 SSH 时隧道转发到本地端口（隧道随 endpoint 在本函数结束时关闭）
        let endpoint = match resolve_endpoint(source, 8123) {
            Ok(e) => e,
            Err(e) => {
                result.error = Some(e);
                return result;
            }
        };

        let database = source.database.as_deref().unwrap_or("default");
        let user = source.user.as_deref().unwrap_or("default");
        // SSL 直连用 https；走 SSH 隧道时已由 ssh 加密，本地仍用 http
        let scheme = if source.ssl.unwrap_or(false) && !source.ssh_enabled.unwrap_or(false) {
            "https"
        } else {
            "http"
        };
        // 走 HTTP(S) 接口，SELECT 以 JSONCompact 返回，DDL/写入返回空体
        let url = format!(
            "{}://{}:{}/?default_format=JSONCompact&database={}",
            scheme,
            endpoint.host,
            endpoint.port,
            urlencode(database)
        );

        for stmt in split_sql(sql) {
            let mut req = ureq::post(&url).set("X-ClickHouse-User", user);
            if let Some(pwd) = source.password.as_deref() {
                req = req.set("X-ClickHouse-Key", pwd);
            }
            match req.send_string(&stmt) {
                Ok(resp) => {
                    let body = resp.into_string().unwrap_or_default();
                    let trimmed = body.trim_start();
                    if trimmed.starts_with('{') {
                        match parse_json_compact(&body) {
                            Some(rs) => result.result_sets.push(rs),
                            None => result.messages.push("OK".to_string()),
                        }
                    } else {
                        result.messages.push("OK".to_string());
                    }
                }
                Err(ureq::Error::Status(code, resp)) => {
                    let msg = resp.into_string().unwrap_or_default();
                    result.error = Some(format!("ClickHouse 错误 {}: {}", code, msg.trim()));
                    break;
                }
                Err(e) => {
                    result.error = Some(format!("连接 ClickHouse 失败: {}", e));
                    break;
                }
            }
        }
        result
    }
}

/// 解析 JSONCompact 响应：{"meta":[{"name":..}], "data":[[..],..]}
fn parse_json_compact(body: &str) -> Option<SqlResultSet> {
    let v: JsonValue = serde_json::from_str(body).ok()?;
    let meta = v.get("meta")?.as_array()?;
    let data = v.get("data")?.as_array()?;
    let columns: Vec<String> = meta
        .iter()
        .map(|m| {
            m.get("name")
                .and_then(|n| n.as_str())
                .unwrap_or("")
                .to_string()
        })
        .collect();
    let rows: Vec<Vec<JsonValue>> = data
        .iter()
        .map(|row| row.as_array().cloned().unwrap_or_default())
        .collect();
    Some(SqlResultSet { columns, rows })
}

/// 最小 URL 编码（数据库名等）
fn urlencode(s: &str) -> String {
    s.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (b as char).to_string()
            }
            _ => format!("%{:02X}", b),
        })
        .collect()
}
