use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct JavaPlugin;

impl LanguagePlugin for JavaPlugin {
    fn get_order(&self) -> i32 {
        5
    }

    fn get_language_name(&self) -> &'static str {
        "Java"
    }

    fn get_language_key(&self) -> &'static str {
        "java"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "java".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["-version"]
    }

    fn get_path_command(&self) -> String {
        "System.out.println(System.getProperty(\"java.home\"));".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("java"),
            before_compile: None,
            extension: String::from("java"),
            execute_home: None,
            run_command: Some(String::from("java $filename")),
            after_compile: Some(String::from("rm -f *.class")),
            template: Some(String::from(
                "import java.util.*;\n\n// Java 示例代码 - CodeForge 代码执行环境\n\npublic class Main {\n    public static void main(String[] args) {\n        System.out.println(\"🎉 欢迎使用 CodeForge!\");\n        System.out.println(\"Welcome to CodeForge!\");\n        System.out.println(\"\");\n\n        System.out.println(\"=========================================\");\n        System.out.println(\"           CodeForge Java             \");\n        System.out.println(\"=========================================\");\n        System.out.println(\"\");\n\n        // 基本输出示例\n        System.out.println(\"✅ Java 运行成功! (Java is working!)\");\n        System.out.println(\"☕ 这是 Java 程序 (This is Java program)\");\n        System.out.println(\"\");\n\n        // 变量操作\n        String name = \"CodeForge\";\n        String version = \"Java\";\n        int number1 = 10;\n        int number2 = 20;\n        int result = number1 + number2;\n\n        System.out.println(\"🔢 简单计算 (Simple calculation):\");\n        System.out.printf(\"%d + %d = %d%n\", number1, number2, result);\n        System.out.println(\"\");\n\n        // 字符串操作\n        System.out.println(\"📝 字符串操作 (String operations):\");\n        System.out.println(\"平台名称 (Platform): \" + name);\n        System.out.println(\"语言版本 (Language): \" + version);\n        System.out.println(\"完整信息 (Full info): \" + name + \" - \" + version);\n        System.out.println(\"\");\n\n        // 循环示例\n        System.out.println(\"🔄 循环输出 (Loop output):\");\n        for (int i = 1; i <= 5; i++) {\n            System.out.printf(\"第 %d 次输出 (Output #%d): Hello from CodeForge!%n\", i, i);\n        }\n        System.out.println(\"\");\n\n        // 数组操作\n        String[] fruits = {\"苹果\", \"香蕉\", \"橙子\", \"葡萄\"};\n        System.out.println(\"🍎 水果列表 (Fruit list):\");\n        for (int i = 0; i < fruits.length; i++) {\n            System.out.printf(\"%d. %s%n\", i + 1, fruits[i]);\n        }\n        System.out.println(\"\");\n\n        // 条件判断\n        int score = 85;\n        System.out.println(\"📊 成绩评估 (Score evaluation):\");\n        if (score >= 90) {\n            System.out.println(\"优秀! (Excellent!)\");\n        } else if (score >= 80) {\n            System.out.println(\"良好! (Good!)\");\n        } else if (score >= 60) {\n            System.out.println(\"及格 (Pass)\");\n        } else {\n            System.out.println(\"需要努力 (Need improvement)\");\n        }\n\n        // 集合操作示例\n        List<String> languages = new ArrayList<>();\n        languages.add(\"Java\");\n        languages.add(\"Python\");\n        languages.add(\"JavaScript\");\n        languages.add(\"Go\");\n\n        System.out.println(\"\");\n        System.out.println(\"📋 编程语言列表 (Programming languages):\");\n        for (int i = 0; i < languages.size(); i++) {\n            System.out.printf(\"%d. %s%n\", i + 1, languages.get(i));\n        }\n\n        // 方法调用示例\n        String greeting = greetUser(\"CodeForge用户\");\n        System.out.println(\"\");\n        System.out.println(\"🎭 方法示例 (Method example):\");\n        System.out.println(greeting);\n\n        System.out.println(\"\");\n        System.out.println(\"🎯 CodeForge Java 代码执行完成!\");\n        System.out.println(\"🎯 CodeForge Java execution completed!\");\n        System.out.println(\"\");\n        System.out.println(\"感谢使用 CodeForge 代码执行环境! 🚀\");\n        System.out.println(\"Thank you for using CodeForge! 🚀\");\n    }\n\n    public static String greetUser(String name) {\n        return String.format(\"Hello, %s! 👋\", name);\n    }\n}",
            )),
            timeout: Some(30),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "java".to_string())
    }
}
