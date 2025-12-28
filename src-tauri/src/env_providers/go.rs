use super::metadata::{Metadata, fetch_metadata_from_cdn, is_cdn_enabled, is_fallback_enabled};
use crate::env_manager::{DownloadStatus, EnvironmentProvider, EnvironmentVersion};
use futures_util::StreamExt;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use tauri::AppHandle;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct GoRelease {
    version: String,
    stable: bool,
    files: Vec<GoFile>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct GoFile {
    filename: String,
    os: String,
    arch: String,
    version: String,
    sha256: String,
    size: u64,
    kind: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct CachedReleases {
    releases: Vec<GoRelease>,
    cached_at: SystemTime,
}

pub struct GoEnvironmentProvider {
    install_dir: PathBuf,
    cache_file: PathBuf,
}

impl GoEnvironmentProvider {
    pub fn new() -> Self {
        let install_dir = Self::get_default_install_dir();
        let cache_file = install_dir.join("releases_cache.json");

        if let Err(e) = std::fs::create_dir_all(&install_dir) {
            error!("创建 Go 安装目录失败: {}", e);
        }

        Self {
            install_dir,
            cache_file,
        }
    }

    fn get_default_install_dir() -> PathBuf {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home_dir.join(".codeforge").join("plugins").join("go")
    }

    fn read_cache(&self) -> Option<Vec<GoRelease>> {
        if !self.cache_file.exists() {
            return None;
        }

        match std::fs::read_to_string(&self.cache_file) {
            Ok(content) => match serde_json::from_str::<CachedReleases>(&content) {
                Ok(cached) => {
                    if cached.releases.is_empty() {
                        warn!("缓存的版本列表为空，将重新获取");
                        return None;
                    }

                    if let Ok(elapsed) = SystemTime::now().duration_since(cached.cached_at) {
                        if elapsed < Duration::from_secs(3600) {
                            info!("使用缓存的 Go 版本列表（缓存时间: {:?}）", elapsed);
                            return Some(cached.releases);
                        } else {
                            info!("缓存已过期（{:?}），将重新获取", elapsed);
                        }
                    }
                }
                Err(e) => {
                    warn!("解析缓存文件失败: {}", e);
                }
            },
            Err(e) => {
                warn!("读取缓存文件失败: {}", e);
            }
        }

        None
    }

    fn write_cache(&self, releases: &[GoRelease]) {
        let cached = CachedReleases {
            releases: releases.to_vec(),
            cached_at: SystemTime::now(),
        };

        match serde_json::to_string_pretty(&cached) {
            Ok(content) => {
                if let Err(e) = std::fs::write(&self.cache_file, content) {
                    warn!("写入缓存文件失败: {}", e);
                } else {
                    info!("已缓存 Go 版本列表");
                }
            }
            Err(e) => {
                warn!("序列化缓存数据失败: {}", e);
            }
        }
    }

    fn get_current_platform() -> &'static str {
        if cfg!(target_os = "macos") {
            if cfg!(target_arch = "aarch64") {
                "macos-aarch64"
            } else {
                "macos-x86_64"
            }
        } else if cfg!(target_os = "linux") {
            if cfg!(target_arch = "aarch64") {
                "linux-aarch64"
            } else {
                "linux-x86_64"
            }
        } else if cfg!(target_os = "windows") {
            "windows-x86_64"
        } else {
            "unknown"
        }
    }

    fn get_os_arch() -> (&'static str, &'static str) {
        let os = if cfg!(target_os = "macos") {
            "darwin"
        } else if cfg!(target_os = "linux") {
            "linux"
        } else if cfg!(target_os = "windows") {
            "windows"
        } else {
            "unknown"
        };

        let arch = if cfg!(target_arch = "aarch64") {
            "arm64"
        } else if cfg!(target_arch = "x86_64") {
            "amd64"
        } else {
            "unknown"
        };

        (os, arch)
    }

    fn parse_metadata_to_versions(
        &self,
        metadata: Metadata,
    ) -> Result<Vec<EnvironmentVersion>, String> {
        let current_platform = Self::get_current_platform();
        let mut versions = Vec::new();
        let mut seen_versions = std::collections::HashSet::new();

        for release in metadata.releases {
            let is_supported = release
                .supported_platforms
                .iter()
                .any(|p| p == current_platform || p.starts_with(&format!("{}-", current_platform)));

            if !is_supported {
                continue;
            }

            if !seen_versions.insert(release.version.clone()) {
                continue;
            }

            let is_installed = self.is_version_installed(&release.version);
            let install_path = if is_installed {
                Some(
                    self.get_version_install_path(&release.version)
                        .to_string_lossy()
                        .to_string(),
                )
            } else {
                None
            };

            versions.push(EnvironmentVersion {
                version: release.version.clone(),
                download_url: release.download_url.clone(),
                fallback_url: Some(release.github_url.clone()),
                install_path,
                is_installed,
                size: Some(release.size),
                release_date: Some(release.published_at.clone()),
            });
        }

        if versions.is_empty() {
            return Err(format!("没有找到支持 {} 平台的版本", current_platform));
        }

        Ok(versions)
    }

    async fn fetch_go_releases(&self) -> Result<Vec<GoRelease>, String> {
        if let Some(cached) = self.read_cache() {
            return Ok(cached);
        }

        info!("从 Go 官方 API 获取版本列表");
        let url = "https://go.dev/dl/?mode=json&include=all";

        let client = reqwest::Client::new();
        let request = client.get(url).header("User-Agent", "CodeForge");

        let response = request
            .send()
            .await
            .map_err(|e| format!("请求 Go API 失败: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Go API 返回错误: {}", response.status()));
        }

        let releases: Vec<GoRelease> = response
            .json()
            .await
            .map_err(|e| format!("解析 Go API 响应失败: {}", e))?;

        self.write_cache(&releases);

        Ok(releases)
    }

    fn get_version_install_path(&self, version: &str) -> PathBuf {
        self.install_dir.join(version)
    }

    fn is_version_installed(&self, version: &str) -> bool {
        let install_path = self.get_version_install_path(version);
        install_path.join("go").join("bin").exists()
    }

    async fn extract_archive(&self, archive_path: &Path, dest_dir: &Path) -> Result<(), String> {
        info!("正在解压文件到: {}", dest_dir.display());

        if archive_path.extension().and_then(|s| s.to_str()) == Some("zip") {
            let file = std::fs::File::open(archive_path)
                .map_err(|e| format!("打开压缩文件失败: {}", e))?;
            let mut archive =
                zip::ZipArchive::new(file).map_err(|e| format!("读取 ZIP 文件失败: {}", e))?;

            archive
                .extract(dest_dir)
                .map_err(|e| format!("解压 ZIP 文件失败: {}", e))?;
        } else {
            let tar_gz = std::fs::File::open(archive_path)
                .map_err(|e| format!("打开压缩文件失败: {}", e))?;
            let tar = flate2::read::GzDecoder::new(tar_gz);
            let mut archive = tar::Archive::new(tar);

            archive
                .unpack(dest_dir)
                .map_err(|e| format!("解压 tar.gz 文件失败: {}", e))?;
        }

        info!("解压完成");
        Ok(())
    }

    async fn download_file(
        &self,
        url: &str,
        dest: &PathBuf,
        app_handle: AppHandle,
        version: &str,
    ) -> Result<(), String> {
        info!("开始下载: {} -> {}", url, dest.display());

        let client = reqwest::Client::builder()
            .user_agent("CodeForge")
            .build()
            .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;

        let response = client
            .get(url)
            .send()
            .await
            .map_err(|e| format!("下载失败: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("下载失败: HTTP {}", response.status()));
        }

        let total_size = response.content_length().unwrap_or(0);
        let mut downloaded: u64 = 0;
        let mut file = std::fs::File::create(dest).map_err(|e| format!("创建文件失败: {}", e))?;

        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| format!("下载数据失败: {}", e))?;
            std::io::Write::write_all(&mut file, &chunk)
                .map_err(|e| format!("写入文件失败: {}", e))?;

            downloaded += chunk.len() as u64;
            let percentage = if total_size > 0 {
                (downloaded as f64 / total_size as f64 * 100.0) as u64
            } else {
                0
            };

            crate::env_manager::emit_download_progress(
                &app_handle,
                "go",
                version,
                downloaded,
                total_size,
                DownloadStatus::Downloading,
            );

            if percentage % 10 == 0 {
                info!("下载进度: {}%", percentage);
            }
        }

        info!("下载完成");
        Ok(())
    }
}

