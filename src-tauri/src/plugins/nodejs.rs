use super::{LanguagePlugin, PluginConfig};

pub struct NodeJSPlugin;

impl LanguagePlugin for NodeJSPlugin {
    fn get_order(&self) -> i32 {
        3
    }

    fn get_language_name(&self) -> &'static str {
        "Node.js"
    }

    fn get_language_key(&self) -> &'static str {
        "nodejs"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "js".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "console.log(process.execPath)".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("nodejs"),
            before_compile: None,
            extension: String::from("js"),
            execute_home: None,
            run_command: Option::from(String::from("node $filename")),
            after_compile: None,
            template: Some(String::from(
                "// Node.js 示例代码 - CodeForge 代码执行环境\n\nconsole.log(\"🎉 欢迎使用 CodeForge!\");\nconsole.log(\"Welcome to CodeForge!\");\nconsole.log(\"\");\n\nconsole.log(\"=========================================\");\nconsole.log(\"         CodeForge Node.js            \");\nconsole.log(\"=========================================\");\nconsole.log(\"\");\n\n// 基本输出示例\nconsole.log(\"✅ Node.js 运行成功! (Node.js is working!)\");\nconsole.log(\"🟢 这是 JavaScript 程序 (This is JavaScript program)\");\nconsole.log(\"\");\n\n// 变量操作\nconst name = \"CodeForge\";\nconst version = \"Node.js\";\nlet number1 = 10;\nlet number2 = 20;\nlet result = number1 + number2;\n\nconsole.log(\"🔢 简单计算 (Simple calculation):\");\nconsole.log(`${number1} + ${number2} = ${result}`);\nconsole.log(\"\");\n\n// 字符串操作\nconsole.log(\"📝 字符串操作 (String operations):\");\nconsole.log(`平台名称 (Platform): ${name}`);\nconsole.log(`语言版本 (Language): ${version}`);\nconsole.log(`完整信息 (Full info): ${name} - ${version}`);\nconsole.log(\"\");\n\n// 循环示例\nconsole.log(\"🔄 循环输出 (Loop output):\");\nfor (let i = 1; i <= 5; i++) {\n    console.log(`第 ${i} 次输出 (Output #${i}): Hello from CodeForge!`);\n}\nconsole.log(\"\");\n\n// 数组操作\nconst fruits = [\"苹果\", \"香蕉\", \"橙子\", \"葡萄\"];\nconsole.log(\"🍎 水果列表 (Fruit list):\");\nfruits.forEach((fruit, index) => {\n    console.log(`${index + 1}. ${fruit}`);\n});\nconsole.log(\"\");\n\n// 条件判断\nconst score = 85;\nconsole.log(\"📊 成绩评估 (Score evaluation):\");\nif (score >= 90) {\n    console.log(\"优秀! (Excellent!)\");\n} else if (score >= 80) {\n    console.log(\"良好! (Good!)\");\n} else if (score >= 60) {\n    console.log(\"及格 (Pass)\");\n} else {\n    console.log(\"需要努力 (Need improvement)\");\n}\n\n// 对象操作示例\nconst user = {\n    name: \"CodeForge用户\",\n    age: 25,\n    skills: [\"JavaScript\", \"Node.js\", \"React\"]\n};\n\nconsole.log(\"\");\nconsole.log(\"📦 对象操作 (Object operations):\");\nconsole.log(`用户名: ${user.name}`);\nconsole.log(`年龄: ${user.age}`);\nconsole.log(`技能: ${user.skills.join(\", \")}`);\n\n// 函数示例\nfunction greetUser(name) {\n    return `Hello, ${name}! 👋`;\n}\n\n// 箭头函数示例\nconst calculateSquare = (num) => num * num;\n\nconsole.log(\"\");\nconsole.log(\"🎭 函数示例 (Function examples):\");\nconst greeting = greetUser(\"CodeForge用户\");\nconsole.log(greeting);\nconsole.log(`5 的平方是: ${calculateSquare(5)}`);\n\n// Promise 示例\nconst delay = (ms) => new Promise(resolve => setTimeout(resolve, ms));\n\nasync function asyncExample() {\n    console.log(\"\");\n    console.log(\"⏱️  异步操作示例 (Async operation example):\");\n    console.log(\"开始异步操作... (Starting async operation...)\");\n    await delay(100);\n    console.log(\"异步操作完成! (Async operation completed!)\");\n}\n\n// 执行异步示例\nasyncExample().then(() => {\n    console.log(\"\");\n    console.log(\"🎯 CodeForge Node.js 代码执行完成!\");\n    console.log(\"🎯 CodeForge Node.js execution completed!\");\n    console.log(\"\");\n    console.log(\"感谢使用 CodeForge 代码执行环境! 🚀\");\n    console.log(\"Thank you for using CodeForge! 🚀\");\n});",
            )),
            timeout: Some(30),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command)
            .unwrap_or_else(|| "node".to_string())
    }
}
