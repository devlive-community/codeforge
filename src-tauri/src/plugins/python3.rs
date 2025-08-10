use super::{ExecutionResult, LanguagePlugin, PluginConfig};

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
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config().unwrap().run_command.unwrap()
    }

    fn post_execute_hook(&self, result: &mut ExecutionResult) -> Result<(), String> {
        // Python 特定的后处理
        if result.success && result.stdout.is_empty() && result.stderr.is_empty() {
            result.stdout = "代码执行成功 (无输出)".to_string();
        }

        // 清理 Python 特定的错误信息
        if !result.stderr.is_empty() {
            result.stderr = result
                .stderr
                .replace("Traceback (most recent call last):", "Error:");
        }

        Ok(())
    }
}
