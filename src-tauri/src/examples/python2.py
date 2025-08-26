# -*- coding: utf-8 -*-
# Python2 示例代码 - CodeForge 代码执行环境

print u"🎉 欢迎使用 CodeForge!"
print u"Welcome to CodeForge!"
print u""

print u"========================================="
print u"         CodeForge Python2            "
print u"========================================="
print u""

# 基本输出示例
print u"✅ Python2 运行成功! (Python2 is working!)"
print u"🐍 这是 Python2 程序 (This is Python2 program)"
print u""

# 变量操作
name = u"CodeForge"
version = u"Python2"
number1 = 10
number2 = 20
result = number1 + number2

print u"🔢 简单计算 (Simple calculation):"
print u"{} + {} = {}".format(number1, number2, result)
print u""

# 字符串操作
print u"📝 字符串操作 (String operations):"
print u"平台名称 (Platform): {}".format(name)
print u"语言版本 (Language): {}".format(version)
print u"完整信息 (Full info): {} - {}".format(name, version)
print u""

# 循环示例
print u"🔄 循环输出 (Loop output):"
for i in range(1, 6):
    print u"第 {} 次输出 (Output #{}): Hello from CodeForge!".format(i, i)
print u""

# 列表操作
fruits = [u"苹果", u"香蕉", u"橙子", u"葡萄"]
print u"🍎 水果列表 (Fruit list):"
for i, fruit in enumerate(fruits):
    print u"{}. {}".format(i + 1, fruit)
print u""

# 条件判断
score = 85
print u"📊 成绩评估 (Score evaluation):"
if score >= 90:
    print u"优秀! (Excellent!)"
elif score >= 80:
    print u"良好! (Good!)"
elif score >= 60:
    print u"及格 (Pass)"
else:
    print u"需要努力 (Need improvement)"

# 字典操作示例
user = {
    u"name": u"CodeForge用户",
    u"age": 25,
    u"skills": [u"Python", u"JavaScript", u"Java"]
}

print u""
print u"📦 字典操作 (Dictionary operations):"
print u"用户名: {}".format(user[u"name"])
print u"年龄: {}".format(user[u"age"])
print u"技能: {}".format(u", ".join(user[u"skills"]))

# 函数示例
def greet_user(name):
    return u"Hello, {}! 👋".format(name)

# 列表推导式示例
numbers = [1, 2, 3, 4, 5]
squares = [x * x for x in numbers]

print u""
print u"🎭 函数和列表推导示例 (Function and list comprehension examples):"
greeting = greet_user(u"CodeForge用户")
print greeting
print u"数字: {}".format(numbers)
print u"平方: {}".format(squares)

# 类示例
class Calculator(object):
    def __init__(self):
        self.name = u"CodeForge计算器"

    def add(self, a, b):
        return a + b

    def multiply(self, a, b):
        return a * b

print u""
print u"🧮 类示例 (Class example):"
calc = Calculator()
print u"计算器名称: {}".format(calc.name)
print u"3 + 7 = {}".format(calc.add(3, 7))
print u"4 × 6 = {}".format(calc.multiply(4, 6))

print u""
print u"🎯 CodeForge Python2 代码执行完成!"
print u"🎯 CodeForge Python2 execution completed!"
print u""
print u"感谢使用 CodeForge 代码执行环境! 🚀"
print u"Thank you for using CodeForge! 🚀"