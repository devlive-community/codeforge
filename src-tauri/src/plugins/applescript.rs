use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct AppleScriptPlugin;

impl LanguagePlugin for AppleScriptPlugin {
    fn get_order(&self) -> i32 {
        15
    }

    fn get_language_name(&self) -> &'static str {
        "AppleScript"
    }

    fn get_language_key(&self) -> &'static str {
        "applescript"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "applescript".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["-e", "version of AppleScript"]
    }

    fn get_path_command(&self) -> String {
        "which osascript".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("applescript"),
            before_compile: None,
            extension: String::from("applescript"),
            execute_home: None,
            run_command: Some(String::from("osascript $filename")),
            after_compile: None,
            template: Some(String::from("-- 在这里输入 AppleScript 代码")),
            timeout: Some(45),
            console_type: Some(String::from("console")),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "osascript".to_string())
    }
}
