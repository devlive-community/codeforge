//! LSP 桥接：按语言拉起语言服务器进程，转发 JSON-RPC（Content-Length 帧）。
//! 后端只做透明转发：前端发送/接收原始 JSON 字符串，握手与协议由前端负责。

use serde::Serialize;
use std::collections::HashMap;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::Mutex as StdMutex;
use tauri::{AppHandle, Emitter, State};

struct Server {
    child: Child,
    stdin: ChildStdin,
}

pub struct LspState {
    servers: StdMutex<HashMap<String, Server>>,
}

impl LspState {
    pub fn new() -> Self {
        Self {
            servers: StdMutex::new(HashMap::new()),
        }
    }
}

#[derive(Clone, Serialize)]
struct LspEvent {
    language: String,
    message: String,
}

/// 语言 -> (可执行名, 参数)。新增语言在此加一行。
fn server_cmd(language: &str) -> Option<(&'static str, Vec<&'static str>)> {
    match language {
        "python3" | "python2" | "python" => Some(("pyright-langserver", vec!["--stdio"])),
        "typescript" | "typescript-nodejs" | "typescript-browser" | "javascript-nodejs"
        | "javascript-browser" | "javascript-jquery" | "nodejs" => {
            Some(("typescript-language-server", vec!["--stdio"]))
        }
        "rust" => Some(("rust-analyzer", vec![])),
        "go" => Some(("gopls", vec![])),
        "c" | "cpp" | "objective-c" | "objective-cpp" => Some(("clangd", vec![])),
        "lua" => Some(("lua-language-server", vec![])),
        "php" => Some(("intelephense", vec!["--stdio"])),
        "ruby" => Some(("solargraph", vec!["stdio"])),
        "html" => Some(("vscode-html-language-server", vec!["--stdio"])),
        "css" => Some(("vscode-css-language-server", vec!["--stdio"])),
        "json" => Some(("vscode-json-language-server", vec!["--stdio"])),
        _ => None,
    }
}

/// GUI 应用 PATH 常缺失，补充常见安装目录
fn extra_bin_dirs() -> Vec<PathBuf> {
    let mut dirs = vec![
        PathBuf::from("/usr/local/bin"),
        PathBuf::from("/opt/homebrew/bin"),
        PathBuf::from("/usr/bin"),
    ];
    if let Some(home) = dirs_home() {
        dirs.push(home.join(".cargo/bin"));
        dirs.push(home.join(".local/bin"));
        dirs.push(home.join("go/bin"));
        dirs.push(home.join(".npm-global/bin"));
    }
    dirs
}

fn dirs_home() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

/// 在 PATH 与常见目录中查找可执行文件全路径
fn find_in_path(prog: &str) -> Option<PathBuf> {
    let exts: Vec<&str> = if cfg!(windows) {
        vec!["", ".cmd", ".exe", ".bat"]
    } else {
        vec![""]
    };
    let mut dirs: Vec<PathBuf> = Vec::new();
    if let Some(path) = std::env::var_os("PATH") {
        dirs.extend(std::env::split_paths(&path));
    }
    dirs.extend(extra_bin_dirs());
    for dir in dirs {
        for ext in &exts {
            let full = dir.join(format!("{}{}", prog, ext));
            if full.is_file() {
                return Some(full);
            }
        }
    }
    None
}

/// 给子进程增广 PATH（语言服务器常依赖 node 等）
fn augmented_path() -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(path) = std::env::var_os("PATH") {
        parts.push(path.to_string_lossy().to_string());
    }
    for d in extra_bin_dirs() {
        parts.push(d.to_string_lossy().to_string());
    }
    parts.join(":")
}

/// 该语言是否有可用的语言服务器
#[tauri::command]
pub fn lsp_available(language: String) -> bool {
    server_cmd(&language)
        .map(|(prog, _)| find_in_path(prog).is_some())
        .unwrap_or(false)
}

