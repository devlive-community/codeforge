import java.util.*;

// Java 示例代码 - CodeForge 代码执行环境

public class Main {
    public static void main(String[] args) {
        System.out.println("🎉 欢迎使用 CodeForge!");
        System.out.println("Welcome to CodeForge!");
        System.out.println("");

        System.out.println("=========================================");
        System.out.println("           CodeForge Java             ");
        System.out.println("=========================================");
        System.out.println("");

        // 基本输出示例
        System.out.println("✅ Java 运行成功! (Java is working!)");
        System.out.println("☕ 这是 Java 程序 (This is Java program)");
        System.out.println("");

        // 变量操作
        String name = "CodeForge";
        String version = "Java";
        int number1 = 10;
        int number2 = 20;
        int result = number1 + number2;

        System.out.println("🔢 简单计算 (Simple calculation):");
        System.out.printf("%d + %d = %d%n", number1, number2, result);
        System.out.println("");

        // 字符串操作
        System.out.println("📝 字符串操作 (String operations):");
        System.out.println("平台名称 (Platform): " + name);
        System.out.println("语言版本 (Language): " + version);
        System.out.println("完整信息 (Full info): " + name + " - " + version);
        System.out.println("");

        // 循环示例
        System.out.println("🔄 循环输出 (Loop output):");
        for (int i = 1; i <= 5; i++) {
            System.out.printf("第 %d 次输出 (Output #%d): Hello from CodeForge!%n", i, i);
        }
        System.out.println("");

        // 数组操作
        String[] fruits = {"苹果", "香蕉", "橙子", "葡萄"};
        System.out.println("🍎 水果列表 (Fruit list):");
        for (int i = 0; i < fruits.length; i++) {
            System.out.printf("%d. %s%n", i + 1, fruits[i]);
        }
        System.out.println("");

        // 条件判断
        int score = 85;
        System.out.println("📊 成绩评估 (Score evaluation):");
        if (score >= 90) {
            System.out.println("优秀! (Excellent!)");
        } else if (score >= 80) {
            System.out.println("良好! (Good!)");
        } else if (score >= 60) {
            System.out.println("及格 (Pass)");
        } else {
            System.out.println("需要努力 (Need improvement)");
        }

        // 集合操作示例
        List<String> languages = new ArrayList<>();
        languages.add("Java");
        languages.add("Python");
        languages.add("JavaScript");
        languages.add("Go");

        System.out.println("");
        System.out.println("📋 编程语言列表 (Programming languages):");
        for (int i = 0; i < languages.size(); i++) {
            System.out.printf("%d. %s%n", i + 1, languages.get(i));
        }

        // 方法调用示例
        String greeting = greetUser("CodeForge用户");
        System.out.println("");
        System.out.println("🎭 方法示例 (Method example):");
        System.out.println(greeting);

        System.out.println("");
        System.out.println("🎯 CodeForge Java 代码执行完成!");
        System.out.println("🎯 CodeForge Java execution completed!");
        System.out.println("");
        System.out.println("感谢使用 CodeForge 代码执行环境! 🚀");
        System.out.println("Thank you for using CodeForge! 🚀");
    }

    public static String greetUser(String name) {
        return String.format("Hello, %s! 👋", name);
    }
}