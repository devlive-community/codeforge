use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct JavaScriptJQueryPlugin;

impl LanguagePlugin for JavaScriptJQueryPlugin {
    fn get_order(&self) -> i32 {
        13
    }

    fn get_language_name(&self) -> &'static str {
        "JavaScript (jQuery)"
    }

    fn get_language_key(&self) -> &'static str {
        "javascript-jquery"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "js".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--"]
    }

    fn get_path_command(&self) -> String {
        "which node".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: self.get_language_key().to_string(),
            before_compile: None,
            extension: String::from("js"),
            execute_home: None,
            run_command: Some(String::from(
                "echo <script src=\"https://code.jquery.com/jquery-3.7.1.min.js\" crossorigin=\"anonymous\"></script>\n<script src=\"file://$filename\"></script>",
            )),
            after_compile: None,
            template: Some(String::from("// 在这里输入 JavaScript (jQuery) 代码")),
            timeout: Some(30),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "node".to_string())
    }
}
