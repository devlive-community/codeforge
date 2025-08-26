// TypeScript Browser 示例代码 - CodeForge 代码执行环境

// 接口定义
interface Person {
    name: string;
    age: number;
    height: number;
}

// 枚举定义
enum Weekday {
    MONDAY = 1,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
    SUNDAY
}

// 类型别名
type CalculationResult = {
    result: number;
    operation: string;
};

// 函数定义
function greetUser(name: string): string {
    return "Hello, " + name + "! 👋";
}

function addNumbers(a: number, b: number): CalculationResult {
    return {
        result: a + b,
        operation: 'addition'
    };
}

function printArray(arr: any[], label: string): void {
    console.log(label + ": " + arr.join(' '));
}

function fibonacci(n: number): number {
    if (n <= 1) {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

// 类示例
class Calculator {
    private history: string[] = [];

    public add(a: number, b: number): number {
        var result = a + b;
        this.history.push(a + " + " + b + " = " + result);
        return result;
    }

    public getHistory(): string[] {
        var copy: string[] = [];
        for (var i = 0; i < this.history.length; i++) {
            copy.push(this.history[i]);
        }
        return copy;
    }
}

// 主函数
function main(): void {
    console.log("🎉 欢迎使用 CodeForge!");
    console.log("Welcome to CodeForge!");
    console.log("");

    console.log("=========================================");
    console.log("         CodeForge TypeScript Browser         ");
    console.log("=========================================");
    console.log("");

    // 基本输出示例
    console.log("✅ TypeScript 运行成功! (TypeScript is working!)");
    console.log("⚡ 这是 TypeScript 程序 (This is TypeScript program)");
    console.log("");

    // 变量操作（强类型）
    var name: string = "CodeForge";
    var version: string = "TypeScript";
    var number1: number = 10;
    var number2: number = 20;
    var calculation: CalculationResult = addNumbers(number1, number2);

    console.log("🔢 简单计算 (Simple calculation):");
    console.log(number1 + " + " + number2 + " = " + calculation.result);
    console.log("");

    // 字符串操作
    console.log("📝 字符串操作 (String operations):");
    console.log("平台名称 (Platform): " + name);
    console.log("语言版本 (Language): " + version);
    console.log("完整信息 (Full info): " + name + " - " + version);
    console.log("");

    // 循环示例
    console.log("🔄 循环输出 (Loop output):");
    for (var i = 1; i <= 5; i++) {
        console.log("第 " + i + " 次输出 (Output #" + i + "): Hello from CodeForge!");
    }
    console.log("");

    // 数组操作（强类型数组）
    console.log("🍎 数组示例 (Array example):");
    var fruits: string[] = ["苹果", "香蕉", "橙子", "葡萄"];
    for (var j = 0; j < fruits.length; j++) {
        console.log((j + 1) + ". " + fruits[j]);
    }
    console.log("");

    // 条件判断
    var score: number = 85;
    console.log("📊 成绩评估 (Score evaluation):");
    if (score >= 90) {
        console.log("优秀! (Excellent!)");
    } else if (score >= 80) {
        console.log("良好! (Good!)");
    } else if (score >= 60) {
        console.log("及格 (Pass)");
    } else {
        console.log("需要努力 (Need improvement)");
    }
    console.log("");

    // 对象和接口示例
    console.log("👤 对象和接口示例 (Object and Interface example):");
    var person: Person = {
        name: "张三",
        age: 25,
        height: 175.5
    };

    console.log("姓名: " + person.name + ", 年龄: " + person.age + ", 身高: " + person.height + " cm");
    console.log("");

    // 函数示例
    console.log("🎭 函数示例 (Function example):");
    console.log(greetUser("CodeForge用户"));
    console.log("");

    // 数组方法示例
    console.log("💾 数组操作示例 (Array operations example):");
    var dynamicArray: number[] = [];
    for (var k = 0; k < 5; k++) {
        dynamicArray.push((k + 1) * 10);
    }
    printArray(dynamicArray, "动态数组");

    // 手动实现 map 功能
    var squares: number[] = [];
    for (var l = 0; l < dynamicArray.length; l++) {
        squares.push(dynamicArray[l] * dynamicArray[l]);
    }

    // 手动实现 filter 功能
    var evenNumbers: number[] = [];
    for (var m = 0; m < dynamicArray.length; m++) {
        if (dynamicArray[m] % 20 === 0) {
            evenNumbers.push(dynamicArray[m]);
        }
    }

    printArray(squares, "平方数");
    printArray(evenNumbers, "能被20整除的数");
    console.log("");

    // 递归示例
    console.log("🔄 递归示例 (Recursion example):");
    var fibN: number = 7;
    var fibResult: number = fibonacci(fibN);
    console.log("斐波那契数列第" + fibN + "项: " + fibResult);
    console.log("");

    // 数学库示例
    console.log("📐 数学库示例 (Math library example):");
    var angle: number = 45.0;
    var radians: number = (angle * Math.PI) / 180.0;
    console.log("sin(" + angle + "°) = " + Math.sin(radians).toFixed(4));
    console.log("cos(" + angle + "°) = " + Math.cos(radians).toFixed(4));
    console.log("sqrt(16) = " + Math.sqrt(16).toFixed(2));
    console.log("");

    // 位操作示例
    console.log("🔧 位操作示例 (Bitwise operations):");
    var a: number = 12;  // 1100 in binary
    var b: number = 10;  // 1010 in binary
    console.log(a + " & " + b + " = " + (a & b) + " (AND)");
    console.log(a + " | " + b + " = " + (a | b) + " (OR)");
    console.log(a + " ^ " + b + " = " + (a ^ b) + " (XOR)");
    console.log("~" + a + " = " + (~a) + " (NOT)");
    console.log("");

    // 枚举示例
    console.log("📋 枚举示例 (Enum example):");
    var today: Weekday = Weekday.WEDNESDAY;
    console.log("今天是星期" + today);
    console.log("");

    // 类示例
    console.log("🎯 类示例 (Class example):");
    var calc = new Calculator();
    calc.add(5, 3);
    calc.add(10, 7);
    console.log("计算历史: " + calc.getHistory().join(", "));
    console.log("");

    // 泛型函数示例
    function identity<T>(arg: T): T {
        return arg;
    }

    console.log("🔧 泛型函数示例 (Generic function example):");
    console.log("字符串泛型: " + identity<string>("Hello"));
    console.log("数字泛型: " + identity<number>(42));
    console.log("");

    // 联合类型示例
    function formatValue(value: string | number): string {
        if (typeof value === "string") {
            return "字符串: " + value;
        } else {
            return "数字: " + value.toString();
        }
    }

    console.log("🎲 联合类型示例 (Union type example):");
    console.log(formatValue("TypeScript"));
    console.log(formatValue(2024));
    console.log("");

    // 回调函数示例（模拟异步操作）
    console.log("⚡ 回调函数示例 (Callback example):");
    function simulateAsync(callback: (message: string) => void): void {
        setTimeout(function() {
            callback("异步操作完成!");
        }, 100);
    }

    simulateAsync(function(message: string) {
        console.log(message);
        console.log("");

        console.log("🎯 CodeForge TypeScript 代码执行完成!");
        console.log("🎯 CodeForge TypeScript execution completed!");
        console.log("");
        console.log("感谢使用 CodeForge 代码执行环境! 🚀");
        console.log("Thank you for using CodeForge! 🚀");
    });
}

// 运行主函数
main();