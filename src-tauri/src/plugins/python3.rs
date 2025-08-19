use super::{LanguagePlugin, PluginConfig};

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
            language: String::from("python3"),
            before_compile: None,
            extension: String::from("py"),
            execute_home: None,
            run_command: Option::from(String::from("python3 $filename")),
            after_compile: None,
            template: Some(String::from(
                "#!/usr/bin/env python3\n# -*- coding: utf-8 -*-\n# Python3 示例代码 - CodeForge 代码执行环境\n\nprint(\"🎉 欢迎使用 CodeForge!\")\nprint(\"Welcome to CodeForge!\")\nprint(\"\")\n\nprint(\"=========================================\")\nprint(\"         CodeForge Python3            \")\nprint(\"=========================================\")\nprint(\"\")\n\n# 基本输出示例\nprint(\"✅ Python3 运行成功! (Python3 is working!)\")\nprint(\"🐍 这是 Python3 程序 (This is Python3 program)\")\nprint(\"\")\n\n# 变量操作\nname = \"CodeForge\"\nversion = \"Python3\"\nnumber1 = 10\nnumber2 = 20\nresult = number1 + number2\n\nprint(\"🔢 简单计算 (Simple calculation):\")\nprint(f\"{number1} + {number2} = {result}\")\nprint(\"\")\n\n# 字符串操作\nprint(\"📝 字符串操作 (String operations):\")\nprint(f\"平台名称 (Platform): {name}\")\nprint(f\"语言版本 (Language): {version}\")\nprint(f\"完整信息 (Full info): {name} - {version}\")\nprint(\"\")\n\n# 循环示例\nprint(\"🔄 循环输出 (Loop output):\")\nfor i in range(1, 6):\n    print(f\"第 {i} 次输出 (Output #{i}): Hello from CodeForge!\")\nprint(\"\")\n\n# 列表操作\nfruits = [\"苹果\", \"香蕉\", \"橙子\", \"葡萄\"]\nprint(\"🍎 水果列表 (Fruit list):\")\nfor i, fruit in enumerate(fruits, 1):\n    print(f\"{i}. {fruit}\")\nprint(\"\")\n\n# 条件判断\nscore = 85\nprint(\"📊 成绩评估 (Score evaluation):\")\nif score >= 90:\n    print(\"优秀! (Excellent!)\")\nelif score >= 80:\n    print(\"良好! (Good!)\")\nelif score >= 60:\n    print(\"及格 (Pass)\")\nelse:\n    print(\"需要努力 (Need improvement)\")\n\n# 字典操作示例\nuser = {\n    \"name\": \"CodeForge用户\",\n    \"age\": 25,\n    \"skills\": [\"Python\", \"JavaScript\", \"Java\"],\n    \"is_active\": True\n}\n\nprint(\"\")\nprint(\"📦 字典操作 (Dictionary operations):\")\nprint(f\"用户名: {user['name']}\")\nprint(f\"年龄: {user['age']}\")\nprint(f\"技能: {', '.join(user['skills'])}\")\nprint(f\"活跃状态: {'是' if user['is_active'] else '否'}\")\n\n# 函数示例\ndef greet_user(name: str) -> str:\n    \"\"\"问候用户的函数\"\"\"\n    return f\"Hello, {name}! 👋\"\n\n# Lambda函数示例\nsquare = lambda x: x ** 2\n\n# 列表推导式示例\nnumbers = [1, 2, 3, 4, 5]\nsquares = [x ** 2 for x in numbers]\neven_squares = [x ** 2 for x in numbers if x % 2 == 0]\n\nprint(\"\")\nprint(\"🎭 函数和推导式示例 (Function and comprehension examples):\")\ngreeting = greet_user(\"CodeForge用户\")\nprint(greeting)\nprint(f\"数字: {numbers}\")\nprint(f\"平方: {squares}\")\nprint(f\"偶数的平方: {even_squares}\")\nprint(f\"5的平方 (使用lambda): {square(5)}\")\n\n# 类示例（使用现代Python特性）\nclass Calculator:\n    \"\"\"简单的计算器类\"\"\"\n    \n    def __init__(self, name: str = \"CodeForge计算器\"):\n        self.name = name\n        self.history = []\n    \n    def add(self, a: float, b: float) -> float:\n        result = a + b\n        self.history.append(f\"{a} + {b} = {result}\")\n        return result\n    \n    def multiply(self, a: float, b: float) -> float:\n        result = a * b\n        self.history.append(f\"{a} × {b} = {result}\")\n        return result\n    \n    def get_history(self) -> list:\n        return self.history\n\nprint(\"\")\nprint(\"🧮 类示例 (Class example):\")\ncalc = Calculator()\nprint(f\"计算器名称: {calc.name}\")\nresult1 = calc.add(3.5, 7.2)\nresult2 = calc.multiply(4, 6)\nprint(f\"3.5 + 7.2 = {result1}\")\nprint(f\"4 × 6 = {result2}\")\nprint(f\"计算历史: {calc.get_history()}\")\n\n# 异常处理示例\nprint(\"\")\nprint(\"⚠️ 异常处理示例 (Exception handling example):\")\ntry:\n    division_result = 10 / 2\n    print(f\"10 ÷ 2 = {division_result}\")\n    # 模拟一个可能的错误\n    risky_operation = 10 / 0  # 这会引发异常\nexcept ZeroDivisionError as e:\n    print(f\"捕获到除零错误: {e}\")\nexcept Exception as e:\n    print(f\"捕获到其他错误: {e}\")\nelse:\n    print(\"没有发生异常\")\nfinally:\n    print(\"异常处理完成\")\n\nprint(\"\")\nprint(\"🎯 CodeForge Python3 代码执行完成!\")\nprint(\"🎯 CodeForge Python3 execution completed!\")\nprint(\"\")\nprint(\"感谢使用 CodeForge 代码执行环境! 🚀\")\nprint(\"Thank you for using CodeForge! 🚀\")",
            )),
            timeout: Some(30),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command)
            .unwrap_or_else(|| "python3".to_string())
    }
}
