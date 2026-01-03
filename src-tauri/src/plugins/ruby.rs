use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct RubyPlugin;

impl LanguagePlugin for RubyPlugin {
    fn get_order(&self) -> i32 {
        14
    }

    fn get_language_name(&self) -> &'static str {
        "Ruby"
    }

    fn get_language_key(&self) -> &'static str {
        "ruby"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "rb".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which ruby".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("ruby"),
            before_compile: None,
            extension: String::from("rb"),
            execute_home: None,
            run_command: Some(String::from("ruby $filename")),
            after_compile: None,
            template: Some(String::from("# 在这里输入 Ruby 代码")),
            timeout: Some(30),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "ruby".to_string())
    }
}
