use super::metadata::{Metadata, fetch_metadata_from_cdn, is_cdn_enabled, is_fallback_enabled};
use crate::env_manager::{
    DownloadStatus, EnvironmentProvider, EnvironmentVersion, download_with_fallback,
    emit_download_progress,
};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use tauri::AppHandle;

// Scala 版本信息（从 GitHub API 获取）
#[derive(Debug, Deserialize, Serialize, Clone)]
struct GithubRelease {
    tag_name: String,
    name: String,
    published_at: String,
    assets: Vec<GithubAsset>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
    size: u64,
}

// 缓存数据结构
#[derive(Debug, Deserialize, Serialize)]
struct CachedReleases {
    releases: Vec<GithubRelease>,
    cached_at: SystemTime,
}

pub struct ScalaEnvironmentProvider {
    install_dir: PathBuf,
    cache_file: PathBuf,
}

impl ScalaEnvironmentProvider {
    pub fn new() -> Self {
        let install_dir = Self::get_default_install_dir();
        let cache_file = install_dir.join("releases_cache.json");

        // 确保安装目录存在
        if let Err(e) = std::fs::create_dir_all(&install_dir) {
            error!("创建 Scala 安装目录失败: {}", e);
        }

        Self {
            install_dir,
            cache_file,
        }
    }

    fn get_default_install_dir() -> PathBuf {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home_dir.join(".codeforge").join("scala")
    }

    // 从缓存读取版本列表
    fn read_cache(&self) -> Option<Vec<GithubRelease>> {
        if !self.cache_file.exists() {
            return None;
        }

        match std::fs::read_to_string(&self.cache_file) {
            Ok(content) => {
                match serde_json::from_str::<CachedReleases>(&content) {
                    Ok(cached) => {
                        // 检查缓存是否过期（1小时）
                        if let Ok(elapsed) = SystemTime::now().duration_since(cached.cached_at) {
                            if elapsed < Duration::from_secs(3600) {
                                info!("使用缓存的 Scala 版本列表（缓存时间: {:?}）", elapsed);
                                return Some(cached.releases);
                            } else {
                                info!("缓存已过期（{:?}），将重新获取", elapsed);
                            }
                        }
                    }
                    Err(e) => {
                        warn!("解析缓存文件失败: {}", e);
                    }
                }
            }
            Err(e) => {
                warn!("读取缓存文件失败: {}", e);
            }
        }

        None
    }

    // 写入缓存
    fn write_cache(&self, releases: &[GithubRelease]) {
        let cached = CachedReleases {
            releases: releases.to_vec(),
            cached_at: SystemTime::now(),
        };

        match serde_json::to_string_pretty(&cached) {
            Ok(content) => {
                if let Err(e) = std::fs::write(&self.cache_file, content) {
                    warn!("写入缓存文件失败: {}", e);
                } else {
                    info!("已缓存 Scala 版本列表");
                }
            }
            Err(e) => {
                warn!("序列化缓存数据失败: {}", e);
            }
        }
    }

