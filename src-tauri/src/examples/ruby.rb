# Ruby 示例代码 - CodeForge 代码执行环境

puts "🎉 欢迎使用 CodeForge!"
puts "Welcome to CodeForge!"
puts ""

puts "========================================="
puts "            CodeForge Ruby            "
puts "========================================="
puts ""

# 基本输出示例
puts "✅ Ruby 运行成功! (Ruby is working!)"
puts "💎 这是 Ruby 脚本 (This is Ruby script)"
puts ""

# 变量操作
name = "CodeForge"
version = "Ruby"
number1 = 10
number2 = 20
result = number1 + number2

puts "🔢 简单计算 (Simple calculation):"
puts "#{number1} + #{number2} = #{result}"
puts ""

# 字符串操作
puts "📝 字符串操作 (String operations):"
puts "平台名称 (Platform): #{name}"
puts "语言版本 (Language): #{version}"
puts "完整信息 (Full info): #{name} - #{version}"
puts ""

# 循环示例
puts "🔄 循环输出 (Loop output):"
(1..5).each do |i|
  puts "第 #{i} 次输出 (Output ##{i}): Hello from CodeForge!"
end
puts ""

# 数组操作
fruits = ["苹果", "香蕉", "橙子", "葡萄"]
puts "🍎 水果列表 (Fruit list):"
fruits.each_with_index do |fruit, index|
  puts "#{index + 1}. #{fruit}"
end
puts ""

# 条件判断
score = 85
puts "📊 成绩评估 (Score evaluation):"
case score
when 90..100
  puts "优秀! (Excellent!)"
when 80..89
  puts "良好! (Good!)"
when 60..79
  puts "及格 (Pass)"
else
  puts "需要努力 (Need improvement)"
end
puts ""

# nil 类型示例
optional_value = 42
puts "🔍 nil 类型示例 (nil example):"
if optional_value.nil?
  puts "值为空 (Value is nil)"
else
  puts "可选值: #{optional_value} (Optional value: #{optional_value})"
end
puts ""

# 方法示例
def greet_user(username)
  "Hello, #{username}! 👋"
end

puts "🎭 方法示例 (Method example):"
greeting = greet_user("CodeForge用户")
puts greeting
puts ""

# 块（Block）示例
puts "🔧 块示例 (Block example):"
numbers = (1..10).to_a
even_numbers = numbers.select { |num| num.even? }
doubled = numbers.map { |num| num * 2 }
sum = numbers.reduce(0) { |acc, num| acc + num }

puts "原始数字 (Original): #{numbers.join(', ')}"
puts "偶数 (Even numbers): #{even_numbers.join(', ')}"
puts "翻倍 (Doubled): #{doubled.join(', ')}"
puts "总和 (Sum): #{sum}"
puts ""

# 哈希（Hash）示例
puts "👤 哈希示例 (Hash example):"
person = {
  name: "张三",
  age: 25,
  city: "北京"
}

puts "姓名: #{person[:name]}, 年龄: #{person[:age]}, 城市: #{person[:city]}"
puts "哈希键: #{person.keys.join(', ')}"
puts "哈希值: #{person.values.join(', ')}"
puts ""

# 符号（Symbol）示例
puts "⚡ 符号示例 (Symbol example):"
status = :active
actions = {
  active: "激活状态",
  inactive: "非活跃状态",
  pending: "等待状态"
}

puts "当前状态: #{status}"
puts "状态描述: #{actions[status]}"
puts ""

# 类和对象示例
puts "🏗️ 类示例 (Class example):"
class Person
  attr_accessor :name, :age
  
  def initialize(name, age)
    @name = name
    @age = age
  end
  
  def introduce
    "我是#{@name}，今年#{@age}岁"
  end
  
  def adult?
    @age >= 18
  end
end

person_obj = Person.new("李四", 28)
puts person_obj.introduce
puts "是否成年: #{person_obj.adult? ? '是' : '否'}"
puts ""

# 继承示例
class Student < Person
  attr_accessor :school
  
  def initialize(name, age, school)
    super(name, age)
    @school = school
  end
  
  def introduce
    super + "，在#{@school}上学"
  end
