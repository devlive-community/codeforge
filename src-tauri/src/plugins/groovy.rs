use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct GroovyPlugin;

impl LanguagePlugin for GroovyPlugin {
    fn get_order(&self) -> i32 {
        19
    }

    fn get_language_name(&self) -> &'static str {
        "Groovy"
    }

    fn get_language_key(&self) -> &'static str {
        "groovy"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "groovy".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which groovy".to_string()
    }

    fn get_command(
        &self,
        _file_path: Option<&str>,
        _is_version: bool,
        _file_name: Option<String>,
    ) -> String {
        if _is_version {
            // 获取版本信息时，返回解释器命令
            return "groovy".to_string();
        }

        // 执行代码时
        if let Some(config) = self.get_config() {
            if let Some(run_cmd) = &config.run_command {
                return if let Some(file_name) = _file_name {
                    run_cmd.replace("$filename", &file_name)
                } else {
                    // 执行代码但没有文件名时，返回原始命令让框架处理 $filename 替换
                    run_cmd.clone()
                };
            }
        }
        self.get_default_command()
    }

    fn get_execute_args(&self, file_path: &str) -> Vec<String> {
        let cmd = if self.get_execute_home().is_some() {
            format!("./groovy {}", file_path)
        } else {
            format!("groovy {}", file_path)
        };

        vec!["-c".to_string(), cmd]
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("groovy"),
            before_compile: None,
            extension: String::from("groovy"),
            execute_home: None,
            run_command: Some(String::from("bash")),
            after_compile: None,
            template: Some(String::from("// 在这里输入 Groovy 代码")),
            timeout: Some(30),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "groovy".to_string())
    }
}
