use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct DPlugin;

impl LanguagePlugin for DPlugin {
    fn get_order(&self) -> i32 {
        21
    }

    fn get_language_name(&self) -> &'static str {
        "D"
    }

    fn get_language_key(&self) -> &'static str {
        "d"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "d".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which rdmd".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("d"),
            before_compile: None,
            extension: String::from("d"),
            execute_home: None,
            // rdmd 直接编译并运行单文件 D 脚本
            run_command: Some(String::from("rdmd $filename")),
            after_compile: None,
            template: Some(String::from(
                "import std.stdio;\n\nvoid main() {\n    writeln(\"Hello, D!\");\n}",
            )),
            timeout: Some(30),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "rdmd".to_string())
    }
}
