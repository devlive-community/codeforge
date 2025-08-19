use super::{LanguagePlugin, PluginConfig};

pub struct GoPlugin;

impl LanguagePlugin for GoPlugin {
    fn get_order(&self) -> i32 {
        4
    }

    fn get_language_name(&self) -> &'static str {
        "Go"
    }

    fn get_language_key(&self) -> &'static str {
        "go"
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["version"]
    }

    fn get_path_command(&self) -> String {
        "package main\nimport (\"fmt\"\n\"runtime\")\nfunc main() { fmt.Println(runtime.GOROOT()) }"
            .to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("go"),
            before_compile: Some(String::from("go mod init temp 2>/dev/null || true")),
            extension: String::from("go"),
            execute_home: None,
            run_command: Some(String::from("go run $filename")),
            after_compile: None,
            template: Some(String::from(
                "package main\n\nimport (\n\t\"fmt\"\n)\n\n// Go 示例代码 - CodeForge 代码执行环境\n\nfunc main() {\n\tfmt.Println(\"🎉 欢迎使用 CodeForge!\")\n\tfmt.Println(\"Welcome to CodeForge!\")\n\tfmt.Println(\"\")\n\n\tfmt.Println(\"=========================================\")\n\tfmt.Println(\"           CodeForge Go               \")\n\tfmt.Println(\"=========================================\")\n\tfmt.Println(\"\")\n\n\t// 基本输出示例\n\tfmt.Println(\"✅ Go 运行成功! (Go is working!)\")\n\tfmt.Println(\"🐹 这是 Go 程序 (This is Go program)\")\n\tfmt.Println(\"\")\n\n\t// 变量操作\n\tname := \"CodeForge\"\n\tversion := \"Go\"\n\tnumber1 := 10\n\tnumber2 := 20\n\tresult := number1 + number2\n\n\tfmt.Println(\"🔢 简单计算 (Simple calculation):\")\n\tfmt.Printf(\"%d + %d = %d\\n\", number1, number2, result)\n\tfmt.Println(\"\")\n\n\t// 字符串操作\n\tfmt.Println(\"📝 字符串操作 (String operations):\")\n\tfmt.Printf(\"平台名称 (Platform): %s\\n\", name)\n\tfmt.Printf(\"语言版本 (Language): %s\\n\", version)\n\tfmt.Printf(\"完整信息 (Full info): %s - %s\\n\", name, version)\n\tfmt.Println(\"\")\n\n\t// 循环示例\n\tfmt.Println(\"🔄 循环输出 (Loop output):\")\n\tfor i := 1; i <= 5; i++ {\n\t\tfmt.Printf(\"第 %d 次输出 (Output #%d): Hello from CodeForge!\\n\", i, i)\n\t}\n\tfmt.Println(\"\")\n\n\t// 切片操作\n\tfruits := []string{\"苹果\", \"香蕉\", \"橙子\", \"葡萄\"}\n\tfmt.Println(\"🍎 水果列表 (Fruit list):\")\n\tfor i, fruit := range fruits {\n\t\tfmt.Printf(\"%d. %s\\n\", i+1, fruit)\n\t}\n\tfmt.Println(\"\")\n\n\t// 条件判断\n\tscore := 85\n\tfmt.Println(\"📊 成绩评估 (Score evaluation):\")\n\tif score >= 90 {\n\t\tfmt.Println(\"优秀! (Excellent!)\")\n\t} else if score >= 80 {\n\t\tfmt.Println(\"良好! (Good!)\")\n\t} else if score >= 60 {\n\t\tfmt.Println(\"及格 (Pass)\")\n\t} else {\n\t\tfmt.Println(\"需要努力 (Need improvement)\")\n\t}\n\n\t// 指针示例\n\tvalue := 42\n\tptr := &value\n\tfmt.Println(\"\")\n\tfmt.Println(\"🔍 指针示例 (Pointer example):\")\n\tfmt.Printf(\"值: %d, 地址: %p (Value: %d, Address: %p)\\n\", *ptr, ptr, *ptr, ptr)\n\n\t// 函数示例\n\tgreeting := greetUser(\"CodeForge用户\")\n\tfmt.Println(\"\")\n\tfmt.Println(\"🎭 函数示例 (Function example):\")\n\tfmt.Println(greeting)\n\n\tfmt.Println(\"\")\n\tfmt.Println(\"🎯 CodeForge Go 代码执行完成!\")\n\tfmt.Println(\"🎯 CodeForge Go execution completed!\")\n\tfmt.Println(\"\")\n\tfmt.Println(\"感谢使用 CodeForge 代码执行环境! 🚀\")\n\tfmt.Println(\"Thank you for using CodeForge! 🚀\")\n}\n\nfunc greetUser(name string) string {\n\treturn fmt.Sprintf(\"Hello, %s! 👋\", name)\n}",
            )),
            timeout: Some(30),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command)
            .unwrap_or_else(|| "go run".to_string())
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "go".to_string())
    }
}
