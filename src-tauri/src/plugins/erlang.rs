use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct ErlangPlugin;

impl LanguagePlugin for ErlangPlugin {
    fn get_order(&self) -> i32 {
        20
    }

    fn get_language_name(&self) -> &'static str {
        "Erlang"
    }

    fn get_language_key(&self) -> &'static str {
        "erlang"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "erl".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which escript".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("erlang"),
            before_compile: None,
            extension: String::from("erl"),
            execute_home: None,
            // escript 直接执行含 main/1 的脚本
            run_command: Some(String::from("escript $filename")),
            after_compile: None,
            template: Some(String::from(
                "main(_) ->\n    io:format(\"Hello, Erlang!~n\").",
            )),
            timeout: Some(30),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "escript".to_string())
    }
}
