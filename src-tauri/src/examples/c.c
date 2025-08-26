#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

// C 示例代码 - CodeForge 代码执行环境

// 函数声明
void greet_user(const char* name);
int add_numbers(int a, int b);
void print_array(int arr[], int size);
int fibonacci(int n);

int main() {
    printf("🎉 欢迎使用 CodeForge!\n");
    printf("Welcome to CodeForge!\n");
    printf("\n");

    printf("=========================================\n");
    printf("            CodeForge C               \n");
    printf("=========================================\n");
    printf("\n");

    // 基本输出示例
    printf("✅ C 运行成功! (C is working!)\n");
    printf("⚡ 这是 C 程序 (This is C program)\n");
    printf("\n");

    // 变量操作
    const char* name = "CodeForge";
    const char* version = "C";
    int number1 = 10;
    int number2 = 20;
    int result = add_numbers(number1, number2);

    printf("🔢 简单计算 (Simple calculation):\n");
    printf("%d + %d = %d\n", number1, number2, result);
    printf("\n");

    // 字符串操作
    printf("📝 字符串操作 (String operations):\n");
    printf("平台名称 (Platform): %s\n", name);
    printf("语言版本 (Language): %s\n", version);
    printf("完整信息 (Full info): %s - %s\n", name, version);
    printf("\n");

    // 循环示例
    printf("🔄 循环输出 (Loop output):\n");
    for (int i = 1; i <= 5; i++) {
        printf("第 %d 次输出 (Output #%d): Hello from CodeForge!\n", i, i);
    }
    printf("\n");

    // 数组操作
    printf("🍎 数组示例 (Array example):\n");
    char* fruits[] = {"苹果", "香蕉", "橙子", "葡萄"};
    int fruits_count = sizeof(fruits) / sizeof(fruits[0]);
    for (int i = 0; i < fruits_count; i++) {
        printf("%d. %s\n", i + 1, fruits[i]);
    }
    printf("\n");

    // 条件判断
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

    // 指针示例
    printf("🔍 指针示例 (Pointer example):\n");
    int value = 42;
    int* ptr = &value;
    printf("值: %d (Value: %d)\n", value, value);
    printf("地址: %p (Address: %p)\n", (void*)ptr, (void*)ptr);
    printf("通过指针访问: %d (Access via pointer: %d)\n", *ptr, *ptr);
    printf("\n");

    // 函数示例
    printf("🎭 函数示例 (Function example):\n");
    greet_user("CodeForge用户");
    printf("\n");

    // 内存分配示例
    printf("💾 动态内存分配 (Dynamic memory allocation):\n");
    int* dynamic_array = (int*)malloc(5 * sizeof(int));
    if (dynamic_array != NULL) {
        for (int i = 0; i < 5; i++) {
            dynamic_array[i] = (i + 1) * 10;
        }
        printf("动态数组: ");
        print_array(dynamic_array, 5);
        free(dynamic_array);
        printf("内存已释放\n");
    }
    printf("\n");

    // 结构体示例
    printf("👤 结构体示例 (Struct example):\n");
    struct Person {
        char name[50];
        int age;
        float height;
    };

    struct Person person;
    strcpy(person.name, "张三");
    person.age = 25;
    person.height = 175.5;

    printf("姓名: %s, 年龄: %d, 身高: %.1f cm\n",
           person.name, person.age, person.height);
    printf("\n");

    // 递归示例
    printf("🔄 递归示例 (Recursion example):\n");
    int fib_n = 7;
    int fib_result = fibonacci(fib_n);
    printf("斐波那契数列第%d项: %d\n", fib_n, fib_result);
    printf("\n");

    // 数学库示例
    printf("📐 数学库示例 (Math library example):\n");
    double angle = 45.0;
    double radians = angle * M_PI / 180.0;
    printf("sin(%.0f°) = %.4f\n", angle, sin(radians));
    printf("cos(%.0f°) = %.4f\n", angle, cos(radians));
    printf("sqrt(16) = %.2f\n", sqrt(16));
    printf("\n");

    // 位操作示例
    printf("🔧 位操作示例 (Bitwise operations):\n");
    int a = 12;  // 1100 in binary
    int b = 10;  // 1010 in binary
    printf("%d & %d = %d (AND)\n", a, b, a & b);
    printf("%d | %d = %d (OR)\n", a, b, a | b);
    printf("%d ^ %d = %d (XOR)\n", a, b, a ^ b);
    printf("~%d = %d (NOT)\n", a, ~a);
    printf("\n");

    // 枚举示例
    printf("📋 枚举示例 (Enum example):\n");
    enum Weekday {
        MONDAY = 1, TUESDAY, WEDNESDAY, THURSDAY, FRIDAY, SATURDAY, SUNDAY
    };
    enum Weekday today = WEDNESDAY;
    printf("今天是星期%d\n", today);
    printf("\n");

    printf("🎯 CodeForge C 代码执行完成!\n");
    printf("🎯 CodeForge C execution completed!\n");
    printf("\n");
    printf("感谢使用 CodeForge 代码执行环境! 🚀\n");
    printf("Thank you for using CodeForge! 🚀\n");

    return 0;
}

// 函数实现
void greet_user(const char* name) {
    printf("Hello, %s! 👋\n", name);
}

int add_numbers(int a, int b) {
    return a + b;
}

void print_array(int arr[], int size) {
    for (int i = 0; i < size; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");
}

int fibonacci(int n) {
    if (n <= 1) {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}