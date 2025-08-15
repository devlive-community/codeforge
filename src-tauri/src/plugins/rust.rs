use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct RustPlugin;

impl LanguagePlugin for RustPlugin {
    fn get_order(&self) -> i32 {
        7
    }

    fn get_language_name(&self) -> &'static str {
        "Rust"
    }

    fn get_language_key(&self) -> &'static str {
        "rust"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "rs".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "rustc --print sysroot".to_string()
    }

    fn get_execute_args(&self, file_path: &str) -> Vec<String> {
        let cmd = if self.get_execute_home().is_some() {
            format!("./rustc {} -o ./main && ./main", file_path)
        } else {
            format!(
                "export PATH=$PATH:$HOME/.cargo/bin && rustc {} -o /tmp/main && /tmp/main",
                file_path
            )
        };

        vec!["-c".to_string(), cmd]
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("rust"),
            before_compile: None,
            extension: String::from("rs"),
            execute_home: None,
            run_command: Some(String::from("bash")),
            after_compile: Some(String::from("rm -f /tmp/main")),
            template: Some(String::from(
                "// Rust 示例代码 - CodeForge 代码执行环境\n\nfn main() {\n    println!(\"🎉 欢迎使用 CodeForge!\");\n    println!(\"Welcome to CodeForge!\");\n    println!(\"\");\n    \n    println!(\"=========================================\");\n    println!(\"           CodeForge Rust             \");\n    println!(\"=========================================\");\n    println!(\"\");\n    \n    // 基本输出示例\n    println!(\"✅ Rust 运行成功! (Rust is working!)\");\n    println!(\"🦀 这是 Rust 版本 (This is Rust)\");\n    println!(\"\");\n    \n    // 简单计算\n    let number1 = 10;\n    let number2 = 20;\n    let result = number1 + number2;\n    \n    println!(\"🔢 简单计算 (Simple calculation):\");\n    println!(\"{} + {} = {}\", number1, number2, result);\n    println!(\"\");\n    \n    // 字符串操作\n    let name = \"CodeForge\";\n    let version = \"Rust\";\n    \n    println!(\"📝 字符串操作 (String operations):\");\n    println!(\"平台名称 (Platform): {}\", name);\n    println!(\"语言版本 (Language): {}\", version);\n    println!(\"完整信息 (Full info): {} - {}\", name, version);\n    println!(\"\");\n    \n    // 循环示例\n    println!(\"🔄 循环输出 (Loop output):\");\n    for i in 1..=5 {\n        println!(\"第 {} 次输出 (Output #{}): Hello from CodeForge!\", i, i);\n    }\n    println!(\"\");\n    \n    // 向量操作\n    let fruits = vec![\"苹果\", \"香蕉\", \"橙子\", \"葡萄\"];\n    println!(\"🍎 水果列表 (Fruit list):\");\n    for (index, fruit) in fruits.iter().enumerate() {\n        println!(\"{}. {}\", index + 1, fruit);\n    }\n    println!(\"\");\n    \n    // 条件判断\n    let score = 85;\n    println!(\"📊 成绩评估 (Score evaluation):\");\n    if score >= 90 {\n        println!(\"优秀! (Excellent!)\");\n    } else if score >= 80 {\n        println!(\"良好! (Good!)\");\n    } else if score >= 60 {\n        println!(\"及格 (Pass)\");\n    } else {\n        println!(\"需要努力 (Need improvement)\");\n    }\n    \n    // Rust 特有的所有权演示\n    println!(\"\");\n    println!(\"🔒 Rust 所有权演示 (Ownership demonstration):\");\n    let mut message = String::from(\"Hello\");\n    message.push_str(\", CodeForge!\");\n    println!(\"可变字符串 (Mutable string): {}\", message);\n    \n    // Option 类型演示\n    let maybe_number: Option<i32> = Some(42);\n    match maybe_number {\n        Some(n) => println!(\"找到数字 (Found number): {}\", n),\n        None => println!(\"没有数字 (No number)\"),\n    }\n    \n    println!(\"\");\n    println!(\"🎯 CodeForge Rust 代码执行完成!\");\n    println!(\"🎯 CodeForge Rust execution completed!\");\n    println!(\"\");\n    println!(\"感谢使用 CodeForge 代码执行环境! 🚀\");\n    println!(\"Thank you for using CodeForge! 🚀\");\n}",
            )),
            timeout: Some(30),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "/tmp/main".to_string())
    }
}
