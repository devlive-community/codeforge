#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod config;
mod logger;
mod plugins;
mod setup;
mod utils;

use crate::setup::app::get_app_info;
use crate::utils::logger::{
    clear_logs, get_log_directory, get_log_files, reset_log_directory, set_log_directory,
};
use config::{get_app_config, get_config_path, init_config, update_app_config};
use std::collections::HashMap;

use log::{debug, error, info, warn};
use plugins::{CodeExecutionRequest, ExecutionResult, LanguageInfo, PluginManager};
use std::fs;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter, State};
use tokio::sync::Mutex;
use uuid::Uuid;

use std::io::{BufRead, BufReader};
use std::sync::OnceLock;
use std::sync::{Arc, mpsc};
use std::thread;

// 执行任务结构
#[derive(Debug)]
struct ExecutionTask {
    #[allow(dead_code)]
    pub language: String,
    #[allow(dead_code)]
    pub process_id: u32,
    pub stop_flag: Arc<tokio::sync::Mutex<bool>>,
}

type ExecutionHistory = Mutex<Vec<ExecutionResult>>;
type PluginManagerState = Mutex<PluginManager>;

// 全局任务管理器
type TaskManager = Arc<tokio::sync::Mutex<HashMap<String, ExecutionTask>>>;
static TASK_MANAGER: OnceLock<TaskManager> = OnceLock::new();

// 初始化任务管理器
fn init_task_manager() -> TaskManager {
    TASK_MANAGER
        .get_or_init(|| Arc::new(tokio::sync::Mutex::new(HashMap::new())))
        .clone()
}

// 停止执行命令
#[tauri::command]
async fn stop_execution(language: String) -> Result<bool, String> {
    let task_manager = init_task_manager();
    let mut guard = task_manager.lock().await;

    if let Some(task) = guard.remove(&language) {
        // 设置停止标志
        {
            let mut stop_flag = task.stop_flag.lock().await;
            *stop_flag = true;
        }
        info!("停止执行 -> 成功设置停止标志给语言 [ {} ]", language);
        Ok(true)
    } else {
        warn!("停止执行 -> 语言 [ {} ] 没有正在运行的任务", language);
        Ok(false)
    }
}

// 检查是否有正在运行的任务
#[tauri::command]
async fn is_execution_running(language: String) -> Result<bool, String> {
    let task_manager = init_task_manager();
    let guard = task_manager.lock().await;
    Ok(guard.contains_key(&language))
}

