use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct SqlPlugin;

impl LanguagePlugin for SqlPlugin {
    fn get_order(&self) -> i32 {
        28
    }

    fn get_language_name(&self) -> &'static str {
        "SQL"
    }

    fn get_language_key(&self) -> &'static str {
        "sql"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "sql".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "sqlite3".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("sql"),
            before_compile: None,
            extension: String::from("sql"),
            execute_home: None,
            // 在内存 SQLite 中执行脚本并以 JSON 输出（前端渲染为表格；需要本机有 sqlite3）
            run_command: Some(String::from("sqlite3 -json :memory: .read $filename")),
            after_compile: None,
            template: Some(String::from(
                "-- 在这里输入 SQL（默认在内存 SQLite 中执行）\nSELECT 'Hello, CodeForge' AS message;\n",
            )),
            timeout: Some(30),
            console_type: Some(String::from("sqltable")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "sqlite3".to_string())
    }
}
