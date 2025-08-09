#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod plugins;
mod setup;

use crate::setup::app::get_app_info;
use chrono::Utc;
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

    // 预处理代码
    let processed_code = plugin
        .pre_execute_hook(&request.code)
        .map_err(|e| format!("Pre-execution hook failed: {}", e))?;

    // 写入代码到临时文件
    fs::write(&file_path, &processed_code)
        .map_err(|e| format!("Failed to write temporary file: {}", e))?;

    let start_time = std::time::Instant::now();
    let mut last_error = String::new();

    // 尝试不同的命令
    for cmd in plugin.get_commands() {
        let args = plugin.get_execute_args(file_path.to_str().unwrap());

        println!("Trying command: {} with args: {:?}", cmd, args);

        let output = Command::new(cmd)
            .args(&args)
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

                return Ok(result);
            }
            Err(e) => {
                last_error = format!("Failed to execute {} - {}", cmd, e);
                continue;
            }
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

    Ok(ExecutionResult {
        success: false,
        stdout: String::new(),
        stderr: format!(
            "{} interpreter not found. Please install {} and ensure it's in your PATH.\n\nLast error: {}\n\nTried commands: {:?}",
            request.language,
            request.language,
            last_error,
            plugin.get_commands()
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
    let manager = plugin_manager.lock().await;
    let plugin = manager
        .get_plugin(&language)
        .ok_or_else(|| format!("Unsupported language: {}", language))?;

    // 尝试不同的命令
    for cmd in plugin.get_commands() {
        println!("Trying command: {} for language: {}", cmd, language);

        let version_output = Command::new(cmd).args(plugin.get_version_args()).output();

        if let Ok(version_out) = version_output {
            if version_out.status.success() {
                let path_result = Command::new(cmd)
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

                return Ok(LanguageInfo {
                    installed: true,
                    version,
                    path,
                    language: plugin.get_language_name().to_string(),
                });
            }
        }
    }

    Ok(LanguageInfo {
        installed: false,
        version: "Not found".to_string(),
        path: format!("Not found - tried: {:?}", plugin.get_commands()),
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
    let build_time = Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(ExecutionHistory::default())
        .manage(PluginManagerState::new(PluginManager::new()))
        .setup(|app| {
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
            get_app_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
