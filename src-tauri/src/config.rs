use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub log_directory: Option<String>,
    pub auto_clear_logs: Option<bool>,
    pub keep_log_days: Option<u32>,
    pub theme: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            log_directory: None,
            auto_clear_logs: Some(true),
            keep_log_days: Some(30),
            theme: Some("system".to_string()),
        }
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
    config: AppConfig,
}

impl ConfigManager {
    pub fn new() -> Result<Self, String> {
        let config_path = Self::get_config_path()?;
        let config = Self::load_config(&config_path)?;

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

    fn load_config(config_path: &PathBuf) -> Result<AppConfig, String> {
        if config_path.exists() {
            match fs::read_to_string(config_path) {
                Ok(content) => match serde_json::from_str::<AppConfig>(&content) {
                    Ok(config) => {
                        info!("读取配置 -> 成功加载配置文件: {:?}", config_path);
                        Ok(config)
                    }
                    Err(e) => {
                        warn!("读取配置 -> 配置文件格式错误，使用默认配置: {}", e);
                        Ok(AppConfig::default())
                    }
                },
                Err(e) => {
                    warn!("读取配置 -> 读取配置文件失败，使用默认配置: {}", e);
                    Ok(AppConfig::default())
                }
            }
        } else {
            info!("读取配置 -> 配置文件不存在，使用默认配置");
            Ok(AppConfig::default())
        }
    }

    pub fn save_config(&self) -> Result<(), String> {
        let content = serde_json::to_string_pretty(&self.config)
            .map_err(|e| format!("序列化配置失败: {}", e))?;

        fs::write(&self.config_path, content).map_err(|e| format!("写入配置文件失败: {}", e))?;

        info!("保存配置 -> 配置文件已保存: {:?}", self.config_path);
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

// 全局配置管理器
use std::sync::Mutex;
static CONFIG_MANAGER: Mutex<Option<ConfigManager>> = Mutex::new(None);

// 初始化配置
pub fn init_config() -> Result<(), String> {
    let config_manager = ConfigManager::new()?;

    // 如果配置中有自定义日志目录，设置到日志系统
    if let Some(log_dir) = config_manager.get_log_directory() {
        info!("初始化 -> 从配置文件加载日志目录: {}", log_dir);
        // 使用内部函数设置，避免循环保存
        if let Err(e) = crate::logger::set_log_directory_internal(log_dir.to_string()) {
            warn!("初始化 -> 应用配置中的日志目录失败: {}", e);
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

// Tauri 命令
use tauri::command;

#[command]
pub async fn get_app_config() -> Result<AppConfig, String> {
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
