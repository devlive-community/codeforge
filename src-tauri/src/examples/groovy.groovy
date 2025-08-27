// Groovy 示例代码 - CodeForge 代码执行环境

// 类定义
class Person {
  String name
  int age
  float height

  Person(String name, int age, float height) {
    this.name = name
    this.age = age
    this.height = height
  }

  void display() {
    println "姓名: ${name}, 年龄: ${age}, 身高: ${height} cm"
  }

  String toString() {
    return "Person(name: ${name}, age: ${age}, height: ${height})"
  }
}

// 函数定义
def greetUser(name) {
  println "Hello, ${name}! 👋"
}

def addNumbers(a, b) {
  return a + b
}

def printList(list) {
  list.each { element ->
    print "${element} "
  }
  println ""
}

def fibonacci(n) {
  if (n <= 1) return n
  return fibonacci(n - 1) + fibonacci(n - 2)
}

// 主程序执行
println "🎉 欢迎使用 CodeForge!"
println "Welcome to CodeForge!"
println ""

println "========================================="
println "            CodeForge Groovy           "
println "========================================="
println ""

// 基本输出示例
println "✅ Groovy 运行成功! (Groovy is working!)"
println "⚡ 这是 Groovy 程序 (This is Groovy program)"
println ""

// 变量操作
def name = "CodeForge"
def version = "Groovy"
def number1 = 10
def number2 = 20
def result = addNumbers(number1, number2)

println "🔢 简单计算 (Simple calculation):"
println "${number1} + ${number2} = ${result}"
println ""

// 字符串操作和插值
println "📝 字符串操作 (String operations):"
println "平台名称 (Platform): ${name}"
println "语言版本 (Language): ${version}"
println "完整信息 (Full info): ${name} - ${version}"
println ""

// 循环示例
println "🔄 循环输出 (Loop output):"
(1..5).each { i ->
  println "第 ${i} 次输出 (Output #${i}): Hello from CodeForge!"
}
println ""

// 列表示例
println "🍎 列表示例 (List example):"
def fruits = ["苹果", "香蕉", "橙子", "葡萄"]
fruits.eachWithIndex { fruit, index ->
  println "${index + 1}. ${fruit}"
}
println ""

// 条件判断
def score = 85
println "📊 成绩评估 (Score evaluation):"
switch (score) {
  case { it >= 90 }:
    println "优秀! (Excellent!)"
    break
  case { it >= 80 }:
    println "良好! (Good!)"
    break
  case { it >= 60 }:
    println "及格 (Pass)"
    break
  default:
    println "需要努力 (Need improvement)"
}
println ""

// Groovy特有的属性访问
println "🔍 动态属性示例 (Dynamic property example):"
def obj = [:]
obj.dynamicProperty = "动态值"
obj.number = 42
println "动态属性: ${obj.dynamicProperty}"
println "数字属性: ${obj.number}"
println "对象内容: ${obj}"
println ""

// 函数示例
println "🎭 函数示例 (Function example):"
greetUser("CodeForge用户")
println ""

// 集合操作示例
println "💾 集合操作示例 (Collection operations):"
def numbers = [1, 2, 3, 4, 5]
def doubled = numbers.collect { it * 2 }
def evens = numbers.findAll { it % 2 == 0 }
def sum = numbers.sum()

println "原始列表: ${numbers}"
println "每个数字乘以2: ${doubled}"
println "偶数: ${evens}"
println "总和: ${sum}"
println ""

// 类和对象示例
println "👤 类和对象示例 (Class and object example):"
def person = new Person("张三", 25, 175.5f)
person.display()
println "toString方法: ${person}"
println ""

// 递归示例
println "🔄 递归示例 (Recursion example):"
def fib_n = 7
def fib_result = fibonacci(fib_n)
println "斐波那契数列第${fib_n}项: ${fib_result}"
println ""

