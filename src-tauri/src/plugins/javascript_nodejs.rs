use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct LanguageNodeJsPlugin;

impl LanguagePlugin for LanguageNodeJsPlugin {
    fn get_order(&self) -> i32 {
        13
    }

    fn get_language_name(&self) -> &'static str {
        "JavaScript (Node.js)"
    }

    fn get_language_key(&self) -> &'static str {
        "javascript-nodejs"
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
        "which node".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("javascript-nodejs"),
            before_compile: None,
            extension: String::from("js"),
            execute_home: None,
            run_command: Some(String::from("node $filename")),
            after_compile: None,
            template: Some(String::from(
                "// JavaScript (Node.js) 示例代码 - CodeForge 代码执行环境\n\nconsole.log('🎉 欢迎使用 CodeForge!');\nconsole.log('Welcome to CodeForge!');\nconsole.log('');\n\nconsole.log('=========================================');\nconsole.log('        CodeForge JavaScript          ');\nconsole.log('=========================================');\nconsole.log('');\n\n// 基本输出示例\nconsole.log('✅ JavaScript 运行成功! (JavaScript is working!)');\nconsole.log('🌟 这是 Node.js 脚本 (This is Node.js script)');\nconsole.log('');\n\n// 变量操作\nconst name = 'CodeForge';\nconst version = 'JavaScript';\nconst number1 = 10;\nconst number2 = 20;\nconst result = number1 + number2;\n\nconsole.log('🔢 简单计算 (Simple calculation):');\nconsole.log(`${number1} + ${number2} = ${result}`);\nconsole.log('');\n\n// 字符串操作\nconsole.log('📝 字符串操作 (String operations):');\nconsole.log(`平台名称 (Platform): ${name}`);\nconsole.log(`语言版本 (Language): ${version}`);\nconsole.log(`完整信息 (Full info): ${name} - ${version}`);\nconsole.log('');\n\n// 循环示例\nconsole.log('🔄 循环输出 (Loop output):');\nfor (let i = 1; i <= 5; i++) {\n    console.log(`第 ${i} 次输出 (Output #${i}): Hello from CodeForge!`);\n}\nconsole.log('');\n\n// 数组操作\nconst fruits = ['苹果', '香蕉', '橙子', '葡萄'];\nconsole.log('🍎 水果列表 (Fruit list):');\nfruits.forEach((fruit, index) => {\n    console.log(`${index + 1}. ${fruit}`);\n});\nconsole.log('');\n\n// 条件判断\nconst score = 85;\nconsole.log('📊 成绩评估 (Score evaluation):');\nif (score >= 90) {\n    console.log('优秀! (Excellent!)');\n} else if (score >= 80) {\n    console.log('良好! (Good!)');\n} else if (score >= 60) {\n    console.log('及格 (Pass)');\n} else {\n    console.log('需要努力 (Need improvement)');\n}\nconsole.log('');\n\n// undefined 和 null 示例\nconsole.log('🔍 undefined/null 示例 (undefined/null example):');\nlet optionalValue = 42;\nif (optionalValue !== undefined && optionalValue !== null) {\n    console.log(`可选值: ${optionalValue} (Optional value: ${optionalValue})`);\n} else {\n    console.log('值为空 (Value is undefined/null)');\n}\nconsole.log('');\n\n// 函数示例\nfunction greetUser(username) {\n    return `Hello, ${username}! 👋`;\n}\n\nconsole.log('🎭 函数示例 (Function example):');\nconst greeting = greetUser('CodeForge用户');\nconsole.log(greeting);\nconsole.log('');\n\n// 箭头函数示例\nconst addNumbers = (a, b) => a + b;\nconst multiplyNumbers = (a, b) => {\n    return a * b;\n};\n\nconsole.log('⚡ 箭头函数示例 (Arrow function example):');\nconsole.log(`5 + 3 = ${addNumbers(5, 3)}`);\nconsole.log(`6 × 7 = ${multiplyNumbers(6, 7)}`);\nconsole.log('');\n\n// 数组高阶函数示例\nconsole.log('🔧 数组高阶函数示例 (Array higher-order functions):');\nconst numbers = Array.from({length: 10}, (_, i) => i + 1);\nconst evenNumbers = numbers.filter(num => num % 2 === 0);\nconst doubled = numbers.map(num => num * 2);\nconst sum = numbers.reduce((acc, num) => acc + num, 0);\n\nconsole.log(`原始数字 (Original): ${numbers.join(', ')}`);\nconsole.log(`偶数 (Even numbers): ${evenNumbers.join(', ')}`);\nconsole.log(`翻倍 (Doubled): ${doubled.join(', ')}`);\nconsole.log(`总和 (Sum): ${sum}`);\nconsole.log('');\n\n// 对象操作示例\nconsole.log('👤 对象示例 (Object example):');\nconst person = {\n    name: '张三',\n    age: 25,\n    city: '北京',\n    introduce() {\n        return `我是${this.name}，今年${this.age}岁，住在${this.city}`;\n    }\n};\n\nconsole.log(`姓名: ${person.name}, 年龄: ${person.age}, 城市: ${person.city}`);\nconsole.log(person.introduce());\nconsole.log('');\n\n// 解构赋值示例\nconsole.log('🔍 解构赋值示例 (Destructuring assignment):');\nconst {name: personName, age: personAge} = person;\nconst [first, second, ...rest] = fruits;\n\nconsole.log(`解构对象 (Destructured object): ${personName}, ${personAge}`);\nconsole.log(`解构数组 (Destructured array): ${first}, ${second}, 其他: [${rest.join(', ')}]`);\nconsole.log('');\n\n// Promise 示例\nconsole.log('🎯 Promise 示例 (Promise example):');\nconst delay = (ms) => new Promise(resolve => setTimeout(resolve, ms));\n\n// 使用 Promise\ndelay(100)\n    .then(() => {\n        console.log('Promise 已完成! (Promise completed!)');\n        return '异步操作结果';\n    })\n    .then(result => {\n        console.log(`异步结果: ${result}`);\n    })\n    .catch(error => {\n        console.error('Promise 错误:', error);\n    });\n\n// async/await 示例\nasync function asyncExample() {\n    try {\n        console.log('⏳ 开始异步操作...');\n        await delay(50);\n        console.log('✅ async/await 完成!');\n        return '异步函数结果';\n    } catch (error) {\n        console.error('async/await 错误:', error);\n    }\n}\n\n// 立即执行异步函数\n(async () => {\n    const result = await asyncExample();\n    console.log(`异步函数返回: ${result}`);\n    console.log('');\n})();\n\n// 类和继承示例\nconsole.log('🏗️ 类示例 (Class example):');\nclass Animal {\n    constructor(name, type) {\n        this.name = name;\n        this.type = type;\n    }\n    \n    speak() {\n        return `${this.name} 发出声音`;\n    }\n}\n\nclass Dog extends Animal {\n    constructor(name) {\n        super(name, '狗');\n    }\n    \n    speak() {\n        return `${this.name} 汪汪叫`;\n    }\n}\n\nconst dog = new Dog('小黄');\nconsole.log(dog.speak());\nconsole.log('');\n\n// 模块和导出示例（注释版本，因为这是单文件）\nconsole.log('📦 模块概念示例 (Module concept example):');\n// export const utilities = {\n//     formatDate: (date) => date.toLocaleDateString('zh-CN'),\n//     randomNumber: () => Math.floor(Math.random() * 100)\n// };\n\nconst utilities = {\n    formatDate: (date) => date.toLocaleDateString('zh-CN'),\n    randomNumber: () => Math.floor(Math.random() * 100)\n};\n\nconsole.log(`当前日期: ${utilities.formatDate(new Date())}`);\nconsole.log(`随机数: ${utilities.randomNumber()}`);\nconsole.log('');\n\n// JSON 操作示例\nconsole.log('📄 JSON 操作示例 (JSON operations):');\nconst data = {\n    users: [\n        {id: 1, name: 'Alice', active: true},\n        {id: 2, name: 'Bob', active: false},\n        {id: 3, name: 'Charlie', active: true}\n    ]\n};\n\nconst jsonString = JSON.stringify(data, null, 2);\nconsole.log('JSON 字符串:');\nconsole.log(jsonString);\n\nconst parsedData = JSON.parse(jsonString);\nconst activeUsers = parsedData.users.filter(user => user.active);\nconsole.log(`活跃用户: ${activeUsers.map(u => u.name).join(', ')}`);\nconsole.log('');\n\n// 错误处理示例\nconsole.log('🚨 错误处理示例 (Error handling):');\ntry {\n    const riskyOperation = () => {\n        const random = Math.random();\n        if (random < 0.5) {\n            throw new Error('随机错误发生了!');\n        }\n        return '操作成功!';\n    };\n    \n    const result2 = riskyOperation();\n    console.log(result2);\n} catch (error) {\n    console.log(`捕获错误: ${error.message}`);\n} finally {\n    console.log('错误处理完成');\n}\nconsole.log('');\n\n// 正则表达式示例\nconsole.log('🔤 正则表达式示例 (Regular expressions):');\nconst text = 'CodeForge 是一个很棒的代码执行环境! Email: contact@codeforge.com';\nconst emailRegex = /\\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Z|a-z]{2,}\\b/g;\nconst emails = text.match(emailRegex);\n\nconsole.log(`文本: ${text}`);\nconsole.log(`找到的邮箱: ${emails ? emails.join(', ') : '无'}`);\nconsole.log('');\n\nconsole.log('🎯 CodeForge JavaScript 代码执行完成!');\nconsole.log('🎯 CodeForge JavaScript execution completed!');\nconsole.log('');\nconsole.log('感谢使用 CodeForge 代码执行环境! 🚀');\nconsole.log('Thank you for using CodeForge! 🚀');",
            )),
            timeout: Some(30),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "node".to_string())
    }
}
