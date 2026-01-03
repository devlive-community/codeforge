use super::{LanguagePlugin, PluginConfig};

pub struct GoPlugin;

impl LanguagePlugin for GoPlugin {
    fn get_order(&self) -> i32 {
        4
    }

    fn get_language_name(&self) -> &'static str {
        "Go"
    }

    fn get_language_key(&self) -> &'static str {
        "go"
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["version"]
    }

    fn get_path_command(&self) -> String {
        "package main\nimport (\"fmt\"\n\"runtime\")\nfunc main() { fmt.Println(runtime.GOROOT()) }"
            .to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("go"),
            before_compile: Some(String::from("go mod init temp 2>/dev/null || true")),
            extension: String::from("go"),
            execute_home: None,
            run_command: Some(String::from("go run $filename")),
            after_compile: None,
            template: Some(String::from("// 在这里输入 Go 代码")),
            timeout: Some(30),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command)
            .unwrap_or_else(|| "go run".to_string())
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "go".to_string())
    }
}