end

student = Student.new("王五", 20, "清华大学")
puts "👨‍🎓 继承示例 (Inheritance example):"
puts student.introduce
puts ""

# 模块（Module）示例
module MathUtils
  def self.square(n)
    n * n
  end
  
  def self.factorial(n)
    return 1 if n <= 1
    n * factorial(n - 1)
  end
end

puts "📐 模块示例 (Module example):"
puts "5的平方: #{MathUtils.square(5)}"
puts "5的阶乘: #{MathUtils.factorial(5)}"
puts ""

# 异常处理示例
puts "🚨 异常处理示例 (Exception handling):"
begin
  risky_number = rand(10)
  if risky_number < 5
    raise "随机数太小了: #{risky_number}"
  else
    puts "随机数正常: #{risky_number}"
  end
rescue => e
  puts "捕获异常: #{e.message}"
ensure
  puts "异常处理完成"
end
puts ""

# 正则表达式示例
puts "🔤 正则表达式示例 (Regular expressions):"
text = "CodeForge 是一个很棒的代码执行环境! Email: contact@codeforge.com"
email_regex = /\\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Z|a-z]{2,}\\b/

if match = text.match(email_regex)
  puts "文本: #{text}"
  puts "找到的邮箱: #{match[0]}"
else
  puts "未找到邮箱"
end
puts ""

# 字符串方法示例
puts "📝 字符串方法示例 (String methods):"
original_string = "  CodeForge Ruby Example  "
puts "原始字符串: '#{original_string}'"
puts "去除空白: '#{original_string.strip}'"
puts "大写: '#{original_string.strip.upcase}'"
puts "小写: '#{original_string.strip.downcase}'"
puts "单词数: #{original_string.strip.split.length}"
puts ""

# 文件操作示例（模拟）
puts "📁 文件操作概念示例 (File operations concept):"
file_content = ["第一行", "第二行", "第三行"]
puts "模拟文件内容:"
file_content.each_with_index do |line, index|
  puts "行#{index + 1}: #{line}"
end
puts ""

# 时间和日期示例
puts "⏰ 时间示例 (Time example):"
current_time = Time.now
puts "当前时间: #{current_time}"
puts "时间戳: #{current_time.to_i}"
puts "格式化时间: #{current_time.strftime('%Y-%m-%d %H:%M:%S')}"
puts ""

# Proc 和 Lambda 示例
puts "🎯 Proc 和 Lambda 示例 (Proc and Lambda):"
# Proc 示例
square_proc = Proc.new { |x| x * x }
puts "Proc 计算 5 的平方: #{square_proc.call(5)}"

# Lambda 示例
cube_lambda = ->(x) { x * x * x }
puts "Lambda 计算 4 的立方: #{cube_lambda.call(4)}"
puts ""

# 元编程示例
puts "🎪 元编程示例 (Metaprogramming example):"
class DynamicClass
  ["name", "age", "email"].each do |attr|
    define_method("get_#{attr}") do
      instance_variable_get("@#{attr}")
    end
    
    define_method("set_#{attr}") do |value|
      instance_variable_set("@#{attr}", value)
    end
  end
end

dynamic_obj = DynamicClass.new
dynamic_obj.set_name("动态对象")
dynamic_obj.set_age(30)
puts "动态生成的方法: #{dynamic_obj.get_name}, 年龄: #{dynamic_obj.get_age}"
puts ""

# 枚举器示例
puts "🔄 枚举器示例 (Enumerator example):"
enum = (1..5).each
puts "枚举器类: #{enum.class}"
puts "枚举器内容: #{enum.to_a.join(', ')}"

# 链式操作
chained_result = (1..20)
                   .select { |n| n % 3 == 0 }
                   .map { |n| "数字: #{n}" }
                   .first(3)
                   .join(" | ")

puts "链式操作结果: #{chained_result}"
puts ""

puts "🎯 CodeForge Ruby 代码执行完成!"
puts "🎯 CodeForge Ruby execution completed!"
puts ""
puts "感谢使用 CodeForge 代码执行环境! 🚀"
puts "Thank you for using CodeForge!"