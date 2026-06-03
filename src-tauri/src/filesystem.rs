use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
pub struct FileNode {
    name: String,
    path: String,
    is_dir: bool,
}

/// 读取目录的直接子项（单层，懒加载用）。目录在前，按名称排序。
#[tauri::command]
pub fn read_directory_tree(path: String) -> Result<Vec<FileNode>, String> {
    let dir = Path::new(&path);
    if !dir.is_dir() {
        return Err(format!("不是有效目录: {}", path));
    }

    let entries = fs::read_dir(dir).map_err(|e| format!("读取目录失败: {}", e))?;
    let mut nodes: Vec<FileNode> = Vec::new();

    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        // 忽略 macOS 元数据文件
        if name == ".DS_Store" {
            continue;
        }
        let p = entry.path();
        let is_dir = p.is_dir();
        nodes.push(FileNode {
            name,
            path: p.to_string_lossy().to_string(),
            is_dir,
        });
    }

    // 目录优先，其次按名称（忽略大小写）排序
    nodes.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    Ok(nodes)
}

/// 默认文本文件大小上限(MB)，超过则拒绝打开，避免编辑器卡死
const DEFAULT_MAX_FILE_SIZE_MB: u64 = 5;

/// 读取文本文件内容（绕开 fs 插件 scope 限制）。
/// max_size_mb 为打开大小上限(MB)，不传则用默认 5MB。
#[tauri::command]
pub fn read_file_text(path: String, max_size_mb: Option<u64>) -> Result<String, String> {
    let limit_mb = max_size_mb.unwrap_or(DEFAULT_MAX_FILE_SIZE_MB).max(1);
    let limit_bytes = limit_mb * 1024 * 1024;

    let meta = fs::metadata(&path).map_err(|e| format!("读取文件失败: {}", e))?;
    if meta.len() > limit_bytes {
        return Err(format!(
            "文件过大（{:.1} MB），超过 {} MB 上限，可在设置中调整",
            meta.len() as f64 / 1024.0 / 1024.0,
            limit_mb
        ));
    }
    // 二进制/非 UTF-8 文件会在此返回错误，避免塞入乱码内容
    fs::read_to_string(&path).map_err(|e| format!("读取文件失败: {}", e))
}

/// 写入文本文件内容
#[tauri::command]
pub fn write_file_text(path: String, content: String) -> Result<(), String> {
    fs::write(&path, content).map_err(|e| format!("写入文件失败: {}", e))
}
