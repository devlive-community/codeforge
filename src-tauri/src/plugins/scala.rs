use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct ScalaPlugin;

impl LanguagePlugin for ScalaPlugin {
    fn get_order(&self) -> i32 {
        9
    }

    fn get_language_name(&self) -> &'static str {
        "Scala"
    }

    fn get_language_key(&self) -> &'static str {
        "scala"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "scala".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["-version"]
    }

    fn get_path_command(&self) -> String {
        "which scala".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        // Windows 使用 .bat，Unix-like 系统使用无后缀的可执行文件
        let (run_cmd, after_cmd) = if cfg!(target_os = "windows") {
            ("bin/scala.bat $filename", "del /f *.class")
        } else {
            ("bin/scala $filename", "rm -f *.class")
        };

        PluginConfig {
            enabled: true,
            language: String::from("scala"),
            before_compile: None,
            extension: String::from("scala"),
            execute_home: None,
            run_command: Some(String::from(run_cmd)),
            after_compile: Some(String::from(after_cmd)),
            template: Some(String::from("// 在这里输入 Scala 代码")),
            timeout: Some(45),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "scala".to_string())
    }
}