    // 检测操作系统和架构，返回合适的下载文件模式
    fn get_download_pattern() -> &'static str {
        if cfg!(target_os = "windows") {
            "x86_64-pc-win32.zip"
        } else if cfg!(target_os = "macos") {
            if cfg!(target_arch = "aarch64") {
                "aarch64-apple-darwin.tar.gz"
            } else {
                "x86_64-apple-darwin.tar.gz"
            }
        } else {
            "x86_64-pc-linux.tar.gz"
        }
    }

    // 从 GitHub Releases 获取版本列表（支持缓存和 Token）
    async fn fetch_github_releases(&self) -> Result<Vec<GithubRelease>, String> {
        // 先尝试从缓存读取
        if let Some(cached_releases) = self.read_cache() {
            return Ok(cached_releases);
        }

        let url = "https://api.github.com/repos/scala/scala3/releases?per_page=20";

        info!("从 GitHub API 获取 Scala 版本列表: {}", url);

        let client = reqwest::Client::builder()
            .user_agent("CodeForge")
            .build()
            .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;

        // 构建请求，如果有 GitHub Token 则添加认证头
        let mut request = client.get(url);

        // 尝试从环境变量获取 GitHub Token
        if let Ok(token) = std::env::var("GITHUB_TOKEN") {
            info!("使用 GITHUB_TOKEN 进行认证");
            request = request.header("Authorization", format!("token {}", token));
        }

        let response = request.send().await.map_err(|e| {
            // 如果请求失败，尝试使用缓存（即使过期）
            if let Some(_cached_releases) = self.read_cache_ignore_expiry() {
                warn!("GitHub API 请求失败，使用过期缓存: {}", e);
                return format!("GitHub API 请求失败，已使用缓存数据: {}", e);
            }
            format!("请求 GitHub API 失败: {}", e)
        })?;

        let status = response.status();

        // 处理 API 限流
        if status.as_u16() == 403 || status.as_u16() == 429 {
            let error_msg = if let Ok(body) = response.text().await {
                if body.contains("rate limit") {
                    warn!("GitHub API 限流，尝试使用缓存");
                    // 尝试使用缓存（即使过期）
                    if let Some(cached_releases) = self.read_cache_ignore_expiry() {
                        return Ok(cached_releases);
                    }
                    format!(
                        "GitHub API 限流已超出。请稍后再试，或设置 GITHUB_TOKEN 环境变量以增加限额。详情: {}",
                        body
                    )
                } else {
                    format!("GitHub API 返回错误 ({}): {}", status, body)
                }
            } else {
                format!("GitHub API 返回错误: {}", status)
            };
            return Err(error_msg);
        }

        if !status.is_success() {
            return Err(format!("GitHub API 返回错误: {}", status));
        }

        let releases: Vec<GithubRelease> = response
            .json()
            .await
            .map_err(|e| format!("解析 GitHub API 响应失败: {}", e))?;

        info!("成功获取 {} 个 Scala 版本", releases.len());

        // 缓存结果
        self.write_cache(&releases);

        Ok(releases)
    }

    // 读取缓存（忽略过期时间）- 用于 API 失败时的降级方案
    fn read_cache_ignore_expiry(&self) -> Option<Vec<GithubRelease>> {
        if !self.cache_file.exists() {
            return None;
        }

        match std::fs::read_to_string(&self.cache_file) {
            Ok(content) => match serde_json::from_str::<CachedReleases>(&content) {
                Ok(cached) => {
                    info!("使用缓存的 Scala 版本列表（忽略过期时间）");
                    Some(cached.releases)
                }
                Err(e) => {
                    warn!("解析缓存文件失败: {}", e);
                    None
                }
            },
            Err(e) => {
                warn!("读取缓存文件失败: {}", e);
                None
            }
        }
    }

    // 获取指定版本的安装路径
    fn get_version_install_path(&self, version: &str) -> PathBuf {
        self.install_dir.join(version)
    }

    // 检查版本是否已安装
    fn is_version_installed(&self, version: &str) -> bool {
        let install_path = self.get_version_install_path(version);
        if !install_path.exists() {
            return false;
        }

        // 检查是否有包含 bin 目录的子目录
        if let Ok(entries) = std::fs::read_dir(&install_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() && path.join("bin").exists() {
                    return true;
                }
            }
        }

        false
    }

    // 将 CDN metadata 转换为 EnvironmentVersion 列表
    fn parse_metadata_to_versions(
        &self,
        metadata: Metadata,
    ) -> Result<Vec<EnvironmentVersion>, String> {
        let mut versions = Vec::new();

        for release in metadata.releases {
            let version = release.version.clone();
            let is_installed = self.is_version_installed(&version);

            // 如果已安装，查找实际的包含 bin 目录的路径
            let install_path = if is_installed {
                let version_dir = self.get_version_install_path(&version);
                let mut actual_path = version_dir.clone();

                if let Ok(entries) = std::fs::read_dir(&version_dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_dir() && path.join("bin").exists() {
                            actual_path = path;
                            break;
                        }
                    }
                }

                Some(actual_path.to_string_lossy().to_string())
            } else {
                None
            };

            versions.push(EnvironmentVersion {
                version: version.clone(),
                download_url: release.download_url.clone(),
                fallback_url: Some(release.github_url.clone()),
                install_path,
                is_installed,
                size: Some(release.size),
                release_date: Some(release.published_at.clone()),
            });
        }

        Ok(versions)
    }

    // 下载文件并显示进度
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

        let response = download_with_fallback(&client, url, "scala", version).await?;

        if !response.status().is_success() {
            return Err(format!("下载失败: HTTP {}", response.status()));
        }

        let total_size = response.content_length().unwrap_or(0);
        info!("文件大小: {} bytes", total_size);

        emit_download_progress(
            &app_handle,
            "scala",
            version,
            0,
            total_size,
            DownloadStatus::Downloading,
        );

        // 创建目标文件
        let mut file = std::fs::File::create(dest).map_err(|e| format!("创建文件失败: {}", e))?;

        let mut downloaded: u64 = 0;
        let mut stream = response.bytes_stream();

        use futures_util::StreamExt;
        use std::io::Write;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| format!("下载数据失败: {}", e))?;
            file.write_all(&chunk)
                .map_err(|e| format!("写入文件失败: {}", e))?;

            downloaded += chunk.len() as u64;

            // 每下载 1MB 发送一次进度更新
            if downloaded % (1024 * 1024) == 0 || downloaded == total_size {
                emit_download_progress(
                    &app_handle,
                    "scala",
                    version,
                    downloaded,
                    total_size,
                    DownloadStatus::Downloading,
                );
            }
        }

        info!("下载完成: {}", dest.display());
        Ok(())
    }

    // 解压文件
    async fn extract_archive(
        &self,
        archive_path: &PathBuf,
        dest_dir: &PathBuf,
        app_handle: AppHandle,
        version: &str,
    ) -> Result<(), String> {
        info!(
            "开始解压: {} -> {}",
            archive_path.display(),
            dest_dir.display()
        );

        emit_download_progress(
            &app_handle,
            "scala",
            version,
            0,
            0,
            DownloadStatus::Extracting,
        );

        std::fs::create_dir_all(dest_dir).map_err(|e| format!("创建目录失败: {}", e))?;

        if archive_path.extension().and_then(|s| s.to_str()) == Some("zip") {
            // 解压 ZIP 文件
            self.extract_zip(archive_path, dest_dir)?;
        } else {
            // 解压 tar.gz 文件
            self.extract_tar_gz(archive_path, dest_dir)?;
        }

        info!("解压完成");
        Ok(())
    }

    fn extract_zip(&self, archive_path: &PathBuf, dest_dir: &Path) -> Result<(), String> {
        use zip::ZipArchive;

        let file =
            std::fs::File::open(archive_path).map_err(|e| format!("打开压缩文件失败: {}", e))?;

        let mut archive = ZipArchive::new(file).map_err(|e| format!("读取 ZIP 文件失败: {}", e))?;

        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .map_err(|e| format!("读取 ZIP 条目失败: {}", e))?;

            let outpath = match file.enclosed_name() {
                Some(path) => dest_dir.join(path),
                None => continue,
            };

            if file.name().ends_with('/') {
                std::fs::create_dir_all(&outpath).map_err(|e| format!("创建目录失败: {}", e))?;
            } else {
                if let Some(p) = outpath.parent() {
                    std::fs::create_dir_all(p).map_err(|e| format!("创建目录失败: {}", e))?;
                }
                let mut outfile =
                    std::fs::File::create(&outpath).map_err(|e| format!("创建文件失败: {}", e))?;
                std::io::copy(&mut file, &mut outfile)
                    .map_err(|e| format!("解压文件失败: {}", e))?;
            }

            // Unix 权限
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = file.unix_mode() {
                    std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode)).ok();
                }
            }
        }

        Ok(())
    }

    fn extract_tar_gz(&self, archive_path: &PathBuf, dest_dir: &PathBuf) -> Result<(), String> {
        use flate2::read::GzDecoder;
        use tar::Archive;

        let file =
            std::fs::File::open(archive_path).map_err(|e| format!("打开压缩文件失败: {}", e))?;

        let gz = GzDecoder::new(file);
        let mut archive = Archive::new(gz);

        archive
            .unpack(dest_dir)
            .map_err(|e| format!("解压 tar.gz 失败: {}", e))?;

        Ok(())
    }

    // 更新配置以使用新版本
    async fn update_plugin_config(
        &self,
        version: &str,
        install_path: &str,
        app_handle: &AppHandle,
    ) -> Result<(), String> {
        use crate::config::{get_app_config_internal, update_app_config};

        info!(
            "更新 Scala 插件配置: 版本={}, 路径={}",
            version, install_path
        );

        let mut config = get_app_config_internal().map_err(|e| format!("获取配置失败: {}", e))?;

        if let Some(ref mut plugins) = config.plugins {
            if let Some(scala_plugin) = plugins.iter_mut().find(|p| p.language == "scala") {
                // execute_home 应该是包含 bin 目录的父目录
                scala_plugin.execute_home = Some(install_path.to_string());

                // 根据操作系统设置 run_command
                let run_cmd = if cfg!(target_os = "windows") {
                    "bin/scala.bat $filename"
                } else {
                    "bin/scala $filename"
                };
                scala_plugin.run_command = Some(String::from(run_cmd));

                info!(
                    "已更新 Scala 插件配置: execute_home={}, run_command={}",
                    install_path, run_cmd
                );
            }
        }

        update_app_config(config, app_handle.clone())
            .await
            .map_err(|e| format!("保存配置失败: {}", e))?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl EnvironmentProvider for ScalaEnvironmentProvider {
    fn get_language(&self) -> &'static str {
        "scala"
    }

    async fn fetch_available_versions(&self) -> Result<Vec<EnvironmentVersion>, String> {
        // 检查 CDN 是否启用
        if is_cdn_enabled() {
            match fetch_metadata_from_cdn("scala").await {
                Ok(metadata) => {
                    info!("使用 CDN metadata 获取版本列表");
                    return self.parse_metadata_to_versions(metadata);
                }
                Err(e) => {
                    warn!("CDN metadata 获取失败: {}", e);

                    // 检查是否启用 fallback
                    if !is_fallback_enabled() {
                        return Err(format!("CDN metadata 获取失败，未启用自动回退: {}", e));
                    }

                    info!("fallback 已启用，回退到 GitHub API");
                }
            }
        } else {
            info!("CDN 未启用，使用 GitHub API");
        }

        let releases = self.fetch_github_releases().await?;
        let pattern = Self::get_download_pattern();

        let mut versions = Vec::new();

        for release in releases {
            // 查找匹配当前平台的资源
            if let Some(asset) = release.assets.iter().find(|a| a.name.contains(pattern)) {
                let version = release.tag_name.trim_start_matches('v').to_string();
                let is_installed = self.is_version_installed(&version);

                // 如果已安装，查找实际的包含 bin 目录的路径
                let install_path = if is_installed {
                    let version_dir = self.get_version_install_path(&version);
                    let mut actual_path = version_dir.clone();

                    if let Ok(entries) = std::fs::read_dir(&version_dir) {
                        for entry in entries.flatten() {
                            let path = entry.path();
                            if path.is_dir() && path.join("bin").exists() {
                                actual_path = path;
                                break;
                            }
                        }
                    }

                    Some(actual_path.to_string_lossy().to_string())
                } else {
                    None
                };

                versions.push(EnvironmentVersion {
                    version: version.clone(),
                    download_url: asset.browser_download_url.clone(),
                    fallback_url: None,
                    install_path,
                    is_installed,
                    size: Some(asset.size),
                    release_date: Some(release.published_at.clone()),
                });
            }
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
                    // 查找实际的包含 bin 目录的路径
                    let mut actual_install_path = path.clone();
                    if let Ok(sub_entries) = std::fs::read_dir(&path) {
                        for sub_entry in sub_entries.flatten() {
                            let sub_path = sub_entry.path();
                            if sub_path.is_dir() && sub_path.join("bin").exists() {
                                actual_install_path = sub_path;
                                break;
                            }
                        }
                    }

                    installed.push(EnvironmentVersion {
                        version: version.clone(),
                        download_url: String::new(),
                        fallback_url: None,
                        install_path: Some(actual_install_path.to_string_lossy().to_string()),
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
        info!("开始下载并安装 Scala {}", version);

        // 检查是否已安装
        if self.is_version_installed(version) {
            return Err(format!("Scala {} 已经安装", version));
        }

        emit_download_progress(
            &app_handle,
            "scala",
            version,
            0,
            0,
            DownloadStatus::Downloading,
        );

        // 获取可用版本
        let available_versions = self.fetch_available_versions().await?;
        let version_info = available_versions
            .iter()
            .find(|v| v.version == version)
            .ok_or_else(|| format!("未找到版本: {}", version))?;

        // 下载文件
        let download_url = &version_info.download_url;
        let file_name = download_url
            .split('/')
            .last()
            .ok_or_else(|| "无效的下载 URL".to_string())?;
        let temp_file = std::env::temp_dir().join(file_name);

        self.download_file(download_url, &temp_file, app_handle.clone(), version)
            .await?;

        // 解压到安装目录
        let install_path = self.get_version_install_path(version);
        self.extract_archive(&temp_file, &install_path, app_handle.clone(), version)
            .await?;

        // 清理临时文件
        std::fs::remove_file(&temp_file).ok();

        emit_download_progress(
            &app_handle,
            "scala",
            version,
            0,
            0,
            DownloadStatus::Installing,
        );

        // 查找解压后的实际目录（可能包含版本号前缀）
        let mut actual_install_path = install_path.clone();
        if let Ok(entries) = std::fs::read_dir(&install_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() && path.join("bin").exists() {
                    actual_install_path = path;
                    break;
                }
            }
        }

        // 更新插件配置
        self.update_plugin_config(version, &actual_install_path.to_string_lossy(), &app_handle)
            .await?;

        emit_download_progress(
            &app_handle,
            "scala",
            version,
            0,
            0,
            DownloadStatus::Completed,
        );

        info!("Scala {} 安装成功", version);
        Ok(actual_install_path.to_string_lossy().to_string())
    }

    async fn switch_version(&self, version: &str, app_handle: AppHandle) -> Result<(), String> {
        info!("切换 Scala 版本到 {}", version);

        if !self.is_version_installed(version) {
            return Err(format!("版本 {} 未安装", version));
        }

        let install_path = self.get_version_install_path(version);

        // 查找实际的安装目录
        let mut actual_install_path = install_path.clone();
        if let Ok(entries) = std::fs::read_dir(&install_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() && path.join("bin").exists() {
                    actual_install_path = path;
                    break;
                }
            }
        }

        self.update_plugin_config(version, &actual_install_path.to_string_lossy(), &app_handle)
            .await?;

        info!("成功切换到 Scala {}", version);
        Ok(())
    }

    async fn get_current_version(&self) -> Result<Option<String>, String> {
        use crate::config::get_app_config_internal;

        let config = get_app_config_internal().map_err(|e| format!("获取配置失败: {}", e))?;

        if let Some(plugins) = config.plugins {
            if let Some(scala_plugin) = plugins.iter().find(|p| p.language == "scala") {
                if let Some(ref execute_home) = scala_plugin.execute_home {
                    // 从路径中提取版本号
                    // execute_home 格式: ~/.codeforge/scala/3.8.0-RC4/scala3-3.8.0-RC4-aarch64-apple-darwin
                    // 我们需要提取 3.8.0-RC4
                    let path = PathBuf::from(execute_home);

                    // 检查路径是否在安装目录下
                    if let Ok(relative) = path.strip_prefix(&self.install_dir) {
                        // 获取第一个路径组件（版本号）
                        if let Some(version_component) = relative.components().next() {
                            if let Some(version) = version_component.as_os_str().to_str() {
                                info!("当前 Scala 版本: {}", version);
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

        info!("已卸载 Scala 版本 {}", version);
        Ok(())
    }
}
