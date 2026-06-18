use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct CrystalPlugin;

impl LanguagePlugin for CrystalPlugin {
    fn get_order(&self) -> i32 {
        19
    }

    fn get_language_name(&self) -> &'static str {
        "Crystal"
    }

    fn get_language_key(&self) -> &'static str {
        "crystal"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "cr".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which crystal".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("crystal"),
            before_compile: None,
            extension: String::from("cr"),
            execute_home: None,
            run_command: Some(String::from("crystal run $filename")),
            after_compile: None,
            template: Some(String::from("puts \"Hello, Crystal!\"")),
            timeout: Some(30),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "crystal".to_string())
    }
}
