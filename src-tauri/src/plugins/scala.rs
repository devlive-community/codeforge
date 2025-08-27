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
        PluginConfig {
            enabled: true,
            language: String::from("scala"),
            before_compile: None,
            extension: String::from("scala"),
            execute_home: None,
            run_command: Some(String::from("scala $filename")),
            after_compile: Some(String::from("rm -f *.class")),
            template: Some(String::from("// 在这里输入 Scala 代码")),
            timeout: Some(45),
            console_type: Some(String::from("console")),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "scala".to_string())
    }
}
