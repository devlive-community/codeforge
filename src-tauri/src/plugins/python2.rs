use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct Python2Plugin;

impl LanguagePlugin for Python2Plugin {
    fn get_order(&self) -> i32 {
        1
    }

    fn get_language_name(&self) -> &'static str {
        "Python 2"
    }

    fn get_language_key(&self) -> &'static str {
        "python2"
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
            language: String::from("python2"),
            before_compile: None,
            extension: String::from("py"),
            execute_home: None,
            run_command: Option::from(String::from("python2 $filename")),
            after_compile: None,
            template: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config().unwrap().run_command.unwrap()
    }
}
