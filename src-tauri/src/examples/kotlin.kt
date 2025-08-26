// Kotlin 示例代码 - CodeForge 代码执行环境

fun main() {
    println("🎉 欢迎使用 CodeForge!")
    println("Welcome to CodeForge!")
    println("")

    println("=========================================")
    println("           CodeForge Kotlin           ")
    println("=========================================")
    println("")

    // 基本输出示例
    println("✅ Kotlin 运行成功! (Kotlin is working!)")
    println("🚀 这是 Kotlin 脚本 (This is Kotlin script)")
    println("")

    // 变量操作
    val name = "CodeForge"
    val version = "Kotlin"
    val number1 = 10
    val number2 = 20
    val result = number1 + number2

    println("🔢 简单计算 (Simple calculation):")
    println("$number1 + $number2 = $result")
    println("")

    // 字符串操作
    println("📝 字符串操作 (String operations):")
    println("平台名称 (Platform): $name")
    println("语言版本 (Language): $version")
    println("完整信息 (Full info): $name - $version")
    println("")

    // 循环示例
    println("🔄 循环输出 (Loop output):")
    for (i in 1..5) {
        println("第 $i 次输出 (Output #$i): Hello from CodeForge!")
    }
    println("")

    // 列表操作
    val fruits = listOf("苹果", "香蕉", "橙子", "葡萄")
    println("🍎 水果列表 (Fruit list):")
    fruits.forEachIndexed { index, fruit ->
        println("${index + 1}. $fruit")
    }
    println("")

    // 条件判断
    val score = 85
    println("📊 成绩评估 (Score evaluation):")
    when {
        score >= 90 -> println("优秀! (Excellent!)")
        score >= 80 -> println("良好! (Good!)")
        score >= 60 -> println("及格 (Pass)")
        else -> println("需要努力 (Need improvement)")
    }

    // 可空类型示例
    var nullableValue: Int? = 42
    println("")
    println("🔍 可空类型示例 (Nullable example):")
    nullableValue?.let { value ->
        println("可空值: $value (Nullable value: $value)")
    } ?: println("值为空 (Value is null)")

    // 函数示例
    fun greetUser(name: String): String {
        return "Hello, $name! 👋"
    }

    println("")
    println("🎭 函数示例 (Function example):")
    val greeting = greetUser("CodeForge用户")
    println(greeting)

    // 扩展函数示例
    fun String.addEmoji(): String = "✨ $this ✨"

    println("")
    println("⚡ 扩展函数示例 (Extension function):")
    val decoratedText = "Kotlin很棒".addEmoji()
    println(decoratedText)

    // 数据类示例
    data class Person(val name: String, val age: Int)
    val person = Person("张三", 25)

    println("")
    println("👤 数据类示例 (Data class example):")
    println("姓名: ${person.name}, 年龄: ${person.age}")

    // 解构声明
    val (personName, personAge) = person
    println("解构后 (Destructured): $personName 今年 $personAge 岁")

    // 高阶函数示例
    println("")
    println("🔧 高阶函数示例 (Higher-order function):")
    val numbers = (1..10).toList()
    val evenNumbers = numbers.filter { it % 2 == 0 }
    val doubled = numbers.map { it * 2 }

    println("原始数字 (Original): ${numbers.joinToString(", ")}")
    println("偶数 (Even numbers): ${evenNumbers.joinToString(", ")}")
    println("翻倍 (Doubled): ${doubled.joinToString(", ")}")

    // Lambda 表达式示例
    println("")
    println("🎯 Lambda 表达式 (Lambda expression):")
    val calculate: (Int, Int) -> Int = { a, b -> a * b }
    val product = calculate(6, 7)
    println("6 × 7 = $product")

    // 集合操作链式调用
    println("")
    println("⛓️ 集合链式操作 (Collection chaining):")
    val result2 = (1..20)
        .filter { it % 3 == 0 }
        .map { "数字: $it" }
        .take(3)
        .joinToString(" | ")
    println("3的倍数前3个: $result2")

    // 字符串模板高级用法
    println("")
    println("📝 字符串模板 (String template):")
    val items = listOf("代码", "测试", "部署")
    val status = "进行中"
    println("任务状态: ${items.joinToString(", ")} - $status")

    println("")
    println("🎯 CodeForge Kotlin 代码执行完成!")
    println("🎯 CodeForge Kotlin execution completed!")
    println("")
    println("感谢使用 CodeForge 代码执行环境! 🚀")
    println("Thank you for using CodeForge! 🚀")
}