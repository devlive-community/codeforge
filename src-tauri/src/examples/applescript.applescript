-- AppleScript 示例代码 - CodeForge 代码执行环境

log "Welcome to CodeForge!"
log ""

log "========================================="
log "         CodeForge AppleScript        "
log "========================================="
log ""

-- 基本输出示例
log "✅ AppleScript 运行成功! (AppleScript is working!)"
log "🍎 这是 AppleScript 脚本 (This is AppleScript script)"
log ""

-- 变量操作
set appName to "CodeForge"
set version to "AppleScript"
set number1 to 10
set number2 to 20
set calcResult to number1 + number2

log "🔢 简单计算 (Simple calculation):"
log (number1 as string) & " + " & (number2 as string) & " = " & (calcResult as string)
log ""

-- 字符串操作
log "📝 字符串操作 (String operations):"
log "平台名称 (Platform): " & appName
log "语言版本 (Language): " & version
log "完整信息 (Full info): " & appName & " - " & version
log ""

-- 循环示例
log "🔄 循环输出 (Loop output):"
repeat with i from 1 to 5
	log "第 " & (i as string) & " 次输出 (Output #" & (i as string) & "): Hello from CodeForge!"
end repeat
log ""

-- 列表操作
set fruits to {"苹果", "香蕉", "橙子", "葡萄"}
log "🍎 水果列表 (Fruit list):"
repeat with i from 1 to count of fruits
	set fruit to item i of fruits
	log (i as string) & ". " & fruit
end repeat
log ""

-- 条件判断
set score to 85
log "📊 成绩评估 (Score evaluation):"
if score ≥ 90 then
	log "优秀! (Excellent!)"
else if score ≥ 80 then
	log "良好! (Good!)"
else if score ≥ 60 then
	log "及格 (Pass)"
else
	log "需要努力 (Need improvement)"
end if
log ""

-- missing value 示例
set optionalValue to 42
log "🔍 missing value 示例 (missing value example):"
if optionalValue is not missing value then
	log "可选值: " & (optionalValue as string) & " (Optional value: " & (optionalValue as string) & ")"
else
	log "值为空 (Value is missing)"
end if
log ""

-- 处理程序（Handler）示例
on greetUser(userName)
	return "Hello, " & userName & "! 👋"
end greetUser

log "🎭 处理程序示例 (Handler example):"
set greeting to greetUser("CodeForge用户")
log greeting
log ""

-- 记录（Record）示例
log "👤 记录示例 (Record example):"
set person to {name:"张三", age:25, city:"北京"}
log "姓名: " & (name of person) & ", 年龄: " & (age of person as string) & ", 城市: " & (city of person)
log ""

-- 日期和时间示例
log "⏰ 日期时间示例 (Date and time example):"
set currentDate to (current date)
log "当前日期: " & (currentDate as string)
set currentTime to time of currentDate
log "当前时间: " & (currentTime as string)
log ""

-- 数学运算示例
log "📐 数学运算示例 (Math operations):"
set mathResult1 to 2 ^ 3 -- 幂运算
set mathResult2 to 17 mod 5 -- 取模运算
set mathResult3 to round (22 / 7) -- 四舍五入
log "2的3次方: " & (mathResult1 as string)
log "17除以5的余数: " & (mathResult2 as string)
log "22/7四舍五入: " & (mathResult3 as string)
log ""

-- 字符串操作示例
log "📝 字符串操作示例 (String operations):"
set originalText to "CodeForge AppleScript Example"
set upperText to do shell script "echo '" & originalText & "' | tr '[:lower:]' '[:upper:]'"
set wordCount to count of words of originalText
log "原始文本: " & originalText
log "大写文本: " & upperText
log "单词数量: " & (wordCount as string)
log ""

-- 文件操作示例（安全的只读操作）
log "📁 文件信息示例 (File info example):"
try
	set homeFolder to (path to home folder) as string
	set desktopPath to homeFolder & "Desktop:"
	log "桌面路径: " & desktopPath

	-- 获取桌面文件夹信息
	tell application "Finder"
		set folderInfo to get info for folder desktopPath
		set folderSize to size of folderInfo
		log "桌面文件夹大小: " & (folderSize as string) & " bytes"
	end tell
on error errMsg
	log "文件操作错误: " & errMsg
end try
log ""

-- 应用程序交互示例
log "💻 应用程序交互示例 (Application interaction):"
try
	tell application "System Events"
		set appList to name of every process whose visible is true
		set runningAppCount to count of appList
		log "当前运行的可见应用程序数量: " & (runningAppCount as string)

		-- 显示前3个应用程序名称
		repeat with i from 1 to 3
			if i ≤ runningAppCount then
				set appName to item i of appList
				log "应用程序 " & (i as string) & ": " & appName
			end if
		end repeat
	end tell
