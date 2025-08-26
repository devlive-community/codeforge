import Foundation
// Swift 示例代码 - CodeForge 代码执行环境

print("🎉 欢迎使用 CodeForge!")
print("Welcome to CodeForge!")
print("")

print("=========================================")
print("           CodeForge Swift            ")
print("=========================================")
print("")

// 基本输出示例
print("✅ Swift 运行成功! (Swift is working!)")
print("🦉 这是 Swift 脚本 (This is Swift script)")
print("")

// 变量操作
let name = "CodeForge"
let version = "Swift"
let number1 = 10
let number2 = 20
let result = number1 + number2

print("🔢 简单计算 (Simple calculation):")
print("\(number1) + \(number2) = \(result)")
print("")

// 字符串操作
print("📝 字符串操作 (String operations):")
print("平台名称 (Platform): \(name)")
print("语言版本 (Language): \(version)")
print("完整信息 (Full info): \(name) - \(version)")
print("")

// 循环示例
print("🔄 循环输出 (Loop output):")
for i in 1...5 {
    print("第 \(i) 次输出 (Output #\(i)): Hello from CodeForge!")
}
print("")

// 数组操作
let fruits = ["苹果", "香蕉", "橙子", "葡萄"]
print("🍎 水果列表 (Fruit list):")
for (index, fruit) in fruits.enumerated() {
    print("\(index + 1). \(fruit)")
}
print("")

// 条件判断
let score = 85
print("📊 成绩评估 (Score evaluation):")
if score >= 90 {
    print("优秀! (Excellent!)")
} else if score >= 80 {
    print("良好! (Good!)")
} else if score >= 60 {
    print("及格 (Pass)")
} else {
    print("需要努力 (Need improvement)")
}

// 可选类型示例
var optionalValue: Int? = 42
print("")
print("🔍 可选类型示例 (Optional example):")
if let unwrappedValue = optionalValue {
    print("可选值: \(unwrappedValue) (Optional value: \(unwrappedValue))")
} else {
    print("值为空 (Value is nil)")
}

// 函数示例
func greetUser(name: String) -> String {
    return "Hello, \(name)! 👋"
}

print("")
print("🎭 函数示例 (Function example):")
let greeting = greetUser(name: "CodeForge用户")
print(greeting)

print("")
print("🎯 CodeForge Swift 代码执行完成!")
print("🎯 CodeForge Swift execution completed!")
print("")
print("感谢使用 CodeForge 代码执行环境! 🚀")
print("Thank you for using CodeForge! 🚀")