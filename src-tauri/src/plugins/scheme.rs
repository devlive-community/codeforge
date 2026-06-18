use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct SchemePlugin;

impl LanguagePlugin for SchemePlugin {
    fn get_order(&self) -> i32 {
        23
    }

    fn get_language_name(&self) -> &'static str {
        "Scheme"
    }

    fn get_language_key(&self) -> &'static str {
        "scheme"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "scm".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which guile".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("scheme"),
            before_compile: None,
            extension: String::from("scm"),
            execute_home: None,
            run_command: Some(String::from("guile $filename")),
            after_compile: None,
            template: Some(String::from("(display \"Hello, Scheme!\")\n(newline)")),
            timeout: Some(30),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "guile".to_string())
    }
}
