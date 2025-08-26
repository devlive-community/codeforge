// Scala 示例代码 - CodeForge 代码执行环境

object Main {
  def main(args: Array[String]): Unit = {
    println("🎉 欢迎使用 CodeForge!")
    println("Welcome to CodeForge!")
    println("")

    println("=========================================")
    println("           CodeForge Scala            ")
    println("=========================================")
    println("")

    // 基本输出示例
    println("✅ Scala 运行成功! (Scala is working!)")
    println("⚡ 这是 Scala 脚本 (This is Scala script)")
    println("")

    // 变量操作
    val name = "CodeForge"
    val version = "Scala"
    val number1 = 10
    val number2 = 20
    val result = number1 + number2

    println("🔢 简单计算 (Simple calculation):")
    println(s"$number1 + $number2 = $result")
    println("")

    // 字符串操作
    println("📝 字符串操作 (String operations):")
    println(s"平台名称 (Platform): $name")
    println(s"语言版本 (Language): $version")
    println(s"完整信息 (Full info): $name - $version")
    println("")

    // 循环示例
    println("🔄 循环输出 (Loop output):")
    for (i <- 1 to 5) {
      println(s"第 $i 次输出 (Output #$i): Hello from CodeForge!")
    }
    println("")

    // 列表操作
    val fruits = List("苹果", "香蕉", "橙子", "葡萄")
    println("🍎 水果列表 (Fruit list):")
    fruits.zipWithIndex.foreach { case (fruit, index) =>
      println(s"${index + 1}. $fruit")
    }
    println("")

    // 条件判断
    val score = 85
    println("📊 成绩评估 (Score evaluation):")
    score match {
      case s if s >= 90 => println("优秀! (Excellent!)")
      case s if s >= 80 => println("良好! (Good!)")
      case s if s >= 60 => println("及格 (Pass)")
      case _ => println("需要努力 (Need improvement)")
    }

    // Option 类型示例
    val optionalValue: Option[Int] = Some(42)
    println("")
    println("🔍 Option 类型示例 (Option example):")
    optionalValue match {
      case Some(value) => println(s"可选值: $value (Optional value: $value)")
      case None => println("值为空 (Value is None)")
    }

    // 函数示例
    def greetUser(name: String): String = {
      s"Hello, $name! 👋"
    }

    println("")
    println("🎭 函数示例 (Function example):")
    val greeting = greetUser("CodeForge用户")
    println(greeting)

    // 集合操作示例
    println("")
    println("🗂️ 集合操作示例 (Collection operations):")
    val numbers = (1 to 10).toList
    val evenNumbers = numbers.filter(_ % 2 == 0)
    val doubled = numbers.map(_ * 2)

    println(s"原始数字 (Original): ${numbers.mkString(", ")}")
    println(s"偶数 (Even numbers): ${evenNumbers.mkString(", ")}")
    println(s"翻倍 (Doubled): ${doubled.mkString(", ")}")

    // Case Class 示例
    case class Person(name: String, age: Int)
    val person = Person("张三", 25)

    println("")
    println("👤 Case Class 示例:")
    println(s"姓名: ${person.name}, 年龄: ${person.age}")

    println("")
    println("🎯 CodeForge Scala 代码执行完成!")
    println("🎯 CodeForge Scala execution completed!")
    println("")
    println("感谢使用 CodeForge 代码执行环境! 🚀")
    println("Thank you for using CodeForge! 🚀")
  }
}