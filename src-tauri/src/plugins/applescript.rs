use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct AppleScriptPlugin;

impl LanguagePlugin for AppleScriptPlugin {
    fn get_order(&self) -> i32 {
        15
    }

    fn get_language_name(&self) -> &'static str {
        "AppleScript"
    }

    fn get_language_key(&self) -> &'static str {
        "applescript"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "applescript".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["-e", "version of AppleScript"]
    }

    fn get_path_command(&self) -> String {
        "which osascript".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("applescript"),
            before_compile: None,
            extension: String::from("applescript"),
            execute_home: None,
            run_command: Some(String::from("osascript $filename")),
            after_compile: None,
            template: Some(String::from(
                "-- AppleScript 示例代码 - CodeForge 代码执行环境\n\nlog \"Welcome to CodeForge!\"\nlog \"\"\n\nlog \"=========================================\"\nlog \"         CodeForge AppleScript        \"\nlog \"=========================================\"\nlog \"\"\n\n-- 基本输出示例\nlog \"✅ AppleScript 运行成功! (AppleScript is working!)\"\nlog \"🍎 这是 AppleScript 脚本 (This is AppleScript script)\"\nlog \"\"\n\n-- 变量操作\nset appName to \"CodeForge\"\nset version to \"AppleScript\"\nset number1 to 10\nset number2 to 20\nset calcResult to number1 + number2\n\nlog \"🔢 简单计算 (Simple calculation):\"\nlog (number1 as string) & \" + \" & (number2 as string) & \" = \" & (calcResult as string)\nlog \"\"\n\n-- 字符串操作\nlog \"📝 字符串操作 (String operations):\"\nlog \"平台名称 (Platform): \" & appName\nlog \"语言版本 (Language): \" & version\nlog \"完整信息 (Full info): \" & appName & \" - \" & version\nlog \"\"\n\n-- 循环示例\nlog \"🔄 循环输出 (Loop output):\"\nrepeat with i from 1 to 5\n\tlog \"第 \" & (i as string) & \" 次输出 (Output #\" & (i as string) & \"): Hello from CodeForge!\"\nend repeat\nlog \"\"\n\n-- 列表操作\nset fruits to {\"苹果\", \"香蕉\", \"橙子\", \"葡萄\"}\nlog \"🍎 水果列表 (Fruit list):\"\nrepeat with i from 1 to count of fruits\n\tset fruit to item i of fruits\n\tlog (i as string) & \". \" & fruit\nend repeat\nlog \"\"\n\n-- 条件判断\nset score to 85\nlog \"📊 成绩评估 (Score evaluation):\"\nif score ≥ 90 then\n\tlog \"优秀! (Excellent!)\"\nelse if score ≥ 80 then\n\tlog \"良好! (Good!)\"\nelse if score ≥ 60 then\n\tlog \"及格 (Pass)\"\nelse\n\tlog \"需要努力 (Need improvement)\"\nend if\nlog \"\"\n\n-- missing value 示例\nset optionalValue to 42\nlog \"🔍 missing value 示例 (missing value example):\"\nif optionalValue is not missing value then\n\tlog \"可选值: \" & (optionalValue as string) & \" (Optional value: \" & (optionalValue as string) & \")\"\nelse\n\tlog \"值为空 (Value is missing)\"\nend if\nlog \"\"\n\n-- 处理程序（Handler）示例\non greetUser(userName)\n\treturn \"Hello, \" & userName & \"! 👋\"\nend greetUser\n\nlog \"🎭 处理程序示例 (Handler example):\"\nset greeting to greetUser(\"CodeForge用户\")\nlog greeting\nlog \"\"\n\n-- 记录（Record）示例\nlog \"👤 记录示例 (Record example):\"\nset person to {name:\"张三\", age:25, city:\"北京\"}\nlog \"姓名: \" & (name of person) & \", 年龄: \" & (age of person as string) & \", 城市: \" & (city of person)\nlog \"\"\n\n-- 日期和时间示例\nlog \"⏰ 日期时间示例 (Date and time example):\"\nset currentDate to (current date)\nlog \"当前日期: \" & (currentDate as string)\nset currentTime to time of currentDate\nlog \"当前时间: \" & (currentTime as string)\nlog \"\"\n\n-- 数学运算示例\nlog \"📐 数学运算示例 (Math operations):\"\nset mathResult1 to 2 ^ 3 -- 幂运算\nset mathResult2 to 17 mod 5 -- 取模运算\nset mathResult3 to round (22 / 7) -- 四舍五入\nlog \"2的3次方: \" & (mathResult1 as string)\nlog \"17除以5的余数: \" & (mathResult2 as string)\nlog \"22/7四舍五入: \" & (mathResult3 as string)\nlog \"\"\n\n-- 字符串操作示例\nlog \"📝 字符串操作示例 (String operations):\"\nset originalText to \"CodeForge AppleScript Example\"\nset upperText to do shell script \"echo '\" & originalText & \"' | tr '[:lower:]' '[:upper:]'\"\nset wordCount to count of words of originalText\nlog \"原始文本: \" & originalText\nlog \"大写文本: \" & upperText\nlog \"单词数量: \" & (wordCount as string)\nlog \"\"\n\n-- 文件操作示例（安全的只读操作）\nlog \"📁 文件信息示例 (File info example):\"\ntry\n\tset homeFolder to (path to home folder) as string\n\tset desktopPath to homeFolder & \"Desktop:\"\n\tlog \"桌面路径: \" & desktopPath\n\t\n\t-- 获取桌面文件夹信息\n\ttell application \"Finder\"\n\t\tset folderInfo to get info for folder desktopPath\n\t\tset folderSize to size of folderInfo\n\t\tlog \"桌面文件夹大小: \" & (folderSize as string) & \" bytes\"\n\tend tell\non error errMsg\n\tlog \"文件操作错误: \" & errMsg\nend try\nlog \"\"\n\n-- 应用程序交互示例\nlog \"💻 应用程序交互示例 (Application interaction):\"\ntry\n\ttell application \"System Events\"\n\t\tset appList to name of every process whose visible is true\n\t\tset runningAppCount to count of appList\n\t\tlog \"当前运行的可见应用程序数量: \" & (runningAppCount as string)\n\t\t\n\t\t-- 显示前3个应用程序名称\n\t\trepeat with i from 1 to 3\n\t\t\tif i ≤ runningAppCount then\n\t\t\t\tset appName to item i of appList\n\t\t\t\tlog \"应用程序 \" & (i as string) & \": \" & appName\n\t\t\tend if\n\t\tend repeat\n\tend tell\non error errMsg\n\tlog \"应用程序查询错误: \" & errMsg\nend try\nlog \"\"\n\n-- 系统信息示例\nlog \"🖥️ 系统信息示例 (System info example):\"\ntry\n\tset systemInfo to system info\n\tset osVersion to system version of systemInfo\n\tset computerName to computer name of systemInfo\n\tlog \"操作系统版本: \" & osVersion\n\tlog \"计算机名称: \" & computerName\non error errMsg\n\tlog \"系统信息获取错误: \" & errMsg\nend try\nlog \"\"\n\n-- 错误处理示例\nlog \"🚨 错误处理示例 (Error handling):\"\ntry\n\tset riskyNumber to random number from 1 to 10\n\tif riskyNumber < 5 then\n\t\terror \"随机数太小了: \" & (riskyNumber as string)\n\telse\n\t\tlog \"随机数正常: \" & (riskyNumber as string)\n\tend if\non error errMsg number errNum\n\tlog \"捕获错误 (\" & (errNum as string) & \"): \" & errMsg\nend try\nlog \"\"\n\n-- 用户输入示例（注释版本，避免阻塞执行）\nlog \"💬 用户交互概念示例 (User interaction concept):\"\n-- set userInput to text returned of (display dialog \"请输入您的名字:\" default answer \"用户\")\n-- log \"用户输入: \" & userInput\nlog \"（实际运行时可以取消注释上面的代码进行用户交互）\"\nlog \"\"\n\n-- 通知示例\nlog \"🔔 通知示例 (Notification example):\"\ntry\n\tdisplay notification \"CodeForge AppleScript 执行完成!\" with title \"CodeForge\" subtitle \"AppleScript 示例\" sound name \"Glass\"\nlog \"通知已发送\"\non error\n\tlog \"通知发送失败（可能需要权限）\"\nend try\nlog \"\"\n\n-- AppleScript 独有的tell块示例\nlog \"📱 Tell 块示例 (Tell block example):\"\ntell application \"Finder\"\n\tset trashCount to count of items in trash\n\tlog \"废纸篓中的项目数量: \" & (trashCount as string)\nend tell\n\n-- 使用 shell 脚本增强功能\nlog \"🐚 Shell 脚本集成示例 (Shell script integration):\"\nset shellOutput to do shell script \"date +%Y-%m-%d\"\nlog \"今天日期 (通过shell): \" & shellOutput\n\nset unameOutput to do shell script \"uname -s\"\nlog \"操作系统内核: \" & unameOutput\nlog \"\"\n\n-- 修复的列表处理示例\nlog \"📋 列表处理示例 (List processing):\"\nset numberList to {1, 2, 3, 4, 5, 6, 7, 8, 9, 10}\nset evenNumbers to {}\nset sumTotal to 0\n\n-- 使用索引循环而不是 \"repeat with num in list\"\nrepeat with i from 1 to count of numberList\n\tset currentNum to item i of numberList\n\tset sumTotal to sumTotal + currentNum\n\tif (currentNum mod 2) = 0 then\n\t\tset end of evenNumbers to currentNum\n\tend if\nend repeat\n\nlog \"原始列表: \" & my listToString(numberList)\nlog \"偶数: \" & my listToString(evenNumbers)\nlog \"总和: \" & (sumTotal as string)\nlog \"\"\n\n-- 辅助处理程序：将列表转换为字符串\non listToString(lst)\n\tset AppleScript's text item delimiters to \", \"\n\tset stringOutput to lst as string\n\tset AppleScript's text item delimiters to \"\"\n\treturn stringOutput\nend listToString\n\n-- 文本分割示例\nlog \"✂️ 文本分割示例 (Text splitting):\"\nset sampleText to \"Apple,Banana,Orange,Grape\"\nset AppleScript's text item delimiters to \",\"\nset fruitList to text items of sampleText\nset AppleScript's text item delimiters to \"\"\n\nlog \"原始文本: \" & sampleText\nrepeat with i from 1 to count of fruitList\n\tlog \"水果 \" & (i as string) & \": \" & (item i of fruitList)\nend repeat\nlog \"\"\n\n-- 额外的列表操作示例\nlog \"🔄 更多列表操作示例 (More list operations):\"\nset originalList to {5, 2, 8, 1, 9, 3}\nset sortedList to my sortList(originalList)\nset reverseList to reverse of originalList\nlog \"原始列表: \" & my listToString(originalList)\nlog \"反转列表: \" & my listToString(reverseList)\nlog \"排序列表: \" & my listToString(sortedList)\nlog \"\"\n\n-- 简单的排序函数\non sortList(inputList)\n\tset sortedList to {}\n\trepeat with i from 1 to count of inputList\n\t\tset end of sortedList to item i of inputList\n\tend repeat\n\t\n\t-- 简单的冒泡排序\n\trepeat with i from 1 to count of sortedList\n\t\trepeat with j from 1 to (count of sortedList) - 1\n\t\t\tif item j of sortedList > item (j + 1) of sortedList then\n\t\t\t\tset temp to item j of sortedList\n\t\t\t\tset item j of sortedList to item (j + 1) of sortedList\n\t\t\t\tset item (j + 1) of sortedList to temp\n\t\t\tend if\n\t\tend repeat\n\tend repeat\n\t\n\treturn sortedList\nend sortList\n\nlog \"🎯 CodeForge AppleScript 代码执行完成!\"\nlog \"🎯 CodeForge AppleScript execution completed!\"\nlog \"\"\nlog \"感谢使用 CodeForge 代码执行环境! 🚀\"\nlog \"Thank you for using CodeForge! 🚀\"",
            )),
            timeout: Some(45),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "osascript".to_string())
    }
}
