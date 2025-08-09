use log::{error, info, warn};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, command};

// 全局日志目录状态
static LOG_DIRECTORY: Mutex<Option<PathBuf>> = Mutex::new(None);

// 获取日志目录
#[command]
pub async fn get_log_directory(app: AppHandle) -> Result<String, String> {
    let log_dir = {
        let guard = LOG_DIRECTORY.lock().unwrap();
        guard.clone()
    };

    let dir = match log_dir {
        Some(dir) => dir,
        None => crate::logger::get_log_directory(&app),
    };

    Ok(dir.to_string_lossy().to_string())
}

// 设置日志目录
#[command]
pub async fn set_log_directory(app: AppHandle, path: String) -> Result<(), String> {
    let new_path = PathBuf::from(&path);
    info!("设置日志 -> 用户设置日志目录为: {}", path);

    // 验证目录是否存在，如果不存在则创建
    if !new_path.exists() {
        std::fs::create_dir_all(&new_path).map_err(|e| format!("无法创建目录: {}", e))?;
    }

    // 验证是否为目录
    if !new_path.is_dir() {
        error!("设置日志 -> 指定的路径不是目录");
        return Err("指定的路径不是目录".to_string());
    }

    // 更新全局日志目录
    {
        info!("设置日志 -> 更新全局日志目录");
        let mut guard = LOG_DIRECTORY.lock().unwrap();
        *guard = Some(new_path);
    }

    // 保存到配置文件
    if let Ok(mut guard) = crate::config::get_config_manager() {
        if let Some(config_manager) = guard.as_mut() {
            if let Err(e) = config_manager.set_log_directory(Some(path.clone())) {
                warn!("设置日志 -> 保存配置失败: {}", e);
            } else {
                info!("设置日志 -> 配置已保存到文件");
            }
        }
    } else {
        warn!("设置日志 -> 无法获取配置管理器");
    }

    // 可选：重新初始化日志系统以立即使用新目录
    // 注意：这会导致当前日志文件切换，可能会丢失一些日志
    if let Err(e) = crate::logger::reinit_logger(&app) {
        warn!("重新初始化日志系统失败: {}", e);
    }

    info!("设置日志 -> 日志目录已设置为: {}", path);
    Ok(())
}

// 重置日志目录
#[command]
pub async fn reset_log_directory(_app: AppHandle) -> Result<(), String> {
    {
        let mut guard = LOG_DIRECTORY.lock().unwrap();
        *guard = None;
    }

    // 从配置文件中移除自定义日志目录
    if let Ok(mut guard) = crate::config::get_config_manager() {
        if let Some(config_manager) = guard.as_mut() {
            if let Err(e) = config_manager.set_log_directory(None) {
                warn!("重置日志 -> 保存配置失败: {}", e);
            } else {
                info!("重置日志 -> 配置已保存到文件");
            }
        }
    } else {
        warn!("重置日志 -> 无法获取配置管理器");
    }

    info!("重置日志 -> 重置日志目录为默认目录");
    Ok(())
}

// 获取日志文件列表
#[command]
pub async fn get_log_files(app: AppHandle) -> Result<Vec<String>, String> {
    let log_dir = {
        let guard = LOG_DIRECTORY.lock().unwrap();
        match &*guard {
            Some(dir) => dir.clone(),
            None => crate::logger::get_log_directory(&app),
        }
    };
    info!("获取日志 -> 日志目录为: {}", log_dir.display());

    let mut log_files = Vec::new();

    if let Ok(entries) = std::fs::read_dir(&log_dir) {
        for entry in entries.flatten() {
            if let Some(filename) = entry.file_name().to_str() {
                if filename.ends_with(".log") && filename.starts_with("codeforge-") {
                    log_files.push(filename.to_string());
                    info!("获取日志 -> 发现日志文件: {}", filename);
                }
            }
        }
    }

    info!("获取日志 -> 找到 {} 个日志文件", log_files.len());
    log_files.sort();
    log_files.reverse(); // 最新的在前面

    Ok(log_files)
}

// 清除日志
#[command]
pub async fn clear_logs(app: AppHandle, keep_days: u32) -> Result<u32, String> {
    use chrono::{Duration, Local};

    let log_dir = {
        let guard = LOG_DIRECTORY.lock().unwrap();
        match &*guard {
            Some(dir) => dir.clone(),
            None => crate::logger::get_log_directory(&app),
        }
    };
    info!("清理日志 -> 日志目录为: {}", log_dir.display());

    let cutoff_date = Local::now() - Duration::days(keep_days as i64);
    let mut deleted_count = 0;
    let mut scanned_count = 0;

    if let Ok(entries) = std::fs::read_dir(&log_dir) {
        for entry in entries.flatten() {
            if let Some(filename) = entry.file_name().to_str() {
                if filename.ends_with(".log") && filename.starts_with("codeforge-") {
                    scanned_count += 1;
                    // 从文件名提取日期 codeforge-2024-08-09.log
                    if let Some(date_str) = filename
                        .strip_prefix("codeforge-")
                        .and_then(|s| s.strip_suffix(".log"))
                    {
                        if let Ok(file_date) =
                            chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                        {
                            if file_date < cutoff_date.date_naive() {
                                if let Err(e) = std::fs::remove_file(entry.path()) {
                                    warn!("清理日志 -> 删除日志文件失败 {}: {}", filename, e);
                                } else {
                                    info!("清理日志 -> 已删除日志文件: {}", filename);
                                    deleted_count += 1;
                                }
                            } else {
                                info!("清理日志 -> 日志文件文件未到期，将被保留: {}", filename);
                            }
                        }
                    }
                }
            }
        }
    }
    info!(
        "清理日志 -> 扫描 {} 个日志文件，删除 {} 个",
        scanned_count, deleted_count
    );

    Ok(deleted_count)
}
