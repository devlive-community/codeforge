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
struct JavaRelease {
    version: String,
    date: String,
    download_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct CachedReleases {
    releases: Vec<JavaRelease>,
    cached_at: SystemTime,
}

pub struct JavaEnvironmentProvider {
    install_dir: PathBuf,
    cache_file: PathBuf,
}

impl JavaEnvironmentProvider {
    pub fn new() -> Self {
        let install_dir = Self::get_default_install_dir();
        let cache_file = install_dir.join("releases_cache.json");

        if let Err(e) = std::fs::create_dir_all(&install_dir) {
            error!("创建 Java 安装目录失败: {}", e);
        }

        Self {
            install_dir,
            cache_file,
        }
    }

    fn get_default_install_dir() -> PathBuf {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home_dir.join(".codeforge").join("plugins").join("java")
    }

    fn read_cache(&self) -> Option<Vec<JavaRelease>> {
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
                            info!("使用缓存的 Java 版本列表（缓存时间: {:?}）", elapsed);
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

    fn write_cache(&self, releases: &[JavaRelease]) {
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

    async fn fetch_java_releases(&self) -> Result<Vec<JavaRelease>, String> {
        if let Some(cached_releases) = self.read_cache() {
            return Ok(cached_releases);
        }

        info!("从 Adoptium API 获取 Java 版本列表");

        let client = reqwest::Client::builder()
            .user_agent("CodeForge")
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;

        let (os, arch) = Self::get_platform_info()?;
        let mut all_releases = Vec::new();

        let versions = vec![21, 17, 11, 8];

        for version in versions {
            let url = format!(
                "https://api.adoptium.net/v3/assets/latest/{}/hotspot?architecture={}&image_type=jdk&os={}&vendor=eclipse",
                version, arch, os
            );

            info!("请求 Java {} 版本信息: {}", version, url);

            let response = match client.get(&url).send().await {
                Ok(resp) if resp.status().is_success() => resp,
                Ok(resp) => {
                    warn!("请求 Java {} 失败: {}", version, resp.status());
                    continue;
                }
                Err(e) => {
                    warn!("请求 Java {} 失败: {}", version, e);
                    continue;
                }
            };

            let releases: Vec<serde_json::Value> = match response.json().await {
                Ok(r) => r,
                Err(e) => {
                    warn!("解析 Java {} 响应失败: {}", version, e);
                    continue;
                }
            };

            for release in releases {
                if let Some(version_obj) = release.get("version") {
                    if let Some(semver) = version_obj.get("semver").and_then(|v| v.as_str()) {
                        if let Some(binary) = release.get("binary") {
                            if let Some(package) = binary.get("package") {
                                if let Some(download_url) =
                                    package.get("link").and_then(|l| l.as_str())
                                {
                                    let date = binary
                                        .get("updated_at")
                                        .and_then(|d| d.as_str())
                                        .unwrap_or("")
                                        .split('T')
                                        .next()
                                        .unwrap_or("")
                                        .to_string();

                                    info!("找到 Java 版本: {} - {}", semver, download_url);

                                    all_releases.push(JavaRelease {
                                        version: semver.to_string(),
                                        date,
                                        download_url: download_url.to_string(),
                                    });
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        if all_releases.is_empty() {
            return Err(format!("未找到 {} {} 平台的 Java 版本信息", os, arch));
        }

        self.write_cache(&all_releases);
        Ok(all_releases)
    }

    fn get_platform_info() -> Result<(&'static str, &'static str), String> {
        let os = if cfg!(target_os = "windows") {
            "windows"
        } else if cfg!(target_os = "macos") {
            "mac"
        } else if cfg!(target_os = "linux") {
            "linux"
        } else {
            return Err("不支持的操作系统".to_string());
        };

        let arch = if cfg!(target_arch = "x86_64") {
            "x64"
        } else if cfg!(target_arch = "aarch64") {
            if cfg!(target_os = "macos") {
                "x64"
            } else {
                "aarch64"
            }
        } else {
            return Err("不支持的架构".to_string());
        };

        Ok((os, arch))
    }

    async fn get_download_url(&self, version: &str) -> Result<String, String> {
        let releases = self.fetch_java_releases().await?;

        releases
            .iter()
            .find(|r| r.version == version)
            .map(|r| r.download_url.clone())
            .ok_or_else(|| format!("未找到版本 {} 的下载地址", version))
    }

    fn get_version_install_path(&self, version: &str) -> PathBuf {
        self.install_dir.join(version)
    }

    fn get_java_home_path(&self, version: &str) -> PathBuf {
        let install_path = self.get_version_install_path(version);
        if cfg!(target_os = "macos") {
            install_path.join("Contents").join("Home")
        } else {
            install_path
        }
    }

    fn is_version_installed(&self, version: &str) -> bool {
        let java_home = self.get_java_home_path(version);
        java_home.join("bin").join("java").exists()
            || java_home.join("bin").join("java.exe").exists()
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
            "java",
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
                "java",
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

        let path_str = archive_path.to_string_lossy();

        if path_str.ends_with(".tar.gz") || path_str.ends_with(".tgz") {
            let tar_gz = std::fs::File::open(archive_path)
                .map_err(|e| format!("打开压缩文件失败: {}", e))?;
            let tar = flate2::read::GzDecoder::new(tar_gz);
            let mut archive = tar::Archive::new(tar);
            archive
                .unpack(extract_to)
                .map_err(|e| format!("解压文件失败: {}", e))?;
        } else if path_str.ends_with(".zip") {
            let file = std::fs::File::open(archive_path)
                .map_err(|e| format!("打开压缩文件失败: {}", e))?;
            let mut archive =
                zip::ZipArchive::new(file).map_err(|e| format!("创建 ZIP 解压器失败: {}", e))?;

            for i in 0..archive.len() {
                let mut file = archive
                    .by_index(i)
                    .map_err(|e| format!("读取 ZIP 文件条目失败: {}", e))?;
                let outpath = extract_to.join(file.name());

                if file.is_dir() {
                    std::fs::create_dir_all(&outpath)
                        .map_err(|e| format!("创建目录失败: {}", e))?;
                } else {
                    if let Some(p) = outpath.parent() {
                        std::fs::create_dir_all(p).map_err(|e| format!("创建父目录失败: {}", e))?;
                    }
                    let mut outfile = std::fs::File::create(&outpath)
                        .map_err(|e| format!("创建文件失败: {}", e))?;
                    std::io::copy(&mut file, &mut outfile)
                        .map_err(|e| format!("复制文件失败: {}", e))?;
                }

                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Some(mode) = file.unix_mode() {
                        std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))
                            .ok();
                    }
                }
            }
        } else {
            return Err("不支持的压缩格式".to_string());
        }

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

        let java_home = self.get_java_home_path(version);

        if let Some(plugins) = config.plugins.as_mut() {
            if let Some(java_plugin) = plugins.iter_mut().find(|p| p.language == "java") {
                java_plugin.execute_home = Some(java_home.to_string_lossy().to_string());

                #[cfg(target_os = "windows")]
                let run_command = "bin\\java $filename".to_string();

                #[cfg(not(target_os = "windows"))]
                let run_command = "bin/java $filename".to_string();

                java_plugin.run_command = Some(run_command);

                info!(
                    "已更新 Java 插件配置: execute_home={}, run_command={}",
                    java_home.display(),
                    java_plugin.run_command.as_ref().unwrap()
                );
            }
        }

        update_app_config(config, app_handle)
            .await
            .map_err(|e| format!("保存配置失败: {}", e))?;

        info!("成功更新 Java 配置");
        Ok(())
    }
}

#[async_trait]
impl EnvironmentProvider for JavaEnvironmentProvider {
    fn get_language(&self) -> &'static str {
        "java"
    }

    async fn fetch_available_versions(&self) -> Result<Vec<EnvironmentVersion>, String> {
        let releases = self.fetch_java_releases().await?;

        Ok(releases
            .into_iter()
            .map(|release| {
                let is_installed = self.is_version_installed(&release.version);
                let install_path = if is_installed {
                    Some(
                        self.get_java_home_path(&release.version)
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
                        let java_home = self.get_java_home_path(version);
                        installed.push(EnvironmentVersion {
                            version: version.to_string(),
                            download_url,
                            fallback_url: None,
                            install_path: Some(java_home.to_string_lossy().to_string()),
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
            "java",
            version,
            0,
            100,
            DownloadStatus::Downloading,
        );

        let download_url = self.get_download_url(version).await?;

        let file_ext = if download_url.ends_with(".tar.gz") {
            "tar.gz"
        } else if download_url.ends_with(".zip") {
            "zip"
        } else {
            "tar.gz"
        };

        let temp_file = self
            .install_dir
            .join(format!("java-{}.{}", version, file_ext));

        let should_download = if is_cdn_enabled() {
            let metadata = fetch_metadata_from_cdn("java").await;
            if let Ok(meta) = metadata {
                if let Some(release) = meta.releases.iter().find(|r| r.version == version) {
                    info!("使用 CDN 下载 Java {}", version);
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
            "java",
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

        if let Some(java_dir) = extracted_dirs.first() {
            std::fs::rename(java_dir.path(), &install_path)
                .map_err(|e| format!("移动安装目录失败: {}", e))?;
        } else {
            return Err("未找到 Java 安装目录".to_string());
        }

        std::fs::remove_dir_all(&temp_extract_dir).ok();
        std::fs::remove_file(&temp_file).ok();

        self.update_plugin_config(version, app_handle.clone())
            .await?;

        crate::env_manager::emit_download_progress(
            &app_handle,
            "java",
            version,
            100,
            100,
            DownloadStatus::Completed,
        );

        let java_home = self.get_java_home_path(version);
        Ok(java_home.to_string_lossy().to_string())
    }

    async fn switch_version(&self, version: &str, app_handle: AppHandle) -> Result<(), String> {
        if !self.is_version_installed(version) {
            return Err(format!("版本 {} 未安装", version));
        }

        self.update_plugin_config(version, app_handle).await?;
        info!("已切换到 Java {}", version);
        Ok(())
    }

    async fn get_current_version(&self) -> Result<Option<String>, String> {
        use crate::config::get_app_config_internal;

        let config = get_app_config_internal().map_err(|e| format!("获取配置失败: {}", e))?;

        if let Some(plugins) = config.plugins {
            if let Some(java_plugin) = plugins.iter().find(|p| p.language == "java") {
                if let Some(ref execute_home) = java_plugin.execute_home {
                    let path = PathBuf::from(execute_home);

                    let version_path = if cfg!(target_os = "macos") {
                        if let Some(parent) = path.parent() {
                            if let Some(grandparent) = parent.parent() {
                                grandparent
                            } else {
                                &path
                            }
                        } else {
                            &path
                        }
                    } else {
                        &path
                    };

                    if let Ok(relative) = version_path.strip_prefix(&self.install_dir) {
                        if let Some(version_component) = relative.components().next() {
                            if let Some(version) = version_component.as_os_str().to_str() {
                                info!("当前 Java 版本: {}", version);
                                return Ok(Some(version.to_string()));
                            }
                        }
                    }

                    if let Some(version) = version_path.file_name().and_then(|n| n.to_str()) {
                        info!("当前 Java 版本: {}", version);
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

        info!("已卸载 Java {}", version);
        Ok(())
    }
}
