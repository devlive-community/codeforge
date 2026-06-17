use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct TclPlugin;

impl LanguagePlugin for TclPlugin {
    fn get_order(&self) -> i32 {
        17
    }

    fn get_language_name(&self) -> &'static str {
        "Tcl"
    }

    fn get_language_key(&self) -> &'static str {
        "tcl"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "tcl".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which tclsh".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("tcl"),
            before_compile: None,
            extension: String::from("tcl"),
            execute_home: None,
            run_command: Some(String::from("tclsh $filename")),
            after_compile: None,
            template: Some(String::from("puts \"Hello, Tcl!\"")),
            timeout: Some(30),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "tclsh".to_string())
    }
}
