use crate::env_manager::{EnvironmentInfo, EnvironmentManager};
use log::info;
use tauri::{AppHandle, Emitter, State};
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
    let result = manager
        .download_and_install_version(&language, &version, app_handle.clone())
        .await;

    if result.is_ok() {
        // 发送配置更新事件通知前端刷新配置
        app_handle.emit("config-updated", ()).ok();
        info!("已发送配置更新事件");
    }

    result
}

#[tauri::command]
pub async fn switch_environment_version(
    language: String,
    version: String,
    app_handle: AppHandle,
    env_manager: State<'_, EnvironmentManagerState>,
) -> Result<(), String> {
    info!("切换 {} 到版本 {}", language, version);
    let manager = env_manager.lock().await;
    let result = manager
        .switch_version(&language, &version, app_handle.clone())
        .await;

    if result.is_ok() {
        app_handle.emit("config-updated", ()).ok();
        info!("已发送配置更新事件");
    }

    result
}

#[tauri::command]
pub async fn get_supported_environment_languages(
    env_manager: State<'_, EnvironmentManagerState>,
) -> Result<Vec<String>, String> {
    let manager = env_manager.lock().await;
    Ok(manager.get_supported_languages())
}

#[tauri::command]
pub async fn uninstall_environment_version(
    language: String,
    version: String,
    env_manager: State<'_, EnvironmentManagerState>,
) -> Result<(), String> {
    info!("卸载 {} 版本 {}", language, version);
    let manager = env_manager.lock().await;
    manager.uninstall_version(&language, &version).await
}
