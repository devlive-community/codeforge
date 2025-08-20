use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct ScalaPlugin;

impl LanguagePlugin for ScalaPlugin {
    fn get_order(&self) -> i32 {
        9
    }

    fn get_language_name(&self) -> &'static str {
        "Scala"
    }

    fn get_language_key(&self) -> &'static str {
        "scala"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "scala".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["-version"]
    }

    fn get_path_command(&self) -> String {
        "which scala".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("scala"),
            before_compile: None,
            extension: String::from("scala"),
            execute_home: None,
            run_command: Some(String::from("scala $filename")),
            after_compile: Some(String::from("rm -f *.class")),
            template: Some(String::from(
                "// Scala 示例代码 - CodeForge 代码执行环境\n\nobject Main {\n  def main(args: Array[String]): Unit = {\n    println(\"🎉 欢迎使用 CodeForge!\")\n    println(\"Welcome to CodeForge!\")\n    println(\"\")\n\n    println(\"=========================================\")\n    println(\"           CodeForge Scala            \")\n    println(\"=========================================\")\n    println(\"\")\n\n    // 基本输出示例\n    println(\"✅ Scala 运行成功! (Scala is working!)\")\n    println(\"⚡ 这是 Scala 脚本 (This is Scala script)\")\n    println(\"\")\n\n    // 变量操作\n    val name = \"CodeForge\"\n    val version = \"Scala\"\n    val number1 = 10\n    val number2 = 20\n    val result = number1 + number2\n\n    println(\"🔢 简单计算 (Simple calculation):\")\n    println(s\"$number1 + $number2 = $result\")\n    println(\"\")\n\n    // 字符串操作\n    println(\"📝 字符串操作 (String operations):\")\n    println(s\"平台名称 (Platform): $name\")\n    println(s\"语言版本 (Language): $version\")\n    println(s\"完整信息 (Full info): $name - $version\")\n    println(\"\")\n\n    // 循环示例\n    println(\"🔄 循环输出 (Loop output):\")\n    for (i <- 1 to 5) {\n      println(s\"第 $i 次输出 (Output #$i): Hello from CodeForge!\")\n    }\n    println(\"\")\n\n    // 列表操作\n    val fruits = List(\"苹果\", \"香蕉\", \"橙子\", \"葡萄\")\n    println(\"🍎 水果列表 (Fruit list):\")\n    fruits.zipWithIndex.foreach { case (fruit, index) =>\n      println(s\"${index + 1}. $fruit\")\n    }\n    println(\"\")\n\n    // 条件判断\n    val score = 85\n    println(\"📊 成绩评估 (Score evaluation):\")\n    score match {\n      case s if s >= 90 => println(\"优秀! (Excellent!)\")\n      case s if s >= 80 => println(\"良好! (Good!)\")\n      case s if s >= 60 => println(\"及格 (Pass)\")\n      case _ => println(\"需要努力 (Need improvement)\")\n    }\n\n    // Option 类型示例\n    val optionalValue: Option[Int] = Some(42)\n    println(\"\")\n    println(\"🔍 Option 类型示例 (Option example):\")\n    optionalValue match {\n      case Some(value) => println(s\"可选值: $value (Optional value: $value)\")\n      case None => println(\"值为空 (Value is None)\")\n    }\n\n    // 函数示例\n    def greetUser(name: String): String = {\n      s\"Hello, $name! 👋\"\n    }\n\n    println(\"\")\n    println(\"🎭 函数示例 (Function example):\")\n    val greeting = greetUser(\"CodeForge用户\")\n    println(greeting)\n\n    // 集合操作示例\n    println(\"\")\n    println(\"🗂️ 集合操作示例 (Collection operations):\")\n    val numbers = (1 to 10).toList\n    val evenNumbers = numbers.filter(_ % 2 == 0)\n    val doubled = numbers.map(_ * 2)\n    \n    println(s\"原始数字 (Original): ${numbers.mkString(\", \")}\")\n    println(s\"偶数 (Even numbers): ${evenNumbers.mkString(\", \")}\")\n    println(s\"翻倍 (Doubled): ${doubled.mkString(\", \")}\")\n    \n    // Case Class 示例\n    case class Person(name: String, age: Int)\n    val person = Person(\"张三\", 25)\n    \n    println(\"\")\n    println(\"👤 Case Class 示例:\")\n    println(s\"姓名: ${person.name}, 年龄: ${person.age}\")\n\n    println(\"\")\n    println(\"🎯 CodeForge Scala 代码执行完成!\")\n    println(\"🎯 CodeForge Scala execution completed!\")\n    println(\"\")\n    println(\"感谢使用 CodeForge 代码执行环境! 🚀\")\n    println(\"Thank you for using CodeForge! 🚀\")\n  }\n}",
            )),
            timeout: Some(45),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "scala".to_string())
    }
}
