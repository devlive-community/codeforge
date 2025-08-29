use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct RPlugin;

impl LanguagePlugin for RPlugin {
    fn get_order(&self) -> i32 {
        23
    }

    fn get_language_name(&self) -> &'static str {
        "R"
    }

    fn get_language_key(&self) -> &'static str {
        "r"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "r".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which R".to_string()
    }

    fn get_execute_args(&self, file_path: &str) -> Vec<String> {
        let cmd = format!("R --vanilla --slave -f {}", file_path);

        vec!["-c".to_string(), cmd]
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("r"),
            before_compile: None,
            extension: String::from("r"),
            execute_home: None,
            run_command: Some(String::from("bash")),
            after_compile: None,
            template: Some(String::from(
                "# 在这里输入 R 代码\n# R - 统计计算和图形的语言和环境",
            )),
            timeout: Some(30),
            console_type: Some(String::from("console")),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "R".to_string())
    }
}
