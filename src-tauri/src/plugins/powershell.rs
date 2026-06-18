use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct PowerShellPlugin;

impl LanguagePlugin for PowerShellPlugin {
    fn get_order(&self) -> i32 {
        16
    }

    fn get_language_name(&self) -> &'static str {
        "PowerShell"
    }

    fn get_language_key(&self) -> &'static str {
        "powershell"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "ps1".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which pwsh".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("powershell"),
            before_compile: None,
            extension: String::from("ps1"),
            execute_home: None,
            run_command: Some(String::from("pwsh -File $filename")),
            after_compile: None,
            template: Some(String::from("Write-Output \"Hello, PowerShell!\"")),
            timeout: Some(30),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "pwsh".to_string())
    }
}
