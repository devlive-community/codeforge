use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct TextPlugin;

impl LanguagePlugin for TextPlugin {
    fn get_order(&self) -> i32 {
        27
    }

    fn get_language_name(&self) -> &'static str {
        "纯文本"
    }

    fn get_language_key(&self) -> &'static str {
        "text"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "txt".to_string())
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
            language: String::from("text"),
            before_compile: None,
            extension: String::from("txt"),
            execute_home: None,
            run_command: Some(String::from("cat $filename")),
            after_compile: None,
            template: Some(String::from("")),
            timeout: Some(30),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "cat".to_string())
    }
}