/// 启动语言服务器；已启动则直接返回 true。
#[tauri::command]
pub fn lsp_start(app: AppHandle, state: State<'_, LspState>, language: String) -> Result<bool, String> {
    {
        let servers = state.servers.lock().map_err(|e| e.to_string())?;
        if servers.contains_key(&language) {
            return Ok(true);
        }
    }
    let (prog, args) = server_cmd(&language).ok_or_else(|| "该语言暂不支持 LSP".to_string())?;
    let exe = find_in_path(prog).ok_or_else(|| format!("未找到语言服务器：{}（请先安装并确保在 PATH 中）", prog))?;

    let mut cmd = Command::new(&exe);
    cmd.args(&args)
        .env("PATH", augmented_path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn().map_err(|e| format!("启动 {} 失败: {}", prog, e))?;
    let stdin = child.stdin.take().ok_or("无法获取 stdin")?;
    let stdout = child.stdout.take().ok_or("无法获取 stdout")?;
    let stderr = child.stderr.take();

    // 读取线程：解析 Content-Length 帧，原样转发 JSON 给前端
    let app_reader = app.clone();
    let lang_reader = language.clone();
    std::thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        loop {
            match read_message(&mut reader) {
                Some(body) => {
                    let _ = app_reader.emit(
                        "lsp:message",
                        LspEvent {
                            language: lang_reader.clone(),
                            message: body,
                        },
                    );
                }
                None => break,
            }
        }
        let _ = app_reader.emit("lsp:exit", lang_reader.clone());
    });

    // 排空 stderr，避免阻塞
    if let Some(mut err) = stderr {
        std::thread::spawn(move || {
            let mut buf = [0u8; 4096];
            while let Ok(n) = err.read(&mut buf) {
                if n == 0 {
                    break;
                }
            }
        });
    }

    state
        .servers
        .lock()
        .map_err(|e| e.to_string())?
        .insert(language, Server { child, stdin });
    Ok(true)
}

/// 向语言服务器发送一条 JSON-RPC（已是完整 JSON 字符串）
#[tauri::command]
pub fn lsp_send(state: State<'_, LspState>, language: String, message: String) -> Result<(), String> {
    let mut servers = state.servers.lock().map_err(|e| e.to_string())?;
    let server = servers
        .get_mut(&language)
        .ok_or_else(|| "语言服务器未启动".to_string())?;
    let frame = format!("Content-Length: {}\r\n\r\n{}", message.as_bytes().len(), message);
    server
        .stdin
        .write_all(frame.as_bytes())
        .map_err(|e| e.to_string())?;
    server.stdin.flush().map_err(|e| e.to_string())?;
    Ok(())
}

/// 停止语言服务器
#[tauri::command]
pub fn lsp_stop(state: State<'_, LspState>, language: String) -> Result<(), String> {
    if let Some(mut server) = state
        .servers
        .lock()
        .map_err(|e| e.to_string())?
        .remove(&language)
    {
        let _ = server.child.kill();
    }
    Ok(())
}

/// 读取一条 LSP 消息（Content-Length 帧）；EOF 返回 None
fn read_message<R: Read>(reader: &mut BufReader<R>) -> Option<String> {
    let mut content_length: usize = 0;
    // 逐字节读 header 行直到空行
    loop {
        let line = read_line(reader)?;
        if line.is_empty() {
            break;
        }
        if let Some(rest) = line.strip_prefix("Content-Length:") {
            content_length = rest.trim().parse().unwrap_or(0);
        }
    }
    if content_length == 0 {
        return Some(String::new());
    }
    let mut body = vec![0u8; content_length];
    reader.read_exact(&mut body).ok()?;
    Some(String::from_utf8_lossy(&body).to_string())
}

/// 读取一行（以 \r\n 结尾），返回不含结尾的内容；EOF 返回 None
fn read_line<R: Read>(reader: &mut BufReader<R>) -> Option<String> {
    let mut buf = Vec::new();
    let mut byte = [0u8; 1];
    loop {
        match reader.read(&mut byte) {
            Ok(0) => return None,
            Ok(_) => {
                if byte[0] == b'\n' {
                    if buf.last() == Some(&b'\r') {
                        buf.pop();
                    }
                    return Some(String::from_utf8_lossy(&buf).to_string());
                }
                buf.push(byte[0]);
            }
            Err(_) => return None,
        }
    }
}
