use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use std::sync::Mutex;
use std::time::SystemTime;
use tauri::{AppHandle, Emitter};

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

/// 快速打开的文件数量上限
const MAX_LIST_FILES: usize = 20000;

/// 递归列出目录下所有文件（用于 Cmd+P 快速打开）。跳过隐藏目录与常见重目录。
#[tauri::command]
pub fn list_files(path: String) -> Result<Vec<String>, String> {
    let root = Path::new(&path);
    if !root.is_dir() {
        return Err(format!("不是有效目录: {}", path));
    }

    let ignore = [
        "node_modules",
        "target",
        "dist",
        "build",
        ".next",
        ".cache",
    ];
    let mut files: Vec<String> = Vec::new();
    let mut stack = vec![root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        if files.len() >= MAX_LIST_FILES {
            break;
        }
        let read = match fs::read_dir(&dir) {
            Ok(r) => r,
            Err(_) => continue,
        };
        for entry in read.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name == ".DS_Store" {
                continue;
            }
            let p = entry.path();
            if p.is_dir() {
                // 跳过隐藏目录与常见重目录
                if name.starts_with('.') || ignore.contains(&name.as_str()) {
                    continue;
                }
                stack.push(p);
            } else {
                files.push(p.to_string_lossy().to_string());
                if files.len() >= MAX_LIST_FILES {
                    break;
                }
            }
        }
    }

    Ok(files)
}

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

/// 新建空文件
#[tauri::command]
pub fn create_file(path: String) -> Result<(), String> {
    if Path::new(&path).exists() {
        return Err("文件已存在".to_string());
    }
    fs::write(&path, "").map_err(|e| format!("创建文件失败: {}", e))
}

/// 新建目录
#[tauri::command]
pub fn create_directory(path: String) -> Result<(), String> {
    if Path::new(&path).exists() {
        return Err("目录已存在".to_string());
    }
    fs::create_dir_all(&path).map_err(|e| format!("创建目录失败: {}", e))
}

/// 重命名/移动
#[tauri::command]
pub fn rename_path(from: String, to: String) -> Result<(), String> {
    if Path::new(&to).exists() {
        return Err("目标已存在".to_string());
    }
    fs::rename(&from, &to).map_err(|e| format!("重命名失败: {}", e))
}

/// 删除文件或目录（递归）
#[tauri::command]
pub fn delete_path(path: String) -> Result<(), String> {
    let p = Path::new(&path);
    if p.is_dir() {
        fs::remove_dir_all(p).map_err(|e| format!("删除目录失败: {}", e))
    } else {
        fs::remove_file(p).map_err(|e| format!("删除文件失败: {}", e))
    }
}

// 全局目录监听器（保持存活；切换目录时替换旧的）
static WATCHER: Mutex<Option<RecommendedWatcher>> = Mutex::new(None);

/// 监听目录变化，变化时向前端发送 `fs-changed` 事件
#[tauri::command]
pub fn watch_directory(path: String, app: AppHandle) -> Result<(), String> {
    let app_handle = app.clone();
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        if res.is_ok() {
            let _ = app_handle.emit("fs-changed", ());
        }
    })
    .map_err(|e| format!("创建文件监听失败: {}", e))?;

    watcher
        .watch(Path::new(&path), RecursiveMode::Recursive)
        .map_err(|e| format!("监听目录失败: {}", e))?;

    // 替换旧监听器（drop 旧的即停止监听）
    let mut guard = WATCHER.lock().map_err(|_| "监听锁错误".to_string())?;
    *guard = Some(watcher);
    Ok(())
}

/// 在系统文件管理器中显示该路径
#[tauri::command]
pub fn reveal_path(path: String) -> Result<(), String> {
    use std::process::Command;

    #[cfg(target_os = "macos")]
    let result = Command::new("open").args(["-R", &path]).spawn();

    #[cfg(target_os = "windows")]
    let result = Command::new("explorer")
        .arg(format!("/select,{}", path))
        .spawn();

    #[cfg(target_os = "linux")]
    let result = {
        let target = Path::new(&path)
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| Path::new(&path).to_path_buf());
        Command::new("xdg-open").arg(target).spawn()
    };

    result
        .map(|_| ())
        .map_err(|e| format!("打开文件管理器失败: {}", e))
}

#[derive(Serialize)]
pub struct TextFileMeta {
    size_bytes: u64,
    line_count: u64,
    is_text: bool,
}

/// 每隔多少行记录一个字节偏移锚点（索引大小 = 行数 / STEP）
const INDEX_STEP: u64 = 200;

