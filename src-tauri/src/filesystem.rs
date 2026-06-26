use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
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
/// 重 I/O 放到阻塞线程池，避免阻塞主线程。
#[tauri::command]
pub async fn list_files(path: String) -> Result<Vec<String>, String> {
    tokio::task::spawn_blocking(move || run_list_files(path))
        .await
        .map_err(|e| format!("列文件任务失败: {}", e))?
}

fn run_list_files(path: String) -> Result<Vec<String>, String> {
    let root = Path::new(&path);
    if !root.is_dir() {
        return Err(format!("不是有效目录: {}", path));
    }

    let ignore = ["node_modules", "target", "dist", "build", ".next", ".cache"];
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
            // 用 file_type 不跟随符号链接，避免软链成环导致无限递归
            let ft = match entry.file_type() {
                Ok(t) => t,
                Err(_) => continue,
            };
            if ft.is_symlink() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            if name == ".DS_Store" {
                continue;
            }
            let p = entry.path();
            if ft.is_dir() {
                // 跳过隐藏目录与常见重目录
                if name.starts_with('.') || ignore.contains(&name.as_str()) {
                    continue;
                }
                stack.push(p);
            } else if ft.is_file() {
                files.push(p.to_string_lossy().to_string());
                if files.len() >= MAX_LIST_FILES {
                    break;
                }
            }
        }
    }

    Ok(files)
}

#[derive(Serialize)]
pub struct SearchMatch {
    path: String,
    line: u32,
    text: String,
}

const MAX_SEARCH_MATCHES: usize = 1000;
const MAX_SEARCH_FILE_SIZE: u64 = 2 * 1024 * 1024;
// 最多扫描的文件数，避免在超大目录中卡死
const MAX_SEARCH_FILES_SCANNED: usize = 50000;

/// 在文件夹内全局搜索文本（大小写不敏感的子串）。
/// 重 I/O 放到阻塞线程池，避免阻塞主线程导致应用无响应。
#[tauri::command]
pub async fn search_in_files(root: String, query: String) -> Result<Vec<SearchMatch>, String> {
    tokio::task::spawn_blocking(move || run_search(root, query))
        .await
        .map_err(|e| format!("搜索任务失败: {}", e))?
}

fn run_search(root: String, query: String) -> Result<Vec<SearchMatch>, String> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return Ok(vec![]);
    }
    let root_path = Path::new(&root);
    if !root_path.is_dir() {
        return Err(format!("不是有效目录: {}", root));
    }

    let ignore = ["node_modules", "target", "dist", "build", ".next", ".cache"];
    let mut matches: Vec<SearchMatch> = Vec::new();
    let mut scanned: usize = 0;
    let mut stack = vec![root_path.to_path_buf()];

    'outer: while let Some(dir) = stack.pop() {
        let read = match fs::read_dir(&dir) {
            Ok(r) => r,
            Err(_) => continue,
        };
        for entry in read.flatten() {
            if matches.len() >= MAX_SEARCH_MATCHES || scanned >= MAX_SEARCH_FILES_SCANNED {
                break 'outer;
            }
            // 用 file_type 不跟随符号链接，避免软链成环导致无限递归
            let ft = match entry.file_type() {
                Ok(t) => t,
                Err(_) => continue,
            };
            if ft.is_symlink() {
                continue;
            }

            let name = entry.file_name().to_string_lossy().to_string();
            if name == ".DS_Store" {
                continue;
            }
            let p = entry.path();

            if ft.is_dir() {
                if name.starts_with('.') || ignore.contains(&name.as_str()) {
                    continue;
                }
                stack.push(p);
                continue;
            }
            if !ft.is_file() {
                continue;
            }

            scanned += 1;
            // 跳过过大文件
            if let Ok(meta) = entry.metadata() {
                if meta.len() > MAX_SEARCH_FILE_SIZE {
                    continue;
                }
            }
            // 二进制/非 UTF-8 读取会失败，自动跳过
            let content = match fs::read_to_string(&p) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let path_str = p.to_string_lossy().to_string();
            for (i, line) in content.lines().enumerate() {
                if line.to_lowercase().contains(&q) {
                    matches.push(SearchMatch {
                        path: path_str.clone(),
                        line: (i + 1) as u32,
                        text: line.chars().take(200).collect(),
                    });
                    if matches.len() >= MAX_SEARCH_MATCHES {
                        break 'outer;
                    }
                }
            }
        }
    }

    Ok(matches)
}

#[derive(Serialize)]
pub struct ReplaceSummary {
    files_changed: usize,
    replacements: usize,
}

/// 在文件夹内全局替换文本（ASCII 大小写不敏感的字面量替换，与搜索语义一致）。
/// 重 I/O 放到阻塞线程池。
#[tauri::command]
pub async fn replace_in_files(
    root: String,
    query: String,
    replacement: String,
) -> Result<ReplaceSummary, String> {
    tokio::task::spawn_blocking(move || run_replace(root, query, replacement))
        .await
        .map_err(|e| format!("替换任务失败: {}", e))?
}

// 根据首字节推断 UTF-8 字符字节数
fn utf8_char_len(b: u8) -> usize {
    if b < 0x80 {
        1
    } else if b >> 5 == 0b110 {
        2
    } else if b >> 4 == 0b1110 {
        3
    } else if b >> 3 == 0b11110 {
        4
    } else {
        1
    }
}

/// ASCII 大小写不敏感的字面量替换，保持非 ASCII 字节按精确匹配，返回新文本与替换次数。
fn replace_ascii_ci(text: &str, needle: &str, replacement: &str) -> (String, usize) {
    let nb = needle.as_bytes();
    let nlen = nb.len();
    if nlen == 0 {
        return (text.to_string(), 0);
    }
    let bytes = text.as_bytes();
    let mut out = String::with_capacity(text.len());
    let mut count = 0usize;
    let mut i = 0usize;
    while i < bytes.len() {
        if i + nlen <= bytes.len() && bytes[i..i + nlen].eq_ignore_ascii_case(nb) {
            out.push_str(replacement);
            count += 1;
            i += nlen;
        } else {
            let end = (i + utf8_char_len(bytes[i])).min(bytes.len());
            out.push_str(&text[i..end]);
            i = end;
        }
    }
    (out, count)
}

