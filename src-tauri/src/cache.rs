use log::info;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheInfo {
    pub plugins_cache_size: u64,
    pub total_cache_size: u64,
}

// 获取缓存目录
fn get_cache_dir() -> Result<PathBuf, String> {
    let home_dir = dirs::home_dir().ok_or("无法获取用户主目录")?;
    Ok(home_dir.join(".codeforge").join("cache"))
}

// 获取插件缓存目录
fn get_plugins_cache_dir() -> Result<PathBuf, String> {
    let cache_dir = get_cache_dir()?;
    Ok(cache_dir.join("plugins"))
}

// 计算目录大小
fn calculate_dir_size(path: &PathBuf) -> u64 {
    if !path.exists() {
        return 0;
    }

    let mut total_size = 0u64;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Ok(metadata) = fs::metadata(&path) {
                    total_size += metadata.len();
                }
            } else if path.is_dir() {
                total_size += calculate_dir_size(&path);
            }
        }
    }

    total_size
}

// 获取缓存信息
#[tauri::command]
pub fn get_cache_info() -> Result<CacheInfo, String> {
    info!("获取缓存信息");

    let cache_dir = get_cache_dir()?;
    let plugins_cache_dir = get_plugins_cache_dir()?;

    let plugins_cache_size = calculate_dir_size(&plugins_cache_dir);
    let total_cache_size = calculate_dir_size(&cache_dir);

    Ok(CacheInfo {
        plugins_cache_size,
        total_cache_size,
    })
}

// 清理插件缓存
#[tauri::command]
pub fn clear_plugins_cache() -> Result<(), String> {
    info!("清理插件缓存");

    let plugins_cache_dir = get_plugins_cache_dir()?;

    if plugins_cache_dir.exists() {
        fs::remove_dir_all(&plugins_cache_dir)
            .map_err(|e| format!("删除插件缓存目录失败: {}", e))?;

        // 重新创建空目录
        fs::create_dir_all(&plugins_cache_dir)
            .map_err(|e| format!("创建插件缓存目录失败: {}", e))?;
    }

    info!("插件缓存已清理");
    Ok(())
}

// 清理所有缓存
#[tauri::command]
pub fn clear_all_cache() -> Result<(), String> {
    info!("清理所有缓存");

    let cache_dir = get_cache_dir()?;

    if cache_dir.exists() {
        fs::remove_dir_all(&cache_dir).map_err(|e| format!("删除缓存目录失败: {}", e))?;

        // 重新创建空目录
        fs::create_dir_all(&cache_dir).map_err(|e| format!("创建缓存目录失败: {}", e))?;
    }

    info!("所有缓存已清理");
    Ok(())
}