on error errMsg
	log "应用程序查询错误: " & errMsg
end try
log ""

-- 系统信息示例
log "🖥️ 系统信息示例 (System info example):"
try
	set systemInfo to system info
	set osVersion to system version of systemInfo
	set computerName to computer name of systemInfo
	log "操作系统版本: " & osVersion
	log "计算机名称: " & computerName
on error errMsg
	log "系统信息获取错误: " & errMsg
end try
log ""

-- 错误处理示例
log "🚨 错误处理示例 (Error handling):"
try
	set riskyNumber to random number from 1 to 10
	if riskyNumber < 5 then
		error "随机数太小了: " & (riskyNumber as string)
	else
		log "随机数正常: " & (riskyNumber as string)
	end if
on error errMsg number errNum
	log "捕获错误 (" & (errNum as string) & "): " & errMsg
end try
log ""

-- 用户输入示例（注释版本，避免阻塞执行）
log "💬 用户交互概念示例 (User interaction concept):"
-- set userInput to text returned of (display dialog "请输入您的名字:" default answer "用户")
-- log "用户输入: " & userInput
log "（实际运行时可以取消注释上面的代码进行用户交互）"
log ""

-- 通知示例
log "🔔 通知示例 (Notification example):"
try
	display notification "CodeForge AppleScript 执行完成!" with title "CodeForge" subtitle "AppleScript 示例" sound name "Glass"
log "通知已发送"
on error
	log "通知发送失败（可能需要权限）"
end try
log ""

-- AppleScript 独有的tell块示例
log "📱 Tell 块示例 (Tell block example):"
tell application "Finder"
	set trashCount to count of items in trash
	log "废纸篓中的项目数量: " & (trashCount as string)
end tell

-- 使用 shell 脚本增强功能
log "🐚 Shell 脚本集成示例 (Shell script integration):"
set shellOutput to do shell script "date +%Y-%m-%d"
log "今天日期 (通过shell): " & shellOutput

set unameOutput to do shell script "uname -s"
log "操作系统内核: " & unameOutput
log ""

-- 修复的列表处理示例
log "📋 列表处理示例 (List processing):"
set numberList to {1, 2, 3, 4, 5, 6, 7, 8, 9, 10}
set evenNumbers to {}
set sumTotal to 0

-- 使用索引循环而不是 "repeat with num in list"
repeat with i from 1 to count of numberList
	set currentNum to item i of numberList
	set sumTotal to sumTotal + currentNum
	if (currentNum mod 2) = 0 then
		set end of evenNumbers to currentNum
	end if
end repeat

log "原始列表: " & my listToString(numberList)
log "偶数: " & my listToString(evenNumbers)
log "总和: " & (sumTotal as string)
log ""

-- 辅助处理程序：将列表转换为字符串
on listToString(lst)
	set AppleScript's text item delimiters to ", "
	set stringOutput to lst as string
	set AppleScript's text item delimiters to ""
	return stringOutput
end listToString

-- 文本分割示例
log "✂️ 文本分割示例 (Text splitting):"
set sampleText to "Apple,Banana,Orange,Grape"
set AppleScript's text item delimiters to ","
set fruitList to text items of sampleText
set AppleScript's text item delimiters to ""

log "原始文本: " & sampleText
repeat with i from 1 to count of fruitList
	log "水果 " & (i as string) & ": " & (item i of fruitList)
end repeat
log ""

-- 额外的列表操作示例
log "🔄 更多列表操作示例 (More list operations):"
set originalList to {5, 2, 8, 1, 9, 3}
set sortedList to my sortList(originalList)
set reverseList to reverse of originalList
log "原始列表: " & my listToString(originalList)
log "反转列表: " & my listToString(reverseList)
log "排序列表: " & my listToString(sortedList)
log ""

-- 简单的排序函数
on sortList(inputList)
	set sortedList to {}
	repeat with i from 1 to count of inputList
		set end of sortedList to item i of inputList
	end repeat

	-- 简单的冒泡排序
	repeat with i from 1 to count of sortedList
		repeat with j from 1 to (count of sortedList) - 1
			if item j of sortedList > item (j + 1) of sortedList then
				set temp to item j of sortedList
				set item j of sortedList to item (j + 1) of sortedList
				set item (j + 1) of sortedList to temp
			end if
		end repeat
	end repeat

	return sortedList
end sortList

log "🎯 CodeForge AppleScript 代码执行完成!"
log "🎯 CodeForge AppleScript execution completed!"
log ""
log "感谢使用 CodeForge 代码执行环境! 🚀"
log "Thank you for using CodeForge! 🚀"