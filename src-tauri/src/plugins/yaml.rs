use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct YamlPlugin;

impl LanguagePlugin for YamlPlugin {
    fn get_order(&self) -> i32 {
        25
    }

    fn get_language_name(&self) -> &'static str {
        "YAML"
    }

    fn get_language_key(&self) -> &'static str {
        "yaml"
    }

    fn get_file_extension(&self) -> String {
        // extension 可能是多个（如 "yaml,yml"），文件名取第一个
        self.get_config()
            .map(|config| {
                config
                    .extension
                    .split(',')
                    .next()
                    .unwrap_or("yaml")
                    .trim()
                    .to_string()
            })
            .unwrap_or_else(|| "yaml".to_string())
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
            language: String::from("yaml"),
            before_compile: None,
            extension: String::from("yaml,yml"),
            execute_home: None,
            // 输出原始 YAML，由前端 YAML 视图（可折叠树）渲染
            run_command: Some(String::from("cat $filename")),
            after_compile: None,
            template: Some(String::from("# 在这里输入 YAML 内容\n")),
            timeout: Some(30),
            console_type: Some(String::from("yaml")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "cat".to_string())
    }
}