fn run_replace(root: String, query: String, replacement: String) -> Result<ReplaceSummary, String> {
    if query.is_empty() {
        return Ok(ReplaceSummary {
            files_changed: 0,
            replacements: 0,
        });
    }
    let root_path = Path::new(&root);
    if !root_path.is_dir() {
        return Err(format!("不是有效目录: {}", root));
    }

    let ignore = ["node_modules", "target", "dist", "build", ".next", ".cache"];
    let mut files_changed = 0usize;
    let mut replacements = 0usize;
    let mut scanned: usize = 0;
    let mut stack = vec![root_path.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let read = match fs::read_dir(&dir) {
            Ok(r) => r,
            Err(_) => continue,
        };
        for entry in read.flatten() {
            if scanned >= MAX_SEARCH_FILES_SCANNED {
                break;
            }
            let ft = match entry.file_type() {
                Ok(t) => t,
                Err(_) => continue,
            };
            if ft.is_symlink() {
                continue;
            }

            let name = entry.file_name().to_string_lossy().to_string();
            if name == ".DS_Store" {
                continue;
            }
            let p = entry.path();

            if ft.is_dir() {
                if name.starts_with('.') || ignore.contains(&name.as_str()) {
                    continue;
                }
                stack.push(p);
                continue;
            }
            if !ft.is_file() {
                continue;
            }

            scanned += 1;
            if let Ok(meta) = entry.metadata() {
                if meta.len() > MAX_SEARCH_FILE_SIZE {
                    continue;
                }
            }
            let content = match fs::read_to_string(&p) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let (new_content, n) = replace_ascii_ci(&content, &query, &replacement);
            if n > 0 && fs::write(&p, new_content).is_ok() {
                files_changed += 1;
                replacements += n;
            }
        }
    }

    Ok(ReplaceSummary {
        files_changed,
        replacements,
    })
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

/// git diff 内容的最大长度（避免给 AI 的 prompt 过大）
const MAX_DIFF_LEN: usize = 20000;

/// 获取目录下的 git 改动 diff（相对 HEAD），用于 AI 生成提交信息。
#[tauri::command]
pub async fn git_diff(root: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let output = std::process::Command::new("git")
            .args(["-C", &root, "diff", "HEAD"])
            .output()
            .map_err(|e| format!("执行 git 失败: {}", e))?;
        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "git diff 失败（是否为 git 仓库？）：{}",
                err.trim()
            ));
        }
        let mut diff = String::from_utf8_lossy(&output.stdout).to_string();
        if diff.len() > MAX_DIFF_LEN {
            diff.truncate(MAX_DIFF_LEN);
            diff.push_str("\n…(diff 过长已截断)");
        }
        Ok(diff)
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

// ===== Git 源代码管理 =====

/// 克隆远程仓库到 dir 下，返回克隆出的仓库目录路径。
#[tauri::command]
pub async fn git_clone(url: String, dir: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        run_git(&dir, &["clone", &url])?;
        // 由 URL 推断仓库目录名（去掉结尾 / 与 .git）
        let name = url
            .trim()
            .trim_end_matches('/')
            .rsplit('/')
            .next()
            .unwrap_or("repo")
            .trim_end_matches(".git");
        Ok(format!("{}/{}", dir.trim_end_matches('/'), name))
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 在目录初始化 Git 仓库。
#[tauri::command]
pub async fn git_init(root: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["init"]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

#[derive(Serialize, Default)]
pub struct EditorConfigResolved {
    /// "tab" | "space"
    indent_style: Option<String>,
    indent_size: Option<u32>,
    tab_width: Option<u32>,
    trim_trailing_whitespace: Option<bool>,
    insert_final_newline: Option<bool>,
}

/// 把 editorconfig glob 转为正则（覆盖 * ** ? {a,b} [..]）
fn editorconfig_glob_to_regex(glob: &str) -> String {
    let chars: Vec<char> = glob.chars().collect();
    let mut re = String::from("^");
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        match c {
            '*' => {
                if i + 1 < chars.len() && chars[i + 1] == '*' {
                    re.push_str(".*");
                    i += 1;
                } else {
                    re.push_str("[^/]*");
                }
            }
            '?' => re.push_str("[^/]"),
            '{' => {
                // {a,b,c} -> (a|b|c)
                let mut j = i + 1;
                let mut buf = String::new();
                let mut alts = String::from("(");
                let mut closed = false;
                while j < chars.len() {
                    match chars[j] {
                        '}' => {
                            alts.push_str(&regex::escape(&buf));
                            alts.push(')');
                            closed = true;
                            j += 1;
                            break;
                        }
                        ',' => {
                            alts.push_str(&regex::escape(&buf));
                            alts.push('|');
                            buf.clear();
                        }
                        ch => buf.push(ch),
                    }
                    j += 1;
                }
                if closed {
                    re.push_str(&alts);
                    i = j;
                    continue;
                } else {
                    re.push_str("\\{");
                }
            }
            '[' => {
                re.push('[');
                let mut j = i + 1;
                if j < chars.len() && chars[j] == '!' {
                    re.push('^');
                    j += 1;
                }
                while j < chars.len() && chars[j] != ']' {
                    re.push(chars[j]);
                    j += 1;
                }
                re.push(']');
                i = j;
                continue;
            }
            '.' | '(' | ')' | '+' | '|' | '^' | '$' | '\\' => {
                re.push('\\');
                re.push(c);
            }
            _ => re.push(c),
        }
        i += 1;
    }
    re.push('$');
    re
}

fn editorconfig_is_root(content: &str) -> bool {
    for line in content.lines() {
        let l = line.trim();
        if l.starts_with('[') {
            break;
        }
        if let Some((k, v)) = l.split_once('=') {
            if k.trim().eq_ignore_ascii_case("root") && v.trim().eq_ignore_ascii_case("true") {
                return true;
            }
        }
    }
    false
}

fn editorconfig_apply(
    content: &str,
    rel_path: &str,
    file_name: &str,
    out: &mut EditorConfigResolved,
) {
    let mut matched = false;
    for line in content.lines() {
        let l = line.trim();
        if l.is_empty() || l.starts_with('#') || l.starts_with(';') {
            continue;
        }
        if let Some(glob) = l.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            // 含 '/' 按相对路径匹配，否则按文件名匹配
            let (pat, target) = if glob.contains('/') {
                (glob.trim_start_matches('/'), rel_path)
            } else {
                (glob, file_name)
            };
            matched = regex::Regex::new(&editorconfig_glob_to_regex(pat))
                .map(|re| re.is_match(target))
                .unwrap_or(false);
            continue;
        }
        if !matched {
            continue;
        }
        if let Some((k, v)) = l.split_once('=') {
            let key = k.trim().to_lowercase();
            let val = v.trim().to_string();
            match key.as_str() {
                "indent_style" => out.indent_style = Some(val.to_lowercase()),
                "indent_size" => {
                    if let Ok(n) = val.parse::<u32>() {
                        out.indent_size = Some(n);
                    }
                }
                "tab_width" => {
                    if let Ok(n) = val.parse::<u32>() {
                        out.tab_width = Some(n);
                    }
                }
                "trim_trailing_whitespace" => {
                    out.trim_trailing_whitespace = Some(val.eq_ignore_ascii_case("true"))
                }
                "insert_final_newline" => {
                    out.insert_final_newline = Some(val.eq_ignore_ascii_case("true"))
                }
                _ => {}
            }
        }
    }
}

/// 解析某文件适用的 .editorconfig（向上查找，近的覆盖远的，遇 root=true 停）。
#[tauri::command]
pub async fn resolve_editorconfig(file_path: String) -> Result<EditorConfigResolved, String> {
    tokio::task::spawn_blocking(move || {
        let path = std::path::PathBuf::from(&file_path);
        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        // 自近向远收集 .editorconfig，遇 root 停
        let mut configs: Vec<std::path::PathBuf> = Vec::new();
        let mut dir = path.parent();
        while let Some(d) = dir {
            let cfg = d.join(".editorconfig");
            if cfg.is_file() {
                let is_root = std::fs::read_to_string(&cfg)
                    .map(|c| editorconfig_is_root(&c))
                    .unwrap_or(false);
                configs.push(cfg);
                if is_root {
                    break;
                }
            }
            dir = d.parent();
        }
        // 自远向近应用（近的覆盖）
        let mut out = EditorConfigResolved::default();
        for cfg in configs.iter().rev() {
            if let (Ok(content), Some(cfg_dir)) = (std::fs::read_to_string(cfg), cfg.parent()) {
                let rel = path
                    .strip_prefix(cfg_dir)
                    .map(|p| p.to_string_lossy().replace('\\', "/"))
                    .unwrap_or_default();
                editorconfig_apply(&content, &rel, &file_name, &mut out);
            }
        }
        Ok(out)
    })
    .await
    .map_err(|e| format!(".editorconfig 解析失败: {}", e))?
}