// 通用的代码执行函数
#[tauri::command]
async fn execute_code(
    request: CodeExecutionRequest,
    history: State<'_, ExecutionHistory>,
    plugin_manager: State<'_, PluginManagerState>,
    app: AppHandle,
) -> Result<ExecutionResult, String> {
    info!("执行代码 -> 调用插件 [ {} ] 开始", request.language);

    // 先停止之前可能正在运行的任务
    let _ = stop_execution(request.language.clone()).await;

    let manager = plugin_manager.lock().await;
    let plugin = manager
        .get_plugin(&request.language)
        .ok_or_else(|| format!("Unsupported language: {}", request.language))?;

    let execution_id = Uuid::new_v4().to_string();
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!(
        "codeforge_{}_{}.{}",
        request.language,
        execution_id,
        plugin.get_file_extension()
    ));

    let processed_code = plugin.pre_execute_hook(&request.code).map_err(|e| {
        error!(
            "执行代码 -> 调用插件 [ {} ] pre_execute_hook 出现错误 {:?}",
            request.language, e
        );
        format!("Pre-execution hook failed: {}", e)
    })?;

    // 写入代码到临时文件
    fs::write(&file_path, &processed_code)
        .map_err(|e| format!("Failed to write temporary file: {}", e))?;

    let start_time = std::time::Instant::now();

    let cmd = plugin.get_command(None);
    let args = plugin.get_execute_args(file_path.to_str().unwrap());
    info!(
        "执行代码 -> 调用插件 [ {} ] 执行命令 {} 携带参数 {}",
        request.language,
        cmd,
        args.join(" ")
    );

    // 发送执行开始事件
    let _ = app.emit(
        "code-execution-start",
        serde_json::json!({
            "language": request.language
        }),
    );

    // 启动子进程
    let mut child = match Command::new(&cmd)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(e) => {
            let execution_time = start_time.elapsed().as_millis();
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let _ = fs::remove_file(&file_path);
            let _ = app.emit(
                "code-execution-complete",
                serde_json::json!({
                    "language": request.language,
                    "success": false
                }),
            );

            error!("执行代码 -> 调用插件 [ {} ] 失败: {}", request.language, e);
            return Ok(ExecutionResult {
                success: false,
                stdout: String::new(),
                stderr: format!(
                    "{} interpreter not found. Please install {} and ensure it's in your PATH.\n\nError: {}",
                    request.language, request.language, e
                ),
                execution_time,
                timestamp,
                language: request.language,
            });
        }
    };

    // 创建停止标志
    let stop_flag = Arc::new(tokio::sync::Mutex::new(false));

    // 将任务添加到管理器
    let task_manager = init_task_manager();
    {
        let mut guard = task_manager.lock().await;
        guard.insert(
            request.language.clone(),
            ExecutionTask {
                language: request.language.clone(),
                process_id: child.id(),
                stop_flag: stop_flag.clone(),
            },
        );
    }

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let (stdout_tx, stdout_rx) = mpsc::channel::<String>();
    let (stderr_tx, stderr_rx) = mpsc::channel::<String>();

    // 读取 stdout
    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines().map_while(Result::ok) {
            if stdout_tx.send(line).is_err() {
                break;
            }
        }
    });

    // 读取 stderr
    thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines().map_while(Result::ok) {
            if stderr_tx.send(line).is_err() {
                break;
            }
        }
    });

    let mut stdout_lines = Vec::new();
    let mut stderr_lines = Vec::new();
    let timeout = std::time::Duration::from_secs(30);

    // 主执行循环
    loop {
        // 检查停止标志
        {
            let stop_guard = stop_flag.lock().await;
            if *stop_guard {
                info!(
                    "执行代码 -> 收到停止信号，终止语言 [ {} ] 的执行",
                    request.language
                );
                let _ = child.kill();
                let _ = child.wait();
                let _ = fs::remove_file(&file_path);

                // 从任务管理器中移除
                {
                    let mut guard = task_manager.lock().await;
                    guard.remove(&request.language);
                }

                let _ = app.emit(
                    "code-execution-stopped",
                    serde_json::json!({
                        "language": request.language
                    }),
                );

                return Err("代码执行被用户停止".to_string());
            }
        }

        // 检查超时
        if start_time.elapsed() > timeout {
            let _ = child.kill();
            let _ = child.wait();
            let _ = fs::remove_file(&file_path);

            // 从任务管理器中移除
            {
                let mut guard = task_manager.lock().await;
                guard.remove(&request.language);
            }

            let _ = app.emit(
                "code-execution-timeout",
                serde_json::json!({
                    "language": request.language
                }),
            );

            error!("执行代码 -> 超时，终止语言 [ {} ] 的执行", request.language);
            return Err("代码执行超时（30秒）".to_string());
        }

        // 读取并发送 stdout
        while let Ok(line) = stdout_rx.try_recv() {
            stdout_lines.push(line.clone());
            let _ = app.emit(
                "code-output",
                serde_json::json!({
                    "type": "stdout",
                    "content": line,
                    "language": request.language
                }),
            );
        }

        // 读取并发送 stderr
        while let Ok(line) = stderr_rx.try_recv() {
            stderr_lines.push(line.clone());
            let _ = app.emit(
                "code-output",
                serde_json::json!({
                    "type": "stderr",
                    "content": line,
                    "language": request.language
                }),
            );
        }

        // 检查进程是否结束
        match child.try_wait() {
            Ok(Some(status)) => {
                // 进程已结束，读取剩余输出
                while let Ok(line) = stdout_rx.try_recv() {
                    stdout_lines.push(line.clone());
                    let _ = app.emit(
                        "code-output",
                        serde_json::json!({
                            "type": "stdout",
                            "content": line,
                            "language": request.language
                        }),
                    );
                }
                while let Ok(line) = stderr_rx.try_recv() {
                    stderr_lines.push(line.clone());
                    let _ = app.emit(
                        "code-output",
                        serde_json::json!({
                            "type": "stderr",
                            "content": line,
                            "language": request.language
                        }),
                    );
                }

                let execution_time = start_time.elapsed().as_millis();
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                let _ = fs::remove_file(&file_path);

                // 从任务管理器中移除
                {
                    let mut guard = task_manager.lock().await;
                    guard.remove(&request.language);
                }

                let mut result = ExecutionResult {
                    success: status.success(),
                    stdout: stdout_lines.join("\n"),
                    stderr: stderr_lines.join("\n"),
                    execution_time,
                    timestamp,
                    language: request.language.clone(),
                };

                let _ = plugin.post_execute_hook(&mut result);

                let _ = app.emit(
                    "code-execution-complete",
                    serde_json::json!({
                        "language": request.language,
                        "success": result.success,
                        "execution_time": result.execution_time
                    }),
                );

                drop(manager);
                let mut history_guard = history.lock().await;
                history_guard.push(result.clone());

                if history_guard.len() > 100 {
                    history_guard.remove(0);
                }

                info!("执行代码 -> 调用插件 [ {} ] 完成", request.language);
                return Ok(result);
            }
            Ok(None) => {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
            Err(e) => {
                let _ = child.kill();
                let _ = child.wait();
                let _ = fs::remove_file(&file_path);

                // 从任务管理器中移除
                {
                    let mut guard = task_manager.lock().await;
                    guard.remove(&request.language);
                }

                let _ = app.emit(
                    "code-execution-error",
                    serde_json::json!({
                        "language": request.language,
                        "error": e.to_string()
                    }),
                );

                return Err(format!("检查进程状态失败: {}", e));
            }
        }
    }
}

