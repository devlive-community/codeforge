use super::metadata::{fetch_metadata_from_cdn, is_cdn_enabled, is_fallback_enabled};
use crate::env_manager::{DownloadStatus, EnvironmentProvider, EnvironmentVersion};
use async_trait::async_trait;
use futures_util::StreamExt;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use tauri::AppHandle;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct RustRelease {
    version: String,
    date: String,
    download_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct CachedReleases {
    releases: Vec<RustRelease>,
    cached_at: SystemTime,
}

pub struct RustEnvironmentProvider {
    install_dir: PathBuf,
    cache_file: PathBuf,
}

impl RustEnvironmentProvider {
    pub fn new() -> Self {
        let install_dir = Self::get_default_install_dir();
        let cache_file = install_dir.join("releases_cache.json");

        if let Err(e) = std::fs::create_dir_all(&install_dir) {
            error!("创建 Rust 安装目录失败: {}", e);
        }

        Self {
            install_dir,
            cache_file,
        }
    }

    fn get_default_install_dir() -> PathBuf {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home_dir.join(".codeforge").join("plugins").join("rust")
    }

    fn read_cache(&self) -> Option<Vec<RustRelease>> {
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
                            info!("使用缓存的 Rust 版本列表（缓存时间: {:?}）", elapsed);
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

    fn write_cache(&self, releases: &[RustRelease]) {
        let cached = CachedReleases {
            releases: releases.to_vec(),
            cached_at: SystemTime::now(),
        };

        match serde_json::to_string_pretty(&cached) {
            Ok(content) => {
                if let Err(e) = std::fs::write(&self.cache_file, content) {
                    error!("写入缓存文件失败: {}", e);
                }
            }
            Err(e) => {
                error!("序列化缓存数据失败: {}", e);
            }
        }
    }

    async fn fetch_rust_releases(&self) -> Result<Vec<RustRelease>, String> {
        if let Some(cached_releases) = self.read_cache() {
            return Ok(cached_releases);
        }

        info!("从官方 API 获取 Rust 版本列表");

        let client = reqwest::Client::builder()
            .user_agent("CodeForge")
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;

        let platform_target = Self::get_platform_target()?;
        let mut all_releases = Vec::new();

        let channels = vec!["stable", "beta", "nightly"];

        for channel in channels {
            let url = format!(
                "https://static.rust-lang.org/dist/channel-rust-{}.toml",
                channel
            );

            let response = match client.get(&url).send().await {
                Ok(resp) if resp.status().is_success() => resp,
                _ => continue,
            };

            let content = match response.text().await {
                Ok(c) => c,
                Err(_) => continue,
            };

            if let Some(release) = Self::parse_channel_toml(&content, &platform_target, channel) {
                all_releases.push(release);
            }
        }

        if all_releases.is_empty() {
            return Err(format!("未找到 {} 平台的 Rust 版本信息", platform_target));
        }

        self.write_cache(&all_releases);
        Ok(all_releases)
    }

    fn parse_channel_toml(
        content: &str,
        platform_target: &str,
        channel: &str,
    ) -> Option<RustRelease> {
        let mut date_str = String::new();
        let mut version_str = String::new();
        let mut download_url = String::new();
        let mut in_rust_section = false;
        let mut in_target_section = false;

        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("date = ") {
                if let Some(date) = trimmed.split('"').nth(1) {
                    date_str = date.to_string();
                }
            } else if trimmed == "[pkg.rust]" {
                in_rust_section = true;
                in_target_section = false;
            } else if in_rust_section && trimmed.starts_with("version = ") {
                if let Some(version) = trimmed.split('"').nth(1) {
                    version_str = version.split_whitespace().next().unwrap_or("").to_string();
                }
            } else if in_rust_section && trimmed == format!("[pkg.rust.target.{}]", platform_target)
            {
                in_target_section = true;
            } else if in_target_section && trimmed.starts_with("url = ") {
                if let Some(url) = trimmed.split('"').nth(1) {
                    download_url = url.to_string();
                }
                break;
            } else if trimmed.starts_with("[pkg.") && !trimmed.starts_with("[pkg.rust") {
                in_rust_section = false;
                in_target_section = false;
            }
        }

        if !version_str.is_empty() && !download_url.is_empty() {
            let display_version = if channel == "stable" {
                version_str.clone()
            } else {
                format!("{} ({})", version_str, channel)
            };

            Some(RustRelease {
                version: display_version,
                date: date_str,
                download_url,
            })
        } else {
            None
        }
    }

    fn get_platform_target() -> Result<String, String> {
        let (arch, os) = Self::get_platform_info()?;
        Ok(format!("{}-{}", arch, os))
    }

    fn get_platform_info() -> Result<(&'static str, &'static str), String> {
        let os = if cfg!(target_os = "windows") {
            "pc-windows-msvc"
        } else if cfg!(target_os = "macos") {
            "apple-darwin"
        } else if cfg!(target_os = "linux") {
            "unknown-linux-gnu"
        } else {
            return Err("不支持的操作系统".to_string());
        };

        let arch = if cfg!(target_arch = "x86_64") {
            "x86_64"
        } else if cfg!(target_arch = "aarch64") {
            "aarch64"
        } else {
            return Err("不支持的架构".to_string());
        };

        Ok((arch, os))
    }

    async fn get_download_url(&self, version: &str) -> Result<String, String> {
        let releases = self.fetch_rust_releases().await?;

        releases
            .iter()
            .find(|r| r.version == version)
            .map(|r| r.download_url.clone())
            .ok_or_else(|| format!("未找到版本 {} 的下载地址", version))
    }

    fn get_version_install_path(&self, version: &str) -> PathBuf {
        self.install_dir.join(version)
    }

    fn is_version_installed(&self, version: &str) -> bool {
        let install_path = self.get_version_install_path(version);
        install_path.join("rustc").join("bin").exists() || install_path.join("bin").exists()
    }

    async fn download_file(
        &self,
        url: &str,
        dest_path: &Path,
        app_handle: &AppHandle,
        version: &str,
    ) -> Result<(), String> {
        info!("开始下载: {}", url);

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(600))
            .build()
            .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;

        let response = client
            .get(url)
            .send()
            .await
            .map_err(|e| format!("下载失败: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("HTTP 请求失败: {}", response.status()));
        }

        let total_size = response.content_length().unwrap_or(0);

        if let Some(parent) = dest_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
        }

        let mut file =
            std::fs::File::create(dest_path).map_err(|e| format!("创建文件失败: {}", e))?;
        let mut downloaded: u64 = 0;
        let mut stream = response.bytes_stream();

        crate::env_manager::emit_download_progress(
            app_handle,
            "rust",
            version,
            0,
            total_size,
            DownloadStatus::Downloading,
        );

        while let Some(item) = stream.next().await {
            let chunk = item.map_err(|e| format!("下载数据失败: {}", e))?;
            std::io::Write::write_all(&mut file, &chunk)
                .map_err(|e| format!("写入文件失败: {}", e))?;
            downloaded += chunk.len() as u64;

            crate::env_manager::emit_download_progress(
                app_handle,
                "rust",
                version,
                downloaded,
                total_size,
                DownloadStatus::Downloading,
            );
        }

        info!("下载完成: {}", dest_path.display());
        Ok(())
    }

    async fn extract_archive(&self, archive_path: &Path, extract_to: &Path) -> Result<(), String> {
        info!(
            "解压文件: {} 到 {}",
            archive_path.display(),
            extract_to.display()
        );

        std::fs::create_dir_all(extract_to).map_err(|e| format!("创建解压目录失败: {}", e))?;

        let tar_gz =
            std::fs::File::open(archive_path).map_err(|e| format!("打开压缩文件失败: {}", e))?;
        let tar = flate2::read::GzDecoder::new(tar_gz);
        let mut archive = tar::Archive::new(tar);
        archive
            .unpack(extract_to)
            .map_err(|e| format!("解压文件失败: {}", e))?;

        info!("解压完成");
        Ok(())
    }

    async fn update_plugin_config(
        &self,
        version: &str,
        app_handle: AppHandle,
    ) -> Result<(), String> {
        use crate::config::{get_app_config_internal, update_app_config};

        let mut config = get_app_config_internal().map_err(|e| format!("获取配置失败: {}", e))?;

        let install_path = self.get_version_install_path(version);

        if let Some(plugins) = config.plugins.as_mut() {
            if let Some(rust_plugin) = plugins.iter_mut().find(|p| p.language == "rust") {
                rust_plugin.execute_home = Some(install_path.to_string_lossy().to_string());

                let rustc_bin = install_path.join("rustc").join("bin");
                let has_rustc_dir = rustc_bin.exists();

                let run_command = if has_rustc_dir {
                    "rustc/bin/rustc $filename -o /tmp/main && /tmp/main".to_string()
                } else {
                    "bin/rustc $filename -o /tmp/main && /tmp/main".to_string()
                };

                rust_plugin.run_command = Some(run_command);

                info!(
                    "已更新 Rust 插件配置: execute_home={}, run_command={}",
                    install_path.display(),
                    rust_plugin.run_command.as_ref().unwrap()
                );
            }
        }

        update_app_config(config, app_handle)
            .await
            .map_err(|e| format!("保存配置失败: {}", e))?;

        info!("成功更新 Rust 配置");
        Ok(())
    }
}

