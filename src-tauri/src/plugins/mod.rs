use serde::{Deserialize, Serialize};

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

// 语言插件接口
pub trait LanguagePlugin: Send + Sync {
    fn get_order(&self) -> i32 {
        0
    }
    fn get_language_name(&self) -> &'static str;
    fn get_file_extension(&self) -> &'static str;
    fn get_commands(&self) -> Vec<&'static str>;
    fn get_version_args(&self) -> Vec<&'static str>;
    fn get_execute_args(&self, file_path: &str) -> Vec<String>;
    fn get_path_command(&self) -> String;

    // 可选的钩子函数
    fn pre_execute_hook(&self, _code: &str) -> Result<String, String> {
        Ok(_code.to_string())
    }

    fn post_execute_hook(&self, _result: &mut ExecutionResult) -> Result<(), String> {
        Ok(())
    }
}

// 重新导出子模块
pub mod manager;
pub mod python2;
pub mod python3;

pub use manager::PluginManager;
