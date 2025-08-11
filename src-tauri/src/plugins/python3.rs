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
            template: None,
            timeout: Some(30),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config().unwrap().run_command.unwrap()
    }
}
