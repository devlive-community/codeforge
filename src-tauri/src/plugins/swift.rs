use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct SwiftPlugin;

impl LanguagePlugin for SwiftPlugin {
    fn get_order(&self) -> i32 {
        8
    }

    fn get_language_name(&self) -> &'static str {
        "Swift"
    }

    fn get_language_key(&self) -> &'static str {
        "swift"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "swift".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which swift".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("swift"),
            before_compile: None,
            extension: String::from("swift"),
            execute_home: None,
            run_command: Some(String::from("swift $filename")),
            after_compile: None,
            template: Some(String::from(
                "import Foundation\n// Swift 示例代码 - CodeForge 代码执行环境\n\nprint(\"🎉 欢迎使用 CodeForge!\")\nprint(\"Welcome to CodeForge!\")\nprint(\"\")\n\nprint(\"=========================================\")\nprint(\"           CodeForge Swift            \")\nprint(\"=========================================\")\nprint(\"\")\n\n// 基本输出示例\nprint(\"✅ Swift 运行成功! (Swift is working!)\")\nprint(\"🦉 这是 Swift 脚本 (This is Swift script)\")\nprint(\"\")\n\n// 变量操作\nlet name = \"CodeForge\"\nlet version = \"Swift\"\nlet number1 = 10\nlet number2 = 20\nlet result = number1 + number2\n\nprint(\"🔢 简单计算 (Simple calculation):\")\nprint(\"\\(number1) + \\(number2) = \\(result)\")\nprint(\"\")\n\n// 字符串操作\nprint(\"📝 字符串操作 (String operations):\")\nprint(\"平台名称 (Platform): \\(name)\")\nprint(\"语言版本 (Language): \\(version)\")\nprint(\"完整信息 (Full info): \\(name) - \\(version)\")\nprint(\"\")\n\n// 循环示例\nprint(\"🔄 循环输出 (Loop output):\")\nfor i in 1...5 {\n    print(\"第 \\(i) 次输出 (Output #\\(i)): Hello from CodeForge!\")\n}\nprint(\"\")\n\n// 数组操作\nlet fruits = [\"苹果\", \"香蕉\", \"橙子\", \"葡萄\"]\nprint(\"🍎 水果列表 (Fruit list):\")\nfor (index, fruit) in fruits.enumerated() {\n    print(\"\\(index + 1). \\(fruit)\")\n}\nprint(\"\")\n\n// 条件判断\nlet score = 85\nprint(\"📊 成绩评估 (Score evaluation):\")\nif score >= 90 {\n    print(\"优秀! (Excellent!)\")\n} else if score >= 80 {\n    print(\"良好! (Good!)\")\n} else if score >= 60 {\n    print(\"及格 (Pass)\")\n} else {\n    print(\"需要努力 (Need improvement)\")\n}\n\n// 可选类型示例\nvar optionalValue: Int? = 42\nprint(\"\")\nprint(\"🔍 可选类型示例 (Optional example):\")\nif let unwrappedValue = optionalValue {\n    print(\"可选值: \\(unwrappedValue) (Optional value: \\(unwrappedValue))\")\n} else {\n    print(\"值为空 (Value is nil)\")\n}\n\n// 函数示例\nfunc greetUser(name: String) -> String {\n    return \"Hello, \\(name)! 👋\"\n}\n\nprint(\"\")\nprint(\"🎭 函数示例 (Function example):\")\nlet greeting = greetUser(name: \"CodeForge用户\")\nprint(greeting)\n\nprint(\"\")\nprint(\"🎯 CodeForge Swift 代码执行完成!\")\nprint(\"🎯 CodeForge Swift execution completed!\")\nprint(\"\")\nprint(\"感谢使用 CodeForge 代码执行环境! 🚀\")\nprint(\"Thank you for using CodeForge! 🚀\")",
            )),
            timeout: Some(30),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "swift".to_string())
    }
}
