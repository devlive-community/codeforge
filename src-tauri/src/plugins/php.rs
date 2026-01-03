use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct PHPPlugin;

impl LanguagePlugin for PHPPlugin {
    fn get_order(&self) -> i32 {
        22
    }

    fn get_language_name(&self) -> &'static str {
        "PHP"
    }

    fn get_language_key(&self) -> &'static str {
        "php"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "php".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["-version"]
    }

    fn get_path_command(&self) -> String {
        "which php".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("php"),
            before_compile: None,
            extension: String::from("php"),
            execute_home: None,
            run_command: Some(String::from("php $filename")),
            after_compile: None,
            template: Some(String::from("<!-- 在这里输入 PHP 代码 -->")),
            timeout: Some(30),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "php".to_string())
    }
}
