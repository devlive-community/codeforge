use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct ObjectiveCppPlugin;

impl LanguagePlugin for ObjectiveCppPlugin {
    fn get_order(&self) -> i32 {
        28
    }

    fn get_language_name(&self) -> &'static str {
        "Objective-C++"
    }

    fn get_language_key(&self) -> &'static str {
        "objective-cpp"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "mm".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which clang++".to_string()
    }

    fn get_command(
        &self,
        _file_path: Option<&str>,
        _is_version: bool,
        _file_name: Option<String>,
    ) -> String {
        if _is_version {
            let clang_command = if self.get_execute_home().is_some() {
                "./clang++"
            } else {
                "clang++"
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
            language: String::from("objective-cpp"),
            before_compile: Some(String::from(
                "clang++ -framework Foundation $filename -o program",
            )),
            extension: String::from("mm"),
            execute_home: None,
            run_command: Some(String::from("./program")),
            after_compile: Some(String::from("rm -f program")),
            template: Some(String::from(
                "// Objective-C++ 示例代码 - CodeForge 代码执行环境\n\n",
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
