use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct TypeScriptBrowserPlugin;

impl LanguagePlugin for TypeScriptBrowserPlugin {
    fn get_order(&self) -> i32 {
        16
    }

    fn get_language_name(&self) -> &'static str {
        "TypeScript (Browser)"
    }

    fn get_language_key(&self) -> &'static str {
        "typescript-browser"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "ts".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--"]
    }

    fn get_path_command(&self) -> String {
        "which tsc".to_string()
    }

    fn get_execute_args(&self, file_path: &str) -> Vec<String> {
        if let Some(config) = self.get_config() {
            if let Some(run_cmd) = &config.run_command {
                let processed_cmd = if run_cmd.contains("$classname") {
                    let class_name = if file_path.ends_with(&self.get_file_extension().to_string())
                    {
                        file_path.replace(&self.get_file_extension().to_string(), "js")
                    } else {
                        file_path.to_string()
                    };
                    run_cmd.replace("$classname", &class_name)
                } else {
                    run_cmd.replace("$filename", file_path)
                };

                return processed_cmd
                    .split_whitespace()
                    .skip(1)
                    .map(|s| s.to_string())
                    .collect();
            }
        }
        vec![file_path.to_string()]
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("typescript-browser"),
            before_compile: Some(String::from(
                "tsc --lib es2017,dom --skipLibCheck $filename",
            )),
            extension: String::from("ts"),
            execute_home: None,
            run_command: Some(String::from(
                "echo <script src=\"file://$classname\"></script>",
            )),
            after_compile: Some(String::from("rm -f *.js")),
            template: Some(String::from("// 在这里输入 TypeScript (Browser) 代码")),
            timeout: Some(30),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "tsc".to_string())
    }
}
