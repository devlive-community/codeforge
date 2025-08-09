use super::{ExecutionResult, LanguagePlugin};

pub struct Python3Plugin;

impl LanguagePlugin for Python3Plugin {
    fn get_order(&self) -> i32 {
        2
    }
    
    fn get_language_name(&self) -> &'static str {
        "Python 3"
    }

    fn get_file_extension(&self) -> &'static str {
        "py"
    }

    fn get_commands(&self) -> Vec<&'static str> {
        vec!["python", "python3"]
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_execute_args(&self, file_path: &str) -> Vec<String> {
        vec![file_path.to_string()]
    }

    fn get_path_command(&self) -> String {
        "import sys; print(sys.executable)".to_string()
    }

    fn pre_execute_hook(&self, code: &str) -> Result<String, String> {
        // 添加一些 Python 特定的预处理
        let processed_code = format!(
            "# CodeForge Python 3 Execution\n# Generated at: {}\n\n{}",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
            code
        );
        Ok(processed_code)
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
