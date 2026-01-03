use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct KotlinPlugin;

impl LanguagePlugin for KotlinPlugin {
    fn get_order(&self) -> i32 {
        10
    }

    fn get_language_name(&self) -> &'static str {
        "Kotlin"
    }

    fn get_language_key(&self) -> &'static str {
        "kotlin"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "kt".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["-version"]
    }

    fn get_path_command(&self) -> String {
        "which kotlinc".to_string()
    }

    fn get_execute_args(&self, file_path: &str) -> Vec<String> {
        if let Some(config) = self.get_config() {
            if let Some(run_cmd) = &config.run_command {
                let processed_cmd = if run_cmd.contains("$classname") {
                    // 为 Kotlin 生成类名：Codeforge_kotlin.kt -> Codeforge_kotlinKt
                    let class_name = std::path::Path::new(file_path)
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string()
                        + "Kt";
                    run_cmd.replace("$classname", &class_name)
                } else {
                    run_cmd.replace("$filename", file_path)
                };

                return processed_cmd
                    .split_whitespace()
                    .skip(1) // 跳过命令部分，只返回参数
                    .map(|s| s.to_string())
                    .collect();
            }
        }
        // 默认情况下，文件路径就是唯一的参数
        vec![file_path.to_string()]
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("kotlin"),
            before_compile: Some(String::from("kotlinc $filename")),
            extension: String::from("kt"),
            execute_home: None,
            run_command: Some(String::from("kotlin $classname")),
            after_compile: Some(String::from("rm -f *.class")),
            template: Some(String::from("// 在这里输入 Kotlin 代码")),
            timeout: Some(60),
            console_type: Some(String::from("console")),
            icon_path: None,
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "kotlin".to_string())
    }
}
