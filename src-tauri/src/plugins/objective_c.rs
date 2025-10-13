use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct ObjectiveCPlugin;

impl LanguagePlugin for ObjectiveCPlugin {
    fn get_order(&self) -> i32 {
        27
    }

    fn get_language_name(&self) -> &'static str {
        "Objective-C"
    }

    fn get_language_key(&self) -> &'static str {
        "objective-c"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "m".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which clang".to_string()
    }

    fn get_command(
        &self,
        _file_path: Option<&str>,
        _is_version: bool,
        _file_name: Option<String>,
    ) -> String {
        if _is_version {
            let clang_command = if self.get_execute_home().is_some() {
                "./clang"
            } else {
                "clang"
            };

            return clang_command.to_string();
        }

        if let Some(config) = self.get_config() {
            if let Some(run_cmd) = &config.run_command {
                return if let Some(file_name) = _file_name {
                    run_cmd.replace("$filename", &file_name)
                } else {
                    run_cmd.clone()
                };
            }
        }
        self.get_default_command()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("objective-c"),
            before_compile: Some(String::from(
                "clang -framework Foundation $filename -o program",
            )),
            extension: String::from("m"),
            execute_home: None,
            run_command: Some(String::from("./program")),
            after_compile: Some(String::from("rm -f program")),
            template: Some(String::from(
                "// Objective-C 示例代码 - CodeForge 代码执行环境\n\n",
            )),
            timeout: Some(30),
            console_type: Some(String::from("console")),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "./program".to_string())
    }
}