/// 追加一段 .gitignore 模板块（按首行标题去重，已存在则跳过）。
#[tauri::command]
pub async fn git_ignore_append_block(root: String, content: String) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        let block = content.trim_end_matches('\n');
        if block.trim().is_empty() {
            return Ok(());
        }
        let path = std::path::Path::new(&root).join(".gitignore");
        let mut existing = std::fs::read_to_string(&path).unwrap_or_default();
        // 用块首行（通常是 # 标题）判重，避免重复插入
        let header = block.lines().next().unwrap_or("").trim();
        if !header.is_empty() && existing.lines().any(|l| l.trim() == header) {
            return Ok(());
        }
        if !existing.is_empty() {
            if !existing.ends_with('\n') {
                existing.push('\n');
            }
            existing.push('\n'); // 与上一块隔一空行
        }
        existing.push_str(block);
        existing.push('\n');
        std::fs::write(&path, existing).map_err(|e| format!("写入 .gitignore 失败: {}", e))
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 把一个匹配模式追加到 .gitignore（已存在则跳过）。
#[tauri::command]
pub async fn git_ignore_add(root: String, pattern: String) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        let p = pattern.trim();
        if p.is_empty() {
            return Ok(());
        }
        let path = std::path::Path::new(&root).join(".gitignore");
        let mut content = std::fs::read_to_string(&path).unwrap_or_default();
        if content.lines().any(|l| l.trim() == p) {
            return Ok(());
        }
        if !content.is_empty() && !content.ends_with('\n') {
            content.push('\n');
        }
        content.push_str(p);
        content.push('\n');
        std::fs::write(&path, content).map_err(|e| format!("写入 .gitignore 失败: {}", e))
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 同步执行 git 子命令，返回标准输出；失败时返回 stderr。
fn run_git(root: &str, args: &[&str]) -> Result<String, String> {
    let mut full: Vec<&str> = vec!["-C", root];
    full.extend_from_slice(args);
    let output = std::process::Command::new("git")
        .args(&full)
        .output()
        .map_err(|e| format!("执行 git 失败: {}", e))?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(err.trim().to_string());
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// 与 run_git 类似，但通过标准输入传入数据（用于 git apply 接收补丁）。
fn run_git_stdin(root: &str, args: &[&str], input: &str) -> Result<String, String> {
    use std::io::Write;
    use std::process::Stdio;
    let mut full: Vec<&str> = vec!["-C", root];
    full.extend_from_slice(args);
    let mut child = std::process::Command::new("git")
        .args(&full)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("执行 git 失败: {}", e))?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(input.as_bytes())
            .map_err(|e| format!("写入 git 输入失败: {}", e))?;
    }
    let output = child
        .wait_with_output()
        .map_err(|e| format!("等待 git 失败: {}", e))?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(err.trim().to_string());
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// 获取单个文件的 unified diff。staged=true 返回已暂存(index vs HEAD)，否则工作区(worktree vs index)。
#[tauri::command]
pub async fn git_file_diff(root: String, rel_path: String, staged: bool) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let mut args: Vec<&str> = vec!["diff"];
        if staged {
            args.push("--cached");
        }
        args.push("--");
        args.push(rel_path.as_str());
        run_git(&root, &args)
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 应用单个 hunk 补丁。cached=true 作用于暂存区，reverse=true 反向应用（用于取消暂存/丢弃）。
#[tauri::command]
pub async fn git_apply_patch(
    root: String,
    patch: String,
    cached: bool,
    reverse: bool,
) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let mut args: Vec<&str> = vec!["apply"];
        if cached {
            args.push("--cached");
        }
        if reverse {
            args.push("--reverse");
        }
        run_git_stdin(&root, &args, &patch)
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

#[derive(Serialize)]
pub struct GitFileStatus {
    /// 相对仓库根的路径
    path: String,
    /// 暂存区状态字符（X）
    index: String,
    /// 工作区状态字符（Y）
    worktree: String,
}

#[derive(Serialize)]
pub struct GitStatus {
    is_repo: bool,
    branch: String,
    ahead: u32,
    behind: u32,
    files: Vec<GitFileStatus>,
}

