use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct ShellPlugin;

impl LanguagePlugin for ShellPlugin {
    fn get_order(&self) -> i32 {
        6
    }

    fn get_language_name(&self) -> &'static str {
        "Shell"
    }

    fn get_language_key(&self) -> &'static str {
        "shell"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "sh".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which bash || which sh".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("shell"),
            before_compile: None,
            extension: String::from("sh"),
            execute_home: None,
            run_command: Some(String::from("bash $filename")),
            after_compile: None,
            template: Some(String::from(
                "#!/bin/bash\n# Shell 示例代码 - CodeForge 代码执行环境\n\necho \"🎉 欢迎使用 CodeForge!\"\necho \"Welcome to CodeForge!\"\necho \"\"\n\necho \"=========================================\"\necho \"           CodeForge Shell            \"\necho \"=========================================\"\necho \"\"\n\n# 基本输出示例\necho \"✅ Shell 运行成功! (Shell is working!)\"\necho \"🐚 这是 Shell 脚本 (This is Shell script)\"\necho \"\"\n\n# 变量操作\nname=\"CodeForge\"\nversion=\"Shell\"\nnumber1=10\nnumber2=20\nresult=$((number1 + number2))\n\necho \"🔢 简单计算 (Simple calculation):\"\necho \"$number1 + $number2 = $result\"\necho \"\"\n\n# 字符串操作\necho \"📝 字符串操作 (String operations):\"\necho \"平台名称 (Platform): $name\"\necho \"语言版本 (Language): $version\"\necho \"完整信息 (Full info): $name - $version\"\necho \"\"\n\n# 循环示例\necho \"🔄 循环输出 (Loop output):\"\nfor i in {1..5}; do\n    echo \"第 $i 次输出 (Output #$i): Hello from CodeForge!\"\ndone\necho \"\"\n\n# 数组操作\nfruits=(\"苹果\" \"香蕉\" \"橙子\" \"葡萄\")\necho \"🍎 水果列表 (Fruit list):\"\nfor i in \"${!fruits[@]}\"; do\n    echo \"$((i + 1)). ${fruits[i]}\"\ndone\necho \"\"\n\n# 条件判断\nscore=85\necho \"📊 成绩评估 (Score evaluation):\"\nif [ $score -ge 90 ]; then\n    echo \"优秀! (Excellent!)\"\nelif [ $score -ge 80 ]; then\n    echo \"良好! (Good!)\"\nelif [ $score -ge 60 ]; then\n    echo \"及格 (Pass)\"\nelse\n    echo \"需要努力 (Need improvement)\"\nfi\n\necho \"\"\necho \"🎯 CodeForge Shell 代码执行完成!\"\necho \"🎯 CodeForge Shell execution completed!\"\necho \"\"\necho \"感谢使用 CodeForge 代码执行环境! 🚀\"\necho \"Thank you for using CodeForge! 🚀\"",
            )),
            timeout: Some(30),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "bash".to_string())
    }
}
