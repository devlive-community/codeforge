use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct MarkdownPlugin;

impl LanguagePlugin for MarkdownPlugin {
    fn get_order(&self) -> i32 {
        26
    }

    fn get_language_name(&self) -> &'static str {
        "Markdown"
    }

    fn get_language_key(&self) -> &'static str {
        "markdown"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "md".to_string())
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
            language: String::from("markdown"),
            before_compile: None,
            extension: String::from("md"),
            execute_home: None,
            // 输出原始 Markdown，由前端渲染为预览
            run_command: Some(String::from("cat $filename")),
            after_compile: None,
            template: Some(String::from("# 标题\n\n在这里输入 Markdown 内容\n")),
            timeout: Some(30),
            console_type: Some(String::from("markdown")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "cat".to_string())
    }
}
