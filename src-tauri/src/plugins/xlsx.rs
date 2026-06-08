use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct XlsxPlugin;

impl LanguagePlugin for XlsxPlugin {
    fn get_order(&self) -> i32 {
        42
    }

    fn get_language_name(&self) -> &'static str {
        "Excel"
    }

    fn get_language_key(&self) -> &'static str {
        "xlsx"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "xlsx".to_string())
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
            language: String::from("xlsx"),
            before_compile: None,
            // 多扩展名：xlsx / xls
            extension: String::from("xlsx,xls"),
            execute_home: None,
            // Excel 为二进制，运行时输出文件路径，由前端用 SheetJS 读取解析
            run_command: Some(String::from("echo $filename")),
            after_compile: None,
            template: None,
            timeout: Some(30),
            console_type: Some(String::from("xlsx")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "echo".to_string())
    }
}
