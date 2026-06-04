use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tauri::{AppHandle, Emitter};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

fn default_base(provider: &str) -> &'static str {
    match provider {
        "anthropic" => "https://api.anthropic.com",
        "deepseek" => "https://api.deepseek.com",
        // openai 及其它兼容服务
        _ => "https://api.openai.com/v1",
    }
}

/// 统一的 AI 对话命令。provider: anthropic | openai | deepseek（其它按 OpenAI 兼容处理）。
#[tauri::command]
pub async fn ai_chat(
    provider: String,
    base_url: Option<String>,
    api_key: String,
    model: String,
    system: Option<String>,
    messages: Vec<ChatMessage>,
) -> Result<String, String> {
    if api_key.trim().is_empty() {
        return Err("未配置 API Key".to_string());
    }

    let base = base_url
        .filter(|b| !b.trim().is_empty())
        .unwrap_or_else(|| default_base(&provider).to_string());
    let base = base.trim_end_matches('/');

    let client = reqwest::Client::new();

    if provider == "anthropic" {
        let url = format!("{}/v1/messages", base);
        let body = json!({
            "model": model,
            "max_tokens": 4096,
            "system": system.unwrap_or_default(),
            "messages": messages.iter().map(|m| json!({"role": m.role, "content": m.content})).collect::<Vec<_>>(),
        });

        let resp = client
            .post(&url)
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;

        let status = resp.status();
        let data: Value = resp
            .json()
            .await
            .map_err(|e| format!("解析响应失败: {}", e))?;

        if !status.is_success() {
            return Err(extract_error(&data, status.as_u16()));
        }

        // content 是文本块数组
        let text = data["content"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|b| b["text"].as_str())
                    .collect::<Vec<_>>()
                    .join("")
            })
            .unwrap_or_default();
        Ok(text)
    } else {
        // OpenAI 兼容
        let url = format!("{}/chat/completions", base);
        let mut msgs: Vec<Value> = Vec::new();
        if let Some(sys) = system.filter(|s| !s.trim().is_empty()) {
            msgs.push(json!({"role": "system", "content": sys}));
        }
        for m in &messages {
            msgs.push(json!({"role": m.role, "content": m.content}));
        }
        let body = json!({ "model": model, "messages": msgs });

        let resp = client
            .post(&url)
            .header("authorization", format!("Bearer {}", api_key))
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;

        let status = resp.status();
        let data: Value = resp
            .json()
            .await
            .map_err(|e| format!("解析响应失败: {}", e))?;

        if !status.is_success() {
            return Err(extract_error(&data, status.as_u16()));
        }

        let text = data["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        Ok(text)
    }
}

/// 流式 AI 对话。逐段通过 `ai-stream-delta` 事件返回，完成时命令 Promise resolve。
#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub async fn ai_chat_stream(
    app: AppHandle,
    stream_id: String,
    provider: String,
    base_url: Option<String>,
    api_key: String,
    model: String,
    system: Option<String>,
    messages: Vec<ChatMessage>,
) -> Result<(), String> {
    if api_key.trim().is_empty() {
        return Err("未配置 API Key".to_string());
    }

    let is_anthropic = provider == "anthropic";
    let base = base_url
        .filter(|b| !b.trim().is_empty())
        .unwrap_or_else(|| default_base(&provider).to_string());
    let base = base.trim_end_matches('/');
    let client = reqwest::Client::new();

    let request = if is_anthropic {
        let body = json!({
            "model": model,
            "max_tokens": 4096,
            "stream": true,
            "system": system.unwrap_or_default(),
            "messages": messages.iter().map(|m| json!({"role": m.role, "content": m.content})).collect::<Vec<_>>(),
        });
        client
            .post(format!("{}/v1/messages", base))
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
    } else {
        let mut msgs: Vec<Value> = Vec::new();
        if let Some(sys) = system.filter(|s| !s.trim().is_empty()) {
            msgs.push(json!({"role": "system", "content": sys}));
        }
        for m in &messages {
            msgs.push(json!({"role": m.role, "content": m.content}));
        }
        let body = json!({ "model": model, "messages": msgs, "stream": true });
        client
            .post(format!("{}/chat/completions", base))
            .header("authorization", format!("Bearer {}", api_key))
            .header("content-type", "application/json")
            .json(&body)
    };

    let resp = request
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    let status = resp.status();
    if !status.is_success() {
        let data: Value = resp.json().await.unwrap_or(Value::Null);
        return Err(extract_error(&data, status.as_u16()));
    }

    let mut stream = resp.bytes_stream();
    let mut buf = String::new();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("读取流失败: {}", e))?;
        buf.push_str(&String::from_utf8_lossy(&chunk));

        // 按行处理 SSE
        while let Some(pos) = buf.find('\n') {
            let line = buf[..pos].trim().to_string();
            buf.drain(..=pos);

            let data = match line.strip_prefix("data:") {
                Some(d) => d.trim(),
                None => continue,
            };
            if data.is_empty() || data == "[DONE]" {
                continue;
            }

            let v: Value = match serde_json::from_str(data) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let delta = if is_anthropic {
                v["delta"]["text"].as_str()
            } else {
                v["choices"][0]["delta"]["content"].as_str()
            };
            if let Some(text) = delta {
                if !text.is_empty() {
                    let _ = app.emit(
                        "ai-stream-delta",
                        json!({"stream_id": stream_id, "delta": text}),
                    );
                }
            }
        }
    }

    Ok(())
}

fn extract_error(data: &Value, status: u16) -> String {
    let msg = data["error"]["message"]
        .as_str()
        .or_else(|| data["error"].as_str())
        .or_else(|| data["message"].as_str())
        .unwrap_or("未知错误");
    format!("AI 接口错误 ({}): {}", status, msg)
}
