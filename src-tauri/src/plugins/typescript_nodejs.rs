use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct TypeScriptNodeJsPlugin;

impl LanguagePlugin for TypeScriptNodeJsPlugin {
    fn get_order(&self) -> i32 {
        16
    }

    fn get_language_name(&self) -> &'static str {
        "TypeScript (Node.js)"
    }

    fn get_language_key(&self) -> &'static str {
        "typescript-nodejs"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "ts".to_string())
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
            language: String::from("typescript-nodejs"),
            before_compile: None,
            extension: String::from("ts"),
            execute_home: None,
            run_command: Some(String::from("node $filename")),
            after_compile: None,
            template: Some(String::from("// 在这里输入 TypeScript (Node.js) 代码")),
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
