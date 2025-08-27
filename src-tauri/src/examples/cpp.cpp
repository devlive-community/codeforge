#include <iostream>
#include <string>
#include <vector>
#include <memory>
#include <cmath>
#include <algorithm>
#include <map>

// C++ 示例代码 - CodeForge 代码执行环境

// 类声明
class Person {
private:
    std::string name;
    int age;
    float height;

public:
    Person(const std::string& n, int a, float h);
    void display() const;
    std::string getName() const { return name; }
    int getAge() const { return age; }
};

// 函数声明
void greetUser(const std::string& name);
int addNumbers(int a, int b);
void printVector(const std::vector<int>& vec);
int fibonacci(int n);
template<typename T>
void printArray(const T& container);

int main() {
    std::cout << "🎉 欢迎使用 CodeForge!" << std::endl;
    std::cout << "Welcome to CodeForge!" << std::endl;
    std::cout << std::endl;

    std::cout << "=========================================" << std::endl;
    std::cout << "            CodeForge C++              " << std::endl;
    std::cout << "=========================================" << std::endl;
    std::cout << std::endl;

    // 基本输出示例
    std::cout << "✅ C++ 运行成功! (C++ is working!)" << std::endl;
    std::cout << "⚡ 这是 C++ 程序 (This is C++ program)" << std::endl;
    std::cout << std::endl;

    // 变量操作
    std::string name = "CodeForge";
    std::string version = "C++";
    int number1 = 10;
    int number2 = 20;
    int result = addNumbers(number1, number2);

    std::cout << "🔢 简单计算 (Simple calculation):" << std::endl;
    std::cout << number1 << " + " << number2 << " = " << result << std::endl;
    std::cout << std::endl;

    // 字符串操作
    std::cout << "📝 字符串操作 (String operations):" << std::endl;
    std::cout << "平台名称 (Platform): " << name << std::endl;
    std::cout << "语言版本 (Language): " << version << std::endl;
    std::cout << "完整信息 (Full info): " << name << " - " << version << std::endl;
    std::cout << std::endl;

    // 循环示例
    std::cout << "🔄 循环输出 (Loop output):" << std::endl;
    for (int i = 1; i <= 5; i++) {
        std::cout << "第 " << i << " 次输出 (Output #" << i << "): Hello from CodeForge!" << std::endl;
    }
    std::cout << std::endl;

    // 容器示例 (vector)
    std::cout << "🍎 容器示例 (Container example):" << std::endl;
    std::vector<std::string> fruits = {"苹果", "香蕉", "橙子", "葡萄"};
    for (size_t i = 0; i < fruits.size(); i++) {
        std::cout << i + 1 << ". " << fruits[i] << std::endl;
    }
    std::cout << std::endl;

    // 条件判断
    int score = 85;
    std::cout << "📊 成绩评估 (Score evaluation):" << std::endl;
    if (score >= 90) {
        std::cout << "优秀! (Excellent!)" << std::endl;
    } else if (score >= 80) {
        std::cout << "良好! (Good!)" << std::endl;
    } else if (score >= 60) {
        std::cout << "及格 (Pass)" << std::endl;
    } else {
        std::cout << "需要努力 (Need improvement)" << std::endl;
    }
    std::cout << std::endl;

    // 引用示例
    std::cout << "🔍 引用示例 (Reference example):" << std::endl;
    int value = 42;
    int& ref = value;
    std::cout << "值: " << value << " (Value: " << value << ")" << std::endl;
    std::cout << "地址: " << &value << " (Address: " << &value << ")" << std::endl;
    std::cout << "通过引用访问: " << ref << " (Access via reference: " << ref << ")" << std::endl;
    ref = 50;
    std::cout << "修改引用后的值: " << value << " (Value after modifying reference: " << value << ")" << std::endl;
    std::cout << std::endl;

    // 函数示例
    std::cout << "🎭 函数示例 (Function example):" << std::endl;
    greetUser("CodeForge用户");
    std::cout << std::endl;

    // 智能指针示例
    std::cout << "💾 智能指针示例 (Smart pointer example):" << std::endl;
    auto dynamicArray = std::make_unique<std::vector<int>>(5);
    for (int i = 0; i < 5; i++) {
        (*dynamicArray)[i] = (i + 1) * 10;
    }
    std::cout << "智能指针管理的动态数组: ";
    printVector(*dynamicArray);
    std::cout << "内存自动释放" << std::endl;
    std::cout << std::endl;

    // 类和对象示例
    std::cout << "👤 类和对象示例 (Class and object example):" << std::endl;
    Person person("张三", 25, 175.5f);
    person.display();
    std::cout << std::endl;

    // 递归示例
    std::cout << "🔄 递归示例 (Recursion example):" << std::endl;
    int fib_n = 7;
    int fib_result = fibonacci(fib_n);
    std::cout << "斐波那契数列第" << fib_n << "项: " << fib_result << std::endl;
    std::cout << std::endl;

    // 数学库示例
    std::cout << "📐 数学库示例 (Math library example):" << std::endl;
    double angle = 45.0;
    double radians = angle * M_PI / 180.0;
    std::cout << "sin(" << angle << "°) = " << std::sin(radians) << std::endl;
    std::cout << "cos(" << angle << "°) = " << std::cos(radians) << std::endl;
    std::cout << "sqrt(16) = " << std::sqrt(16) << std::endl;
    std::cout << std::endl;

    // 位操作示例
    std::cout << "🔧 位操作示例 (Bitwise operations):" << std::endl;
    int a = 12;  // 1100 in binary
    int b = 10;  // 1010 in binary
    std::cout << a << " & " << b << " = " << (a & b) << " (AND)" << std::endl;
    std::cout << a << " | " << b << " = " << (a | b) << " (OR)" << std::endl;
    std::cout << a << " ^ " << b << " = " << (a ^ b) << " (XOR)" << std::endl;
    std::cout << "~" << a << " = " << (~a) << " (NOT)" << std::endl;
    std::cout << std::endl;

    // 枚举示例
    std::cout << "📋 枚举示例 (Enum example):" << std::endl;
    enum class Weekday {
        MONDAY = 1, TUESDAY, WEDNESDAY, THURSDAY, FRIDAY, SATURDAY, SUNDAY
    };
    Weekday today = Weekday::WEDNESDAY;
    std::cout << "今天是星期" << static_cast<int>(today) << std::endl;
    std::cout << std::endl;

    // STL算法示例
    std::cout << "🚀 STL算法示例 (STL algorithm example):" << std::endl;
    std::vector<int> numbers = {5, 2, 8, 1, 9, 3};
    std::cout << "原始数组: ";
    printVector(numbers);

    std::sort(numbers.begin(), numbers.end());
    std::cout << "排序后数组: ";
    printVector(numbers);

    auto it = std::find(numbers.begin(), numbers.end(), 8);
    if (it != numbers.end()) {
        std::cout << "找到数字 8 在位置: " << (it - numbers.begin()) << std::endl;
    }
    std::cout << std::endl;

    // Map容器示例
    std::cout << "🗺️ Map容器示例 (Map container example):" << std::endl;
    std::map<std::string, int> scores = {
        {"张三", 85},
        {"李四", 92},
        {"王五", 78}
    };

    for (const auto& pair : scores) {
        std::cout << pair.first << ": " << pair.second << "分" << std::endl;
    }
    std::cout << std::endl;

    // Lambda表达式示例
    std::cout << "🔗 Lambda表达式示例 (Lambda expression example):" << std::endl;
    auto multiply = [](int x, int y) -> int {
        return x * y;
    };
    std::cout << "Lambda计算 5 * 3 = " << multiply(5, 3) << std::endl;
    std::cout << std::endl;

    std::cout << "🎯 CodeForge C++ 代码执行完成!" << std::endl;
    std::cout << "🎯 CodeForge C++ execution completed!" << std::endl;
    std::cout << std::endl;
    std::cout << "感谢使用 CodeForge 代码执行环境! 🚀" << std::endl;
    std::cout << "Thank you for using CodeForge! 🚀" << std::endl;

    return 0;
}

// 类实现
Person::Person(const std::string& n, int a, float h) : name(n), age(a), height(h) {}

void Person::display() const {
    std::cout << "姓名: " << name << ", 年龄: " << age << ", 身高: " << height << " cm" << std::endl;
}

// 函数实现
void greetUser(const std::string& name) {
    std::cout << "Hello, " << name << "! 👋" << std::endl;
}

int addNumbers(int a, int b) {
    return a + b;
}

void printVector(const std::vector<int>& vec) {
    for (const auto& element : vec) {
        std::cout << element << " ";
    }
    std::cout << std::endl;
}

int fibonacci(int n) {
    if (n <= 1) {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

template<typename T>
void printArray(const T& container) {
    for (const auto& element : container) {
        std::cout << element << " ";
    }
    std::cout << std::endl;
}