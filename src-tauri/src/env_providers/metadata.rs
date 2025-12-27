use log::info;
use serde::{Deserialize, Serialize};

// 通用的 CDN Metadata 结构
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MetadataRelease {
    pub version: String,
    pub display_name: String,
    pub published_at: String,
    pub download_url: String,
    pub github_url: String,
    pub file_name: String,
    pub size: u64,
    #[serde(default)]
    pub supported_platforms: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
    #[serde(default)]
    pub language: String,
    pub releases: Vec<MetadataRelease>,
}

// 通用的 CDN 配置检查函数
pub fn is_cdn_enabled() -> bool {
    use crate::config::get_app_config_internal;

    get_app_config_internal()
        .ok()
        .and_then(|config| config.environment_mirror)
        .and_then(|mirror| mirror.enabled)
        .unwrap_or(false)
}

// 通用的 fallback 配置检查函数
pub fn is_fallback_enabled() -> bool {
    use crate::config::get_app_config_internal;

    get_app_config_internal()
        .ok()
        .and_then(|config| config.environment_mirror)
        .and_then(|mirror| mirror.fallback_enabled)
        .unwrap_or(false)
}

// 通用的从 CDN 获取 metadata 函数
pub async fn fetch_metadata_from_cdn(language: &str) -> Result<Metadata, String> {
    use crate::config::get_app_config_internal;

    let config = get_app_config_internal().map_err(|e| format!("读取配置失败: {}", e))?;

    let cdn_enabled = config
        .environment_mirror
        .as_ref()
        .and_then(|m| m.enabled)
        .unwrap_or(false);

    if !cdn_enabled {
        return Err("CDN 未启用".to_string());
    }

    let base_url = config
        .environment_mirror
        .as_ref()
        .and_then(|m| m.base_url.as_ref())
        .ok_or("CDN 地址未配置")?;

    let metadata_url = format!("{}/global/plugins/{}/metadata.json", base_url, language);
    info!("从 CDN 获取 {} metadata: {}", language, metadata_url);

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;

    let response = client
        .get(&metadata_url)
        .send()
        .await
        .map_err(|e| format!("请求 CDN metadata 失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("CDN 返回错误状态码: {}", response.status()));
    }

    let metadata: Metadata = response
        .json()
        .await
        .map_err(|e| format!("解析 metadata.json 失败: {}", e))?;

    info!("成功从 CDN 获取 {} 个版本", metadata.releases.len());
    Ok(metadata)
}
