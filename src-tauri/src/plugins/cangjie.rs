use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct CangjiePlugin;

impl LanguagePlugin for CangjiePlugin {
    fn get_order(&self) -> i32 {
        24
    }

    fn get_language_name(&self) -> &'static str {
        "Cangjie"
    }

    fn get_language_key(&self) -> &'static str {
        "cangjie"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "cj".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which cjc".to_string()
    }

    fn get_command(
        &self,
        _file_path: Option<&str>,
        _is_version: bool,
        _file_name: Option<String>,
    ) -> String {
        if _is_version {
            let cjc_command = if self.get_execute_home().is_some() {
                "./cjc"
            } else {
                "cjc"
            };

            return cjc_command.to_string();
        }

        // 执行代码时
        if let Some(config) = self.get_config() {
            if let Some(run_cmd) = &config.run_command {
                return if let Some(file_name) = _file_name {
                    run_cmd.replace("$filename", &file_name)
                } else {
                    // 执行代码但没有文件名时，返回原始命令让框架处理 $filename 替换
                    run_cmd.clone()
                };
            }
        }
        self.get_default_command()
    }

    fn get_execute_args(&self, file_path: &str) -> Vec<String> {
        let cjc_command = if self.get_execute_home().is_some() {
            "./cjc"
        } else {
            "cjc"
        };

        let cmd = format!("{} {}", cjc_command, file_path);

        vec!["-c".to_string(), cmd]
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("cangjie"),
            before_compile: Some(String::from("cjc $filename -o main")),
            extension: String::from("cj"),
            execute_home: None,
            run_command: Some(String::from("main")),
            after_compile: Some(String::from("rm -f main")),
            template: Some(String::from(
                "// 在这里输入仓颉代码\n// 仓颉 - 华为开发的现代编程语言",
            )),
            timeout: Some(30),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "cjc".to_string())
    }
}
