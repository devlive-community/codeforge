use crate::plugins::PluginManager;
use log::{debug, error, info};
use std::fs;
use tauri::{AppHandle, Manager, State};
use tokio::sync::Mutex;

pub type PluginManagerState = Mutex<PluginManager>;

#[tauri::command]
pub async fn load_example(
    language: String,
    app_handle: AppHandle,
    plugin_manager: State<'_, PluginManagerState>,
) -> Result<String, String> {
    info!("加载示例 -> 语言: {}", language);

    // 获取插件管理器并检查语言是否支持
    let manager = plugin_manager.lock().await;
    let plugin = manager
        .get_plugin(&language)
        .ok_or_else(|| format!("暂未支持 {} 语言，请前往 github 提供 issus", language))?;

    // 获取该语言的文件扩展名
    let file_extension = plugin.get_file_extension();
    drop(manager); // 释放锁

    // 获取应用资源目录
    let resource_dir = app_handle
        .path()
        .resource_dir()
        .map_err(|e| format!("无法获取资源目录: {}", e))?;

    let example_file_name = format!("{}.{}", language, file_extension);
    let example_path = resource_dir
        .join("src")
        .join("examples")
        .join(&example_file_name);

    debug!("加载示例 -> 文件路径: {:?}", example_path);

    // 检查文件是否存在
    if !example_path.exists() {
        error!("加载示例 -> 文件不存在: {:?}", example_path);
        return Err(format!("示例文件不存在: {}", example_file_name));
    }

    // 读取文件内容
    match fs::read_to_string(&example_path) {
        Ok(content) => {
            info!("加载示例 -> 成功加载: {}", example_file_name);
            Ok(content)
        }
        Err(e) => {
            error!("加载示例 -> 读取文件失败: {:?}", e);
            Err(format!("读取示例文件失败: {}", e))
        }
    }
}
