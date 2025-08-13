use log::{error, info};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use tauri::{AppHandle, Emitter, command};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub version: String,
    pub date: String,
    pub notes: String,
    pub size: u64,
    pub url: String,
}

#[derive(Debug, Serialize)]
struct UpdateProgress {
    progress: f64,
    speed: u64,
    status: String,
}

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    published_at: String,
    body: String,
    assets: Vec<GitHubAsset>,
}

#[derive(Debug, Deserialize)]
struct GitHubAsset {
    name: String,
    size: u64,
    browser_download_url: String,
}

// 检查更新命令
#[command]
pub async fn check_for_updates() -> Result<Option<UpdateInfo>, String> {
    let repo_owner = "devlive-community";
    let repo_name = "codeforge";

    info!("检查更新 -> 检查更新中...");
    let current_version = env!("CARGO_PKG_VERSION");

    match get_latest_release(repo_owner, repo_name).await {
        Ok(release) => {
            let latest_version = release.tag_name.trim_start_matches('v');

            if version_is_newer(latest_version, current_version) {
                if let Some(asset) = get_platform_asset(&release.assets) {
                    Ok(Some(UpdateInfo {
                        version: latest_version.to_string(),
                        date: release.published_at,
                        notes: release.body,
                        size: asset.size,
                        url: asset.browser_download_url.clone(),
                    }))
                } else {
                    error!("检查更新 -> 未找到适合当前平台的更新文件");
                    Err("未找到适合当前平台的更新文件".to_string())
                }
            } else {
                info!("检查更新 -> 没有更新");
                Ok(None)
            }
        }
        Err(e) => Err(format!("检查更新失败: {}", e)),
    }
}

// 开始更新命令
#[command]
pub async fn start_update(update_info: UpdateInfo, app_handle: AppHandle) -> Result<(), String> {
    let app_handle_clone = app_handle.clone();
    info!("检查更新 -> 开始更新: {}", update_info.version);

    tokio::spawn(async move {
        let app_handle_for_error = app_handle_clone.clone();
        if let Err(e) = do_update(update_info, app_handle_clone).await {
            info!("检查更新 -> 更新失败: {}", e);
            let error_data = serde_json::json!({ "error": e.to_string() });
            let _ = app_handle_for_error.emit("update-error", &error_data);
        }
    });

    Ok(())
}

// 获取最新版本
async fn get_latest_release(
    owner: &str,
    repo: &str,
) -> Result<GitHubRelease, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        owner, repo
    );
    info!("检查更新 -> 获取最新版本: {}", url);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "CodeForge-Updater")
        .send()
        .await?;

    if !response.status().is_success() {
        error!("检查更新 -> 获取最新版本失败: {}", response.status());
        return Err(format!("GitHub API 错误: {}", response.status()).into());
    }

    let release: GitHubRelease = response.json().await?;
    info!("检查更新 -> 最新版本: {}", release.tag_name);
    Ok(release)
}

// 查找平台对应的文件
fn get_platform_asset(assets: &[GitHubAsset]) -> Option<&GitHubAsset> {
    let os = std::env::consts::OS;
    info!("检查更新 -> 当前平台: {}", os);

    for asset in assets {
        let name = asset.name.to_lowercase();

        let matches = match os {
            "windows" => name.contains("windows") || name.contains("win") || name.ends_with(".exe"),
            "macos" => name.contains("macos") || name.contains("darwin") || name.ends_with(".dmg"),
            "linux" => name.contains("linux") || name.ends_with(".appimage"),
            _ => false,
        };

        if matches {
            info!("检查更新 -> 匹配更新文件: {}", asset.name);
            return Some(asset);
        }
    }

    // 如果没找到匹配的，返回第一个
    assets.first()
}

