use crate::config::get_app_config_internal;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// 通用结构定义
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExecutionResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub execution_time: u128,
    pub timestamp: u64,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeExecutionRequest {
    pub code: String,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageInfo {
    pub installed: bool,
    pub version: String,
    pub path: String,
    pub language: String,
}

// 插件配置结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub enabled: bool,                  // 插件是否启用
    pub execute_home: Option<String>,   // 插件的执行路径
    pub extension: String,              // 插件支持的文件扩展名
    pub language: String,               // 插件所属语言
    pub before_compile: Option<String>, // 插件在编译前执行的命令
    pub after_compile: Option<String>,  // 插件在编译完成后执行的命令
    pub run_command: Option<String>,    // 插件执行的命令，例如 "python2 $filename"
    pub template: Option<String>,       // 插件的模板
    pub timeout: Option<u64>,           // 插件的超时时间
}

// 语言插件接口
pub trait LanguagePlugin: Send + Sync {
    // 获取插件优先级
    fn get_order(&self) -> i32 {
        0
    }

    // 获取插件名称
    fn get_language_name(&self) -> &'static str;

    // 获取插件唯一标记
    fn get_language_key(&self) -> &'static str;

    // 获取插件支持的文件扩展名
    fn get_file_extension(&self) -> String {
        self.get_config().unwrap().extension.clone()
    }

    // 获取执行目录
    fn get_execute_home(&self) -> Option<PathBuf> {
        self.get_config()
            .and_then(|config| config.execute_home.clone())
            .filter(|path| !path.trim().is_empty()) // 过滤掉空字符串和只有空白字符的字符串
            .map(PathBuf::from)
    }

    // 获取超时时间
    fn get_timeout(&self) -> u64 {
        self.get_config()
            .map(|config| config.timeout.unwrap_or(30))
            .unwrap_or(30)
    }

    // 获取插件支持的命令
    fn get_command(&self, file_path: Option<&str>) -> String {
        if let Some(config) = self.get_config() {
            if let Some(run_cmd) = &config.run_command {
                if let Some(path) = file_path {
                    return run_cmd.replace("$filename", path);
                } else {
                    return run_cmd
                        .split_whitespace()
                        .next()
                        .unwrap_or(&config.language)
                        .to_string();
                }
            }
        }
        self.get_default_command()
    }

    // 获取插件配置
    fn get_config(&self) -> Option<PluginConfig> {
        // 获取全局应用配置
        if let Ok(app_config) = get_app_config_internal() {
            // 检查是否有插件配置
            if let Some(ref plugins) = app_config.plugins {
                // 根据当前插件的语言名称过滤配置
                let language_name = self.get_language_key();

                // 查找匹配的插件配置
                let found_config = plugins
                    .iter()
                    .find(|config| config.language == language_name)
                    .cloned();

                debug!(
                    "执行代码 -> 获取插件 [ {} ] 配置 {:?}",
                    language_name, found_config
                );
                return found_config;
            }
        }

        // 如果没有找到配置，返回默认配置
        debug!(
            "执行代码 -> 插件 [ {} ] 未找到配置，使用默认配置",
            self.get_language_key()
        );
        Some(self.get_default_config())
    }

    // 检查插件是否启用
    #[allow(dead_code)]
    fn is_enabled(&self) -> bool {
        self.get_config()
            .map(|config| config.enabled)
            .unwrap_or(false)
    }

    fn get_version_args(&self) -> Vec<&'static str>;

    fn get_execute_args(&self, file_path: &str) -> Vec<String> {
        if let Some(config) = self.get_config() {
            if let Some(run_cmd) = &config.run_command {
                // 替换 $filename 后分割，跳过第一个元素（命令本身）
                let full_cmd = run_cmd.replace("$filename", file_path);
                return full_cmd
                    .split_whitespace()
                    .skip(1) // 跳过命令部分，只返回参数
                    .map(|s| s.to_string())
                    .collect();
            }
        }
        // 默认情况下，文件路径就是唯一的参数
        vec![file_path.to_string()]
    }

    fn get_path_command(&self) -> String;

    // 构建默认配置
    fn get_default_config(&self) -> PluginConfig;

    // 获取默认命令
    fn get_default_command(&self) -> String;

