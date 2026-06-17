use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct FSharpPlugin;

impl LanguagePlugin for FSharpPlugin {
    fn get_order(&self) -> i32 {
        18
    }

    fn get_language_name(&self) -> &'static str {
        "F#"
    }

    fn get_language_key(&self) -> &'static str {
        "fsharp"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "fsx".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which dotnet".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("fsharp"),
            before_compile: None,
            extension: String::from("fsx"),
            execute_home: None,
            run_command: Some(String::from("dotnet fsi $filename")),
            after_compile: None,
            template: Some(String::from("printfn \"Hello, F#!\"")),
            timeout: Some(30),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "dotnet".to_string())
    }
}
