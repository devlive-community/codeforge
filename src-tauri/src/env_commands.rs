use crate::env_manager::{EnvironmentInfo, EnvironmentManager};
use log::info;
use tauri::{AppHandle, State};
use tokio::sync::Mutex;

pub type EnvironmentManagerState = Mutex<EnvironmentManager>;

#[tauri::command]
pub async fn get_environment_info(
    language: String,
    env_manager: State<'_, EnvironmentManagerState>,
) -> Result<EnvironmentInfo, String> {
    info!("获取 {} 环境信息", language);
    let manager = env_manager.lock().await;
    manager.get_environment_info(&language).await
}

#[tauri::command]
pub async fn download_and_install_version(
    language: String,
    version: String,
    app_handle: AppHandle,
    env_manager: State<'_, EnvironmentManagerState>,
) -> Result<String, String> {
    info!("下载并安装 {} 版本 {}", language, version);
    let manager = env_manager.lock().await;
    manager.download_and_install_version(&language, &version, app_handle).await
}

#[tauri::command]
pub async fn switch_environment_version(
    language: String,
    version: String,
    env_manager: State<'_, EnvironmentManagerState>,
) -> Result<(), String> {
    info!("切换 {} 到版本 {}", language, version);
    let manager = env_manager.lock().await;
    manager.switch_version(&language, &version).await
}

#[tauri::command]
pub async fn get_supported_environment_languages(
    env_manager: State<'_, EnvironmentManagerState>,
) -> Result<Vec<String>, String> {
    let manager = env_manager.lock().await;
    Ok(manager.get_supported_languages())
}