#[async_trait]
impl EnvironmentProvider for RustEnvironmentProvider {
    fn get_language(&self) -> &'static str {
        "rust"
    }

    async fn fetch_available_versions(&self) -> Result<Vec<EnvironmentVersion>, String> {
        let releases = self.fetch_rust_releases().await?;

        Ok(releases
            .into_iter()
            .map(|release| {
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

                EnvironmentVersion {
                    version: release.version.clone(),
                    download_url: release.download_url.clone(),
                    fallback_url: None,
                    install_path,
                    is_installed,
                    size: None,
                    release_date: Some(release.date),
                }
            })
            .collect())
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
                if let Some(version) = path.file_name().and_then(|n| n.to_str()) {
                    if version != "releases_cache.json" && self.is_version_installed(version) {
                        let download_url = self.get_download_url(version).await.unwrap_or_default();
                        installed.push(EnvironmentVersion {
                            version: version.to_string(),
                            download_url,
                            fallback_url: None,
                            install_path: Some(path.to_string_lossy().to_string()),
                            is_installed: true,
                            size: None,
                            release_date: None,
                        });
                    }
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
        if self.is_version_installed(version) {
            return Err(format!("版本 {} 已安装", version));
        }

        crate::env_manager::emit_download_progress(
            &app_handle,
            "rust",
            version,
            0,
            100,
            DownloadStatus::Downloading,
        );

        let download_url = self.get_download_url(version).await?;
        let temp_file = self.install_dir.join(format!("rust-{}.tar.gz", version));

        let should_download = if is_cdn_enabled() {
            let metadata = fetch_metadata_from_cdn("rust").await;
            if let Ok(meta) = metadata {
                if let Some(release) = meta.releases.iter().find(|r| r.version == version) {
                    info!("使用 CDN 下载 Rust {}", version);
                    self.download_file(&release.download_url, &temp_file, &app_handle, version)
                        .await
                } else if is_fallback_enabled() {
                    info!("CDN 中未找到版本，使用官方源");
                    self.download_file(&download_url, &temp_file, &app_handle, version)
                        .await
                } else {
                    Err("CDN 中未找到该版本且未启用回退".to_string())
                }
            } else if is_fallback_enabled() {
                info!("获取 CDN 元数据失败，使用官方源");
                self.download_file(&download_url, &temp_file, &app_handle, version)
                    .await
            } else {
                Err("获取 CDN 元数据失败且未启用回退".to_string())
            }
        } else {
            self.download_file(&download_url, &temp_file, &app_handle, version)
                .await
        };

        should_download?;

        crate::env_manager::emit_download_progress(
            &app_handle,
            "rust",
            version,
            0,
            100,
            DownloadStatus::Extracting,
        );

        let temp_extract_dir = self.install_dir.join(format!("temp_{}", version));
        self.extract_archive(&temp_file, &temp_extract_dir).await?;

        let install_path = self.get_version_install_path(version);

        let extracted_dirs: Vec<_> = std::fs::read_dir(&temp_extract_dir)
            .map_err(|e| format!("读取临时目录失败: {}", e))?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .collect();

        if let Some(rust_dir) = extracted_dirs.first() {
            std::fs::rename(rust_dir.path(), &install_path)
                .map_err(|e| format!("移动安装目录失败: {}", e))?;
        } else {
            return Err("未找到 Rust 安装目录".to_string());
        }

        std::fs::remove_dir_all(&temp_extract_dir).ok();
        std::fs::remove_file(&temp_file).ok();

        self.update_plugin_config(version, app_handle.clone())
            .await?;

        crate::env_manager::emit_download_progress(
            &app_handle,
            "rust",
            version,
            100,
            100,
            DownloadStatus::Completed,
        );

        Ok(install_path.to_string_lossy().to_string())
    }

    async fn switch_version(&self, version: &str, app_handle: AppHandle) -> Result<(), String> {
        if !self.is_version_installed(version) {
            return Err(format!("版本 {} 未安装", version));
        }

        self.update_plugin_config(version, app_handle).await?;
        info!("已切换到 Rust {}", version);
        Ok(())
    }

    async fn get_current_version(&self) -> Result<Option<String>, String> {
        use crate::config::get_app_config_internal;

        let config = get_app_config_internal().map_err(|e| format!("获取配置失败: {}", e))?;

        if let Some(plugins) = config.plugins {
            if let Some(rust_plugin) = plugins.iter().find(|p| p.language == "rust") {
                if let Some(ref execute_home) = rust_plugin.execute_home {
                    let path = PathBuf::from(execute_home);

                    if let Ok(relative) = path.strip_prefix(&self.install_dir) {
                        if let Some(version_component) = relative.components().next() {
                            if let Some(version) = version_component.as_os_str().to_str() {
                                info!("当前 Rust 版本: {}", version);
                                return Ok(Some(version.to_string()));
                            }
                        }
                    }

                    if let Some(version) = path.file_name().and_then(|n| n.to_str()) {
                        info!("当前 Rust 版本: {}", version);
                        return Ok(Some(version.to_string()));
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
        let install_path = self.get_version_install_path(version);

        if !install_path.exists() {
            return Err(format!("版本 {} 未安装", version));
        }

        std::fs::remove_dir_all(&install_path).map_err(|e| format!("删除安装目录失败: {}", e))?;

        info!("已卸载 Rust {}", version);
        Ok(())
    }
}
