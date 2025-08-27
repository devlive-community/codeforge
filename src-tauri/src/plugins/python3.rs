use super::{LanguagePlugin, PluginConfig};

pub struct Python3Plugin;

impl LanguagePlugin for Python3Plugin {
    fn get_order(&self) -> i32 {
        2
    }

    fn get_language_name(&self) -> &'static str {
        "Python 3"
    }

    fn get_language_key(&self) -> &'static str {
        "python3"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "py".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "import sys; print(sys.executable)".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("python3"),
            before_compile: None,
            extension: String::from("py"),
            execute_home: None,
            run_command: Option::from(String::from("python3 $filename")),
            after_compile: None,
            template: Some(String::from("# 在这里输入 Python 3 代码")),
            timeout: Some(30),
            console_type: Some(String::from("console")),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command)
            .unwrap_or_else(|| "python3".to_string())
    }
}
