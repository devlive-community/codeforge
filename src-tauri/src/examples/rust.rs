// Rust 示例代码 - CodeForge 代码执行环境

fn main() {
    println!("🎉 欢迎使用 CodeForge!");
    println!("Welcome to CodeForge!");
    println!("");

    println!("=========================================");
    println!("           CodeForge Rust             ");
    println!("=========================================");
    println!("");

    // 基本输出示例
    println!("✅ Rust 运行成功! (Rust is working!)");
    println!("🦀 这是 Rust 版本 (This is Rust)");
    println!("");

    // 简单计算
    let number1 = 10;
    let number2 = 20;
    let result = number1 + number2;

    println!("🔢 简单计算 (Simple calculation):");
    println!("{} + {} = {}", number1, number2, result);
    println!("");

    // 字符串操作
    let name = "CodeForge";
    let version = "Rust";

    println!("📝 字符串操作 (String operations):");
    println!("平台名称 (Platform): {}", name);
    println!("语言版本 (Language): {}", version);
    println!("完整信息 (Full info): {} - {}", name, version);
    println!("");

    // 循环示例
    println!("🔄 循环输出 (Loop output):");
    for i in 1..=5 {
        println!("第 {} 次输出 (Output #{}): Hello from CodeForge!", i, i);
    }
    println!("");

    // 向量操作
    let fruits = vec!["苹果", "香蕉", "橙子", "葡萄"];
    println!("🍎 水果列表 (Fruit list):");
    for (index, fruit) in fruits.iter().enumerate() {
        println!("{}. {}", index + 1, fruit);
    }
    println!("");

    // 条件判断
    let score = 85;
    println!("📊 成绩评估 (Score evaluation):");
    if score >= 90 {
        println!("优秀! (Excellent!)");
    } else if score >= 80 {
        println!("良好! (Good!)");
    } else if score >= 60 {
        println!("及格 (Pass)");
    } else {
        println!("需要努力 (Need improvement)");
    }

    // Rust 特有的所有权演示
    println!("");
    println!("🔒 Rust 所有权演示 (Ownership demonstration):");
    let mut message = String::from("Hello");
    message.push_str(", CodeForge!");
    println!("可变字符串 (Mutable string): {}", message);

    // Option 类型演示
    let maybe_number: Option<i32> = Some(42);
    match maybe_number {
        Some(n) => println!("找到数字 (Found number): {}", n),
        None => println!("没有数字 (No number)"),
    }

    println!("");
    println!("🎯 CodeForge Rust 代码执行完成!");
    println!("🎯 CodeForge Rust execution completed!");
    println!("");
    println!("感谢使用 CodeForge 代码执行环境! 🚀");
    println!("Thank you for using CodeForge! 🚀");
}