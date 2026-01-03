use super::metadata::{fetch_metadata_from_cdn, is_cdn_enabled, is_fallback_enabled};
use crate::env_manager::{DownloadStatus, EnvironmentProvider, EnvironmentVersion};
use async_trait::async_trait;
use futures_util::StreamExt;
use log::info;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use tauri::AppHandle;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct PhpRelease {
    version: String,
    date: String,
    download_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct CachedReleases {
    releases: Vec<PhpRelease>,
    cached_at: SystemTime,
}

pub struct PhpEnvironmentProvider {
    install_dir: PathBuf,
    cache_file: PathBuf,
}

impl PhpEnvironmentProvider {
    pub fn new() -> Self {
        let home_dir = dirs::home_dir().expect("无法获取用户主目录");
        let install_dir = home_dir.join(".codeforge").join("plugins").join("php");
        let cache_file = install_dir.join("releases_cache.json");

        std::fs::create_dir_all(&install_dir).ok();

        Self {
            install_dir,
            cache_file,
        }
    }

    fn read_cache(&self) -> Option<Vec<PhpRelease>> {
        if !self.cache_file.exists() {
            return None;
        }

        let content = std::fs::read_to_string(&self.cache_file).ok()?;
        let cached: CachedReleases = serde_json::from_str(&content).ok()?;

        let cache_age = SystemTime::now()
            .duration_since(cached.cached_at)
            .unwrap_or(Duration::from_secs(u64::MAX));

        if cache_age < Duration::from_secs(3600) {
            info!("使用缓存的 PHP 版本列表");
            Some(cached.releases)
        } else {
            None
        }
    }

    fn write_cache(&self, releases: &[PhpRelease]) {
        let cached = CachedReleases {
            releases: releases.to_vec(),
            cached_at: SystemTime::now(),
        };

        if let Ok(content) = serde_json::to_string_pretty(&cached) {
            std::fs::write(&self.cache_file, content).ok();
        }
    }

    async fn fetch_php_releases(&self) -> Result<Vec<PhpRelease>, String> {
        if let Some(cached) = self.read_cache() {
            return Ok(cached);
        }

        info!("从 GitHub API 获取 PHP 版本列表");

        let (os, arch) = Self::get_platform_info()?;

        // macOS 暂不支持自动安装 PHP，建议用户通过 Homebrew 安装
        if os == "darwin" {
            return Err("macOS 平台暂不支持自动安装 PHP。请使用 Homebrew 安装:\n  brew install php\n或访问 https://www.php.net/downloads 下载源码编译安装。".to_string());
        }

        let client = reqwest::Client::builder()
            .user_agent("CodeForge")
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;

        // Linux 使用 php-builder 仓库
        let repo_url =
            "https://api.github.com/repos/shivammathur/php-builder/releases?per_page=100";

        info!("使用仓库: {}", repo_url);

        let response = client
            .get(repo_url)
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("HTTP 请求失败: {}", response.status()));
        }

        let releases: Vec<serde_json::Value> = response
            .json()
            .await
            .map_err(|e| format!("解析 JSON 失败: {}", e))?;

        let mut php_releases = Vec::new();

        info!("正在搜索平台: Linux {}", arch);

        for release in releases {
            if let Some(tag_name) = release["tag_name"].as_str() {
                // Linux 版本标签格式为纯版本号或 php-版本号
                let version = tag_name
                    .trim_start_matches("php-")
                    .trim_start_matches("php_");

                if let Some(assets) = release["assets"].as_array() {
                    for asset in assets {
                        if let Some(name) = asset["name"].as_str() {
                            // Linux: 匹配 ubuntu 和 debian 的 tar.xz 或 tar.zst 文件
                            // 文件名格式: php_8.3+ubuntu22.04_arm64.tar.xz 或 php_8.3+debian12.tar.xz
                            let is_compressed =
                                name.ends_with(".tar.xz") || name.ends_with(".tar.zst");
                            let is_php_package =
                                name.starts_with("php_") || name.starts_with("php-");
                            let is_not_debug = !name.contains("dbgsym");

                            // 匹配架构（如果文件名中包含架构信息）或默认 x86_64 包
                            let matches_arch = if arch == "arm64" {
                                name.contains("arm64") || name.contains("aarch64")
                            } else {
                                // x86_64 包通常不带架构后缀
                                !name.contains("arm64") && !name.contains("aarch64")
                            };

                            if is_compressed && is_php_package && is_not_debug && matches_arch {
                                if let Some(download_url) = asset["browser_download_url"].as_str() {
                                    let date = release["published_at"]
                                        .as_str()
                                        .and_then(|s| s.split('T').next())
                                        .unwrap_or("")
                                        .to_string();

                                    info!(
                                        "找到 PHP 版本: {} - {} - {}",
                                        version, name, download_url
                                    );

                                    php_releases.push(PhpRelease {
                                        version: version.to_string(),
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

        if php_releases.is_empty() {
            return Err(format!(
                "未找到 Linux {} 平台的 PHP 版本。请检查 https://github.com/shivammathur/php-builder/releases 是否有适合您平台的版本。",
                arch
            ));
        }

        info!("成功获取 {} 个 PHP 版本", php_releases.len());
        self.write_cache(&php_releases);
        Ok(php_releases)
    }

    fn get_platform_info() -> Result<(&'static str, &'static str), String> {
        let os = if cfg!(target_os = "macos") {
            "darwin"
        } else if cfg!(target_os = "linux") {
            "linux"
        } else {
            return Err("不支持的操作系统".to_string());
        };

        let arch = if cfg!(target_arch = "aarch64") {
            "arm64"
        } else if cfg!(target_arch = "x86_64") {
            "x86_64"
        } else {
            return Err("不支持的架构".to_string());
        };

        Ok((os, arch))
    }

    async fn get_download_url(&self, version: &str) -> Result<String, String> {
        let releases = self.fetch_php_releases().await?;

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
        install_path.join("bin").join("php").exists()
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
            "php",
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
                "php",
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

        let file =
            std::fs::File::open(archive_path).map_err(|e| format!("打开压缩文件失败: {}", e))?;

        let path_str = archive_path.to_string_lossy();

        // 根据文件扩展名选择不同的解压方式
        if path_str.ends_with(".tar.xz") {
            // 解压 .tar.xz
            let xz_decoder = xz2::read::XzDecoder::new(file);
            let mut archive = tar::Archive::new(xz_decoder);
            archive
                .unpack(extract_to)
                .map_err(|e| format!("解压 tar.xz 文件失败: {}", e))?;
        } else if path_str.ends_with(".tar.zst") {
            // 解压 .tar.zst
            let zst_decoder = zstd::stream::read::Decoder::new(file)
                .map_err(|e| format!("创建 zstd 解码器失败: {}", e))?;
            let mut archive = tar::Archive::new(zst_decoder);
            archive
                .unpack(extract_to)
                .map_err(|e| format!("解压 tar.zst 文件失败: {}", e))?;
        } else {
            // 解压 .tar.gz
            let gz_decoder = flate2::read::GzDecoder::new(file);
            let mut archive = tar::Archive::new(gz_decoder);
            archive
                .unpack(extract_to)
                .map_err(|e| format!("解压 tar.gz 文件失败: {}", e))?;
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

        let install_path = self.get_version_install_path(version);

        if let Some(plugins) = config.plugins.as_mut() {
            if let Some(php_plugin) = plugins.iter_mut().find(|p| p.language == "php") {
                php_plugin.execute_home = Some(install_path.to_string_lossy().to_string());
                php_plugin.run_command = Some("bin/php $filename".to_string());

                info!(
                    "已更新 PHP 插件配置: execute_home={}, run_command={}",
                    install_path.display(),
                    php_plugin.run_command.as_ref().unwrap()
                );
            }
        }

        update_app_config(config, app_handle)
            .await
            .map_err(|e| format!("保存配置失败: {}", e))?;

        info!("成功更新 PHP 配置");
        Ok(())
    }
}

#[async_trait]
impl EnvironmentProvider for PhpEnvironmentProvider {
    fn get_language(&self) -> &'static str {
        "php"
    }

    async fn fetch_available_versions(&self) -> Result<Vec<EnvironmentVersion>, String> {
        let releases = self.fetch_php_releases().await?;

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
            "php",
            version,
            0,
            100,
            DownloadStatus::Downloading,
        );

        let download_url = self.get_download_url(version).await?;

        // 根据下载 URL 确定文件扩展名
        let file_ext = if download_url.ends_with(".tar.xz") {
            "tar.xz"
        } else if download_url.ends_with(".tar.zst") {
            "tar.zst"
        } else {
            "tar.gz"
        };
        let temp_file = self
            .install_dir
            .join(format!("php-{}.{}", version, file_ext));

        let should_download = if is_cdn_enabled() {
            let metadata = fetch_metadata_from_cdn("php").await;
            if let Ok(meta) = metadata {
                if let Some(release) = meta.releases.iter().find(|r| r.version == version) {
                    info!("使用 CDN 下载 PHP {}", version);
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
            "php",
            version,
            0,
            100,
            DownloadStatus::Extracting,
        );

        let temp_extract_dir = self.install_dir.join(format!("temp_{}", version));
        self.extract_archive(&temp_file, &temp_extract_dir).await?;

        let install_path = self.get_version_install_path(version);

        // PHP 压缩包内可能直接包含文件或者有一层目录
        let extracted_dirs: Vec<_> = std::fs::read_dir(&temp_extract_dir)
            .map_err(|e| format!("读取临时目录失败: {}", e))?
            .filter_map(|e| e.ok())
            .collect();

        if extracted_dirs.len() == 1 && extracted_dirs[0].path().is_dir() {
            // 如果只有一个目录，移动该目录
            std::fs::rename(extracted_dirs[0].path(), &install_path)
                .map_err(|e| format!("移动安装目录失败: {}", e))?;
        } else {
            // 否则移动整个临时目录
            std::fs::rename(&temp_extract_dir, &install_path)
                .map_err(|e| format!("移动安装目录失败: {}", e))?;
        }

        std::fs::remove_dir_all(&temp_extract_dir).ok();
        std::fs::remove_file(&temp_file).ok();

        self.update_plugin_config(version, app_handle.clone())
            .await?;

        crate::env_manager::emit_download_progress(
            &app_handle,
            "php",
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
        info!("已切换到 PHP {}", version);
        Ok(())
    }

    async fn get_current_version(&self) -> Result<Option<String>, String> {
        use crate::config::get_app_config_internal;

        let config = get_app_config_internal().map_err(|e| format!("获取配置失败: {}", e))?;

        if let Some(plugins) = config.plugins {
            if let Some(php_plugin) = plugins.iter().find(|p| p.language == "php") {
                if let Some(ref execute_home) = php_plugin.execute_home {
                    let path = PathBuf::from(execute_home);

                    if let Ok(relative) = path.strip_prefix(&self.install_dir) {
                        if let Some(version_component) = relative.components().next() {
                            if let Some(version) = version_component.as_os_str().to_str() {
                                info!("当前 PHP 版本: {}", version);
                                return Ok(Some(version.to_string()));
                            }
                        }
                    }

                    if let Some(version) = path.file_name().and_then(|n| n.to_str()) {
                        info!("当前 PHP 版本: {}", version);
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

        info!("已卸载 PHP {}", version);
        Ok(())
    }
}
