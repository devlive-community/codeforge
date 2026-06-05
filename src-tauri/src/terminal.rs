use portable_pty::{CommandBuilder, MasterPty, PtySize, native_pty_system};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::Mutex as StdMutex;
use tauri::{AppHandle, Emitter, State};

/// 单个终端会话：保留 master（用于改尺寸）与 writer（用于写入）
struct TermSession {
    master: Box<dyn MasterPty + Send>,
    writer: Box<dyn Write + Send>,
}

#[derive(Default)]
pub struct TerminalState {
    sessions: StdMutex<HashMap<String, TermSession>>,
}

impl TerminalState {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, serde::Serialize)]
struct TerminalOutput {
    id: String,
    data: Vec<u8>,
}

/// 创建一个终端会话：启动 shell，并在后台线程把输出通过事件推给前端。
#[tauri::command]
pub fn terminal_create(
    id: String,
    cwd: Option<String>,
    cols: u16,
    rows: u16,
    app: AppHandle,
    state: State<'_, TerminalState>,
) -> Result<(), String> {
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| format!("打开 PTY 失败: {}", e))?;

    // 选择 shell：优先 $SHELL，其次按平台默认
    let shell = std::env::var("SHELL").unwrap_or_else(|_| {
        if cfg!(target_os = "windows") {
            "powershell.exe".to_string()
        } else {
            "/bin/bash".to_string()
        }
    });
    let mut cmd = CommandBuilder::new(shell);
    if let Some(dir) = cwd.as_ref().filter(|d| !d.is_empty()) {
        cmd.cwd(dir);
    }
    cmd.env("TERM", "xterm-256color");

    let mut child = pair
        .slave
        .spawn_command(cmd)
        .map_err(|e| format!("启动 shell 失败: {}", e))?;

    let mut reader = pair
        .master
        .try_clone_reader()
        .map_err(|e| format!("读取 PTY 失败: {}", e))?;
    let writer = pair
        .master
        .take_writer()
        .map_err(|e| format!("写入 PTY 失败: {}", e))?;

    {
        let mut sessions = state
            .sessions
            .lock()
            .map_err(|_| "终端状态锁错误".to_string())?;
        sessions.insert(
            id.clone(),
            TermSession {
                master: pair.master,
                writer,
            },
        );
    }

    // 后台读输出 → 事件
    let app_reader = app.clone();
    let read_id = id.clone();
    std::thread::spawn(move || {
        let mut buf = [0u8; 4096];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    let _ = app_reader.emit(
                        "terminal-output",
                        TerminalOutput {
                            id: read_id.clone(),
                            data: buf[..n].to_vec(),
                        },
                    );
                }
                Err(_) => break,
            }
        }
        // 进程结束，通知前端
        let _ = app_reader.emit("terminal-exit", read_id.clone());
    });

    // 等待子进程结束（独立线程，避免僵尸进程）
    std::thread::spawn(move || {
        let _ = child.wait();
    });

    Ok(())
}

/// 写入终端（用户键入）
#[tauri::command]
pub fn terminal_write(
    id: String,
    data: String,
    state: State<'_, TerminalState>,
) -> Result<(), String> {
    let mut sessions = state
        .sessions
        .lock()
        .map_err(|_| "终端状态锁错误".to_string())?;
    if let Some(s) = sessions.get_mut(&id) {
        s.writer
            .write_all(data.as_bytes())
            .map_err(|e| format!("写入终端失败: {}", e))?;
        let _ = s.writer.flush();
    }
    Ok(())
}

/// 调整终端尺寸
#[tauri::command]
pub fn terminal_resize(
    id: String,
    cols: u16,
    rows: u16,
    state: State<'_, TerminalState>,
) -> Result<(), String> {
    let sessions = state
        .sessions
        .lock()
        .map_err(|_| "终端状态锁错误".to_string())?;
    if let Some(s) = sessions.get(&id) {
        s.master
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| format!("调整终端尺寸失败: {}", e))?;
    }
    Ok(())
}

/// 关闭终端会话
#[tauri::command]
pub fn terminal_kill(id: String, state: State<'_, TerminalState>) -> Result<(), String> {
    let mut sessions = state
        .sessions
        .lock()
        .map_err(|_| "终端状态锁错误".to_string())?;
    sessions.remove(&id);
    Ok(())
}
