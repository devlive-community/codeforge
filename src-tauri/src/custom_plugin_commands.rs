use crate::config::{get_app_config_internal, get_config_manager};
use crate::execution::PluginManagerState;
use crate::plugins::PluginConfig;
use crate::plugins::custom::CustomPlugin;
use log::info;
use std::fs;
use tauri::{AppHandle, Emitter, State, command};

#[command]
pub async fn add_custom_plugin(
    config: PluginConfig,
    app_handle: AppHandle,
    plugin_manager: State<'_, PluginManagerState>,
) -> Result<(), String> {
    {
        let mut guard = get_config_manager()?;
        if let Some(config_manager) = guard.as_mut() {
            let mut app_config = config_manager.get_config().clone();

            let mut custom_plugins = app_config.custom_plugins.unwrap_or_default();

            if custom_plugins.iter().any(|p| p.language == config.language) {
                return Err(format!("自定义插件 {} 已存在", config.language));
            }

            custom_plugins.push(config.clone());
            app_config.custom_plugins = Some(custom_plugins);

            config_manager.update_config(app_config)?;
        } else {
            return Err("配置管理器未初始化".to_string());
        }
    }

    let mut manager = plugin_manager.lock().await;
    let custom_plugin = CustomPlugin::new(config.clone());
    manager.register_plugin(config.language.clone(), Box::new(custom_plugin));

    app_handle.emit("config-updated", ()).ok();
    info!("自定义插件 {} 已添加", config.language);

    Ok(())
}

#[command]
pub async fn update_custom_plugin(
    config: PluginConfig,
    app_handle: AppHandle,
    plugin_manager: State<'_, PluginManagerState>,
) -> Result<(), String> {
    {
        let mut guard = get_config_manager()?;
        if let Some(config_manager) = guard.as_mut() {
            let mut app_config = config_manager.get_config().clone();

            let mut custom_plugins = app_config.custom_plugins.unwrap_or_default();

            if let Some(index) = custom_plugins
                .iter()
                .position(|p| p.language == config.language)
            {
                custom_plugins[index] = config.clone();
            } else {
                return Err(format!("自定义插件 {} 不存在", config.language));
            }

            app_config.custom_plugins = Some(custom_plugins);

            config_manager.update_config(app_config)?;
        } else {
            return Err("配置管理器未初始化".to_string());
        }
    }

    let mut manager = plugin_manager.lock().await;
    manager.unregister_plugin(&config.language);
    let custom_plugin = CustomPlugin::new(config.clone());
    manager.register_plugin(config.language.clone(), Box::new(custom_plugin));

    app_handle.emit("config-updated", ()).ok();
    info!("自定义插件 {} 已更新", config.language);

    Ok(())
}

#[command]
pub async fn remove_custom_plugin(
    language: String,
    app_handle: AppHandle,
    plugin_manager: State<'_, PluginManagerState>,
) -> Result<(), String> {
    {
        let mut guard = get_config_manager()?;
        if let Some(config_manager) = guard.as_mut() {
            let mut app_config = config_manager.get_config().clone();

            let mut custom_plugins = app_config.custom_plugins.unwrap_or_default();

            custom_plugins.retain(|p| p.language != language);

            app_config.custom_plugins = Some(custom_plugins);

            config_manager.update_config(app_config)?;
        } else {
            return Err("配置管理器未初始化".to_string());
        }
    }

    let mut manager = plugin_manager.lock().await;
    manager.unregister_plugin(&language);

    app_handle.emit("config-updated", ()).ok();
    info!("自定义插件 {} 已移除", language);

    Ok(())
}

#[command]
pub async fn get_custom_plugins() -> Result<Vec<PluginConfig>, String> {
    let app_config = get_app_config_internal()?;
    Ok(app_config.custom_plugins.unwrap_or_default())
}

#[command]
pub async fn save_custom_icon(
    language: String,
    icon_data: Vec<u8>,
    file_extension: String,
) -> Result<String, String> {
    let home_dir = dirs::home_dir().ok_or("无法获取用户主目录")?;
    let icons_dir = home_dir.join(".codeforge").join("custom-icons");

    fs::create_dir_all(&icons_dir).map_err(|e| format!("创建图标目录失败: {}", e))?;

    let icon_path = icons_dir.join(format!("{}.{}", language, file_extension));

    fs::write(&icon_path, icon_data).map_err(|e| format!("保存图标文件失败: {}", e))?;

    info!("自定义图标已保存: {:?}", icon_path);

    Ok(icon_path.to_string_lossy().to_string())
}
