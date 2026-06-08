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

/// 可在设置中一键安装的语言服务器清单（用于检测与安装）
/// (id, 展示名, 用于检测的可执行名, 安装命令)
fn server_defs() -> Vec<(&'static str, &'static str, &'static str, &'static str)> {
    vec![
        ("python", "Python (pyright)", "pyright-langserver", "npm i -g pyright"),
        (
            "typescript",
            "TypeScript / JavaScript",
            "typescript-language-server",
            "npm i -g typescript-language-server typescript",
        ),
        ("rust", "Rust (rust-analyzer)", "rust-analyzer", "rustup component add rust-analyzer"),
        ("go", "Go (gopls)", "gopls", "go install golang.org/x/tools/gopls@latest"),
        ("clangd", "C / C++ (clangd)", "clangd", "brew install llvm"),
        ("lua", "Lua", "lua-language-server", "brew install lua-language-server"),
        ("php", "PHP (intelephense)", "intelephense", "npm i -g intelephense"),
        ("ruby", "Ruby (solargraph)", "solargraph", "gem install solargraph"),
        (
            "web",
            "HTML / CSS / JSON",
            "vscode-html-language-server",
            "npm i -g vscode-langservers-extracted",
        ),
    ]
}

#[derive(Serialize)]
pub struct LspServerInfo {
    id: String,
    label: String,
    program: String,
    installed: bool,
    install: String,
}

/// 列出可安装的语言服务器及其安装状态
#[tauri::command]
pub fn lsp_server_list() -> Vec<LspServerInfo> {
    server_defs()
        .into_iter()
        .map(|(id, label, program, install)| LspServerInfo {
            id: id.to_string(),
            label: label.to_string(),
            program: program.to_string(),
            installed: find_in_path(program).is_some(),
            install: install.to_string(),
        })
        .collect()
}

/// 一键安装某语言服务器：执行其安装命令并实时输出日志（事件 lsp:install / lsp:install-done）
#[tauri::command]
pub fn lsp_install(app: AppHandle, id: String) -> Result<(), String> {
    let def = server_defs()
        .into_iter()
        .find(|(d, ..)| *d == id)
        .ok_or_else(|| "未知的语言服务器".to_string())?;
    let cmd_str = def.3.to_string();

    let (shell, flag) = if cfg!(windows) { ("cmd", "/C") } else { ("sh", "-c") };
    let mut child = Command::new(shell)
        .arg(flag)
        .arg(&cmd_str)
        .env("PATH", augmented_path())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("无法执行安装命令: {}", e))?;

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let id_done = id.clone();
    let app_done = app.clone();

    let emit_lines = |app: AppHandle, id: String, reader: Option<Box<dyn Read + Send>>| {
        if let Some(r) = reader {
            std::thread::spawn(move || {
                let mut buf = BufReader::new(r);
                loop {
                    match read_raw_line(&mut buf) {
                        Some(line) => {
                            let _ = app.emit("lsp:install", (id.clone(), line));
                        }
                        None => break,
                    }
                }
            });
        }
    };
    emit_lines(app.clone(), id.clone(), stdout.map(|s| Box::new(s) as Box<dyn Read + Send>));
    emit_lines(app.clone(), id.clone(), stderr.map(|s| Box::new(s) as Box<dyn Read + Send>));

    std::thread::spawn(move || {
        let success = child.wait().map(|s| s.success()).unwrap_or(false);
        let _ = app_done.emit("lsp:install-done", (id_done, success));
    });
    Ok(())
}

/// 读取一行普通文本（以 \n 结尾，用于安装日志）
fn read_raw_line<R: Read>(reader: &mut BufReader<R>) -> Option<String> {
    let mut buf = Vec::new();
    let mut byte = [0u8; 1];
    loop {
        match reader.read(&mut byte) {
            Ok(0) => {
                if buf.is_empty() {
                    return None;
                }
                return Some(String::from_utf8_lossy(&buf).to_string());
            }
            Ok(_) => {
                if byte[0] == b'\n' {
                    return Some(String::from_utf8_lossy(&buf).to_string());
                }
                if byte[0] != b'\r' {
                    buf.push(byte[0]);
                }
            }
            Err(_) => return None,
        }
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