/// 文件行偏移索引：offsets[k] 为第 k*STEP 行起始的字节偏移，支持随机定位
struct FileIndex {
    offsets: Vec<u64>,
    line_count: u64,
    size: u64,
    is_text: bool,
    mtime: Option<SystemTime>,
}

static INDEX_CACHE: Mutex<Option<HashMap<String, FileIndex>>> = Mutex::new(None);

/// 扫描整个文件构建行偏移索引（每个文件只做一次，结果缓存）
fn build_index(path: &str) -> Result<FileIndex, String> {
    let meta = fs::metadata(path).map_err(|e| format!("读取文件失败: {}", e))?;
    let size = meta.len();
    let mtime = meta.modified().ok();

    let file = fs::File::open(path).map_err(|e| format!("读取文件失败: {}", e))?;
    let mut reader = BufReader::new(file);
    let mut buf = [0u8; 65536];

    let mut offsets: Vec<u64> = vec![0]; // 第 0 行从偏移 0 开始
    let mut line_index: u64 = 0;
    let mut offset: u64 = 0;
    let mut is_text = true;
    let mut first = true;
    let mut last_byte: u8 = 0;

    loop {
        let n = reader
            .read(&mut buf)
            .map_err(|e| format!("读取文件失败: {}", e))?;
        if n == 0 {
            break;
        }
        if first {
            if buf[..n].contains(&0) {
                is_text = false;
            }
            first = false;
        }
        for &b in &buf[..n] {
            offset += 1;
            last_byte = b;
            if b == b'\n' {
                line_index += 1;
                // 此处 offset 即下一行（line_index 行）的起始
                if line_index % INDEX_STEP == 0 {
                    offsets.push(offset);
                }
            }
        }
    }

    let line_count = if size > 0 && last_byte != b'\n' {
        line_index + 1
    } else {
        line_index
    };

    Ok(FileIndex {
        offsets,
        line_count,
        size,
        is_text,
        mtime,
    })
}

/// 取得（或构建）文件索引，返回所需标量，避免长时间持锁。
/// 回调在持锁状态下访问 &FileIndex，返回任意结果。
fn with_index<T>(path: &str, f: impl FnOnce(&FileIndex) -> T) -> Result<T, String> {
    let mtime = fs::metadata(path).ok().and_then(|m| m.modified().ok());

    let mut guard = INDEX_CACHE
        .lock()
        .map_err(|_| "索引缓存锁错误".to_string())?;
    let cache = guard.get_or_insert_with(HashMap::new);

    let stale = match cache.get(path) {
        Some(idx) => idx.mtime != mtime,
        None => true,
    };
    if stale {
        let idx = build_index(path)?;
        cache.insert(path.to_string(), idx);
    }

    let idx = cache.get(path).unwrap();
    Ok(f(idx))
}

/// 获取文本文件元信息：大小、总行数、是否为文本（用于决定可编辑打开还是只读查看）。
#[tauri::command]
pub fn get_text_file_meta(path: String) -> Result<TextFileMeta, String> {
    with_index(&path, |idx| TextFileMeta {
        size_bytes: idx.size,
        line_count: idx.line_count,
        is_text: idx.is_text,
    })
}

/// 按行范围读取文件（只读查看器虚拟滚动用）。借助行偏移索引随机定位，做到 O(窗口)。
#[tauri::command]
pub fn read_file_lines(path: String, start: u64, count: u64) -> Result<Vec<String>, String> {
    // 取出最近的锚点偏移与其对应行号
    let (anchor_offset, anchor_line) = with_index(&path, |idx| {
        let mut k = (start / INDEX_STEP) as usize;
        if k >= idx.offsets.len() {
            k = idx.offsets.len() - 1;
        }
        (idx.offsets[k], k as u64 * INDEX_STEP)
    })?;

    let file = fs::File::open(&path).map_err(|e| format!("读取文件失败: {}", e))?;
    let mut reader = BufReader::new(file);
    reader
        .seek(SeekFrom::Start(anchor_offset))
        .map_err(|e| format!("定位文件失败: {}", e))?;

    let end = start.saturating_add(count);
    let mut lines: Vec<String> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let idx = anchor_line + i as u64;
        if idx < start {
            continue;
        }
        if idx >= end {
            break;
        }
        match line {
            Ok(l) => lines.push(l),
            Err(_) => lines.push(String::from("\u{FFFD}")),
        }
    }

    Ok(lines)
}
