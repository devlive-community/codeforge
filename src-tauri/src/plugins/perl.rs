use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct PerlPlugin;

impl LanguagePlugin for PerlPlugin {
    fn get_order(&self) -> i32 {
        30
    }

    fn get_language_name(&self) -> &'static str {
        "Perl"
    }

    fn get_language_key(&self) -> &'static str {
        "perl"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "pl".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which perl".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("perl"),
            before_compile: None,
            extension: String::from("pl"),
            execute_home: None,
            run_command: Some(String::from("perl $filename")),
            after_compile: None,
            template: Some(String::from("print \"Hello, Perl!\\n\";")),
            timeout: Some(30),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "perl".to_string())
    }
}