// 数学运算示例
println "📐 数学运算示例 (Math operations):"
def angle = 45.0
def radians = Math.toRadians(angle)
println "sin(${angle}°) = ${Math.sin(radians).round(4)}"
println "cos(${angle}°) = ${Math.cos(radians).round(4)}"
println "sqrt(16) = ${Math.sqrt(16)}"
println ""

// 位操作示例
println "🔧 位操作示例 (Bitwise operations):"
def a = 12  // 1100 in binary
def b = 10  // 1010 in binary
println "${a} & ${b} = ${a & b} (AND)"
println "${a} | ${b} = ${a | b} (OR)"
println "${a} ^ ${b} = ${a ^ b} (XOR)"
println "~${a} = ${~a} (NOT)"
println ""

// 枚举示例
println "📋 枚举示例 (Enum example):"
enum Weekday {
  MONDAY(1), TUESDAY(2), WEDNESDAY(3), THURSDAY(4),
  FRIDAY(5), SATURDAY(6), SUNDAY(7)

  final int value

  Weekday(int value) {
    this.value = value
  }
}

def today = Weekday.WEDNESDAY
println "今天是星期${today.value}: ${today}"
println ""

// 闭包示例
println "🚀 闭包示例 (Closure example):"
def multiplier = { x, y -> x * y }
def greeter = { userName -> "Hello, ${userName}!" }
def calculator = { operation, num1, num2 ->
  switch(operation) {
    case 'add': return num1 + num2
    case 'multiply': return num1 * num2
    case 'subtract': return num1 - num2
    default: return 0
  }
}

println "闭包乘法 5 * 3 = ${multiplier(5, 3)}"
println "闭包问候: ${greeter('Groovy')}"
println "闭包计算器 (add, 10, 5) = ${calculator('add', 10, 5)}"
println ""

// Map操作示例
println "🗺️ Map操作示例 (Map operations):"
def scores = [
        "张三": 85,
        "李四": 92,
        "王五": 78
]

scores.each { key, value ->
  println "${key}: ${value}分"
}

def highScores = scores.findAll { key, value -> value >= 85 }
println "高分学生: ${highScores}"
println ""

// 正则表达式示例
println "🔍 正则表达式示例 (Regex example):"
def text = "CodeForge 2025 是最好的代码执行平台!"
def pattern = ~/\d+/
def matcher = text =~ pattern
if (matcher) {
  println "找到数字: ${matcher[0]}"
}

def emailPattern = ~/\w+@\w+\.\w+/
def email = "test@codeforge.com"
println "邮箱验证 '${email}': ${email ==~ emailPattern}"
println ""

// 异常处理示例
println "⚠️ 异常处理示例 (Exception handling):"
try {
  def divisionResult = 10 / 0
} catch (ArithmeticException e) {
  println "捕获异常: ${e.message}"
} finally {
  println "finally块总是执行"
}
println ""

// 文件操作示例
println "📁 字符串操作示例 (String operations):"
def multilineString = """
这是一个
多行字符串
示例
"""
println "多行字符串长度: ${multilineString.trim().split('\n').size()} 行"

def template = "Hello \${userName}, welcome to \${platformName}!"
def binding = [userName: 'User', platformName: 'CodeForge']
def engine = new groovy.text.SimpleTemplateEngine()
def templateResult = engine.createTemplate(template).make(binding)
println "模板结果: ${templateResult}"
println ""

// 元编程示例
println "🎯 元编程示例 (Metaprogramming example):"
String.metaClass.reverseString = {
  delegate.reverse()
}

String.metaClass.addPrefix = { prefix ->
  prefix + delegate
}

def str = "CodeForge"
println "原字符串: ${str}"
println "反转字符串: ${str.reverseString()}"
println "添加前缀: ${str.addPrefix('Hello ')}"
println ""

println "🎯 CodeForge Groovy 代码执行完成!"
println "🎯 CodeForge Groovy execution completed!"
println ""
println "感谢使用 CodeForge 代码执行环境! 🚀"
println "Thank you for using CodeForge! 🚀"