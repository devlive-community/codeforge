use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct CPlugin;

impl LanguagePlugin for CPlugin {
    fn get_order(&self) -> i32 {
        12
    }

    fn get_language_name(&self) -> &'static str {
        "C"
    }

    fn get_language_key(&self) -> &'static str {
        "c"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "c".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which gcc".to_string()
    }

    fn get_command(
        &self,
        _file_path: Option<&str>,
        _is_version: bool,
        _file_name: Option<String>,
    ) -> String {
        if _is_version {
            // 获取版本信息时，返回编译器命令
            return "gcc".to_string();
        }

        // 执行代码时
        if let Some(config) = self.get_config() {
            if let Some(run_cmd) = &config.run_command {
                return if let Some(file_name) = _file_name {
                    run_cmd.replace("$filename", &file_name)
                } else {
                    // 执行代码但没有文件名时，返回原始命令让框架处理 $filename 替换
                    run_cmd.clone()
                };
            }
        }
        self.get_default_command()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("c"),
            before_compile: Some(String::from("gcc $filename -o $filename")),
            extension: String::from("c"),
            execute_home: None,
            run_command: Some(String::from("$filename")),
            after_compile: Some(String::from("rm -f $filename")),
            template: Some(String::from(
                "#include <stdio.h>\n#include <stdlib.h>\n#include <string.h>\n#include <math.h>\n\n// C 示例代码 - CodeForge 代码执行环境\n\n// 函数声明\nvoid greet_user(const char* name);\nint add_numbers(int a, int b);\nvoid print_array(int arr[], int size);\nint fibonacci(int n);\n\nint main() {\n    printf(\"🎉 欢迎使用 CodeForge!\\n\");\n    printf(\"Welcome to CodeForge!\\n\");\n    printf(\"\\n\");\n\n    printf(\"=========================================\\n\");\n    printf(\"            CodeForge C               \\n\");\n    printf(\"=========================================\\n\");\n    printf(\"\\n\");\n\n    // 基本输出示例\n    printf(\"✅ C 运行成功! (C is working!)\\n\");\n    printf(\"⚡ 这是 C 程序 (This is C program)\\n\");\n    printf(\"\\n\");\n\n    // 变量操作\n    const char* name = \"CodeForge\";\n    const char* version = \"C\";\n    int number1 = 10;\n    int number2 = 20;\n    int result = add_numbers(number1, number2);\n\n    printf(\"🔢 简单计算 (Simple calculation):\\n\");\n    printf(\"%d + %d = %d\\n\", number1, number2, result);\n    printf(\"\\n\");\n\n    // 字符串操作\n    printf(\"📝 字符串操作 (String operations):\\n\");\n    printf(\"平台名称 (Platform): %s\\n\", name);\n    printf(\"语言版本 (Language): %s\\n\", version);\n    printf(\"完整信息 (Full info): %s - %s\\n\", name, version);\n    printf(\"\\n\");\n\n    // 循环示例\n    printf(\"🔄 循环输出 (Loop output):\\n\");\n    for (int i = 1; i <= 5; i++) {\n        printf(\"第 %d 次输出 (Output #%d): Hello from CodeForge!\\n\", i, i);\n    }\n    printf(\"\\n\");\n\n    // 数组操作\n    printf(\"🍎 数组示例 (Array example):\\n\");\n    char* fruits[] = {\"苹果\", \"香蕉\", \"橙子\", \"葡萄\"};\n    int fruits_count = sizeof(fruits) / sizeof(fruits[0]);\n    for (int i = 0; i < fruits_count; i++) {\n        printf(\"%d. %s\\n\", i + 1, fruits[i]);\n    }\n    printf(\"\\n\");\n\n    // 条件判断\n    int score = 85;\n    printf(\"📊 成绩评估 (Score evaluation):\\n\");\n    if (score >= 90) {\n        printf(\"优秀! (Excellent!)\\n\");\n    } else if (score >= 80) {\n        printf(\"良好! (Good!)\\n\");\n    } else if (score >= 60) {\n        printf(\"及格 (Pass)\\n\");\n    } else {\n        printf(\"需要努力 (Need improvement)\\n\");\n    }\n    printf(\"\\n\");\n\n    // 指针示例\n    printf(\"🔍 指针示例 (Pointer example):\\n\");\n    int value = 42;\n    int* ptr = &value;\n    printf(\"值: %d (Value: %d)\\n\", value, value);\n    printf(\"地址: %p (Address: %p)\\n\", (void*)ptr, (void*)ptr);\n    printf(\"通过指针访问: %d (Access via pointer: %d)\\n\", *ptr, *ptr);\n    printf(\"\\n\");\n\n    // 函数示例\n    printf(\"🎭 函数示例 (Function example):\\n\");\n    greet_user(\"CodeForge用户\");\n    printf(\"\\n\");\n\n    // 内存分配示例\n    printf(\"💾 动态内存分配 (Dynamic memory allocation):\\n\");\n    int* dynamic_array = (int*)malloc(5 * sizeof(int));\n    if (dynamic_array != NULL) {\n        for (int i = 0; i < 5; i++) {\n            dynamic_array[i] = (i + 1) * 10;\n        }\n        printf(\"动态数组: \");\n        print_array(dynamic_array, 5);\n        free(dynamic_array);\n        printf(\"内存已释放\\n\");\n    }\n    printf(\"\\n\");\n\n    // 结构体示例\n    printf(\"👤 结构体示例 (Struct example):\\n\");\n    struct Person {\n        char name[50];\n        int age;\n        float height;\n    };\n    \n    struct Person person;\n    strcpy(person.name, \"张三\");\n    person.age = 25;\n    person.height = 175.5;\n    \n    printf(\"姓名: %s, 年龄: %d, 身高: %.1f cm\\n\", \n           person.name, person.age, person.height);\n    printf(\"\\n\");\n\n    // 递归示例\n    printf(\"🔄 递归示例 (Recursion example):\\n\");\n    int fib_n = 7;\n    int fib_result = fibonacci(fib_n);\n    printf(\"斐波那契数列第%d项: %d\\n\", fib_n, fib_result);\n    printf(\"\\n\");\n\n    // 数学库示例\n    printf(\"📐 数学库示例 (Math library example):\\n\");\n    double angle = 45.0;\n    double radians = angle * M_PI / 180.0;\n    printf(\"sin(%.0f°) = %.4f\\n\", angle, sin(radians));\n    printf(\"cos(%.0f°) = %.4f\\n\", angle, cos(radians));\n    printf(\"sqrt(16) = %.2f\\n\", sqrt(16));\n    printf(\"\\n\");\n\n    // 位操作示例\n    printf(\"🔧 位操作示例 (Bitwise operations):\\n\");\n    int a = 12;  // 1100 in binary\n    int b = 10;  // 1010 in binary\n    printf(\"%d & %d = %d (AND)\\n\", a, b, a & b);\n    printf(\"%d | %d = %d (OR)\\n\", a, b, a | b);\n    printf(\"%d ^ %d = %d (XOR)\\n\", a, b, a ^ b);\n    printf(\"~%d = %d (NOT)\\n\", a, ~a);\n    printf(\"\\n\");\n\n    // 枚举示例\n    printf(\"📋 枚举示例 (Enum example):\\n\");\n    enum Weekday {\n        MONDAY = 1, TUESDAY, WEDNESDAY, THURSDAY, FRIDAY, SATURDAY, SUNDAY\n    };\n    enum Weekday today = WEDNESDAY;\n    printf(\"今天是星期%d\\n\", today);\n    printf(\"\\n\");\n\n    printf(\"🎯 CodeForge C 代码执行完成!\\n\");\n    printf(\"🎯 CodeForge C execution completed!\\n\");\n    printf(\"\\n\");\n    printf(\"感谢使用 CodeForge 代码执行环境! 🚀\\n\");\n    printf(\"Thank you for using CodeForge! 🚀\\n\");\n    \n    return 0;\n}\n\n// 函数实现\nvoid greet_user(const char* name) {\n    printf(\"Hello, %s! 👋\\n\", name);\n}\n\nint add_numbers(int a, int b) {\n    return a + b;\n}\n\nvoid print_array(int arr[], int size) {\n    for (int i = 0; i < size; i++) {\n        printf(\"%d \", arr[i]);\n    }\n    printf(\"\\n\");\n}\n\nint fibonacci(int n) {\n    if (n <= 1) {\n        return n;\n    }\n    return fibonacci(n - 1) + fibonacci(n - 2);\n}",
            )),
            timeout: Some(30),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "gcc".to_string())
    }
}
