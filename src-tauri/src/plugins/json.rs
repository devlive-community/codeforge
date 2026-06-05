use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct JsonPlugin;

impl LanguagePlugin for JsonPlugin {
    fn get_order(&self) -> i32 {
        23
    }

    fn get_language_name(&self) -> &'static str {
        "JSON"
    }

    fn get_language_key(&self) -> &'static str {
        "json"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "json".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--"]
    }

    fn get_path_command(&self) -> String {
        "--".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("json"),
            before_compile: None,
            extension: String::from("json"),
            execute_home: None,
            // 输出原始 JSON 内容，由前端 JSON 视图（可折叠树）渲染
            run_command: Some(String::from("cat $filename")),
            after_compile: None,
            template: Some(String::from("{\n  \n}")),
            timeout: Some(30),
            console_type: Some(String::from("json")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "cat".to_string())
    }
}
