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

use log::{debug, error, info};
use plugins::{CodeExecutionRequest, ExecutionResult, LanguageInfo, PluginManager};
use std::fs;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::State;
use tokio::sync::Mutex;
use uuid::Uuid;

type ExecutionHistory = Mutex<Vec<ExecutionResult>>;
type PluginManagerState = Mutex<PluginManager>;

// 通用的代码执行函数
#[tauri::command]
async fn execute_code(
    request: CodeExecutionRequest,
    history: State<'_, ExecutionHistory>,
    plugin_manager: State<'_, PluginManagerState>,
) -> Result<ExecutionResult, String> {
    info!("执行代码 -> 调用插件 [ {} ] 开始", request.language);
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
    let mut _last_error: String = String::new();

    let cmd = plugin.get_command(None);
    let args = plugin.get_execute_args(file_path.to_str().unwrap());
    info!(
        "执行代码 -> 调用插件 [ {} ] 执行命令 {} 携带参数 {}",
        request.language,
        cmd,
        args.join(" ")
    );

    let output = Command::new(&cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    match output {
        Ok(output) => {
            let execution_time = start_time.elapsed().as_millis();
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            // 清理临时文件
            let _ = fs::remove_file(&file_path);

            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            let mut result = ExecutionResult {
                success: output.status.success(),
                stdout,
                stderr,
                execution_time,
                timestamp,
                language: request.language.clone(),
            };

            // 后处理
            let _ = plugin.post_execute_hook(&mut result);

            // 添加到执行历史
            drop(manager); // 释放插件管理器锁
            let mut history_guard = history.lock().await;
            history_guard.push(result.clone());

            // 保持历史记录不超过100条
            if history_guard.len() > 100 {
                history_guard.remove(0);
            }

            info!("执行代码 -> 调用插件 [ {} ] 完成", request.language);
            return Ok(result);
        }
        Err(e) => {
            _last_error = format!("Failed to execute {} - {}", cmd, e);
        }
    }

    // 如果所有命令都失败了
    let execution_time = start_time.elapsed().as_millis();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 清理临时文件
    let _ = fs::remove_file(&file_path);

    error!("执行代码 -> 调用插件 [ {} ] 失败", request.language);
    Ok(ExecutionResult {
        success: false,
        stdout: String::new(),
        stderr: format!(
            "{} interpreter not found. Please install {} and ensure it's in your PATH.\n\nLast error: {}\n\nTried commands: {:?}",
            request.language,
            request.language,
            _last_error,
            plugin
                .get_command(Some(file_path.to_str().unwrap()))
                .to_string()
        ),
        execution_time,
        timestamp,
        language: request.language,
    })
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
