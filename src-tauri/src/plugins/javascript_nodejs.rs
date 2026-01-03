use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct JavaScriptNodeJsPlugin;

impl LanguagePlugin for JavaScriptNodeJsPlugin {
    fn get_order(&self) -> i32 {
        13
    }

    fn get_language_name(&self) -> &'static str {
        "JavaScript (Node.js)"
    }

    fn get_language_key(&self) -> &'static str {
        "javascript-nodejs"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "js".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which node".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("javascript-nodejs"),
            before_compile: None,
            extension: String::from("js"),
            execute_home: None,
            run_command: Some(String::from("node $filename")),
            after_compile: None,
            template: Some(String::from("// 在这里输入 JavaScript (Node.js) 代码")),
            timeout: Some(30),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "node".to_string())
    }
}
