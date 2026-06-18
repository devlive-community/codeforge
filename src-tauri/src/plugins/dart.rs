use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct DartPlugin;

impl LanguagePlugin for DartPlugin {
    fn get_order(&self) -> i32 {
        29
    }

    fn get_language_name(&self) -> &'static str {
        "Dart"
    }

    fn get_language_key(&self) -> &'static str {
        "dart"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "dart".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which dart".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("dart"),
            before_compile: None,
            extension: String::from("dart"),
            execute_home: None,
            run_command: Some(String::from("dart run $filename")),
            after_compile: None,
            template: Some(String::from("void main() {\n  print('Hello, Dart!');\n}")),
            timeout: Some(30),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "dart".to_string())
    }
}
