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
            template: Some(String::from(
                "// Kotlin 示例代码 - CodeForge 代码执行环境\n\nfun main() {\n    println(\"🎉 欢迎使用 CodeForge!\")\n    println(\"Welcome to CodeForge!\")\n    println(\"\")\n\n    println(\"=========================================\")\n    println(\"           CodeForge Kotlin           \")\n    println(\"=========================================\")\n    println(\"\")\n\n    // 基本输出示例\n    println(\"✅ Kotlin 运行成功! (Kotlin is working!)\")\n    println(\"🚀 这是 Kotlin 脚本 (This is Kotlin script)\")\n    println(\"\")\n\n    // 变量操作\n    val name = \"CodeForge\"\n    val version = \"Kotlin\"\n    val number1 = 10\n    val number2 = 20\n    val result = number1 + number2\n\n    println(\"🔢 简单计算 (Simple calculation):\")\n    println(\"$number1 + $number2 = $result\")\n    println(\"\")\n\n    // 字符串操作\n    println(\"📝 字符串操作 (String operations):\")\n    println(\"平台名称 (Platform): $name\")\n    println(\"语言版本 (Language): $version\")\n    println(\"完整信息 (Full info): $name - $version\")\n    println(\"\")\n\n    // 循环示例\n    println(\"🔄 循环输出 (Loop output):\")\n    for (i in 1..5) {\n        println(\"第 $i 次输出 (Output #$i): Hello from CodeForge!\")\n    }\n    println(\"\")\n\n    // 列表操作\n    val fruits = listOf(\"苹果\", \"香蕉\", \"橙子\", \"葡萄\")\n    println(\"🍎 水果列表 (Fruit list):\")\n    fruits.forEachIndexed { index, fruit ->\n        println(\"${index + 1}. $fruit\")\n    }\n    println(\"\")\n\n    // 条件判断\n    val score = 85\n    println(\"📊 成绩评估 (Score evaluation):\")\n    when {\n        score >= 90 -> println(\"优秀! (Excellent!)\")\n        score >= 80 -> println(\"良好! (Good!)\")\n        score >= 60 -> println(\"及格 (Pass)\")\n        else -> println(\"需要努力 (Need improvement)\")\n    }\n\n    // 可空类型示例\n    var nullableValue: Int? = 42\n    println(\"\")\n    println(\"🔍 可空类型示例 (Nullable example):\")\n    nullableValue?.let { value ->\n        println(\"可空值: $value (Nullable value: $value)\")\n    } ?: println(\"值为空 (Value is null)\")\n\n    // 函数示例\n    fun greetUser(name: String): String {\n        return \"Hello, $name! 👋\"\n    }\n\n    println(\"\")\n    println(\"🎭 函数示例 (Function example):\")\n    val greeting = greetUser(\"CodeForge用户\")\n    println(greeting)\n\n    // 扩展函数示例\n    fun String.addEmoji(): String = \"✨ $this ✨\"\n    \n    println(\"\")\n    println(\"⚡ 扩展函数示例 (Extension function):\")\n    val decoratedText = \"Kotlin很棒\".addEmoji()\n    println(decoratedText)\n\n    // 数据类示例\n    data class Person(val name: String, val age: Int)\n    val person = Person(\"张三\", 25)\n    \n    println(\"\")\n    println(\"👤 数据类示例 (Data class example):\")\n    println(\"姓名: ${person.name}, 年龄: ${person.age}\")\n    \n    // 解构声明\n    val (personName, personAge) = person\n    println(\"解构后 (Destructured): $personName 今年 $personAge 岁\")\n\n    // 高阶函数示例\n    println(\"\")\n    println(\"🔧 高阶函数示例 (Higher-order function):\")\n    val numbers = (1..10).toList()\n    val evenNumbers = numbers.filter { it % 2 == 0 }\n    val doubled = numbers.map { it * 2 }\n    \n    println(\"原始数字 (Original): ${numbers.joinToString(\", \")}\")\n    println(\"偶数 (Even numbers): ${evenNumbers.joinToString(\", \")}\")\n    println(\"翻倍 (Doubled): ${doubled.joinToString(\", \")}\")\n\n    // Lambda 表达式示例\n    println(\"\")\n    println(\"🎯 Lambda 表达式 (Lambda expression):\")\n    val calculate: (Int, Int) -> Int = { a, b -> a * b }\n    val product = calculate(6, 7)\n    println(\"6 × 7 = $product\")\n\n    // 集合操作链式调用\n    println(\"\")\n    println(\"⛓️ 集合链式操作 (Collection chaining):\")\n    val result2 = (1..20)\n        .filter { it % 3 == 0 }\n        .map { \"数字: $it\" }\n        .take(3)\n        .joinToString(\" | \")\n    println(\"3的倍数前3个: $result2\")\n\n    // 字符串模板高级用法\n    println(\"\")\n    println(\"📝 字符串模板 (String template):\")\n    val items = listOf(\"代码\", \"测试\", \"部署\")\n    val status = \"进行中\"\n    println(\"任务状态: ${items.joinToString(\", \")} - $status\")\n    \n    println(\"\")\n    println(\"🎯 CodeForge Kotlin 代码执行完成!\")\n    println(\"🎯 CodeForge Kotlin execution completed!\")\n    println(\"\")\n    println(\"感谢使用 CodeForge 代码执行环境! 🚀\")\n    println(\"Thank you for using CodeForge! 🚀\")\n}",
            )),
            timeout: Some(60),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "kotlin".to_string())
    }
}
