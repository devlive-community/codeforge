use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct OCamlPlugin;

impl LanguagePlugin for OCamlPlugin {
    fn get_order(&self) -> i32 {
        25
    }

    fn get_language_name(&self) -> &'static str {
        "OCaml"
    }

    fn get_language_key(&self) -> &'static str {
        "ocaml"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "ml".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["-version"]
    }

    fn get_path_command(&self) -> String {
        "which ocaml".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("ocaml"),
            before_compile: None,
            extension: String::from("ml"),
            execute_home: None,
            run_command: Some(String::from("ocaml $filename")),
            after_compile: None,
            template: Some(String::from("let () = print_endline \"Hello, OCaml!\"")),
            timeout: Some(30),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "ocaml".to_string())
    }
}
