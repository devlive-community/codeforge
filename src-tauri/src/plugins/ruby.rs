use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct RubyPlugin;

impl LanguagePlugin for RubyPlugin {
    fn get_order(&self) -> i32 {
        14
    }

    fn get_language_name(&self) -> &'static str {
        "Ruby"
    }

    fn get_language_key(&self) -> &'static str {
        "ruby"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "rb".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["--version"]
    }

    fn get_path_command(&self) -> String {
        "which ruby".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("ruby"),
            before_compile: None,
            extension: String::from("rb"),
            execute_home: None,
            run_command: Some(String::from("ruby $filename")),
            after_compile: None,
            template: Some(String::from(
                "# Ruby 示例代码 - CodeForge 代码执行环境\n\nputs \"🎉 欢迎使用 CodeForge!\"\nputs \"Welcome to CodeForge!\"\nputs \"\"\n\nputs \"=========================================\"\nputs \"            CodeForge Ruby            \"\nputs \"=========================================\"\nputs \"\"\n\n# 基本输出示例\nputs \"✅ Ruby 运行成功! (Ruby is working!)\"\nputs \"💎 这是 Ruby 脚本 (This is Ruby script)\"\nputs \"\"\n\n# 变量操作\nname = \"CodeForge\"\nversion = \"Ruby\"\nnumber1 = 10\nnumber2 = 20\nresult = number1 + number2\n\nputs \"🔢 简单计算 (Simple calculation):\"\nputs \"#{number1} + #{number2} = #{result}\"\nputs \"\"\n\n# 字符串操作\nputs \"📝 字符串操作 (String operations):\"\nputs \"平台名称 (Platform): #{name}\"\nputs \"语言版本 (Language): #{version}\"\nputs \"完整信息 (Full info): #{name} - #{version}\"\nputs \"\"\n\n# 循环示例\nputs \"🔄 循环输出 (Loop output):\"\n(1..5).each do |i|\n  puts \"第 #{i} 次输出 (Output ##{i}): Hello from CodeForge!\"\nend\nputs \"\"\n\n# 数组操作\nfruits = [\"苹果\", \"香蕉\", \"橙子\", \"葡萄\"]\nputs \"🍎 水果列表 (Fruit list):\"\nfruits.each_with_index do |fruit, index|\n  puts \"#{index + 1}. #{fruit}\"\nend\nputs \"\"\n\n# 条件判断\nscore = 85\nputs \"📊 成绩评估 (Score evaluation):\"\ncase score\nwhen 90..100\n  puts \"优秀! (Excellent!)\"\nwhen 80..89\n  puts \"良好! (Good!)\"\nwhen 60..79\n  puts \"及格 (Pass)\"\nelse\n  puts \"需要努力 (Need improvement)\"\nend\nputs \"\"\n\n# nil 类型示例\noptional_value = 42\nputs \"🔍 nil 类型示例 (nil example):\"\nif optional_value.nil?\n  puts \"值为空 (Value is nil)\"\nelse\n  puts \"可选值: #{optional_value} (Optional value: #{optional_value})\"\nend\nputs \"\"\n\n# 方法示例\ndef greet_user(username)\n  \"Hello, #{username}! 👋\"\nend\n\nputs \"🎭 方法示例 (Method example):\"\ngreeting = greet_user(\"CodeForge用户\")\nputs greeting\nputs \"\"\n\n# 块（Block）示例\nputs \"🔧 块示例 (Block example):\"\nnumbers = (1..10).to_a\neven_numbers = numbers.select { |num| num.even? }\ndoubled = numbers.map { |num| num * 2 }\nsum = numbers.reduce(0) { |acc, num| acc + num }\n\nputs \"原始数字 (Original): #{numbers.join(', ')}\"\nputs \"偶数 (Even numbers): #{even_numbers.join(', ')}\"\nputs \"翻倍 (Doubled): #{doubled.join(', ')}\"\nputs \"总和 (Sum): #{sum}\"\nputs \"\"\n\n# 哈希（Hash）示例\nputs \"👤 哈希示例 (Hash example):\"\nperson = {\n  name: \"张三\",\n  age: 25,\n  city: \"北京\"\n}\n\nputs \"姓名: #{person[:name]}, 年龄: #{person[:age]}, 城市: #{person[:city]}\"\nputs \"哈希键: #{person.keys.join(', ')}\"\nputs \"哈希值: #{person.values.join(', ')}\"\nputs \"\"\n\n# 符号（Symbol）示例\nputs \"⚡ 符号示例 (Symbol example):\"\nstatus = :active\nactions = {\n  active: \"激活状态\",\n  inactive: \"非活跃状态\",\n  pending: \"等待状态\"\n}\n\nputs \"当前状态: #{status}\"\nputs \"状态描述: #{actions[status]}\"\nputs \"\"\n\n# 类和对象示例\nputs \"🏗️ 类示例 (Class example):\"\nclass Person\n  attr_accessor :name, :age\n  \n  def initialize(name, age)\n    @name = name\n    @age = age\n  end\n  \n  def introduce\n    \"我是#{@name}，今年#{@age}岁\"\n  end\n  \n  def adult?\n    @age >= 18\n  end\nend\n\nperson_obj = Person.new(\"李四\", 28)\nputs person_obj.introduce\nputs \"是否成年: #{person_obj.adult? ? '是' : '否'}\"\nputs \"\"\n\n# 继承示例\nclass Student < Person\n  attr_accessor :school\n  \n  def initialize(name, age, school)\n    super(name, age)\n    @school = school\n  end\n  \n  def introduce\n    super + \"，在#{@school}上学\"\n  end\nend\n\nstudent = Student.new(\"王五\", 20, \"清华大学\")\nputs \"👨‍🎓 继承示例 (Inheritance example):\"\nputs student.introduce\nputs \"\"\n\n# 模块（Module）示例\nmodule MathUtils\n  def self.square(n)\n    n * n\n  end\n  \n  def self.factorial(n)\n    return 1 if n <= 1\n    n * factorial(n - 1)\n  end\nend\n\nputs \"📐 模块示例 (Module example):\"\nputs \"5的平方: #{MathUtils.square(5)}\"\nputs \"5的阶乘: #{MathUtils.factorial(5)}\"\nputs \"\"\n\n# 异常处理示例\nputs \"🚨 异常处理示例 (Exception handling):\"\nbegin\n  risky_number = rand(10)\n  if risky_number < 5\n    raise \"随机数太小了: #{risky_number}\"\n  else\n    puts \"随机数正常: #{risky_number}\"\n  end\nrescue => e\n  puts \"捕获异常: #{e.message}\"\nensure\n  puts \"异常处理完成\"\nend\nputs \"\"\n\n# 正则表达式示例\nputs \"🔤 正则表达式示例 (Regular expressions):\"\ntext = \"CodeForge 是一个很棒的代码执行环境! Email: contact@codeforge.com\"\nemail_regex = /\\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Z|a-z]{2,}\\b/\n\nif match = text.match(email_regex)\n  puts \"文本: #{text}\"\n  puts \"找到的邮箱: #{match[0]}\"\nelse\n  puts \"未找到邮箱\"\nend\nputs \"\"\n\n# 字符串方法示例\nputs \"📝 字符串方法示例 (String methods):\"\noriginal_string = \"  CodeForge Ruby Example  \"\nputs \"原始字符串: '#{original_string}'\"\nputs \"去除空白: '#{original_string.strip}'\"\nputs \"大写: '#{original_string.strip.upcase}'\"\nputs \"小写: '#{original_string.strip.downcase}'\"\nputs \"单词数: #{original_string.strip.split.length}\"\nputs \"\"\n\n# 文件操作示例（模拟）\nputs \"📁 文件操作概念示例 (File operations concept):\"\nfile_content = [\"第一行\", \"第二行\", \"第三行\"]\nputs \"模拟文件内容:\"\nfile_content.each_with_index do |line, index|\n  puts \"行#{index + 1}: #{line}\"\nend\nputs \"\"\n\n# 时间和日期示例\nputs \"⏰ 时间示例 (Time example):\"\ncurrent_time = Time.now\nputs \"当前时间: #{current_time}\"\nputs \"时间戳: #{current_time.to_i}\"\nputs \"格式化时间: #{current_time.strftime('%Y-%m-%d %H:%M:%S')}\"\nputs \"\"\n\n# Proc 和 Lambda 示例\nputs \"🎯 Proc 和 Lambda 示例 (Proc and Lambda):\"\n# Proc 示例\nsquare_proc = Proc.new { |x| x * x }\nputs \"Proc 计算 5 的平方: #{square_proc.call(5)}\"\n\n# Lambda 示例\ncube_lambda = ->(x) { x * x * x }\nputs \"Lambda 计算 4 的立方: #{cube_lambda.call(4)}\"\nputs \"\"\n\n# 元编程示例\nputs \"🎪 元编程示例 (Metaprogramming example):\"\nclass DynamicClass\n  [\"name\", \"age\", \"email\"].each do |attr|\n    define_method(\"get_#{attr}\") do\n      instance_variable_get(\"@#{attr}\")\n    end\n    \n    define_method(\"set_#{attr}\") do |value|\n      instance_variable_set(\"@#{attr}\", value)\n    end\n  end\nend\n\ndynamic_obj = DynamicClass.new\ndynamic_obj.set_name(\"动态对象\")\ndynamic_obj.set_age(30)\nputs \"动态生成的方法: #{dynamic_obj.get_name}, 年龄: #{dynamic_obj.get_age}\"\nputs \"\"\n\n# 枚举器示例\nputs \"🔄 枚举器示例 (Enumerator example):\"\nenum = (1..5).each\nputs \"枚举器类: #{enum.class}\"\nputs \"枚举器内容: #{enum.to_a.join(', ')}\"\n\n# 链式操作\nchained_result = (1..20)\n                   .select { |n| n % 3 == 0 }\n                   .map { |n| \"数字: #{n}\" }\n                   .first(3)\n                   .join(\" | \")\n\nputs \"链式操作结果: #{chained_result}\"\nputs \"\"\n\nputs \"🎯 CodeForge Ruby 代码执行完成!\"\nputs \"🎯 CodeForge Ruby execution completed!\"\nputs \"\"\nputs \"感谢使用 CodeForge 代码执行环境! 🚀\"\nputs \"Thank you for using CodeForge! 🚀\"",
            )),
            timeout: Some(30),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "ruby".to_string())
    }
}
