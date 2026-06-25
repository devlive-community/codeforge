//! DAP 桥接：按语言拉起调试适配器进程，转发 DAP 消息（与 LSP 同为 Content-Length 帧）。
//! 后端只做透明转发：前端发送/接收原始 JSON 字符串，握手与协议由前端的 DAP 客户端负责。
//! 复用 lsp.rs 的 find_in_path / augmented_path / read_message。

use crate::lsp::{augmented_path, find_in_path, read_message, read_raw_line};
use serde::Serialize;
use std::collections::HashMap;
use std::io::{BufReader, Read, Write};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex as StdMutex;
use tauri::{AppHandle, Emitter, State};

struct Adapter {
    child: Child,
    // 写入走独立线程，避免 stdin 写阻塞主线程
    tx: std::sync::mpsc::Sender<Vec<u8>>,
}

pub struct DapState {
    // 以会话 key 区分（前端通常传语言名，单语言单会话）
    adapters: StdMutex<HashMap<String, Adapter>>,
}

impl DapState {
    pub fn new() -> Self {
        Self {
            adapters: StdMutex::new(HashMap::new()),
        }
    }
}

#[derive(Clone, Serialize)]
struct DapBatch {
    session: String,
    messages: Vec<String>,
}

/// 语言 -> (适配器可执行名, 启动参数)。适配器仅负责说 DAP；
/// 具体调试目标(program/args/cwd)由前端在 launch 请求中给出。新增语言在此加一行。
fn adapter_cmd(language: &str) -> Option<(&'static str, Vec<&'static str>)> {
    match language {
        // debugpy：python -m debugpy.adapter（需 pip install debugpy）
        "python" | "python3" | "python2" => Some(("python3", vec!["-m", "debugpy.adapter"])),
        // delve：dlv dap
        "go" => Some(("dlv", vec!["dap"])),
        // lldb-dap（LLVM 自带）：适用于 Rust / C / C++
        "rust" | "c" | "cpp" => Some(("lldb-dap", vec![])),
        _ => None,
    }
}

/// 该语言是否有可用的调试适配器。Python 进一步校验 debugpy 模块是否可导入。
#[tauri::command]
pub fn dap_available(language: String) -> bool {
    let Some((prog, _)) = adapter_cmd(&language) else {
        return false;
    };
    let Some(exe) = find_in_path(prog) else {
        return false;
    };
    if matches!(language.as_str(), "python" | "python3" | "python2") {
        return Command::new(&exe)
            .args(["-c", "import debugpy"])
            .env("PATH", augmented_path())
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
    }
    true
}

