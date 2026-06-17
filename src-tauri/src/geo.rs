//! 地图下钻所需的省/市级 geojson 获取：按 adcode 从 DataV GeoAtlas 拉取，
//! 落盘缓存到 ~/.codeforge/cache/geo/<adcode>.json，首次联网后即可离线复用。

use std::fs;
use std::path::PathBuf;

fn geo_cache_dir() -> Result<PathBuf, String> {
    let home_dir = dirs::home_dir().ok_or("无法获取用户主目录")?;
    let dir = home_dir.join(".codeforge").join("cache").join("geo");
    fs::create_dir_all(&dir).map_err(|e| format!("创建地图缓存目录失败: {}", e))?;
    Ok(dir)
}

/// 取某 adcode 的省/市级边界 geojson（字符串）。优先读本地缓存，缺失时联网拉取并缓存。
#[tauri::command]
pub async fn fetch_area_geojson(adcode: String) -> Result<String, String> {
    // 仅允许纯数字 adcode，避免路径注入与无效请求
    if adcode.is_empty() || !adcode.chars().all(|c| c.is_ascii_digit()) {
        return Err(format!("非法 adcode: {}", adcode));
    }

    let cache_path = geo_cache_dir()?.join(format!("{}.json", adcode));
    if let Ok(cached) = fs::read_to_string(&cache_path) {
        if !cached.trim().is_empty() {
            return Ok(cached);
        }
    }

    let url = format!(
        "https://geo.datav.aliyun.com/areas_v3/bound/{}_full.json",
        adcode
    );
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("请求地图数据失败: {}", e))?;
    if !resp.status().is_success() {
        return Err(format!("地图数据 HTTP {}", resp.status()));
    }
    let body = resp
        .text()
        .await
        .map_err(|e| format!("读取地图数据失败: {}", e))?;

    // 写缓存失败不阻断返回（内存里已有数据）
    let _ = fs::write(&cache_path, &body);
    Ok(body)
}
