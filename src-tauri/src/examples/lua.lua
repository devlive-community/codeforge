-- Lua 示例代码 - CodeForge 代码执行环境

print("🎉 欢迎使用 CodeForge!")
print("Welcome to CodeForge!")
print("")

print("=========================================")
print("          CodeForge Lua               ")
print("=========================================")
print("")

-- 基本输出示例
print("✅ Lua运行成功! (Lua is working!)")
print("⚡ 这是Lua程序 (This is Lua program)")
print("")

-- 变量操作
local name = "CodeForge"
local version = "Lua"
local number1 = 10
local number2 = 20
local result = number1 + number2

print("🔢 简单计算 (Simple calculation):")
print(string.format("%d + %d = %d", number1, number2, result))
print("")

-- 字符串操作
print("📝 字符串操作 (String operations):")
print("平台名称 (Platform): " .. name)
print("语言版本 (Language): " .. version)
print("完整信息 (Full info): " .. name .. " - " .. version)
print("")

-- 表(数组)操作
print("🍎 表示例 (Table example):")
local fruits = {"苹果", "香蕉", "橙子", "葡萄"}
for i, fruit in ipairs(fruits) do
    print(string.format("%d. %s", i, fruit))
end
print("")

-- 条件判断
local score = 85
print("📊 成绩评估 (Score evaluation):")
if score >= 90 then
    print("优秀! (Excellent!)")
elseif score >= 80 then
    print("良好! (Good!)")
elseif score >= 60 then
    print("及格 (Pass)")
else
    print("需要努力 (Need improvement)")
end
print("")

-- 循环示例
print("🔄 循环输出 (Loop output):")
for i = 1, 5 do
    print(string.format("第 %d 次输出 (Output #%d): Hello from CodeForge!", i, i))
end
print("")

-- while循环示例
print("🔁 While循环示例 (While loop example):")
local counter = 1
while counter <= 3 do
    print(string.format("While循环: 第 %d 次", counter))
    counter = counter + 1
end
print("")

-- 函数示例
local function greet(msg)
    return "🎯 " .. msg
end

print(greet("CodeForge Lua代码执行完成!"))
print(greet("CodeForge Lua execution completed!"))
print("")
print("感谢使用 CodeForge 代码执行环境! 🚀")
print("Thank you for using CodeForge! 🚀")