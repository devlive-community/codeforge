use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct TsvPlugin;

impl LanguagePlugin for TsvPlugin {
    fn get_order(&self) -> i32 {
        41
    }

    fn get_language_name(&self) -> &'static str {
        "TSV"
    }

    fn get_language_key(&self) -> &'static str {
        "tsv"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "tsv".to_string())
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
            language: String::from("tsv"),
            before_compile: None,
            extension: String::from("tsv"),
            execute_home: None,
            // 输出原始 TSV 内容，由前端「数据表/图表」视图解析渲染（自动识别制表符分隔）
            run_command: Some(String::from("cat $filename")),
            after_compile: None,
            template: Some(String::from("name\tvalue\nA\t1\nB\t2\nC\t3")),
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
