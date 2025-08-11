use super::{LanguagePlugin, PluginConfig};

pub struct NodeJSPlugin;

impl LanguagePlugin for NodeJSPlugin {
    fn get_order(&self) -> i32 {
        3
    }

    fn get_language_name(&self) -> &'static str {
        "NodeJS"
    }

    fn get_language_key(&self) -> &'static str {
        "nodejs"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "js".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "console.log(process.execPath)".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("nodejs"),
            before_compile: None,
            extension: String::from("js"),
            execute_home: None,
            run_command: Option::from(String::from("node $filename")),
            after_compile: None,
            template: None,
            timeout: Some(30),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command)
            .unwrap_or_else(|| "node".to_string())
    }
}
