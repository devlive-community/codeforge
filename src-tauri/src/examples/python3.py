#!/usr/bin/env python3
# -*- coding: utf-8 -*-
# Python3 示例代码 - CodeForge 代码执行环境

print("🎉 欢迎使用 CodeForge!")
print("Welcome to CodeForge!")
print("")

print("=========================================")
print("         CodeForge Python3            ")
print("=========================================")
print("")

# 基本输出示例
print("✅ Python3 运行成功! (Python3 is working!)")
print("🐍 这是 Python3 程序 (This is Python3 program)")
print("")

# 变量操作
name = "CodeForge"
version = "Python3"
number1 = 10
number2 = 20
result = number1 + number2

print("🔢 简单计算 (Simple calculation):")
print(f"{number1} + {number2} = {result}")
print("")

# 字符串操作
print("📝 字符串操作 (String operations):")
print(f"平台名称 (Platform): {name}")
print(f"语言版本 (Language): {version}")
print(f"完整信息 (Full info): {name} - {version}")
print("")

# 循环示例
print("🔄 循环输出 (Loop output):")
for i in range(1, 6):
    print(f"第 {i} 次输出 (Output #{i}): Hello from CodeForge!")
print("")

# 列表操作
fruits = ["苹果", "香蕉", "橙子", "葡萄"]
print("🍎 水果列表 (Fruit list):")
for i, fruit in enumerate(fruits, 1):
    print(f"{i}. {fruit}")
print("")

# 条件判断
score = 85
print("📊 成绩评估 (Score evaluation):")
if score >= 90:
    print("优秀! (Excellent!)")
elif score >= 80:
    print("良好! (Good!)")
elif score >= 60:
    print("及格 (Pass)")
else:
    print("需要努力 (Need improvement)")

# 字典操作示例
user = {
    "name": "CodeForge用户",
    "age": 25,
    "skills": ["Python", "JavaScript", "Java"],
    "is_active": True
}

print("")
print("📦 字典操作 (Dictionary operations):")
print(f"用户名: {user['name']}")
print(f"年龄: {user['age']}")
print(f"技能: {', '.join(user['skills'])}")
print(f"活跃状态: {'是' if user['is_active'] else '否'}")

# 函数示例
def greet_user(name: str) -> str:
    """问候用户的函数"""
    return f"Hello, {name}! 👋"

# Lambda函数示例
square = lambda x: x ** 2

# 列表推导式示例
numbers = [1, 2, 3, 4, 5]
squares = [x ** 2 for x in numbers]
even_squares = [x ** 2 for x in numbers if x % 2 == 0]

print("")
print("🎭 函数和推导式示例 (Function and comprehension examples):")
greeting = greet_user("CodeForge用户")
print(greeting)
print(f"数字: {numbers}")
print(f"平方: {squares}")
print(f"偶数的平方: {even_squares}")
print(f"5的平方 (使用lambda): {square(5)}")

# 类示例（使用现代Python特性）
class Calculator:
    """简单的计算器类"""

    def __init__(self, name: str = "CodeForge计算器"):
        self.name = name
        self.history = []

    def add(self, a: float, b: float) -> float:
        result = a + b
        self.history.append(f"{a} + {b} = {result}")
        return result

    def multiply(self, a: float, b: float) -> float:
        result = a * b
        self.history.append(f"{a} × {b} = {result}")
        return result

    def get_history(self) -> list:
        return self.history

print("")
print("🧮 类示例 (Class example):")
calc = Calculator()
print(f"计算器名称: {calc.name}")
result1 = calc.add(3.5, 7.2)
result2 = calc.multiply(4, 6)
print(f"3.5 + 7.2 = {result1}")
print(f"4 × 6 = {result2}")
print(f"计算历史: {calc.get_history()}")

# 异常处理示例
print("")
print("⚠️ 异常处理示例 (Exception handling example):")
try:
    division_result = 10 / 2
    print(f"10 ÷ 2 = {division_result}")
    # 模拟一个可能的错误
    risky_operation = 10 / 0  # 这会引发异常
except ZeroDivisionError as e:
    print(f"捕获到除零错误: {e}")
except Exception as e:
    print(f"捕获到其他错误: {e}")
else:
    print("没有发生异常")
finally:
    print("异常处理完成")

print("")
print("🎯 CodeForge Python3 代码执行完成!")
print("🎯 CodeForge Python3 execution completed!")
print("")
print("感谢使用 CodeForge 代码执行环境! 🚀")
print("Thank you for using CodeForge! 🚀")