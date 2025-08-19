use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct Python2Plugin;

impl LanguagePlugin for Python2Plugin {
    fn get_order(&self) -> i32 {
        1
    }

    fn get_language_name(&self) -> &'static str {
        "Python 2"
    }

    fn get_language_key(&self) -> &'static str {
        "python2"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "py".to_string())
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
            language: String::from("python2"),
            before_compile: None,
            extension: String::from("py"),
            execute_home: None,
            run_command: Option::from(String::from("python2 $filename")),
            after_compile: None,
            template: Some(String::from(
                "# -*- coding: utf-8 -*-\n# Python2 示例代码 - CodeForge 代码执行环境\n\nprint u\"🎉 欢迎使用 CodeForge!\"\nprint u\"Welcome to CodeForge!\"\nprint u\"\"\n\nprint u\"=========================================\"\nprint u\"         CodeForge Python2            \"\nprint u\"=========================================\"\nprint u\"\"\n\n# 基本输出示例\nprint u\"✅ Python2 运行成功! (Python2 is working!)\"\nprint u\"🐍 这是 Python2 程序 (This is Python2 program)\"\nprint u\"\"\n\n# 变量操作\nname = u\"CodeForge\"\nversion = u\"Python2\"\nnumber1 = 10\nnumber2 = 20\nresult = number1 + number2\n\nprint u\"🔢 简单计算 (Simple calculation):\"\nprint u\"{} + {} = {}\".format(number1, number2, result)\nprint u\"\"\n\n# 字符串操作\nprint u\"📝 字符串操作 (String operations):\"\nprint u\"平台名称 (Platform): {}\".format(name)\nprint u\"语言版本 (Language): {}\".format(version)\nprint u\"完整信息 (Full info): {} - {}\".format(name, version)\nprint u\"\"\n\n# 循环示例\nprint u\"🔄 循环输出 (Loop output):\"\nfor i in range(1, 6):\n    print u\"第 {} 次输出 (Output #{}): Hello from CodeForge!\".format(i, i)\nprint u\"\"\n\n# 列表操作\nfruits = [u\"苹果\", u\"香蕉\", u\"橙子\", u\"葡萄\"]\nprint u\"🍎 水果列表 (Fruit list):\"\nfor i, fruit in enumerate(fruits):\n    print u\"{}. {}\".format(i + 1, fruit)\nprint u\"\"\n\n# 条件判断\nscore = 85\nprint u\"📊 成绩评估 (Score evaluation):\"\nif score >= 90:\n    print u\"优秀! (Excellent!)\"\nelif score >= 80:\n    print u\"良好! (Good!)\"\nelif score >= 60:\n    print u\"及格 (Pass)\"\nelse:\n    print u\"需要努力 (Need improvement)\"\n\n# 字典操作示例\nuser = {\n    u\"name\": u\"CodeForge用户\",\n    u\"age\": 25,\n    u\"skills\": [u\"Python\", u\"JavaScript\", u\"Java\"]\n}\n\nprint u\"\"\nprint u\"📦 字典操作 (Dictionary operations):\"\nprint u\"用户名: {}\".format(user[u\"name\"])\nprint u\"年龄: {}\".format(user[u\"age\"])\nprint u\"技能: {}\".format(u\", \".join(user[u\"skills\"]))\n\n# 函数示例\ndef greet_user(name):\n    return u\"Hello, {}! 👋\".format(name)\n\n# 列表推导式示例\nnumbers = [1, 2, 3, 4, 5]\nsquares = [x * x for x in numbers]\n\nprint u\"\"\nprint u\"🎭 函数和列表推导示例 (Function and list comprehension examples):\"\ngreeting = greet_user(u\"CodeForge用户\")\nprint greeting\nprint u\"数字: {}\".format(numbers)\nprint u\"平方: {}\".format(squares)\n\n# 类示例\nclass Calculator(object):\n    def __init__(self):\n        self.name = u\"CodeForge计算器\"\n    \n    def add(self, a, b):\n        return a + b\n    \n    def multiply(self, a, b):\n        return a * b\n\nprint u\"\"\nprint u\"🧮 类示例 (Class example):\"\ncalc = Calculator()\nprint u\"计算器名称: {}\".format(calc.name)\nprint u\"3 + 7 = {}\".format(calc.add(3, 7))\nprint u\"4 × 6 = {}\".format(calc.multiply(4, 6))\n\nprint u\"\"\nprint u\"🎯 CodeForge Python2 代码执行完成!\"\nprint u\"🎯 CodeForge Python2 execution completed!\"\nprint u\"\"\nprint u\"感谢使用 CodeForge 代码执行环境! 🚀\"\nprint u\"Thank you for using CodeForge! 🚀\"",
            )),
            timeout: Some(30),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command)
            .unwrap_or_else(|| "python2".to_string())
    }
}
