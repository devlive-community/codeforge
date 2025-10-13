use crate::plugins::{LanguageInfo, PluginManager};
use log::{debug, error, info};
use regex::Regex;
use std::process::Command;
use tauri::State;
use tokio::sync::Mutex;

pub type PluginManagerState = Mutex<PluginManager>;

fn extract_version(output: &str) -> String {
    let output = output.trim();

    let version_patterns = vec![
        r"(?i)version\s+([0-9]+\.[0-9]+\.[0-9]+(?:-[a-zA-Z0-9]+)?)",
        r"(?i)version\s+([0-9]+\.[0-9]+(?:\.[0-9]+)?)",
        r"\bv?([0-9]+\.[0-9]+\.[0-9]+(?:-[a-zA-Z0-9]+)?)\b",
        r"\bv?([0-9]+\.[0-9]+(?:\.[0-9]+)?)\b",
        r#""([0-9]+\.[0-9]+\.[0-9]+)""#,
    ];

    for pattern in version_patterns {
        if let Ok(re) = Regex::new(pattern) {
            if let Some(cap) = re.captures(output) {
                if let Some(version) = cap.get(1) {
                    return version.as_str().to_string();
                }
            }
        }
    }

    if let Some(first_line) = output.lines().next() {
        if !first_line.is_empty() {
            return first_line.to_string();
        }
    }

    output.to_string()
}

#[tauri::command]
pub async fn get_info(
    language: String,
    plugin_manager: State<'_, PluginManagerState>,
) -> Result<LanguageInfo, String> {
    info!("获取环境 -> 调用插件 [ {} ] 开始", language);
    let manager = plugin_manager.lock().await;
    let plugin = manager
        .get_plugin(&language)
        .ok_or_else(|| format!("Unsupported language: {}", language))?;

    plugin.pre_execute_hook("", "").map_err(|e| {
        error!(
            "获取环境 -> 调用插件 [ {} ] pre_execute_hook 出现错误 {:?}",
            language, e
        );

        error!("获取环境 -> 调用插件 [ {} ] 失败", language);
        format!("Pre-execution hook failed: {}", e)
    })?;

    let cmd = plugin.get_command(None, true, None);
    debug!("获取环境 -> 插件 [ {} ] 命令 {}", language, cmd);

    let version_output = Command::new(&cmd).args(plugin.get_version_args()).output();
    debug!(
        "获取环境 -> 插件 [ {} ] 版本, 结果 {:?}",
        plugin.get_language_key(),
        version_output
    );
    if let Ok(version_out) = version_output {
        if version_out.status.success() {
            let path_result = Command::new(&cmd)
                .arg("-c")
                .arg(plugin.get_path_command())
                .output();

            let mut raw_version = String::from_utf8_lossy(&version_out.stdout)
                .trim()
                .to_string();

            if raw_version.is_empty() {
                info!(
                    "获取环境 -> 调用插件 [ {} ] 版本为空，通过 stderr 获取",
                    language
                );
                raw_version = String::from_utf8_lossy(&version_out.stderr)
                    .trim()
                    .to_string();
            }

            let version = extract_version(&raw_version);
            let final_version = if version.is_empty() {
                raw_version
            } else {
                version
            };

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
                version: final_version,
                path,
                language: plugin.get_language_name().to_string(),
            });
        }
    }

    error!("获取环境 -> 调用插件 [ {} ] 失败", language);
    Ok(LanguageInfo {
        installed: false,
        version: "Not found".to_string(),
        path: format!(
            "Not found - tried: {:?}",
            plugin.get_command(None, true, None)
        ),
        language: plugin.get_language_name().to_string(),
    })
}

#[tauri::command]
pub async fn get_supported_languages(
    plugin_manager: State<'_, PluginManagerState>,
) -> Result<Vec<serde_json::Value>, String> {
    let manager = plugin_manager.lock().await;
    Ok(manager.get_supported_languages())
}
