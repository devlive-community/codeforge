use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct JuliaPlugin;

impl LanguagePlugin for JuliaPlugin {
    fn get_order(&self) -> i32 {
        31
    }

    fn get_language_name(&self) -> &'static str {
        "Julia"
    }

    fn get_language_key(&self) -> &'static str {
        "julia"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "jl".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which julia".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("julia"),
            before_compile: None,
            extension: String::from("jl"),
            execute_home: None,
            run_command: Some(String::from("julia $filename")),
            after_compile: None,
            template: Some(String::from("println(\"Hello, Julia!\")")),
            timeout: Some(30),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "julia".to_string())
    }
}
