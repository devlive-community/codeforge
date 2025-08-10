use crate::plugins::PluginConfig;
// 全局配置管理器
use crate::PluginManagerState;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Manager, command};

static CONFIG_MANAGER: Mutex<Option<ConfigManager>> = Mutex::new(None);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub log_directory: Option<String>,
    pub auto_clear_logs: Option<bool>,
    pub keep_log_days: Option<u32>,
    pub theme: Option<String>,
    pub plugins: Option<Vec<PluginConfig>>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            log_directory: None,
            auto_clear_logs: Some(true),
            keep_log_days: Some(30),
            theme: Some("system".to_string()),
            plugins: Some(vec![]),
        }
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
    config: AppConfig,
}

impl ConfigManager {
    pub fn new(app_handle: Option<&AppHandle>) -> Result<Self, String> {
        let config_path = Self::get_config_path()?;
        let config = Self::load_config(&config_path, app_handle)?;

        Ok(Self {
            config_path,
            config,
        })
    }

    fn get_config_path() -> Result<PathBuf, String> {
        let home_dir = dirs::home_dir().ok_or_else(|| "无法获取用户主目录".to_string())?;

        let config_dir = home_dir.join(".codeforge");
        let config_file = config_dir.join("config.json");

        // 确保配置目录存在
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).map_err(|e| format!("创建配置目录失败: {}", e))?;
        }

        Ok(config_file)
    }

    fn load_config(
        config_path: &PathBuf,
        app_handle: Option<&AppHandle>,
    ) -> Result<AppConfig, String> {
        println!("读取配置 -> 正在读取配置文件 {:?}", config_path);
        if config_path.exists() {
            match fs::read_to_string(config_path) {
                Ok(content) => match serde_json::from_str::<AppConfig>(&content) {
                    Ok(mut config) => {
                        println!("读取配置 -> 成功加载配置文件: {:?}", config_path);

                        // 检查 plugins 是否为 null，如果是则加载默认配置
                        if config.plugins.is_none() {
                            println!("读取配置 -> plugins 为 null，加载默认插件配置");
                            config.plugins = Self::get_default_plugins_config(app_handle);
                        }

                        Ok(config)
                    }
                    Err(e) => {
                        warn!("读取配置 -> 配置文件格式错误，使用默认配置: {}", e);
                        Ok(Self::create_default_config(app_handle))
                    }
                },
                Err(e) => {
                    warn!("读取配置 -> 读取配置文件失败，使用默认配置: {}", e);
                    Ok(Self::create_default_config(app_handle))
                }
            }
        } else {
            println!("读取配置 -> 配置文件不存在，使用默认配置");
            Ok(Self::create_default_config(app_handle))
        }
    }

    fn get_default_plugins_config(app_handle: Option<&AppHandle>) -> Option<Vec<PluginConfig>> {
        if let Some(handle) = app_handle {
            // 从 Tauri 状态中获取 PluginManager
            if let Some(plugin_manager_state) = handle.try_state::<PluginManagerState>() {
                // 同步访问插件管理器
                if let Ok(manager) = plugin_manager_state.try_lock() {
                    return Some(manager.get_all_plugin_default_config());
                } else {
                    println!("读取配置 -> 无法获取插件管理器锁，使用空配置");
                }
            } else {
                println!("读取配置 -> 无法获取插件管理器状态，使用空配置");
            }
        }
        Some(vec![])
    }

    fn create_default_config(app_handle: Option<&AppHandle>) -> AppConfig {
        AppConfig {
            log_directory: None,
            auto_clear_logs: Some(true),
            keep_log_days: Some(30),
            theme: Some("system".to_string()),
            plugins: Self::get_default_plugins_config(app_handle),
        }
    }

    pub fn save_config(&self) -> Result<(), String> {
        let content = serde_json::to_string_pretty(&self.config)
            .map_err(|e| format!("序列化配置失败: {}", e))?;

        fs::write(&self.config_path, content).map_err(|e| format!("写入配置文件失败: {}", e))?;

        info!("保存配置 -> 配置文件已保存 {}", self.config_path.display());
        Ok(())
    }

    pub fn get_config(&self) -> &AppConfig {
        &self.config
    }

    pub fn get_log_directory(&self) -> Option<&str> {
        self.config.log_directory.as_deref()
    }

    pub fn set_log_directory(&mut self, path: Option<String>) -> Result<(), String> {
        self.config.log_directory = path;
        self.save_config()
    }
}

// 初始化配置
pub fn init_config(app_handle: Option<&AppHandle>) -> Result<(), String> {
    let config_manager = ConfigManager::new(app_handle)?;

    // 如果配置中有自定义日志目录，设置到日志系统
    if let Some(log_dir) = config_manager.get_log_directory() {
        println!("读取配置 -> 从配置文件加载日志目录: {}", log_dir);
        // 使用内部函数设置，避免循环保存
        if let Err(e) = crate::logger::set_log_directory_internal(log_dir.to_string()) {
            warn!("读取配置 -> 应用配置中的日志目录失败: {}", e);
        }
    }

    let mut guard = CONFIG_MANAGER.lock().unwrap();
    *guard = Some(config_manager);

    Ok(())
}

pub fn get_config_manager() -> Result<std::sync::MutexGuard<'static, Option<ConfigManager>>, String>
{
    CONFIG_MANAGER
        .lock()
        .map_err(|e| format!("获取配置管理器失败: {}", e))
}

#[command]
pub async fn get_app_config() -> Result<AppConfig, String> {
    let guard = get_config_manager()?;
    if let Some(config_manager) = guard.as_ref() {
        Ok(config_manager.get_config().clone())
    } else {
        Err("配置管理器未初始化".to_string())
    }
}

pub fn get_app_config_internal() -> Result<AppConfig, String> {
    let guard = get_config_manager()?;
    if let Some(config_manager) = guard.as_ref() {
        Ok(config_manager.get_config().clone())
    } else {
        Err("配置管理器未初始化".to_string())
    }
}

#[command]
pub async fn update_app_config(config: AppConfig) -> Result<(), String> {
    let mut guard = get_config_manager()?;
    if let Some(config_manager) = guard.as_mut() {
        config_manager.config = config;
        config_manager.save_config()
    } else {
        Err("配置管理器未初始化".to_string())
    }
}

#[command]
pub async fn get_config_path() -> Result<String, String> {
    ConfigManager::get_config_path().map(|path| path.to_string_lossy().to_string())
}