#[async_trait::async_trait]
impl EnvironmentProvider for GoEnvironmentProvider {
    fn get_language(&self) -> &'static str {
        "go"
    }

    async fn fetch_available_versions(&self) -> Result<Vec<EnvironmentVersion>, String> {
        if is_cdn_enabled() {
            match fetch_metadata_from_cdn("go").await {
                Ok(metadata) => {
                    info!("使用 CDN metadata 获取版本列表");
                    return self.parse_metadata_to_versions(metadata);
                }
                Err(e) => {
                    warn!("CDN metadata 获取失败: {}", e);

                    if !is_fallback_enabled() {
                        return Err(format!("CDN metadata 获取失败，未启用自动回退: {}", e));
                    }

                    info!("fallback 已启用，回退到 GitHub API");
                }
            }
        } else {
            info!("CDN 未启用，使用 GitHub API");
        }

        let releases = self.fetch_go_releases().await?;
        let (target_os, target_arch) = Self::get_os_arch();

        let mut versions = Vec::new();
        let mut seen_versions = std::collections::HashSet::new();

        for release in releases {
            let version = release.version.trim_start_matches("go").to_string();

            if !seen_versions.insert(version.clone()) {
                continue;
            }

            if let Some(file) = release
                .files
                .iter()
                .find(|f| f.os == target_os && f.arch == target_arch && f.kind == "archive")
            {
                let is_installed = self.is_version_installed(&version);

                let install_path = if is_installed {
                    Some(
                        self.get_version_install_path(&version)
                            .to_string_lossy()
                            .to_string(),
                    )
                } else {
                    None
                };

                let download_url = format!("https://go.dev/dl/{}", file.filename);

                versions.push(EnvironmentVersion {
                    version: version.clone(),
                    download_url,
                    fallback_url: None,
                    install_path,
                    is_installed,
                    size: Some(file.size),
                    release_date: None,
                });
            }
        }

        if versions.is_empty() {
            return Err("未找到可用的 Go 版本".to_string());
        }

        Ok(versions)
    }

    async fn get_installed_versions(&self) -> Result<Vec<EnvironmentVersion>, String> {
        let mut installed = Vec::new();

        if !self.install_dir.exists() {
            return Ok(installed);
        }

        let entries =
            std::fs::read_dir(&self.install_dir).map_err(|e| format!("读取安装目录失败: {}", e))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let version = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();

                if self.is_version_installed(&version) {
                    installed.push(EnvironmentVersion {
                        version: version.clone(),
                        download_url: String::new(),
                        fallback_url: None,
                        install_path: Some(path.to_string_lossy().to_string()),
                        is_installed: true,
                        size: None,
                        release_date: None,
                    });
                }
            }
        }

        Ok(installed)
    }

    async fn download_and_install(
        &self,
        version: &str,
        app_handle: AppHandle,
    ) -> Result<String, String> {
        info!("开始下载并安装 Go {}", version);

        if self.is_version_installed(version) {
            return Err(format!("Go {} 已经安装", version));
        }

        crate::env_manager::emit_download_progress(
            &app_handle,
            "go",
            version,
            0,
            0,
            DownloadStatus::Downloading,
        );

        let available_versions = self.fetch_available_versions().await?;

        let version_info = available_versions
            .iter()
            .find(|v| v.version == version)
            .ok_or_else(|| format!("未找到版本 {}", version))?;

        let download_url = &version_info.download_url;
        let file_name = download_url
            .split('/')
            .last()
            .ok_or_else(|| "无效的下载 URL".to_string())?;
        let temp_file = std::env::temp_dir().join(file_name);

        self.download_file(download_url, &temp_file, app_handle.clone(), version)
            .await?;

        let install_path = self.get_version_install_path(version);

        crate::env_manager::emit_download_progress(
            &app_handle,
            "go",
            version,
            0,
            0,
            DownloadStatus::Extracting,
        );

        self.extract_archive(&temp_file, &install_path).await?;

        std::fs::remove_file(&temp_file).ok();

        let go_root = install_path.join("go");
        let mut config = crate::config::get_app_config()
            .await
            .map_err(|e| format!("获取配置失败: {}", e))?;

        if let Some(plugins) = &mut config.plugins {
            if let Some(go_plugin) = plugins.iter_mut().find(|p| p.language == "go") {
                go_plugin.execute_home = Some(go_root.to_string_lossy().to_string());

                // 根据操作系统设置 run_command
                let run_cmd = if cfg!(target_os = "windows") {
                    "bin/go.exe run $filename"
                } else {
                    "bin/go run $filename"
                };
                go_plugin.run_command = Some(String::from(run_cmd));

                info!(
                    "已更新 Go 插件配置: execute_home={}, run_command={}",
                    go_root.display(),
                    run_cmd
                );
            }
        }

        crate::config::update_app_config(config, app_handle.clone())
            .await
            .map_err(|e| format!("保存配置失败: {}", e))?;

        crate::env_manager::emit_download_progress(
            &app_handle,
            "go",
            version,
            0,
            0,
            DownloadStatus::Completed,
        );

        info!("Go {} 安装成功", version);
        Ok(install_path.to_string_lossy().to_string())
    }

    async fn switch_version(&self, version: &str, app_handle: AppHandle) -> Result<(), String> {
        info!("切换 Go 版本到 {}", version);

        if !self.is_version_installed(version) {
            return Err(format!("版本 {} 未安装", version));
        }

        let install_path = self.get_version_install_path(version);
        let go_root = install_path.join("go");

        let mut config = crate::config::get_app_config()
            .await
            .map_err(|e| format!("获取配置失败: {}", e))?;

        if let Some(plugins) = &mut config.plugins {
            if let Some(go_plugin) = plugins.iter_mut().find(|p| p.language == "go") {
                go_plugin.execute_home = Some(go_root.to_string_lossy().to_string());

                // 根据操作系统设置 run_command
                let run_cmd = if cfg!(target_os = "windows") {
                    "bin/go.exe run $filename"
                } else {
                    "bin/go run $filename"
                };
                go_plugin.run_command = Some(String::from(run_cmd));

                info!(
                    "已更新 Go 插件配置: execute_home={}, run_command={}",
                    go_root.display(),
                    run_cmd
                );
            }
        }

        crate::config::update_app_config(config, app_handle.clone())
            .await
            .map_err(|e| format!("保存配置失败: {}", e))?;

        info!("成功切换到 Go {}", version);
        Ok(())
    }

    async fn get_current_version(&self) -> Result<Option<String>, String> {
        use crate::config::get_app_config_internal;

        let config = get_app_config_internal().map_err(|e| format!("获取配置失败: {}", e))?;

        if let Some(plugins) = config.plugins {
            if let Some(go_plugin) = plugins.iter().find(|p| p.language == "go") {
                if let Some(ref execute_home) = go_plugin.execute_home {
                    let path = PathBuf::from(execute_home);

                    if let Ok(relative) = path.strip_prefix(&self.install_dir) {
                        if let Some(version_component) = relative.components().next() {
                            if let Some(version) = version_component.as_os_str().to_str() {
                                info!("当前 Go 版本: {}", version);
                                return Ok(Some(version.to_string()));
                            }
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    fn get_install_dir(&self) -> PathBuf {
        self.install_dir.clone()
    }

    async fn uninstall_version(&self, version: &str) -> Result<(), String> {
        let version_dir = self.install_dir.join(version);

        if !version_dir.exists() {
            return Err(format!("版本 {} 未安装", version));
        }

        let current_version = self.get_current_version().await.ok().flatten();
        if current_version.as_deref() == Some(version) {
            return Err(format!("无法卸载当前正在使用的版本 {}", version));
        }

        std::fs::remove_dir_all(&version_dir).map_err(|e| format!("删除版本目录失败: {}", e))?;

        info!("已卸载 Go 版本 {}", version);
        Ok(())
    }
}
