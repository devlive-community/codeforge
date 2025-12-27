use super::metadata::{Metadata, fetch_metadata_from_cdn, is_cdn_enabled, is_fallback_enabled};
use crate::env_manager::{
    DownloadStatus, EnvironmentProvider, EnvironmentVersion, emit_download_progress,
};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use tauri::AppHandle;

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

#[derive(Debug, Deserialize, Serialize)]
struct CachedReleases {
    releases: Vec<GithubRelease>,
    cached_at: SystemTime,
}

pub struct ClojureEnvironmentProvider {
    install_dir: PathBuf,
    cache_file: PathBuf,
}

impl ClojureEnvironmentProvider {
    pub fn new() -> Self {
        let install_dir = Self::get_default_install_dir();
        let cache_file = install_dir.join("releases_cache.json");

        if let Err(e) = std::fs::create_dir_all(&install_dir) {
            error!("创建 Clojure 安装目录失败: {}", e);
        }

        Self {
            install_dir,
            cache_file,
        }
    }

    fn get_default_install_dir() -> PathBuf {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home_dir.join(".codeforge").join("plugins").join("clojure")
    }

    fn read_cache(&self) -> Option<Vec<GithubRelease>> {
        if !self.cache_file.exists() {
            return None;
        }

        match std::fs::read_to_string(&self.cache_file) {
            Ok(content) => match serde_json::from_str::<CachedReleases>(&content) {
                Ok(cached) => {
                    if let Ok(elapsed) = SystemTime::now().duration_since(cached.cached_at) {
                        if elapsed < Duration::from_secs(3600) {
                            info!("使用缓存的 Clojure 版本列表（缓存时间: {:?}）", elapsed);
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
                    info!("已缓存 Clojure 版本列表");
                }
            }
            Err(e) => {
                warn!("序列化缓存数据失败: {}", e);
            }
        }
    }

    fn get_download_pattern() -> &'static str {
        // Clojure 工具包是跨平台的，文件名格式为 clojure-tools-{version}.tar.gz
        "clojure-tools-"
    }

    // 获取当前系统平台
    fn get_current_platform() -> &'static str {
        if cfg!(target_os = "macos") {
            "macos"
        } else if cfg!(target_os = "linux") {
            "linux"
        } else if cfg!(target_os = "windows") {
            "windows"
        } else {
            "unknown"
        }
    }

