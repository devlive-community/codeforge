use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct CsvPlugin;

impl LanguagePlugin for CsvPlugin {
    fn get_order(&self) -> i32 {
        40
    }

    fn get_language_name(&self) -> &'static str {
        "CSV"
    }

    fn get_language_key(&self) -> &'static str {
        "csv"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "csv".to_string())
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
            language: String::from("csv"),
            before_compile: None,
            extension: String::from("csv"),
            execute_home: None,
            // 输出原始 CSV 内容，由前端「数据表/图表」视图解析渲染
            run_command: Some(String::from("cat $filename")),
            after_compile: None,
            template: Some(String::from("name,value\nA,1\nB,2\nC,3")),
            timeout: Some(30),
            console_type: Some(String::from("table")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "cat".to_string())
    }
}