/// 获取 git 状态（分支、领先/落后、各文件暂存/工作区状态）。
#[tauri::command]
pub async fn git_status(root: String) -> Result<GitStatus, String> {
    tokio::task::spawn_blocking(move || {
        // 先确认是否在 git 仓库中
        if run_git(&root, &["rev-parse", "--is-inside-work-tree"]).is_err() {
            return Ok(GitStatus {
                is_repo: false,
                branch: String::new(),
                ahead: 0,
                behind: 0,
                files: vec![],
            });
        }

        let out = run_git(&root, &["status", "--porcelain", "--branch"])?;
        let mut branch = String::new();
        let mut ahead = 0u32;
        let mut behind = 0u32;
        let mut files = Vec::new();

        for line in out.lines() {
            if let Some(rest) = line.strip_prefix("## ") {
                // 形如：main...origin/main [ahead 1, behind 2]
                let name_part = rest.split("...").next().unwrap_or(rest);
                branch = name_part.trim().to_string();
                if let Some(start) = rest.find('[') {
                    let bracket = &rest[start + 1..rest.find(']').unwrap_or(rest.len())];
                    for seg in bracket.split(',') {
                        let seg = seg.trim();
                        if let Some(n) = seg.strip_prefix("ahead ") {
                            ahead = n.trim().parse().unwrap_or(0);
                        } else if let Some(n) = seg.strip_prefix("behind ") {
                            behind = n.trim().parse().unwrap_or(0);
                        }
                    }
                }
                continue;
            }
            if line.len() < 3 {
                continue;
            }
            let index = &line[0..1];
            let worktree = &line[1..2];
            let mut path = line[3..].to_string();
            // 重命名形如 "old -> new"，取新路径
            if let Some(pos) = path.find(" -> ") {
                path = path[pos + 4..].to_string();
            }
            // 去除可能的引号包裹
            let path = path.trim_matches('"').to_string();
            files.push(GitFileStatus {
                path,
                index: index.to_string(),
                worktree: worktree.to_string(),
            });
        }

        Ok(GitStatus {
            is_repo: true,
            branch,
            ahead,
            behind,
            files,
        })
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 暂存指定文件（相对路径或绝对路径均可）。
#[tauri::command]
pub async fn git_stage(root: String, paths: Vec<String>) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        let mut args = vec!["add", "--"];
        let refs: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();
        args.extend_from_slice(&refs);
        run_git(&root, &args).map(|_| ())
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 把某文件恢复到指定提交时的版本（checkout <ref> -- <file>，写入工作区）。
#[tauri::command]
pub async fn git_restore_file(root: String, rel_path: String, hash: String) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        run_git(&root, &["checkout", &hash, "--", &rel_path]).map(|_| ())
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 丢弃已跟踪文件的改动：暂存区与工作区一并恢复到 HEAD（不可恢复）。
/// 未跟踪文件不在此处理（由前端删除）。
#[tauri::command]
pub async fn git_discard(root: String, paths: Vec<String>) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        let mut args = vec!["restore", "--source=HEAD", "--staged", "--worktree", "--"];
        let refs: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();
        args.extend_from_slice(&refs);
        run_git(&root, &args).map(|_| ())
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 取消暂存指定文件。
#[tauri::command]
pub async fn git_unstage(root: String, paths: Vec<String>) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        let mut args = vec!["reset", "-q", "HEAD", "--"];
        let refs: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();
        args.extend_from_slice(&refs);
        run_git(&root, &args).map(|_| ())
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 提交已暂存的改动。
/// amend=true 修正上次提交（message 为空则保留原信息）；all=true 自动暂存已跟踪改动（-a）；signoff=true 追加 Signed-off-by（-s）。
#[tauri::command]
pub async fn git_commit(
    root: String,
    message: String,
    amend: bool,
    all: bool,
    signoff: bool,
) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let mut args: Vec<&str> = vec!["commit"];
        if amend {
            args.push("--amend");
        }
        if all {
            args.push("-a");
        }
        if signoff {
            args.push("-s");
        }
        if amend && message.trim().is_empty() {
            args.push("--no-edit");
        } else {
            args.push("-m");
            args.push(&message);
        }
        run_git(&root, &args)
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 推送当前分支。
#[tauri::command]
pub async fn git_push(root: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["push"]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 预览将被 git clean 删除的未跟踪文件/目录（dry-run）。
#[tauri::command]
pub async fn git_clean_preview(root: String) -> Result<Vec<String>, String> {
    tokio::task::spawn_blocking(move || {
        let out = run_git(&root, &["clean", "-nd"])?;
        Ok(out
            .lines()
            .filter_map(|l| l.strip_prefix("Would remove ").map(|s| s.to_string()))
            .collect())
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 清理未跟踪文件与目录（git clean -fd，不可恢复）。
#[tauri::command]
pub async fn git_clean(root: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["clean", "-fd"]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 拉取并合并远程当前分支。
#[tauri::command]
pub async fn git_pull(root: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["pull"]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 带变基拉取（pull --rebase）。
#[tauri::command]
pub async fn git_pull_rebase(root: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["pull", "--rebase"]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 安全强制推送（--force-with-lease）。
#[tauri::command]
pub async fn git_push_force(root: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["push", "--force-with-lease"]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 推送所有标签。
#[tauri::command]
pub async fn git_push_tags(root: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["push", "--tags"]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 删除远程分支。
#[tauri::command]
pub async fn git_delete_remote_branch(
    root: String,
    remote: String,
    branch: String,
) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["push", &remote, "--delete", &branch]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 抓取远程更新（不合并）。
#[tauri::command]
pub async fn git_fetch(root: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["fetch", "--all", "--prune"]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

#[derive(Serialize)]
pub struct GitStashEntry {
    /// 形如 stash@{0}
    reference: String,
    message: String,
}

/// 列出 stash 列表。
#[tauri::command]
pub async fn git_stash_list(root: String) -> Result<Vec<GitStashEntry>, String> {
    tokio::task::spawn_blocking(move || {
        let out = run_git(&root, &["stash", "list", "--pretty=format:%gd\x1f%s"])?;
        let mut list = Vec::new();
        for line in out.lines() {
            let p: Vec<&str> = line.split('\u{1f}').collect();
            if p.len() >= 2 {
                list.push(GitStashEntry {
                    reference: p[0].to_string(),
                    message: p[1].to_string(),
                });
            }
        }
        Ok(list)
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 暂存当前改动（含未跟踪文件）。message 为空则用默认信息。
#[tauri::command]
pub async fn git_stash_push(root: String, message: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let mut args = vec!["stash", "push", "--include-untracked"];
        if !message.trim().is_empty() {
            args.push("-m");
            args.push(message.as_str());
        }
        run_git(&root, &args)
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 应用某个 stash 但保留（apply）。
#[tauri::command]
pub async fn git_stash_apply(root: String, reference: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["stash", "apply", &reference]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 查看某个 stash 的补丁内容。
#[tauri::command]
pub async fn git_stash_show(root: String, reference: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["stash", "show", "-p", &reference]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 应用并移除某个 stash（pop）。
#[tauri::command]
pub async fn git_stash_pop(root: String, reference: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["stash", "pop", &reference]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 丢弃某个 stash（drop）。
#[tauri::command]
pub async fn git_stash_drop(root: String, reference: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["stash", "drop", &reference]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

#[derive(Serialize)]
pub struct GitBranches {
    current: String,
    branches: Vec<String>,
}

/// 列出本地分支与当前分支。
#[tauri::command]
pub async fn git_branches(root: String) -> Result<GitBranches, String> {
    tokio::task::spawn_blocking(move || {
        let current = run_git(&root, &["rev-parse", "--abbrev-ref", "HEAD"])?
            .trim()
            .to_string();
        let out = run_git(&root, &["branch", "--format=%(refname:short)"])?;
        let branches = out
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();
        Ok(GitBranches { current, branches })
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 切换分支。
#[tauri::command]
pub async fn git_checkout(root: String, branch: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["checkout", &branch]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 列出远程跟踪分支（如 origin/main），排除 HEAD 指针。
#[tauri::command]
pub async fn git_remote_branches(root: String) -> Result<Vec<String>, String> {
    tokio::task::spawn_blocking(move || {
        let out = run_git(&root, &["branch", "-r", "--format=%(refname:short)"])?;
        Ok(out
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty() && !l.contains("->"))
            .collect())
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 检出远程分支并建立本地跟踪分支（checkout -t origin/x）。
#[tauri::command]
pub async fn git_checkout_track(root: String, remote_branch: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["checkout", "-t", &remote_branch]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 新建并切换到分支。
#[tauri::command]
pub async fn git_branch_create(root: String, name: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["checkout", "-b", &name]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 删除分支（安全删除，未合并会失败）。
#[tauri::command]
pub async fn git_branch_delete(root: String, name: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["branch", "-d", &name]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 重命名分支。
#[tauri::command]
pub async fn git_branch_rename(root: String, old: String, new: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["branch", "-m", &old, &new]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 把指定分支合并到当前分支。
#[tauri::command]
pub async fn git_merge(root: String, branch: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["merge", "--no-edit", &branch]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

#[derive(Serialize)]
pub struct GitCommit {
    hash: String,
    short: String,
    author: String,
    date: String,
    subject: String,
}

/// 提交历史（分页）：limit 条，跳过 skip 条。revision 指定时查看该分支/引用的历史。
#[tauri::command]
pub async fn git_log(
    root: String,
    limit: u32,
    skip: u32,
    revision: Option<String>,
    grep: Option<String>,
    author: Option<String>,
) -> Result<Vec<GitCommit>, String> {
    tokio::task::spawn_blocking(move || {
        let n = format!("-n{}", limit);
        let sk = format!("--skip={}", skip);
        let grep = grep.unwrap_or_default();
        let author = author.unwrap_or_default();
        let grep_arg = format!("--grep={}", grep.trim());
        let author_arg = format!("--author={}", author.trim());
        // 字段以 \x1f 分隔、每提交一行；%s 为单行主题
        let mut args = vec![
            "log",
            n.as_str(),
            sk.as_str(),
            "--date=format:%Y-%m-%d %H:%M",
            "--pretty=format:%H\x1f%h\x1f%an\x1f%ad\x1f%s",
        ];
        // 提交信息搜索（大小写不敏感）与作者过滤
        if !grep.trim().is_empty() {
            args.push("-i");
            args.push(grep_arg.as_str());
        }
        if !author.trim().is_empty() {
            args.push(author_arg.as_str());
        }
        let rev = revision.unwrap_or_default();
        if !rev.trim().is_empty() {
            args.push(rev.as_str());
        }
        let out = run_git(&root, &args)?;
        let mut commits = Vec::new();
        for line in out.lines() {
            let p: Vec<&str> = line.split('\u{1f}').collect();
            if p.len() >= 5 {
                commits.push(GitCommit {
                    hash: p[0].to_string(),
                    short: p[1].to_string(),
                    author: p[2].to_string(),
                    date: p[3].to_string(),
                    subject: p[4].to_string(),
                });
            }
        }
        Ok(commits)
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

#[derive(Serialize)]
pub struct GitWorktree {
    path: String,
    head: String,
    /// 分支短名；分离 HEAD 或裸仓库时为空
    branch: String,
    bare: bool,
    detached: bool,
    locked: bool,
}

/// 列出 worktree（解析 worktree list --porcelain）。
#[tauri::command]
pub async fn git_worktrees(root: String) -> Result<Vec<GitWorktree>, String> {
    tokio::task::spawn_blocking(move || {
        let out = run_git(&root, &["worktree", "list", "--porcelain"])?;
        let mut list: Vec<GitWorktree> = Vec::new();
        let mut cur: Option<GitWorktree> = None;
        for line in out.lines() {
            if let Some(p) = line.strip_prefix("worktree ") {
                if let Some(w) = cur.take() {
                    list.push(w);
                }
                cur = Some(GitWorktree {
                    path: p.to_string(),
                    head: String::new(),
                    branch: String::new(),
                    bare: false,
                    detached: false,
                    locked: false,
                });
            } else if let Some(w) = cur.as_mut() {
                if let Some(h) = line.strip_prefix("HEAD ") {
                    w.head = h.chars().take(8).collect();
                } else if let Some(b) = line.strip_prefix("branch ") {
                    w.branch = b.strip_prefix("refs/heads/").unwrap_or(b).to_string();
                } else if line == "bare" {
                    w.bare = true;
                } else if line == "detached" {
                    w.detached = true;
                } else if line.starts_with("locked") {
                    w.locked = true;
                }
            }
        }
        if let Some(w) = cur.take() {
            list.push(w);
        }
        Ok(list)
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 新增 worktree。ref 为空则由 git 按路径名自动建分支。
#[tauri::command]
pub async fn git_worktree_add(
    root: String,
    path: String,
    reference: String,
) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let mut args = vec!["worktree", "add", path.as_str()];
        if !reference.trim().is_empty() {
            args.push(reference.as_str());
        }
        run_git(&root, &args)
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 移除 worktree。
#[tauri::command]
pub async fn git_worktree_remove(root: String, path: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["worktree", "remove", &path]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 清理失效的 worktree 记录。
#[tauri::command]
pub async fn git_worktree_prune(root: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["worktree", "prune"]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

#[derive(Serialize)]
pub struct GitSubmodule {
    path: String,
    hash: String,
    /// ok / uninitialized / modified / conflict
    state: String,
    describe: String,
}

/// 列出子模块状态（解析 git submodule status）。
#[tauri::command]
pub async fn git_submodules(root: String) -> Result<Vec<GitSubmodule>, String> {
    tokio::task::spawn_blocking(move || {
        let out = run_git(&root, &["submodule", "status"])?;
        let mut list: Vec<GitSubmodule> = Vec::new();
        for line in out.lines() {
            if line.trim().is_empty() {
                continue;
            }
            // 形如 " <sha> <path> (<describe>)"，首字符表示状态
            let prefix = line.chars().next().unwrap_or(' ');
            let state = match prefix {
                '-' => "uninitialized",
                '+' => "modified",
                'U' => "conflict",
                _ => "ok",
            }
            .to_string();
            let rest = line[1..].trim();
            let mut it = rest.splitn(2, ' ');
            let hash = it.next().unwrap_or("").to_string();
            let tail = it.next().unwrap_or("");
            let (path, describe) = match tail.find(" (") {
                Some(idx) => (
                    tail[..idx].trim().to_string(),
                    tail[idx + 2..].trim_end_matches(')').to_string(),
                ),
                None => (tail.trim().to_string(), String::new()),
            };
            if !path.is_empty() {
                list.push(GitSubmodule {
                    path,
                    hash,
                    state,
                    describe,
                });
            }
        }
        Ok(list)
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 更新子模块（init + recursive）。path 为空则更新全部。
#[tauri::command]
pub async fn git_submodule_update(root: String, path: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let mut args = vec!["submodule", "update", "--init", "--recursive"];
        if !path.trim().is_empty() {
            args.push("--");
            args.push(path.as_str());
        }
        run_git(&root, &args)
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 同步子模块 URL 配置（submodule sync --recursive）。
#[tauri::command]
pub async fn git_submodule_sync(root: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["submodule", "sync", "--recursive"]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

#[derive(Serialize)]
pub struct GitGraphCommit {
    hash: String,
    short: String,
    parents: Vec<String>,
    refs: String,
    author: String,
    date: String,
    subject: String,
}

/// 提交图数据：--all --topo-order，含父提交与引用名，供前端绘制分支图。
#[tauri::command]
pub async fn git_graph(root: String, limit: u32) -> Result<Vec<GitGraphCommit>, String> {
    tokio::task::spawn_blocking(move || {
        let n = format!("-n{}", limit);
        let args = vec![
            "log",
            n.as_str(),
            "--all",
            "--topo-order",
            "--date=format:%Y-%m-%d %H:%M",
            "--pretty=format:%H\x1f%h\x1f%P\x1f%D\x1f%an\x1f%ad\x1f%s",
        ];
        let out = run_git(&root, &args)?;
        let mut commits = Vec::new();
        for line in out.lines() {
            let p: Vec<&str> = line.split('\u{1f}').collect();
            if p.len() >= 7 {
                let parents = p[2]
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
                commits.push(GitGraphCommit {
                    hash: p[0].to_string(),
                    short: p[1].to_string(),
                    parents,
                    refs: p[3].to_string(),
                    author: p[4].to_string(),
                    date: p[5].to_string(),
                    subject: p[6].to_string(),
                });
            }
        }
        Ok(commits)
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

#[derive(Serialize)]
pub struct GitRemote {
    name: String,
    url: String,
}

/// 列出远程（名称 + fetch URL）。
#[tauri::command]
pub async fn git_remotes(root: String) -> Result<Vec<GitRemote>, String> {
    tokio::task::spawn_blocking(move || {
        let out = run_git(&root, &["remote", "-v"])?;
        let mut list: Vec<GitRemote> = Vec::new();
        for line in out.lines() {
            // 形如 "origin\tgit@...(fetch)"，仅取 fetch 行
            if !line.contains("(fetch)") {
                continue;
            }
            let mut it = line.split_whitespace();
            let name = it.next().unwrap_or("").to_string();
            let url = it.next().unwrap_or("").to_string();
            if !name.is_empty() {
                list.push(GitRemote { name, url });
            }
        }
        Ok(list)
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 添加远程。
#[tauri::command]
pub async fn git_remote_add(root: String, name: String, url: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["remote", "add", &name, &url]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 删除远程。
#[tauri::command]
pub async fn git_remote_remove(root: String, name: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["remote", "remove", &name]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 探测进行中的 git 操作：merge / rebase / cherry-pick / revert / none。
#[tauri::command]
pub async fn git_op_state(root: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let git_dir = run_git(&root, &["rev-parse", "--git-dir"])?
            .trim()
            .to_string();
        let p = std::path::Path::new(&git_dir);
        let base = if p.is_absolute() {
            std::path::PathBuf::from(&git_dir)
        } else {
            std::path::Path::new(&root).join(&git_dir)
        };
        let state = if base.join("rebase-merge").exists() || base.join("rebase-apply").exists() {
            "rebase"
        } else if base.join("MERGE_HEAD").exists() {
            "merge"
        } else if base.join("CHERRY_PICK_HEAD").exists() {
            "cherry-pick"
        } else if base.join("REVERT_HEAD").exists() {
            "revert"
        } else {
            "none"
        };
        Ok(state.to_string())
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 校验进行中操作名，避免拼接任意子命令。
fn valid_op(op: &str) -> bool {
    matches!(op, "merge" | "rebase" | "cherry-pick" | "revert")
}

/// 中止进行中的操作。
#[tauri::command]
pub async fn git_op_abort(root: String, op: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        if !valid_op(&op) {
            return Err(format!("未知操作: {}", op));
        }
        run_git(&root, &[op.as_str(), "--abort"])
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 继续进行中的操作（冲突解决后）。用 core.editor=true 避免弹编辑器。
#[tauri::command]
pub async fn git_op_continue(root: String, op: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        if !valid_op(&op) {
            return Err(format!("未知操作: {}", op));
        }
        run_git(
            &root,
            &["-c", "core.editor=true", op.as_str(), "--continue"],
        )
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 跳过当前提交（rebase / cherry-pick / revert，merge 无此操作）。
#[tauri::command]
pub async fn git_op_skip(root: String, op: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        if !matches!(op.as_str(), "rebase" | "cherry-pick" | "revert") {
            return Err(format!("该操作不支持跳过: {}", op));
        }
        run_git(&root, &[op.as_str(), "--skip"])
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

#[derive(Serialize)]
pub struct GitBisectState {
    active: bool,
    /// 当前待测 HEAD 短哈希
    head: String,
    /// 当前待测提交主题
    subject: String,
    /// git bisect 最近输出（剩余步数 / 首个坏提交等）
    message: String,
}

/// 查询二分定位状态：是否进行中、当前待测提交。
#[tauri::command]
pub async fn git_bisect_state(root: String) -> Result<GitBisectState, String> {
    tokio::task::spawn_blocking(move || {
        let git_dir = run_git(&root, &["rev-parse", "--git-dir"])?
            .trim()
            .to_string();
        let p = std::path::Path::new(&git_dir);
        let base = if p.is_absolute() {
            std::path::PathBuf::from(&git_dir)
        } else {
            std::path::Path::new(&root).join(&git_dir)
        };
        let active = base.join("BISECT_START").exists();
        let mut head = String::new();
        let mut subject = String::new();
        if active {
            if let Ok(out) = run_git(&root, &["log", "-1", "--pretty=format:%h\x1f%s", "HEAD"]) {
                let parts: Vec<&str> = out.split('\u{1f}').collect();
                if parts.len() >= 2 {
                    head = parts[0].to_string();
                    subject = parts[1].to_string();
                }
            }
        }
        Ok(GitBisectState {
            active,
            head,
            subject,
            message: String::new(),
        })
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 二分定位子命令：start / good / bad / skip / reset。rev 可选，仅 good/bad 使用。
#[tauri::command]
pub async fn git_bisect(root: String, action: String, rev: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        if !matches!(action.as_str(), "start" | "good" | "bad" | "skip" | "reset") {
            return Err(format!("未知二分操作: {}", action));
        }
        let mut args = vec!["bisect", action.as_str()];
        if matches!(action.as_str(), "good" | "bad") && !rev.trim().is_empty() {
            args.push(rev.as_str());
        }
        run_git(&root, &args)
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

#[derive(Deserialize)]
pub struct RebaseTodo {
    /// pick / squash / fixup / drop
    action: String,
    hash: String,
}

/// 交互式 rebase（非交互执行）：按 todos（须为旧→新顺序）改写 base 之上的提交。
/// 通过 sequence.editor 注入 todo、core.editor=true 接受默认合并信息，避免弹编辑器。
/// 仅在类 Unix 平台支持。
#[tauri::command]
pub async fn git_rebase_interactive(
    root: String,
    base: String,
    todos: Vec<RebaseTodo>,
) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        if !cfg!(unix) {
            return Err("当前平台暂不支持交互式 rebase".to_string());
        }
        if todos.is_empty() {
            return Err("没有可处理的提交".to_string());
        }
        let mut lines = String::new();
        for t in &todos {
            if !matches!(t.action.as_str(), "pick" | "squash" | "fixup" | "drop") {
                return Err(format!("未知 rebase 动作: {}", t.action));
            }
            lines.push_str(&t.action);
            lines.push(' ');
            lines.push_str(&t.hash);
            lines.push('\n');
        }

        let stamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let tmp = std::env::temp_dir().join(format!(
            "codeforge-rebase-{}-{}.txt",
            std::process::id(),
            stamp
        ));
        std::fs::write(&tmp, &lines).map_err(|e| format!("写入 rebase todo 失败: {}", e))?;
        let tmp_str = tmp.to_string_lossy().to_string();

        // git 会把待编辑文件路径追加到该命令后再经 shell 执行，故用 cp 覆盖之
        let seq_editor = format!("sequence.editor=cp '{}'", tmp_str);
        let result = std::process::Command::new("git")
            .args([
                "-C",
                &root,
                "-c",
                seq_editor.as_str(),
                "-c",
                "core.editor=true",
                "rebase",
                "-i",
                base.as_str(),
            ])
            .output();
        let _ = std::fs::remove_file(&tmp);

        let output = result.map_err(|e| format!("执行 git 失败: {}", e))?;
        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            let out = String::from_utf8_lossy(&output.stdout);
            return Err(format!("{}{}", out, err).trim().to_string());
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 拣选某提交到当前分支（cherry-pick）。
#[tauri::command]
pub async fn git_cherry_pick(root: String, hash: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["cherry-pick", &hash]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 把当前分支的上游设为 <remote>/<branch>。
#[tauri::command]
pub async fn git_set_upstream(
    root: String,
    remote: String,
    branch: String,
) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let target = format!("{}/{}", remote, branch);
        run_git(&root, &["branch", &format!("--set-upstream-to={}", target)])
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 列出标签（按创建时间倒序）。
#[tauri::command]
pub async fn git_tags(root: String) -> Result<Vec<String>, String> {
    tokio::task::spawn_blocking(move || {
        let out = run_git(&root, &["tag", "--sort=-creatordate"])?;
        Ok(out
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect())
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 读取仓库本地身份配置 (user.name / user.email)，返回 [name, email]，未设置则为空串。
#[tauri::command]
pub async fn git_get_identity(root: String) -> Result<Vec<String>, String> {
    tokio::task::spawn_blocking(move || {
        let name = run_git(&root, &["config", "--local", "user.name"]).unwrap_or_default();
        let email = run_git(&root, &["config", "--local", "user.email"]).unwrap_or_default();
        Ok(vec![name.trim().to_string(), email.trim().to_string()])
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 设置仓库本地身份配置。传入空串则忽略对应项。
#[tauri::command]
pub async fn git_set_identity(root: String, name: String, email: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        if !name.trim().is_empty() {
            run_git(&root, &["config", "--local", "user.name", name.trim()])?;
        }
        if !email.trim().is_empty() {
            run_git(&root, &["config", "--local", "user.email", email.trim()])?;
        }
        Ok(String::new())
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 读取仓库本地签名配置，返回 [gpgsign("true"/""), signingkey]。
#[tauri::command]
pub async fn git_get_signing(root: String) -> Result<Vec<String>, String> {
    tokio::task::spawn_blocking(move || {
        let sign = run_git(&root, &["config", "--local", "commit.gpgsign"])
            .unwrap_or_default()
            .trim()
            .to_string();
        let key = run_git(&root, &["config", "--local", "user.signingkey"])
            .unwrap_or_default()
            .trim()
            .to_string();
        let enabled = if sign == "true" {
            "true".to_string()
        } else {
            String::new()
        };
        Ok(vec![enabled, key])
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 设置仓库本地签名配置：commit.gpgsign 开关；key 非空则设 user.signingkey。
#[tauri::command]
pub async fn git_set_signing(root: String, enabled: bool, key: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        run_git(
            &root,
            &[
                "config",
                "--local",
                "commit.gpgsign",
                if enabled { "true" } else { "false" },
            ],
        )?;
        if !key.trim().is_empty() {
            run_git(&root, &["config", "--local", "user.signingkey", key.trim()])?;
        }
        Ok(String::new())
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 常见客户端 git 钩子名（用于校验，避免路径穿越）。
const GIT_HOOK_NAMES: [&str; 13] = [
    "applypatch-msg",
    "pre-applypatch",
    "post-applypatch",
    "pre-commit",
    "pre-merge-commit",
    "prepare-commit-msg",
    "commit-msg",
    "post-commit",
    "pre-rebase",
    "post-checkout",
    "post-merge",
    "pre-push",
    "post-rewrite",
];

fn git_hooks_dir(root: &str) -> Result<std::path::PathBuf, String> {
    let git_dir = run_git(root, &["rev-parse", "--git-dir"])?
        .trim()
        .to_string();
    let p = std::path::Path::new(&git_dir);
    let base = if p.is_absolute() {
        std::path::PathBuf::from(&git_dir)
    } else {
        std::path::Path::new(root).join(&git_dir)
    };
    Ok(base.join("hooks"))
}

#[cfg(unix)]
fn is_executable(path: &std::path::Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path)
        .map(|m| m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn is_executable(path: &std::path::Path) -> bool {
    path.exists()
}

#[derive(Serialize)]
pub struct GitHook {
    name: String,
    /// 同名钩子文件已存在
    active: bool,
    /// 具备可执行权限（类 Unix）
    executable: bool,
}

/// 列出常见客户端钩子及其状态。
#[tauri::command]
pub async fn git_hooks(root: String) -> Result<Vec<GitHook>, String> {
    tokio::task::spawn_blocking(move || {
        let dir = git_hooks_dir(&root)?;
        let mut list = Vec::new();
        for name in GIT_HOOK_NAMES {
            let path = dir.join(name);
            let active = path.is_file();
            list.push(GitHook {
                name: name.to_string(),
                active,
                executable: active && is_executable(&path),
            });
        }
        Ok(list)
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 读取某钩子内容（不存在返回空串）。
#[tauri::command]
pub async fn git_hook_read(root: String, name: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        if !GIT_HOOK_NAMES.contains(&name.as_str()) {
            return Err(format!("未知钩子: {}", name));
        }
        let path = git_hooks_dir(&root)?.join(&name);
        if path.is_file() {
            std::fs::read_to_string(&path).map_err(|e| format!("读取钩子失败: {}", e))
        } else {
            Ok(String::new())
        }
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 写入某钩子内容并按 executable 设置可执行权限（类 Unix）。
/// executable 仅在类 Unix 平台使用；非 Unix 下显式 allow，避免 clippy -D warnings 失败
/// （参数名需保持不变以匹配前端 Tauri 调用，故不能改名为 _executable）。
#[tauri::command]
#[cfg_attr(not(unix), allow(unused_variables))]
pub async fn git_hook_save(
    root: String,
    name: String,
    content: String,
    executable: bool,
) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        if !GIT_HOOK_NAMES.contains(&name.as_str()) {
            return Err(format!("未知钩子: {}", name));
        }
        let dir = git_hooks_dir(&root)?;
        std::fs::create_dir_all(&dir).map_err(|e| format!("创建 hooks 目录失败: {}", e))?;
        let path = dir.join(&name);
        std::fs::write(&path, &content).map_err(|e| format!("写入钩子失败: {}", e))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = if executable { 0o755 } else { 0o644 };
            let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(mode));
        }
        Ok(String::new())
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 删除某钩子文件。
#[tauri::command]
pub async fn git_hook_delete(root: String, name: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        if !GIT_HOOK_NAMES.contains(&name.as_str()) {
            return Err(format!("未知钩子: {}", name));
        }
        let path = git_hooks_dir(&root)?.join(&name);
        if path.is_file() {
            std::fs::remove_file(&path).map_err(|e| format!("删除钩子失败: {}", e))?;
        }
        Ok(String::new())
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 创建标签。hash 为空则打在 HEAD；message 非空则创建附注标签（-a -m）。
#[tauri::command]
pub async fn git_tag_create(
    root: String,
    name: String,
    hash: String,
    message: String,
) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let mut args: Vec<&str> = vec!["tag"];
        if !message.trim().is_empty() {
            args.push("-a");
            args.push(name.as_str());
            args.push("-m");
            args.push(message.as_str());
        } else {
            args.push(name.as_str());
        }
        if !hash.trim().is_empty() {
            args.push(hash.as_str());
        }
        run_git(&root, &args)
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 删除标签。
#[tauri::command]
pub async fn git_tag_delete(root: String, name: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["tag", "-d", &name]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 还原某次提交（生成一条反向提交，历史保留）。
#[tauri::command]
pub async fn git_revert(root: String, hash: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || run_git(&root, &["revert", "--no-edit", &hash]))
        .await
        .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 重置到某提交。mode 为 soft / mixed / hard，默认 mixed。
#[tauri::command]
pub async fn git_reset(root: String, hash: String, mode: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let flag = match mode.as_str() {
            "soft" => "--soft",
            "hard" => "--hard",
            _ => "--mixed",
        };
        run_git(&root, &["reset", flag, &hash])
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

#[derive(Serialize)]
pub struct GitCompareResult {
    /// head 相对 base 领先 / 落后的提交数
    ahead: u32,
    behind: u32,
    /// base...head 的差异补丁（含 stat）
    patch: String,
}

/// 对比两个 ref：base...head 的领先/落后提交数与差异补丁。
#[tauri::command]
pub async fn git_compare(
    root: String,
    base: String,
    head: String,
) -> Result<GitCompareResult, String> {
    tokio::task::spawn_blocking(move || {
        let range = format!("{}...{}", base, head);
        // rev-list --left-right --count 输出 "左 右"：左=base 独有(落后)，右=head 独有(领先)
        let counts = run_git(&root, &["rev-list", "--left-right", "--count", &range])?;
        let mut it = counts.split_whitespace();
        let behind: u32 = it.next().unwrap_or("0").parse().unwrap_or(0);
        let ahead: u32 = it.next().unwrap_or("0").parse().unwrap_or(0);
        let mut patch = run_git(&root, &["diff", "--stat", "-p", &range])?;
        const MAX: usize = 200_000;
        if patch.len() > MAX {
            patch.truncate(MAX);
            patch.push_str("\n…(内容过长已截断)");
        }
        Ok(GitCompareResult {
            ahead,
            behind,
            patch,
        })
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 某个文件的提交历史（分页）。
#[tauri::command]
pub async fn git_log_file(
    root: String,
    rel_path: String,
    limit: u32,
    skip: u32,
) -> Result<Vec<GitCommit>, String> {
    tokio::task::spawn_blocking(move || {
        let n = format!("-n{}", limit);
        let sk = format!("--skip={}", skip);
        let args = vec![
            "log",
            n.as_str(),
            sk.as_str(),
            "--date=format:%Y-%m-%d %H:%M",
            "--pretty=format:%H\x1f%h\x1f%an\x1f%ad\x1f%s",
            "--follow",
            "--",
            rel_path.as_str(),
        ];
        let out = run_git(&root, &args)?;
        let mut commits = Vec::new();
        for line in out.lines() {
            let p: Vec<&str> = line.split('\u{1f}').collect();
            if p.len() >= 5 {
                commits.push(GitCommit {
                    hash: p[0].to_string(),
                    short: p[1].to_string(),
                    author: p[2].to_string(),
                    date: p[3].to_string(),
                    subject: p[4].to_string(),
                });
            }
        }
        Ok(commits)
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

#[derive(Serialize)]
pub struct GitReflogEntry {
    short: String,
    selector: String,
    subject: String,
    date: String,
}

/// HEAD 引用日志（reflog），用于误操作恢复。
#[tauri::command]
pub async fn git_reflog(root: String, limit: u32) -> Result<Vec<GitReflogEntry>, String> {
    tokio::task::spawn_blocking(move || {
        let n = format!("-n{}", limit);
        let args = vec![
            "reflog",
            n.as_str(),
            "--date=format:%Y-%m-%d %H:%M",
            "--pretty=format:%h\x1f%gd\x1f%gs\x1f%ad",
        ];
        let out = run_git(&root, &args)?;
        let mut entries = Vec::new();
        for line in out.lines() {
            let p: Vec<&str> = line.split('\u{1f}').collect();
            if p.len() >= 4 {
                entries.push(GitReflogEntry {
                    short: p[0].to_string(),
                    selector: p[1].to_string(),
                    subject: p[2].to_string(),
                    date: p[3].to_string(),
                });
            }
        }
        Ok(entries)
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// 某次提交的详情补丁（git show，含 stat 与 diff）。
#[tauri::command]
pub async fn git_show(root: String, hash: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let mut out = run_git(&root, &["show", "--stat", "-p", &hash])?;
        // 超大提交截断，避免渲染卡顿
        const MAX_SHOW_LEN: usize = 200_000;
        if out.len() > MAX_SHOW_LEN {
            out.truncate(MAX_SHOW_LEN);
            out.push_str("\n…(内容过长已截断)");
        }
        Ok(out)
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

#[derive(Serialize)]
pub struct GitBlameLine {
    short: String,
    author: String,
    date: String,
    content: String,
}

/// 对某文件做 git blame，返回逐行的提交短哈希/作者/日期/内容。
#[tauri::command]
pub async fn git_blame(root: String, rel_path: String) -> Result<Vec<GitBlameLine>, String> {
    tokio::task::spawn_blocking(move || {
        let out = run_git(&root, &["blame", "--line-porcelain", "--", &rel_path])?;
        let mut lines = Vec::new();
        let mut hash = String::new();
        let mut author = String::new();
        let mut date = String::new();
        for raw in out.lines() {
            if let Some(content) = raw.strip_prefix('\t') {
                // 一行内容收尾：推入当前累积的提交信息
                lines.push(GitBlameLine {
                    short: hash.chars().take(8).collect(),
                    author: author.clone(),
                    date: date.clone(),
                    content: content.to_string(),
                });
            } else if let Some(a) = raw.strip_prefix("author ") {
                author = a.to_string();
            } else if let Some(ts) = raw.strip_prefix("author-time ") {
                // epoch 秒 → 仅取日期
                if let Ok(secs) = ts.trim().parse::<i64>() {
                    date = format_epoch_date(secs);
                }
            } else if raw.len() >= 40 && raw.as_bytes()[0].is_ascii_hexdigit() {
                // 形如 "<40hash> <orig> <final> [<n>]"，取首个 token 作哈希
                hash = raw.split(' ').next().unwrap_or("").to_string();
            }
        }
        Ok(lines)
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))?
}

/// epoch 秒转 YYYY-MM-DD（UTC，无需第三方库）。
fn format_epoch_date(secs: i64) -> String {
    let days = secs.div_euclid(86400);
    // 1970-01-01 起的天数转公历日期
    let mut y = 1970i64;
    let mut d = days;
    loop {
        let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
        let dy = if leap { 366 } else { 365 };
        if d >= dy {
            d -= dy;
            y += 1;
        } else {
            break;
        }
    }
    let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    let mdays = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut m = 0usize;
    while m < 12 && d >= mdays[m] {
        d -= mdays[m];
        m += 1;
    }
    format!("{:04}-{:02}-{:02}", y, m + 1, d + 1)
}

#[derive(Serialize)]
pub struct GitHeadFile {
    /// 该文件是否存在于 HEAD（不存在则为新增/未跟踪文件）
    exists: bool,
    content: String,
}

/// 获取某文件在 HEAD 中的内容（用于编辑器行内差异标记）。
/// rel_path 为相对仓库根的路径。
#[tauri::command]
pub async fn git_file_head(root: String, rel_path: String) -> Result<GitHeadFile, String> {
    tokio::task::spawn_blocking(move || {
        let spec = format!("HEAD:{}", rel_path);
        match run_git(&root, &["show", &spec]) {
            Ok(content) => GitHeadFile {
                exists: true,
                content,
            },
            // 文件不在 HEAD 中（新增/未跟踪）：返回不存在
            Err(_) => GitHeadFile {
                exists: false,
                content: String::new(),
            },
        }
    })
    .await
    .map_err(|e| format!("git 任务失败: {}", e))
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
/// 首次会全量扫描建索引，放到阻塞线程池，避免阻塞主线程。
#[tauri::command]
pub async fn get_text_file_meta(path: String) -> Result<TextFileMeta, String> {
    tokio::task::spawn_blocking(move || {
        with_index(&path, |idx| TextFileMeta {
            size_bytes: idx.size,
            line_count: idx.line_count,
            is_text: idx.is_text,
        })
    })
    .await
    .map_err(|e| format!("读取文件信息任务失败: {}", e))?
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