// 通用的环境信息获取函数
#[tauri::command]
async fn get_info(
    language: String,
    plugin_manager: State<'_, PluginManagerState>,
) -> Result<LanguageInfo, String> {
    info!("获取环境 -> 调用插件 [ {} ] 开始", language);
    let manager = plugin_manager.lock().await;
    let plugin = manager
        .get_plugin(&language)
        .ok_or_else(|| format!("Unsupported language: {}", language))?;

    plugin.pre_execute_hook("").map_err(|e| {
        error!(
            "获取环境 -> 调用插件 [ {} ] pre_execute_hook 出现错误 {:?}",
            language, e
        );

        error!("获取环境 -> 调用插件 [ {} ] 失败", language);
        format!("Pre-execution hook failed: {}", e)
    })?;

    let cmd = plugin.get_command(None);
    debug!("获取环境 -> 插件 [ {} ] 命令 {}", language, cmd);

    let version_output = Command::new(&cmd).args(plugin.get_version_args()).output();
    if let Ok(version_out) = version_output {
        if version_out.status.success() {
            let path_result = Command::new(&cmd)
                .arg("-c")
                .arg(plugin.get_path_command())
                .output();

            let version = String::from_utf8_lossy(&version_out.stdout)
                .trim()
                .to_string();

            let path = if let Ok(path_out) = path_result {
                if path_out.status.success() {
                    String::from_utf8_lossy(&path_out.stdout).trim().to_string()
                } else {
                    "Command found but path unavailable".to_string()
                }
            } else {
                "Path detection failed".to_string()
            };

            info!("获取环境 -> 调用插件 [ {} ] 完成", language);
            return Ok(LanguageInfo {
                installed: true,
                version,
                path,
                language: plugin.get_language_name().to_string(),
            });
        }
    }

    error!("获取环境 -> 调用插件 [ {} ] 失败", language);
    Ok(LanguageInfo {
        installed: false,
        version: "Not found".to_string(),
        path: format!("Not found - tried: {:?}", plugin.get_command(None)),
        language: plugin.get_language_name().to_string(),
    })
}

// 获取支持的语言列表
#[tauri::command]
async fn get_supported_languages(
    plugin_manager: State<'_, PluginManagerState>,
) -> Result<Vec<serde_json::Value>, String> {
    let manager = plugin_manager.lock().await;
    Ok(manager.get_supported_languages())
}

#[tauri::command]
async fn get_execution_history(
    history: State<'_, ExecutionHistory>,
) -> Result<Vec<ExecutionResult>, String> {
    let history_guard = history.lock().await;
    Ok(history_guard.clone())
}

#[tauri::command]
async fn clear_execution_history(history: State<'_, ExecutionHistory>) -> Result<(), String> {
    let mut history_guard = history.lock().await;
    history_guard.clear();
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(ExecutionHistory::default())
        .manage(PluginManagerState::new(PluginManager::new()))
        .setup(|app| {
            // 第一步：初始化配置系统
            if let Err(e) = init_config(Some(app.handle())) {
                eprintln!("Failed to initialize config: {}", e);
            }

            // 第二步：初始化日志系统
            if let Err(e) = logger::setup_logger(app.handle()) {
                eprintln!("Failed to setup logger: {}", e);
            }

            // 初始化应用菜单
            info!("初始化 -> 初始化应用菜单");
            let menu = setup::menu::create_menu(app.handle())?;
            app.set_menu(menu)?;
            setup::menu::setup_menu_handler(app.handle());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            execute_code,
            get_info,
            get_supported_languages,
            get_execution_history,
            clear_execution_history,
            stop_execution,
            is_execution_running,
            get_app_info,
            // 日志相关命令
            get_log_directory,
            set_log_directory,
            reset_log_directory,
            get_log_files,
            clear_logs,
            // 配置相关命令
            get_app_config,
            update_app_config,
            get_config_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
