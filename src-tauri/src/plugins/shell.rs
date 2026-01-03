use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct ShellPlugin;

impl LanguagePlugin for ShellPlugin {
    fn get_order(&self) -> i32 {
        6
    }

    fn get_language_name(&self) -> &'static str {
        "ShellScript"
    }

    fn get_language_key(&self) -> &'static str {
        "shell"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "sh".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which bash || which sh".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("shell"),
            before_compile: None,
            extension: String::from("sh"),
            execute_home: None,
            run_command: Some(String::from("bash $filename")),
            after_compile: None,
            template: Some(String::from("# 在这里输入 Shell 代码")),
            timeout: Some(30),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "bash".to_string())
    }
}