    // 将 metadata 转换为 EnvironmentVersion 列表
    fn parse_metadata_to_versions(
        &self,
        metadata: Metadata,
    ) -> Result<Vec<EnvironmentVersion>, String> {
        let current_platform = Self::get_current_platform();
        let mut versions = Vec::new();

        for release in metadata.releases {
            // 检查是否支持当前平台
            if !release
                .supported_platforms
                .contains(&current_platform.to_string())
            {
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
                download_url: release.download_url.clone(), // 直接使用 metadata 中的 CDN 下载地址
                fallback_url: Some(release.github_url.clone()), // 保存 GitHub URL 作为备用
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

    async fn fetch_github_releases(&self) -> Result<Vec<GithubRelease>, String> {
        if let Some(cached_releases) = self.read_cache() {
            return Ok(cached_releases);
        }

        let url = "https://api.github.com/repos/clojure/brew-install/releases?per_page=20";

        info!("从 GitHub API 获取 Clojure 版本列表: {}", url);

        let client = reqwest::Client::builder()
            .user_agent("CodeForge")
            .build()
            .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;

        let mut request = client.get(url);

        if let Ok(token) = std::env::var("GITHUB_TOKEN") {
            info!("使用 GITHUB_TOKEN 进行认证");
            request = request.header("Authorization", format!("token {}", token));
        }

        let response = request.send().await.map_err(|e| {
            if let Some(_cached_releases) = self.read_cache_ignore_expiry() {
                warn!("GitHub API 请求失败，使用过期缓存: {}", e);
                return format!("GitHub API 请求失败，已使用缓存数据: {}", e);
            }
            format!("请求 GitHub API 失败: {}", e)
        })?;

        let status = response.status();

        if status.as_u16() == 403 || status.as_u16() == 429 {
            let error_msg = if let Ok(body) = response.text().await {
                if body.contains("rate limit") {
                    warn!("GitHub API 限流，尝试使用缓存");
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

        info!("成功获取 {} 个 Clojure 版本", releases.len());

        self.write_cache(&releases);

        Ok(releases)
    }

    fn read_cache_ignore_expiry(&self) -> Option<Vec<GithubRelease>> {
        if !self.cache_file.exists() {
            return None;
        }

        match std::fs::read_to_string(&self.cache_file) {
            Ok(content) => match serde_json::from_str::<CachedReleases>(&content) {
                Ok(cached) => {
                    info!("使用缓存的 Clojure 版本列表（忽略过期时间）");
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

    fn get_version_install_path(&self, version: &str) -> PathBuf {
        self.install_dir.join(version)
    }

    fn is_version_installed(&self, version: &str) -> bool {
        let install_path = self.get_version_install_path(version);
        if !install_path.exists() {
            return false;
        }

        // Clojure 安装后的 bin 目录结构
        let bin_path = install_path.join("bin");
        if !bin_path.exists() {
            return false;
        }

        // 检查 clojure 和 clj 脚本是否存在
        let clojure_bin = bin_path.join("clojure");
        let clj_bin = bin_path.join("clj");

        clojure_bin.exists() && clj_bin.exists()
    }

    async fn download_file(
        &self,
        url: &str,
        fallback_url: Option<&String>,
        dest: &PathBuf,
        app_handle: AppHandle,
        version: &str,
    ) -> Result<(), String> {
        use crate::config::get_app_config_internal;

        info!("开始下载: {} -> {}", url, dest.display());

        let client = reqwest::Client::builder()
            .user_agent("CodeForge")
            .build()
            .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;

        // 尝试从主 URL 下载
        let response = match client.get(url).send().await {
            Ok(resp) if resp.status().is_success() => {
                info!("下载成功");
                resp
            }
            Ok(resp) => {
                let status = resp.status();
                warn!("下载失败: HTTP {}", status);

                // 检查是否启用 fallback 且有 fallback URL
                if let Some(fb_url) = fallback_url {
                    let fallback_enabled = get_app_config_internal()
                        .ok()
                        .and_then(|config| config.environment_mirror)
                        .and_then(|mirror| mirror.fallback_enabled)
                        .unwrap_or(false);

                    if fallback_enabled {
                        info!("尝试使用备用 URL: {}", fb_url);
                        client
                            .get(fb_url)
                            .send()
                            .await
                            .map_err(|e| format!("备用 URL 下载失败: {}", e))?
                    } else {
                        return Err(format!("下载失败 (HTTP {}), 未启用自动回退", status));
                    }
                } else {
                    return Err(format!("下载失败: HTTP {}", status));
                }
            }
            Err(e) => {
                warn!("下载失败: {}", e);

                // 检查是否启用 fallback 且有 fallback URL
                if let Some(fb_url) = fallback_url {
                    let fallback_enabled = get_app_config_internal()
                        .ok()
                        .and_then(|config| config.environment_mirror)
                        .and_then(|mirror| mirror.fallback_enabled)
                        .unwrap_or(false);

                    if fallback_enabled {
                        info!("尝试使用备用 URL: {}", fb_url);
                        client
                            .get(fb_url)
                            .send()
                            .await
                            .map_err(|e| format!("备用 URL 下载失败: {}", e))?
                    } else {
                        return Err(format!("下载失败 ({}), 未启用自动回退", e));
                    }
                } else {
                    return Err(format!("下载失败: {}", e));
                }
            }
        };

        if !response.status().is_success() {
            return Err(format!("下载失败: HTTP {}", response.status()));
        }

        let total_size = response.content_length().unwrap_or(0);
        info!("文件大小: {} bytes", total_size);

        emit_download_progress(
            &app_handle,
            "clojure",
            version,
            0,
            total_size,
            DownloadStatus::Downloading,
        );

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

            if downloaded % (1024 * 1024) == 0 || downloaded == total_size {
                emit_download_progress(
                    &app_handle,
                    "clojure",
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
            "clojure",
            version,
            0,
            0,
            DownloadStatus::Extracting,
        );

        std::fs::create_dir_all(dest_dir).map_err(|e| format!("创建目录失败: {}", e))?;

        // Clojure 在所有平台都使用 tar.gz 格式
        self.extract_tar_gz(archive_path, dest_dir)?;

        info!("解压完成");
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

    // 组织 Clojure 安装目录结构
    fn organize_installation(&self, temp_dir: &Path, install_path: &Path) -> Result<(), String> {
        std::fs::create_dir_all(install_path).map_err(|e| format!("创建安装目录失败: {}", e))?;

        let tools_dir = temp_dir.join("clojure-tools");
        if !tools_dir.exists() {
            return Err("解压后未找到 clojure-tools 目录".to_string());
        }

        // 创建 bin 和 libexec 目录
        let bin_dir = install_path.join("bin");
        let libexec_dir = install_path.join("libexec");
        std::fs::create_dir_all(&bin_dir).map_err(|e| format!("创建 bin 目录失败: {}", e))?;
        std::fs::create_dir_all(&libexec_dir)
            .map_err(|e| format!("创建 libexec 目录失败: {}", e))?;

        // 移动脚本文件到 bin 目录，并替换 PREFIX 占位符
        for script in &["clojure", "clj"] {
            let src = tools_dir.join(script);
            let dst = bin_dir.join(script);
            if src.exists() {
                // 读取脚本内容
                let content = std::fs::read_to_string(&src)
                    .map_err(|e| format!("读取 {} 失败: {}", script, e))?;

                // 替换 PREFIX 占位符为实际的安装路径
                let install_dir_str = install_path.to_string_lossy();
                let modified_content = content.replace(
                    "install_dir=PREFIX",
                    &format!("install_dir={}", install_dir_str),
                );

                // 写入修改后的内容
                std::fs::write(&dst, modified_content)
                    .map_err(|e| format!("写入 {} 失败: {}", script, e))?;

                // 设置可执行权限
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    std::fs::set_permissions(&dst, std::fs::Permissions::from_mode(0o755))
                        .map_err(|e| format!("设置 {} 权限失败: {}", script, e))?;
                }
            }
        }

        // 移动 jar 文件和配置文件到 libexec 目录
        let entries = std::fs::read_dir(&tools_dir)
            .map_err(|e| format!("读取 clojure-tools 目录失败: {}", e))?;

        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();

            // 跳过脚本文件和 install.sh
            if file_name == "clojure"
                || file_name == "clj"
                || file_name == "install.sh"
                || file_name.ends_with(".1")
            {
                continue;
            }

            // 复制其他文件到 libexec
            if path.is_file() {
                let dst = libexec_dir.join(&file_name);
                std::fs::copy(&path, &dst)
                    .map_err(|e| format!("复制 {} 失败: {}", file_name, e))?;
            }
        }

        info!("Clojure 安装目录组织完成: {}", install_path.display());
        Ok(())
    }

    async fn update_plugin_config(
        &self,
        version: &str,
        install_path: &str,
        app_handle: &AppHandle,
    ) -> Result<(), String> {
        use crate::config::{get_app_config_internal, update_app_config};

        info!(
            "更新 Clojure 插件配置: 版本={}, 路径={}",
            version, install_path
        );

        let mut config = get_app_config_internal().map_err(|e| format!("获取配置失败: {}", e))?;

        if let Some(ref mut plugins) = config.plugins {
            if let Some(clojure_plugin) = plugins.iter_mut().find(|p| p.language == "clojure") {
                clojure_plugin.execute_home = Some(install_path.to_string());
                clojure_plugin.run_command = Some(String::from("bin/clojure $filename"));

                info!(
                    "已更新 Clojure 插件配置: execute_home={}, run_command=bin/clojure $filename",
                    install_path
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
impl EnvironmentProvider for ClojureEnvironmentProvider {
    fn get_language(&self) -> &'static str {
        "clojure"
    }

    async fn fetch_available_versions(&self) -> Result<Vec<EnvironmentVersion>, String> {
        // 检查 CDN 是否启用
        if is_cdn_enabled() {
            match fetch_metadata_from_cdn("clojure").await {
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
            if let Some(asset) = release.assets.iter().find(|a| a.name.contains(pattern)) {
                let version = release.tag_name.trim_start_matches('v').to_string();
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

                versions.push(EnvironmentVersion {
                    version: version.clone(),
                    download_url: asset.browser_download_url.clone(),
                    fallback_url: None, // GitHub API 获取的版本没有 CDN URL，所以不需要 fallback
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
        info!("开始下载并安装 Clojure {}", version);

        if self.is_version_installed(version) {
            return Err(format!("Clojure {} 已经安装", version));
        }

        emit_download_progress(
            &app_handle,
            "clojure",
            version,
            0,
            0,
            DownloadStatus::Downloading,
        );

        let available_versions = self.fetch_available_versions().await?;
        let version_info = available_versions
            .iter()
            .find(|v| v.version == version)
            .ok_or_else(|| format!("未找到版本: {}", version))?;

        let download_url = &version_info.download_url;
        let fallback_url = version_info.fallback_url.as_ref();
        let file_name = download_url
            .split('/')
            .last()
            .ok_or_else(|| "无效的下载 URL".to_string())?;
        let temp_file = std::env::temp_dir().join(file_name);

        self.download_file(
            download_url,
            fallback_url,
            &temp_file,
            app_handle.clone(),
            version,
        )
        .await?;

        let install_path = self.get_version_install_path(version);
        let temp_extract_dir = std::env::temp_dir().join(format!("clojure-tools-{}", version));

        self.extract_archive(&temp_file, &temp_extract_dir, app_handle.clone(), version)
            .await?;

        std::fs::remove_file(&temp_file).ok();

        emit_download_progress(
            &app_handle,
            "clojure",
            version,
            0,
            0,
            DownloadStatus::Installing,
        );

        // 组织安装目录结构
        self.organize_installation(&temp_extract_dir, &install_path)?;

        // 清理临时解压目录
        std::fs::remove_dir_all(&temp_extract_dir).ok();

        self.update_plugin_config(version, &install_path.to_string_lossy(), &app_handle)
            .await?;

        emit_download_progress(
            &app_handle,
            "clojure",
            version,
            0,
            0,
            DownloadStatus::Completed,
        );

        info!("Clojure {} 安装成功", version);
        Ok(install_path.to_string_lossy().to_string())
    }

    async fn switch_version(&self, version: &str, app_handle: AppHandle) -> Result<(), String> {
        info!("切换 Clojure 版本到 {}", version);

        if !self.is_version_installed(version) {
            return Err(format!("版本 {} 未安装", version));
        }

        let install_path = self.get_version_install_path(version);

        self.update_plugin_config(version, &install_path.to_string_lossy(), &app_handle)
            .await?;

        info!("成功切换到 Clojure {}", version);
        Ok(())
    }

    async fn get_current_version(&self) -> Result<Option<String>, String> {
        use crate::config::get_app_config_internal;

        let config = get_app_config_internal().map_err(|e| format!("获取配置失败: {}", e))?;

        if let Some(plugins) = config.plugins {
            if let Some(clojure_plugin) = plugins.iter().find(|p| p.language == "clojure") {
                if let Some(ref execute_home) = clojure_plugin.execute_home {
                    let path = PathBuf::from(execute_home);

                    if let Ok(relative) = path.strip_prefix(&self.install_dir) {
                        if let Some(version_component) = relative.components().next() {
                            if let Some(version) = version_component.as_os_str().to_str() {
                                info!("当前 Clojure 版本: {}", version);
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

        info!("已卸载 Clojure 版本 {}", version);
        Ok(())
    }
}
