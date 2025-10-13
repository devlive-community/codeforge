#import <Foundation/Foundation.h>

int main(int argc, const char * argv[]) {
    @autoreleasepool {
        printf("🎉 欢迎使用 CodeForge!\n");
        printf("Welcome to CodeForge!\n");
        printf("\n");

        printf("=========================================\n");
        printf("          CodeForge Objective-C          \n");
        printf("=========================================\n");
        printf("\n");

        printf("✅ Objective-C运行成功! (Objective-C is working!)\n");
        printf("⚡ 这是Objective-C程序 (This is Objective-C program)\n");
        printf("\n");

        int number1 = 10;
        int number2 = 20;
        int result = number1 + number2;

        printf("🔢 简单计算 (Simple calculation):\n");
        printf("%d + %d = %d\n", number1, number2, result);
        printf("\n");

        printf("📝 字符串操作 (String operations):\n");
        printf("平台名称 (Platform): CodeForge\n");
        printf("语言版本 (Language): Objective-C\n");
        printf("完整信息 (Full info): CodeForge - Objective-C\n");
        printf("\n");

        printf("🍎 数组示例 (Array example):\n");
        NSArray *fruits = @[@"苹果", @"香蕉", @"橙子", @"葡萄"];
        for (int i = 0; i < fruits.count; i++) {
            printf("%d. %s\n", i + 1, [fruits[i] UTF8String]);
        }
        printf("\n");

        int score = 85;
        printf("📊 成绩评估 (Score evaluation):\n");
        if (score >= 90) {
            printf("优秀! (Excellent!)\n");
        } else if (score >= 80) {
            printf("良好! (Good!)\n");
        } else if (score >= 60) {
            printf("及格 (Pass)\n");
        } else {
            printf("需要努力 (Need improvement)\n");
        }
        printf("\n");

        printf("🔄 循环输出 (Loop output):\n");
        for (int i = 1; i <= 5; i++) {
            printf("第 %d 次输出 (Output #%d): Hello from CodeForge!\n", i, i);
        }
        printf("\n");

        printf("🔁 While循环示例 (While loop example):\n");
        int counter = 1;
        while (counter <= 3) {
            printf("While循环: 第 %d 次\n", counter);
            counter++;
        }
        printf("\n");

        printf("🎯 CodeForge Objective-C代码执行完成!\n");
        printf("🎯 CodeForge Objective-C execution completed!\n");
        printf("\n");
        printf("感谢使用 CodeForge 代码执行环境! 🚀\n");
        printf("Thank you for using CodeForge! 🚀\n");
    }
    return 0;
}