use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct LessPlugin;

impl LanguagePlugin for LessPlugin {
    fn get_order(&self) -> i32 {
        22
    }

    fn get_language_name(&self) -> &'static str {
        "Less"
    }

    fn get_language_key(&self) -> &'static str {
        "less"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "less".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "lessc --version".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("less"),
            before_compile: None,
            extension: String::from("less"),
            execute_home: None,
            // 用 lessc 编译为 CSS 输出到控制台
            run_command: Some(String::from("lessc $filename")),
            after_compile: None,
            template: Some(String::from("// 在这里输入 Less 代码")),
            timeout: Some(30),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "lessc".to_string())
    }
}
