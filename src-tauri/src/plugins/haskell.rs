use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct HaskellPlugin;

impl LanguagePlugin for HaskellPlugin {
    fn get_order(&self) -> i32 {
        25
    }

    fn get_language_name(&self) -> &'static str {
        "Haskell"
    }

    fn get_language_key(&self) -> &'static str {
        "haskell"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "hs".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which ghc".to_string()
    }

    fn get_command(
        &self,
        _file_path: Option<&str>,
        _is_version: bool,
        _file_name: Option<String>,
    ) -> String {
        if _is_version {
            let ghc_command = if self.get_execute_home().is_some() {
                "./ghc"
            } else {
                "ghc"
            };

            return ghc_command.to_string();
        }

        // 执行代码时
        if let Some(config) = self.get_config() {
            if let Some(run_cmd) = &config.run_command {
                return if let Some(file_name) = _file_name {
                    run_cmd.replace("$filename", &file_name)
                } else {
                    // 执行代码但没有文件名时，返回原始命令让框架处理 $filename 替换
                    run_cmd.clone()
                };
            }
        }
        self.get_default_command()
    }

    fn get_execute_args(&self, file_path: &str) -> Vec<String> {
        let ghc_command = if self.get_execute_home().is_some() {
            "./ghc"
        } else {
            "ghc"
        };

        // 对于 Haskell，通常先编译再运行
        // 这里假设编译后的可执行文件名为 main
        let cmd = format!("{} {} -o main && ./main", ghc_command, file_path);

        vec!["-c".to_string(), cmd]
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("haskell"),
            before_compile: Some(String::from("ghc $filename -o /tmp/main")),
            extension: String::from("hs"),
            execute_home: None,
            run_command: Some(String::from("/tmp/main")),
            after_compile: Some(String::from("rm -f /tmp/main /tmp/main.hi /tmp/main.o")),
            template: Some(String::from(
                "-- 在这里输入 Haskell 代码\n-- Haskell - 纯函数式编程语言\n",
            )),
            timeout: Some(30),
            console_type: Some(String::from("console")),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "./main".to_string())
    }
}
