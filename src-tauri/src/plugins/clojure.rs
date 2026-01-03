use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct ClojurePlugin;

impl LanguagePlugin for ClojurePlugin {
    fn get_order(&self) -> i32 {
        11
    }

    fn get_language_name(&self) -> &'static str {
        "Clojure"
    }

    fn get_language_key(&self) -> &'static str {
        "clojure"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "clj".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["-e", "(println (clojure-version))"]
    }

    fn get_path_command(&self) -> String {
        "which clojure".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("clojure"),
            before_compile: None,
            extension: String::from("clj"),
            execute_home: None,
            run_command: Some(String::from("clojure $filename")),
            after_compile: None,
            template: Some(String::from(";; 在这里输入 Clojure 代码")),
            timeout: Some(45),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "clojure".to_string())
    }
}
