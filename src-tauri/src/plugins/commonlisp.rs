use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct CommonLispPlugin;

impl LanguagePlugin for CommonLispPlugin {
    fn get_order(&self) -> i32 {
        22
    }

    fn get_language_name(&self) -> &'static str {
        "Common Lisp"
    }

    fn get_language_key(&self) -> &'static str {
        "commonlisp"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "lisp".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which sbcl".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("commonlisp"),
            before_compile: None,
            extension: String::from("lisp"),
            execute_home: None,
            run_command: Some(String::from("sbcl --script $filename")),
            after_compile: None,
            template: Some(String::from("(format t \"Hello, Common Lisp!~%\")")),
            timeout: Some(30),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "sbcl".to_string())
    }
}