    // 预执行钩子
    fn pre_execute_hook(&self, code: &str) -> Result<String, String> {
        info!(
            "执行代码 -> 插件 [ {} ] 处理 pre_execute_hook 开始",
            self.get_language_key()
        );

        if let Some(config) = self.get_config() {
            // 1. 处理 before_compile 命令（直接在 Rust 中处理）
            if let Some(before_cmd) = &config.before_compile {
                info!(
                    "执行代码 -> 插件 [ {} ] 处理 pre_execute_hook 处理环境变量: {}",
                    self.get_language_key(),
                    before_cmd
                );

                self.handle_environment_setup(before_cmd)?;
            }

            // 2. 切换到 execute_home 目录
            if let Some(execute_home) = self.get_execute_home() {
                info!(
                    "执行代码 -> 插件 [ {} ] 处理 pre_execute_hook 切换到执行目录 {}",
                    self.get_language_key(),
                    execute_home.display()
                );
                std::env::set_current_dir(&execute_home)
                    .map_err(|e| format!("切换目录失败: {}", e))?;
            }
        }

        info!(
            "执行代码 -> 插件 [ {} ] 处理 pre_execute_hook 结束",
            self.get_language_key()
        );

        Ok(code.to_string())
    }

    fn handle_environment_setup(&self, command: &str) -> Result<(), String> {
        // 处理 export 命令（Unix/Linux/macOS）
        if command.starts_with("export ") {
            return self.handle_export_command(command);
        }

        // 处理 set 命令（Windows）
        if command.starts_with("set ") {
            return self.handle_set_command(command);
        }

        // 处理其他通用环境设置
        self.execute_cross_platform_command(command)
    }

    fn handle_export_command(&self, command: &str) -> Result<(), String> {
        if let Some(env_part) = command.strip_prefix("export ") {
            if let Some((key, value)) = env_part.split_once("=") {
                let value = value.trim_matches('"').trim_matches('\'');
                let expanded_value = self.expand_env_vars(value);
                let key = key.trim();

                // 先记录日志，再设置环境变量
                info!("设置环境变量 {}={}", key, expanded_value);

                // 使用 unsafe 块设置环境变量
                unsafe {
                    std::env::set_var(key, expanded_value);
                }
            }
        }
        Ok(())
    }

    fn handle_set_command(&self, command: &str) -> Result<(), String> {
        if let Some(env_part) = command.strip_prefix("set ") {
            if let Some((key, value)) = env_part.split_once("=") {
                let value = value.trim_matches('"').trim_matches('\'');
                let expanded_value = self.expand_env_vars(value);
                let key = key.trim();

                // 先记录日志，再设置环境变量
                info!("设置环境变量 {}={}", key, expanded_value);

                // 使用 unsafe 块设置环境变量
                unsafe {
                    std::env::set_var(key, expanded_value);
                }
            }
        }
        Ok(())
    }

    fn expand_env_vars(&self, value: &str) -> String {
        let mut result = value.to_string();

        // 处理 Unix 风格的环境变量 $VAR
        if result.contains("$PATH") {
            if let Ok(current_path) = std::env::var("PATH") {
                result = result.replace("$PATH", &current_path);
            }
        }

        // 处理 Windows 风格的环境变量 %VAR%
        if result.contains("%PATH%") {
            if let Ok(current_path) = std::env::var("PATH") {
                result = result.replace("%PATH%", &current_path);
            }
        }

        result
    }

    fn execute_cross_platform_command(&self, command: &str) -> Result<(), String> {
        let output = if cfg!(target_os = "windows") {
            std::process::Command::new("cmd")
                .args(&["/C", command])
                .output()
        } else {
            std::process::Command::new("sh")
                .args(&["-c", command])
                .output()
        };

        let output = output.map_err(|e| format!("执行命令失败: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "命令执行失败: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }

    // 后执行钩子
    fn post_execute_hook(&self, result: &mut ExecutionResult) -> Result<(), String> {
        if result.success && result.stdout.is_empty() && result.stderr.is_empty() {
            result.stdout = "代码执行成功 (无输出)".to_string();
        }

        Ok(())
    }
}

// 重新导出子模块
pub mod manager;
pub mod python2;
pub mod python3;
pub use manager::PluginManager;