// 版本比较
fn version_is_newer(new_version: &str, current_version: &str) -> bool {
    info!(
        "检查更新 -> 当前版本: {} 最新版本: {}",
        current_version, new_version
    );
    let new_parts: Vec<u32> = new_version
        .split('.')
        .filter_map(|s| s.parse().ok())
        .collect();
    let current_parts: Vec<u32> = current_version
        .split('.')
        .filter_map(|s| s.parse().ok())
        .collect();

    for i in 0..std::cmp::max(new_parts.len(), current_parts.len()) {
        let new_part = new_parts.get(i).unwrap_or(&0);
        let current_part = current_parts.get(i).unwrap_or(&0);

        match new_part.cmp(current_part) {
            std::cmp::Ordering::Greater => return true,
            std::cmp::Ordering::Less => return false,
            std::cmp::Ordering::Equal => continue,
        }
    }

    false
}

// 执行更新
async fn do_update(
    update_info: UpdateInfo,
    app_handle: AppHandle,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 创建临时目录
    info!("检查更新 -> 创建临时目录");
    let temp_dir = std::env::temp_dir().join("codeforge_update");
    std::fs::create_dir_all(&temp_dir)?;

    let file_name = format!("update_{}.exe", update_info.version);
    let download_path = temp_dir.join(&file_name);

    info!("检查更新 -> 下载更新文件: {}", update_info.url);
    // 下载文件
    let client = reqwest::Client::new();
    let response = client.get(&update_info.url).send().await?;

    if !response.status().is_success() {
        error!("检查更新 -> 下载失败: {}", response.status());
        return Err(format!("下载失败: {}", response.status()).into());
    }

    let total_size = response.content_length().unwrap_or(update_info.size);
    let mut downloaded = 0u64;
    let mut file = std::fs::File::create(&download_path)?;
    let start_time = SystemTime::now();

    use std::io::Write;
    let mut stream = response.bytes_stream();
    use futures_util::StreamExt;

    info!("检查更新 -> 开始下载");
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk)?;
        downloaded += chunk.len() as u64;

        // 计算进度
        let progress = (downloaded as f64 / total_size as f64) * 70.0;
        let elapsed = start_time.elapsed().unwrap_or_default().as_secs_f64();
        let speed = if elapsed > 0.0 {
            (downloaded as f64 / elapsed) as u64
        } else {
            0
        };

        info!("检查更新 -> 下载进度: {:.2}%", progress);
        // 发送进度
        let progress_data = UpdateProgress {
            progress,
            speed,
            status: "downloading".to_string(),
        };
        let _ = app_handle.emit("update-progress", &progress_data);

        // 避免过于频繁的更新
        if downloaded % (512 * 1024) == 0 {
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
    }

    file.flush()?;
    drop(file);

    info!("检查更新 -> 安装更新文件: {}", update_info.version);
    for i in 71..=100 {
        let progress_data = UpdateProgress {
            progress: i as f64,
            speed: 0,
            status: "installing".to_string(),
        };

        info!("检查更新 -> 安装进度: {:.2}%", progress_data.progress);
        let _ = app_handle.emit("update-progress", &progress_data);
        tokio::time::sleep(tokio::time::Duration::from_millis(30)).await;
    }

    // 执行安装
    do_install(&download_path).await?;

    // 完成
    let complete_data = serde_json::json!({});
    let _ = app_handle.emit("update-complete", &complete_data);

    Ok(())
}

// 安装更新
async fn do_install(
    update_path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    #[cfg(target_os = "windows")]
    {
        let current_exe = std::env::current_exe()?;
        let backup_path = current_exe.with_extension("exe.backup");

        // 备份当前文件
        std::fs::copy(&current_exe, &backup_path)?;

        // 创建更新脚本
        let script_path = std::env::temp_dir().join("update.bat");
        let script_content = format!(
            r#"@echo off
timeout /t 2 /nobreak >nul
copy /Y "{}" "{}"
start "" "{}"
del "%~f0"
"#,
            update_path.display(),
            current_exe.display(),
            current_exe.display()
        );

        std::fs::write(&script_path, script_content)?;

        // 启动脚本
        std::process::Command::new("cmd")
            .args(&["/C", "start", "", "/min", script_path.to_str().unwrap()])
            .spawn()?;

        // 等待一秒后退出
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        std::process::exit(0);
    }

    #[cfg(not(target_os = "windows"))]
    {
        let current_exe = std::env::current_exe()?;
        std::fs::copy(update_path, &current_exe)?;
    }

    Ok(())
}
