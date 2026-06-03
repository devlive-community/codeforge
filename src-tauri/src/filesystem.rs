use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use std::sync::Mutex;
use std::time::SystemTime;

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

    let mut guard = INDEX_CACHE.lock().map_err(|_| "索引缓存锁错误".to_string())?;
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
