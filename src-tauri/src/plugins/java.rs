use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct JavaPlugin;

impl LanguagePlugin for JavaPlugin {
    fn get_order(&self) -> i32 {
        5
    }

    fn get_language_name(&self) -> &'static str {
        "Java"
    }

    fn get_language_key(&self) -> &'static str {
        "java"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "java".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["-version"]
    }

    fn get_path_command(&self) -> String {
        "System.out.println(System.getProperty(\"java.home\"));".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("java"),
            before_compile: None,
            extension: String::from("java"),
            execute_home: None,
            run_command: Some(String::from("java $filename")),
            after_compile: Some(String::from("rm -f *.class")),
            template: Some(String::from("// 在这里输入 Java 代码")),
            timeout: Some(30),
            console_type: Some(String::from("console")),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "java".to_string())
    }
}
