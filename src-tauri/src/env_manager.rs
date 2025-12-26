use async_trait::async_trait;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter};

// 环境版本信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnvironmentVersion {
    pub version: String,
    pub download_url: String,
    pub fallback_url: Option<String>, // 备用下载地址（如 GitHub URL）
    pub install_path: Option<String>,
    pub is_installed: bool,
    pub size: Option<u64>,
    pub release_date: Option<String>,
}

// 环境管理器响应
#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentInfo {
    pub language: String,
    pub current_version: Option<String>,
    pub installed_versions: Vec<EnvironmentVersion>,
    pub available_versions: Vec<EnvironmentVersion>,
}

// 下载进度事件
#[derive(Debug, Serialize, Clone)]
pub struct DownloadProgress {
    pub language: String,
    pub version: String,
    pub downloaded: u64,
    pub total: u64,
    pub percentage: f64,
    pub status: DownloadStatus,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DownloadStatus {
    Downloading,
    Extracting,
    Installing,
    Completed,
    _Failed,
}

// 语言环境提供者特征
#[async_trait]
pub trait EnvironmentProvider: Send + Sync {
    // 获取语言名称
    fn get_language(&self) -> &'static str;

    // 获取可用版本列表
    async fn fetch_available_versions(&self) -> Result<Vec<EnvironmentVersion>, String>;

    // 获取已安装版本列表
    async fn get_installed_versions(&self) -> Result<Vec<EnvironmentVersion>, String>;

    // 下载并安装指定版本
    async fn download_and_install(
        &self,
        version: &str,
        app_handle: AppHandle,
    ) -> Result<String, String>;

    // 切换到指定版本
    async fn switch_version(&self, version: &str) -> Result<(), String>;

    // 获取当前激活的版本
    async fn get_current_version(&self) -> Result<Option<String>, String>;

    // 获取安装目录
    #[allow(dead_code)]
    fn get_install_dir(&self) -> PathBuf;
}

// 环境管理器
pub struct EnvironmentManager {
    providers: HashMap<String, Box<dyn EnvironmentProvider>>,
}

impl EnvironmentManager {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    pub fn register_provider(&mut self, provider: Box<dyn EnvironmentProvider>) {
        let language = provider.get_language().to_string();
        info!("注册环境提供者: {}", language);
        self.providers.insert(language, provider);
    }

    pub async fn get_environment_info(&self, language: &str) -> Result<EnvironmentInfo, String> {
        let provider = self
            .providers
            .get(language)
            .ok_or_else(|| format!("暂未支持 {} 语言，请前往 github 提供 issues", language))?;

        info!("获取 {} 环境信息", language);

        let current_version = provider.get_current_version().await.ok().flatten();
        let installed_versions = provider.get_installed_versions().await.unwrap_or_default();
        let available_versions = provider
            .fetch_available_versions()
            .await
            .unwrap_or_default();

        Ok(EnvironmentInfo {
            language: language.to_string(),
            current_version,
            installed_versions,
            available_versions,
        })
    }

    pub async fn download_and_install_version(
        &self,
        language: &str,
        version: &str,
        app_handle: AppHandle,
    ) -> Result<String, String> {
        let provider = self
            .providers
            .get(language)
            .ok_or_else(|| format!("暂未支持 {} 语言，请前往 github 提供 issues", language))?;

        info!("开始下载并安装 {} 版本 {}", language, version);
        provider.download_and_install(version, app_handle).await
    }

    pub async fn switch_version(&self, language: &str, version: &str) -> Result<(), String> {
        let provider = self
            .providers
            .get(language)
            .ok_or_else(|| format!("暂未支持 {} 语言，请前往 github 提供 issues", language))?;

        info!("切换 {} 到版本 {}", language, version);
        provider.switch_version(version).await
    }

    pub fn get_supported_languages(&self) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }
}

// 辅助函数：发送下载进度事件
pub fn emit_download_progress(
    app_handle: &AppHandle,
    language: &str,
    version: &str,
    downloaded: u64,
    total: u64,
    status: DownloadStatus,
) {
    let percentage = if total > 0 {
        (downloaded as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    let progress = DownloadProgress {
        language: language.to_string(),
        version: version.to_string(),
        downloaded,
        total,
        percentage,
        status,
    };

    if let Err(e) = app_handle.emit("env-download-progress", progress) {
        error!("发送下载进度事件失败: {}", e);
    }
}

// 将 GitHub 下载 URL 转换为 CDN 镜像 URL
pub fn convert_to_cdn_url(
    original_url: &str,
    language: &str,
    version: &str,
) -> Result<String, String> {
    use crate::config::get_app_config_internal;

    let config = get_app_config_internal().map_err(|e| format!("获取配置失败: {}", e))?;

    let mirror_config = config.environment_mirror.as_ref();
    let enabled = mirror_config.and_then(|m| m.enabled).unwrap_or(false);

    if !enabled {
        return Ok(original_url.to_string());
    }

    let base_url = mirror_config
        .and_then(|m| m.base_url.as_ref())
        .ok_or_else(|| "CDN 基础 URL 未配置".to_string())?;

    // 从原始 URL 中提取文件名
    let file_name = original_url
        .split('/')
        .last()
        .ok_or_else(|| "无效的下载 URL".to_string())?;

    // 构建 CDN URL: {base_url}/{language}/{version}/{filename}
    let cdn_url = format!(
        "{}/{}/{}/{}",
        base_url.trim_end_matches('/'),
        language,
        version,
        file_name
    );

    info!("转换下载 URL: {} -> {}", original_url, cdn_url);
    Ok(cdn_url)
}

// 尝试从 CDN 下载，失败则回退到原始 URL
pub async fn download_with_fallback(
    client: &reqwest::Client,
    original_url: &str,
    language: &str,
    version: &str,
) -> Result<reqwest::Response, String> {
    use crate::config::get_app_config_internal;

    // 检查是否启用自动回退
    let fallback_enabled = get_app_config_internal()
        .ok()
        .and_then(|config| config.environment_mirror)
        .and_then(|mirror| mirror.fallback_enabled)
        .unwrap_or(false);

    // 首先尝试从 CDN 下载
    match convert_to_cdn_url(original_url, language, version) {
        Ok(cdn_url) if cdn_url != original_url => {
            info!("尝试从 CDN 下载: {}", cdn_url);
            match client.get(&cdn_url).send().await {
                Ok(response) if response.status().is_success() => {
                    info!("CDN 下载成功");
                    return Ok(response);
                }
                Ok(response) => {
                    let status = response.status();
                    if fallback_enabled {
                        info!("CDN 下载失败 (HTTP {}), 回退到原始 URL", status);
                    } else {
                        return Err(format!("CDN 下载失败 (HTTP {}), 未启用自动回退", status));
                    }
                }
                Err(e) => {
                    if fallback_enabled {
                        info!("CDN 下载失败 ({}), 回退到原始 URL", e);
                    } else {
                        return Err(format!("CDN 下载失败 ({}), 未启用自动回退", e));
                    }
                }
            }
        }
        Ok(_) => {
            info!("CDN 镜像未启用，使用原始 URL");
        }
        Err(e) => {
            info!("CDN URL 转换失败 ({}), 使用原始 URL", e);
        }
    }

    // 回退到原始 URL
    info!("从原始 URL 下载: {}", original_url);
    client
        .get(original_url)
        .send()
        .await
        .map_err(|e| format!("下载失败: {}", e))
}
