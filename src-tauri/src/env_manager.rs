use async_trait::async_trait;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter};

// 环境版本信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnvironmentVersion {
    pub version: String,
    pub download_url: String,
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
    Failed,
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

    pub async fn get_environment_info(
        &self,
        language: &str,
    ) -> Result<EnvironmentInfo, String> {
        let provider = self
            .providers
            .get(language)
            .ok_or_else(|| format!("暂未支持 {} 语言，请前往 github 提供 issus", language))?;

        info!("获取 {} 环境信息", language);

        let current_version = provider.get_current_version().await.ok().flatten();
        let installed_versions = provider.get_installed_versions().await.unwrap_or_default();
        let available_versions = provider.fetch_available_versions().await.unwrap_or_default();

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
            .ok_or_else(|| format!("暂未支持 {} 语言，请前往 github 提供 issus", language))?;

        info!("开始下载并安装 {} 版本 {}", language, version);
        provider.download_and_install(version, app_handle).await
    }

    pub async fn switch_version(&self, language: &str, version: &str) -> Result<(), String> {
        let provider = self
            .providers
            .get(language)
            .ok_or_else(|| format!("暂未支持 {} 语言，请前往 github 提供 issus", language))?;

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
