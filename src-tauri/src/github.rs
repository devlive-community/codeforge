//! GitHub REST API 集成：用用户配置的 token 直接在应用内创建/查看 PR 与 Issue。
use serde::Serialize;
use serde_json::{Value, json};

const API: &str = "https://api.github.com";

#[derive(Serialize)]
pub struct PrItem {
    number: u64,
    title: String,
    url: String,
    state: String,
    head: String,
    base: String,
    author: String,
    draft: bool,
}

#[derive(Serialize)]
pub struct IssueItem {
    number: u64,
    title: String,
    url: String,
    state: String,
    author: String,
    comments: u64,
}

fn client() -> reqwest::Client {
    reqwest::Client::new()
}

// 统一的鉴权 GET
async fn api_get(token: &str, path: &str) -> Result<Value, String> {
    let resp = client()
        .get(format!("{}{}", API, path))
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "CodeForge")
        .header("X-GitHub-Api-Version", "2022-11-28")
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
    Ok(data)
}

// 统一的鉴权 POST
async fn api_post(token: &str, path: &str, body: Value) -> Result<Value, String> {
    let resp = client()
        .post(format!("{}{}", API, path))
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "CodeForge")
        .header("X-GitHub-Api-Version", "2022-11-28")
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
    Ok(data)
}

fn extract_error(data: &Value, status: u16) -> String {
    let msg = data["message"].as_str().unwrap_or("未知错误");
    // 附带首个字段级错误，便于定位
    let detail = data["errors"][0]["message"]
        .as_str()
        .map(|s| format!("（{}）", s))
        .unwrap_or_default();
    format!("GitHub API 错误 ({}): {}{}", status, msg, detail)
}

fn pr_from(v: &Value) -> PrItem {
    PrItem {
        number: v["number"].as_u64().unwrap_or(0),
        title: v["title"].as_str().unwrap_or("").to_string(),
        url: v["html_url"].as_str().unwrap_or("").to_string(),
        state: v["state"].as_str().unwrap_or("").to_string(),
        head: v["head"]["ref"].as_str().unwrap_or("").to_string(),
        base: v["base"]["ref"].as_str().unwrap_or("").to_string(),
        author: v["user"]["login"].as_str().unwrap_or("").to_string(),
        draft: v["draft"].as_bool().unwrap_or(false),
    }
}

fn issue_from(v: &Value) -> IssueItem {
    IssueItem {
        number: v["number"].as_u64().unwrap_or(0),
        title: v["title"].as_str().unwrap_or("").to_string(),
        url: v["html_url"].as_str().unwrap_or("").to_string(),
        state: v["state"].as_str().unwrap_or("").to_string(),
        author: v["user"]["login"].as_str().unwrap_or("").to_string(),
        comments: v["comments"].as_u64().unwrap_or(0),
    }
}

fn require_token(token: &str) -> Result<(), String> {
    if token.trim().is_empty() {
        return Err("未配置 GitHub Token（设置 → 通用 → GitHub）".to_string());
    }
    Ok(())
}

/// 列出仓库的 Pull Request（默认 open）。
#[tauri::command]
pub async fn github_list_prs(
    token: String,
    owner: String,
    repo: String,
    state: Option<String>,
) -> Result<Vec<PrItem>, String> {
    require_token(&token)?;
    let st = state.unwrap_or_else(|| "open".to_string());
    let data = api_get(
        &token,
        &format!("/repos/{}/{}/pulls?state={}&per_page=50", owner, repo, st),
    )
    .await?;
    Ok(data
        .as_array()
        .map(|a| a.iter().map(pr_from).collect())
        .unwrap_or_default())
}

/// 创建 Pull Request。
#[tauri::command]
pub async fn github_create_pr(
    token: String,
    owner: String,
    repo: String,
    title: String,
    head: String,
    base: String,
    body: Option<String>,
    draft: Option<bool>,
) -> Result<PrItem, String> {
    require_token(&token)?;
    let payload = json!({
        "title": title,
        "head": head,
        "base": base,
        "body": body.unwrap_or_default(),
        "draft": draft.unwrap_or(false),
    });
    let data = api_post(&token, &format!("/repos/{}/{}/pulls", owner, repo), payload).await?;
    Ok(pr_from(&data))
}

/// 列出仓库的 Issue（默认 open；GitHub 会把 PR 也算作 issue，这里过滤掉）。
#[tauri::command]
pub async fn github_list_issues(
    token: String,
    owner: String,
    repo: String,
    state: Option<String>,
) -> Result<Vec<IssueItem>, String> {
    require_token(&token)?;
    let st = state.unwrap_or_else(|| "open".to_string());
    let data = api_get(
        &token,
        &format!("/repos/{}/{}/issues?state={}&per_page=50", owner, repo, st),
    )
    .await?;
    Ok(data
        .as_array()
        .map(|a| {
            a.iter()
                .filter(|v| v.get("pull_request").is_none())
                .map(issue_from)
                .collect()
        })
        .unwrap_or_default())
}

/// 创建 Issue。
#[tauri::command]
pub async fn github_create_issue(
    token: String,
    owner: String,
    repo: String,
    title: String,
    body: Option<String>,
) -> Result<IssueItem, String> {
    require_token(&token)?;
    let payload = json!({ "title": title, "body": body.unwrap_or_default() });
    let data = api_post(&token, &format!("/repos/{}/{}/issues", owner, repo), payload).await?;
    Ok(issue_from(&data))
}
