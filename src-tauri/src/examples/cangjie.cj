// 仓颉示例代码 - CodeForge 代码执行环境

main(): Unit {
    println("🎉 欢迎使用 CodeForge!")
    println("Welcome to CodeForge!")
    println("")

    println("=========================================")
    println("          CodeForge Cangjie            ")
    println("=========================================")
    println("")

    // 基本输出示例
    println("✅ 仓颉运行成功! (Cangjie is working!)")
    println("⚡ 这是仓颉程序 (This is Cangjie program)")
    println("")

    // 变量操作
    let name = "CodeForge"
    let version = "Cangjie"
    let number1 = 10
    let number2 = 20
    let result = number1 + number2

    println("🔢 简单计算 (Simple calculation):")
    println("${number1} + ${number2} = ${result}")
    println("")

    // 字符串操作和插值
    println("📝 字符串操作 (String operations):")
    println("平台名称 (Platform): ${name}")
    println("语言版本 (Language): ${version}")
    println("完整信息 (Full info): ${name} - ${version}")
    println("")

    // 数组操作
    println("🍎 数组示例 (Array example):")
    let fruits = ["苹果", "香蕉", "橙子", "葡萄"]
    for (i in 0..fruits.size) {
        println("${i + 1}. ${fruits[i]}")
    }
    println("")

    // 条件判断
    let score = 85
    println("📊 成绩评估 (Score evaluation):")
    if (score >= 90) {
        println("优秀! (Excellent!)")
    } else if (score >= 80) {
        println("良好! (Good!)")
    } else if (score >= 60) {
        println("及格 (Pass)")
    } else {
        println("需要努力 (Need improvement)")
    }
    println("")

    // 循环示例
    println("🔄 循环输出 (Loop output):")
    for (i in 1..6) {
        println("第 ${i} 次输出 (Output #${i}): Hello from CodeForge!")
    }
    println("")

    // while循环示例
    println("🔁 While循环示例 (While loop example):")
    var counter = 1
    while (counter <= 3) {
        println("While循环: 第 ${counter} 次")
        counter += 1
    }
    println("")

    println("🎯 CodeForge 仓颉代码执行完成!")
    println("🎯 CodeForge Cangjie execution completed!")
    println("")
    println("感谢使用 CodeForge 代码执行环境! 🚀")
    println("Thank you for using CodeForge! 🚀")
}
