use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct SvgPlugin;

impl LanguagePlugin for SvgPlugin {
    fn get_order(&self) -> i32 {
        22
    }

    fn get_language_name(&self) -> &'static str {
        "SVG"
    }

    fn get_language_key(&self) -> &'static str {
        "svg"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "svg".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--"]
    }

    fn get_path_command(&self) -> String {
        "--".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("svg"),
            before_compile: None,
            extension: String::from("svg"),
            execute_home: None,
            run_command: Some(String::from("cat $filename")),
            after_compile: None,
            template: Some(String::from(
                "<!-- 在这里输入 SVG 代码 -->\n<!-- SVG (Scalable Vector Graphics) 矢量图形 -->",
            )),
            timeout: Some(30),
            console_type: Some(String::from("web")),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "cat".to_string())
    }
}
