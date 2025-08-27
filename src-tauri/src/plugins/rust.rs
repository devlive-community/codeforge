use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct RustPlugin;

impl LanguagePlugin for RustPlugin {
    fn get_order(&self) -> i32 {
        7
    }

    fn get_language_name(&self) -> &'static str {
        "Rust"
    }

    fn get_language_key(&self) -> &'static str {
        "rust"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "rs".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "rustc --print sysroot".to_string()
    }

    fn get_execute_args(&self, file_path: &str) -> Vec<String> {
        let cmd = if self.get_execute_home().is_some() {
            format!("./rustc {} -o ./main && ./main", file_path)
        } else {
            format!(
                "export PATH=$PATH:$HOME/.cargo/bin && rustc {} -o /tmp/main && /tmp/main",
                file_path
            )
        };

        vec!["-c".to_string(), cmd]
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("rust"),
            before_compile: None,
            extension: String::from("rs"),
            execute_home: None,
            run_command: Some(String::from("bash")),
            after_compile: Some(String::from("rm -f /tmp/main")),
            template: Some(String::from("# 在这里输入 Rust 代码")),
            timeout: Some(30),
            console_type: Some(String::from("console")),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "/tmp/main".to_string())
    }
}