/// 启动调试适配器；已启动则直接返回 true。session 作为多会话区分键（前端传语言名即可）。
#[tauri::command]
pub fn dap_start(
    app: AppHandle,
    state: State<'_, DapState>,
    session: String,
    language: String,
) -> Result<bool, String> {
    {
        let adapters = state.adapters.lock().map_err(|e| e.to_string())?;
        if adapters.contains_key(&session) {
            return Ok(true);
        }
    }
    let (prog, args) = adapter_cmd(&language).ok_or_else(|| "该语言暂不支持调试".to_string())?;
    let exe = find_in_path(prog)
        .ok_or_else(|| format!("未找到调试适配器：{}（请先安装并确保在 PATH 中）", prog))?;

    let mut cmd = Command::new(&exe);
    cmd.args(&args)
        .env("PATH", augmented_path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd
        .spawn()
        .map_err(|e| format!("启动 {} 失败: {}", prog, e))?;
    let stdin = child.stdin.take().ok_or("无法获取 stdin")?;
    let stdout = child.stdout.take().ok_or("无法获取 stdout")?;
    let stderr = child.stderr.take();

    // 读取线程：解析 Content-Length 帧，丢进 channel
    let (msg_tx, msg_rx) = std::sync::mpsc::channel::<String>();
    std::thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        while let Some(body) = read_message(&mut reader) {
            if msg_tx.send(body).is_err() {
                break;
            }
        }
    });

    // 发射线程：合批转发（output/variables 等可能突发），会话结束发 dap:exit
    let app_reader = app.clone();
    let sess_emit = session.clone();
    std::thread::spawn(move || {
        while let Ok(first) = msg_rx.recv() {
            let mut batch = vec![first];
            while let Ok(m) = msg_rx.try_recv() {
                batch.push(m);
                if batch.len() >= 256 {
                    break;
                }
            }
            let _ = app_reader.emit(
                "dap:messages",
                DapBatch {
                    session: sess_emit.clone(),
                    messages: batch,
                },
            );
        }
        let _ = app_reader.emit("dap:exit", sess_emit.clone());
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

    // 写入线程：独占 stdin，从 channel 取帧写入
    let (tx, rx) = std::sync::mpsc::channel::<Vec<u8>>();
    std::thread::spawn(move || {
        let mut stdin = stdin;
        while let Ok(frame) = rx.recv() {
            if stdin.write_all(&frame).is_err() {
                break;
            }
            if stdin.flush().is_err() {
                break;
            }
        }
    });

    state
        .adapters
        .lock()
        .map_err(|e| e.to_string())?
        .insert(session, Adapter { child, tx });
    Ok(true)
}

/// 向调试适配器发送一条 DAP 消息（已是完整 JSON 字符串）
#[tauri::command]
pub fn dap_send(
    state: State<'_, DapState>,
    session: String,
    message: String,
) -> Result<(), String> {
    let adapters = state.adapters.lock().map_err(|e| e.to_string())?;
    let adapter = adapters
        .get(&session)
        .ok_or_else(|| "调试会话未启动".to_string())?;
    let frame = format!("Content-Length: {}\r\n\r\n{}", message.len(), message);
    adapter
        .tx
        .send(frame.into_bytes())
        .map_err(|_| "调试适配器写入通道已关闭".to_string())?;
    Ok(())
}

/// 停止调试会话
#[tauri::command]
pub fn dap_stop(state: State<'_, DapState>, session: String) -> Result<(), String> {
    if let Some(mut adapter) = state
        .adapters
        .lock()
        .map_err(|e| e.to_string())?
        .remove(&session)
    {
        let _ = adapter.child.kill();
    }
    Ok(())
}

/// 为调试构建编译型语言，返回可执行文件路径。
/// Rust: cargo build → target/debug/<package>；C/C++: 用 cc/c++ -g 编译到临时文件。
#[tauri::command]
pub async fn dap_build(
    language: String,
    file_path: String,
    root: String,
) -> Result<String, String> {
    tokio::task::spawn_blocking(move || match language.as_str() {
        "rust" => build_rust(&root),
        "c" => build_cc("cc", &file_path),
        "cpp" => build_cc("c++", &file_path),
        _ => Err(format!("{} 暂不支持编译调试", language)),
    })
    .await
    .map_err(|e| format!("构建任务失败: {}", e))?
}

fn build_rust(root: &str) -> Result<String, String> {
    let exe =
        find_in_path("cargo").ok_or_else(|| "未找到 cargo（请安装 Rust 工具链）".to_string())?;
    let out = Command::new(&exe)
        .args(["build"])
        .current_dir(root)
        .env("PATH", augmented_path())
        .output()
        .map_err(|e| format!("执行 cargo 失败: {}", e))?;
    if !out.status.success() {
        return Err(format!(
            "cargo build 失败:\n{}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    // 解析 Cargo.toml 的 package name
    let toml = std::fs::read_to_string(std::path::Path::new(root).join("Cargo.toml"))
        .map_err(|e| format!("读取 Cargo.toml 失败: {}", e))?;
    let mut name = String::new();
    for line in toml.lines() {
        let l = line.trim();
        if let Some(rest) = l.strip_prefix("name") {
            if let Some(v) = rest.trim_start_matches(['=', ' ']).strip_prefix('"') {
                if let Some(end) = v.find('"') {
                    name = v[..end].to_string();
                    break;
                }
            }
        }
    }
    if name.is_empty() {
        return Err("无法从 Cargo.toml 解析 package name".to_string());
    }
    let mut bin = std::path::Path::new(root).join("target/debug").join(&name);
    if cfg!(windows) {
        bin.set_extension("exe");
    }
    if !bin.is_file() {
        return Err(format!("未找到可执行文件: {}", bin.display()));
    }
    Ok(bin.to_string_lossy().to_string())
}

fn build_cc(compiler: &str, file_path: &str) -> Result<String, String> {
    let exe =
        find_in_path(compiler).ok_or_else(|| format!("未找到编译器 {}（请安装）", compiler))?;
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let mut out_path = std::env::temp_dir().join(format!("codeforge-dbg-{}", stamp));
    if cfg!(windows) {
        out_path.set_extension("exe");
    }
    let out_str = out_path.to_string_lossy().to_string();
    let result = Command::new(&exe)
        .args(["-g", "-O0", "-o", &out_str, file_path])
        .env("PATH", augmented_path())
        .output()
        .map_err(|e| format!("执行 {} 失败: {}", compiler, e))?;
    if !result.status.success() {
        return Err(format!(
            "编译失败:\n{}",
            String::from_utf8_lossy(&result.stderr)
        ));
    }
    Ok(out_str)
}

// 可安装的调试适配器：(id, 展示名, 安装命令)
fn adapter_defs() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "debugpy",
            "Python (debugpy)",
            "python3 -m pip install debugpy",
        ),
        (
            "delve",
            "Go (delve)",
            "go install github.com/go-delve/delve/cmd/dlv@latest",
        ),
        (
            "lldb-dap",
            "Rust / C / C++ (lldb-dap，随 LLVM 提供)",
            "brew install llvm",
        ),
    ]
}

fn adapter_installed(id: &str) -> bool {
    match id {
        // debugpy 校验模块可导入
        "debugpy" => dap_available("python".to_string()),
        "delve" => find_in_path("dlv").is_some(),
        // lldb-dap 随 LLVM 提供，brew 的 llvm 是 keg-only 不在 PATH，额外探测常见路径
        "lldb-dap" => {
            find_in_path("lldb-dap").is_some()
                || std::path::Path::new("/opt/homebrew/opt/llvm/bin/lldb-dap").is_file()
                || std::path::Path::new("/usr/local/opt/llvm/bin/lldb-dap").is_file()
        }
        _ => false,
    }
}

#[derive(Serialize)]
pub struct DapAdapterInfo {
    id: String,
    label: String,
    installed: bool,
    install: String,
}

/// 列出可安装的调试适配器及其安装状态
#[tauri::command]
pub fn dap_adapter_list() -> Vec<DapAdapterInfo> {
    adapter_defs()
        .into_iter()
        .map(|(id, label, install)| DapAdapterInfo {
            id: id.to_string(),
            label: label.to_string(),
            installed: adapter_installed(id),
            install: install.to_string(),
        })
        .collect()
}

/// 一键安装某调试适配器：执行安装命令并实时输出日志（事件 dap:install / dap:install-done）
#[tauri::command]
pub fn dap_install(app: AppHandle, id: String) -> Result<(), String> {
    let def = adapter_defs()
        .into_iter()
        .find(|(d, ..)| *d == id)
        .ok_or_else(|| "未知的调试适配器".to_string())?;
    let cmd_str = def.2.to_string();

    let (shell, flag) = if cfg!(windows) {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };
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
                while let Some(line) = read_raw_line(&mut buf) {
                    let _ = app.emit("dap:install", (id.clone(), line));
                }
            });
        }
    };
    emit_lines(
        app.clone(),
        id.clone(),
        stdout.map(|s| Box::new(s) as Box<dyn Read + Send>),
    );
    emit_lines(
        app.clone(),
        id.clone(),
        stderr.map(|s| Box::new(s) as Box<dyn Read + Send>),
    );

    std::thread::spawn(move || {
        let success = child.wait().map(|s| s.success()).unwrap_or(false);
        let _ = app_done.emit("dap:install-done", (id_done, success));
    });
    Ok(())
}
