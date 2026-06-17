use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct PascalPlugin;

impl LanguagePlugin for PascalPlugin {
    fn get_order(&self) -> i32 {
        24
    }

    fn get_language_name(&self) -> &'static str {
        "Pascal"
    }

    fn get_language_key(&self) -> &'static str {
        "pascal"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "pas".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which instantfpc".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("pascal"),
            before_compile: None,
            extension: String::from("pas"),
            execute_home: None,
            // InstantFPC 直接编译并运行单文件 Pascal 脚本
            run_command: Some(String::from("instantfpc $filename")),
            after_compile: None,
            template: Some(String::from("begin\n  writeln('Hello, Pascal!');\nend.")),
            timeout: Some(30),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "instantfpc".to_string())
    }
}
