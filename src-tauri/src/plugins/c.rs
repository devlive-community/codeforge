use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct CPlugin;

impl LanguagePlugin for CPlugin {
    fn get_order(&self) -> i32 {
        12
    }

    fn get_language_name(&self) -> &'static str {
        "C"
    }

    fn get_language_key(&self) -> &'static str {
        "c"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "c".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which gcc".to_string()
    }

    fn get_command(
        &self,
        _file_path: Option<&str>,
        _is_version: bool,
        _file_name: Option<String>,
    ) -> String {
        if _is_version {
            // 获取版本信息时，返回编译器命令
            return "gcc".to_string();
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

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("c"),
            before_compile: Some(String::from("gcc $filename -o $filename")),
            extension: String::from("c"),
            execute_home: None,
            run_command: Some(String::from("$filename")),
            after_compile: Some(String::from("rm -f $filename")),
            template: Some(String::from("// 在这里输入 C 代码")),
            timeout: Some(30),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "gcc".to_string())
    }
}
