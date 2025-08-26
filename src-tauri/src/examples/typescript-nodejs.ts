// TypeScript Node.js 示例代码 - CodeForge 代码执行环境

function greetUser(name) {
    return `Hello, ${name}! 👋`;
}

// 函数定义
function addNumbers(a, b) {
    return {
        result: a + b,
        operation: 'addition'
    };
}

function printArray(arr, label) {
    console.log(`${label}: ${arr.join(' ')}`);
}

function fibonacci(n) {
    if (n <= 1) {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

// 枚举模拟
const Weekday = {
    MONDAY: 1,
    TUESDAY: 2,
    WEDNESDAY: 3,
    THURSDAY: 4,
    FRIDAY: 5,
    SATURDAY: 6,
    SUNDAY: 7
};

// 主函数
function main() {
    console.log("🎉 欢迎使用 CodeForge!");
    console.log("Welcome to CodeForge!");
    console.log("");

    console.log("=========================================");
    console.log("         CodeForge TypeScript Node.js         ");
    console.log("=========================================");
    console.log("");

    // 基本输出示例
    console.log("✅ TypeScript 运行成功! (TypeScript is working!)");
    console.log("⚡ 这是 TypeScript 程序 (This is TypeScript program)");
    console.log("");

    // 变量操作
    const name = "CodeForge";
    const version = "TypeScript";
    const number1 = 10;
    const number2 = 20;
    const calculation = addNumbers(number1, number2);

    console.log("🔢 简单计算 (Simple calculation):");
    console.log(`${number1} + ${number2} = ${calculation.result}`);
    console.log("");

    // 字符串操作
    console.log("📝 字符串操作 (String operations):");
    console.log(`平台名称 (Platform): ${name}`);
    console.log(`语言版本 (Language): ${version}`);
    console.log(`完整信息 (Full info): ${name} - ${version}`);
    console.log("");

    // 循环示例
    console.log("🔄 循环输出 (Loop output):");
    for (let i = 1; i <= 5; i++) {
        console.log(`第 ${i} 次输出 (Output #${i}): Hello from CodeForge!`);
    }
    console.log("");

    // 数组操作
    console.log("🍎 数组示例 (Array example):");
    const fruits = ["苹果", "香蕉", "橙子", "葡萄"];
    fruits.forEach((fruit, index) => {
        console.log(`${index + 1}. ${fruit}`);
    });
    console.log("");

    // 条件判断
    const score = 85;
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

    // 对象示例
    console.log("👤 对象示例 (Object example):");
    const person = {
        name: "张三",
        age: 25,
        height: 175.5
    };

    console.log(`姓名: ${person.name}, 年龄: ${person.age}, 身高: ${person.height} cm`);
    console.log("");

    // 函数示例
    console.log("🎭 函数示例 (Function example):");
    console.log(greetUser("CodeForge用户"));
    console.log("");

    // 数组方法示例
    console.log("💾 数组方法示例 (Array methods example):");
    const dynamicArray = Array.from({ length: 5 }, (_, i) => (i + 1) * 10);
    printArray(dynamicArray, "动态数组");

    // Map 和 Filter 示例
    const squares = dynamicArray.map(x => x * x);
    const evenNumbers = dynamicArray.filter(x => x % 20 === 0);
    printArray(squares, "平方数");
    printArray(evenNumbers, "能被20整除的数");
    console.log("");

    // 解构赋值示例
    console.log("🔍 解构赋值示例 (Destructuring example):");
    const [first, second, ...rest] = fruits;
    console.log(`第一个水果: ${first}`);
    console.log(`第二个水果: ${second}`);
    console.log(`其余水果: ${rest.join(', ')}`);

    const { name: personName, age } = person;
    console.log(`通过解构获取: ${personName}, ${age}岁`);
    console.log("");

    // 递归示例
    console.log("🔄 递归示例 (Recursion example):");
    const fibN = 7;
    const fibResult = fibonacci(fibN);
    console.log(`斐波那契数列第${fibN}项: ${fibResult}`);
    console.log("");

    // 数学库示例
    console.log("📐 数学库示例 (Math library example):");
    const angle = 45.0;
    const radians = (angle * Math.PI) / 180.0;
    console.log(`sin(${angle}°) = ${Math.sin(radians).toFixed(4)}`);
    console.log(`cos(${angle}°) = ${Math.cos(radians).toFixed(4)}`);
    console.log(`sqrt(16) = ${Math.sqrt(16).toFixed(2)}`);
    console.log("");

    // 位操作示例
    console.log("🔧 位操作示例 (Bitwise operations):");
    const a = 12;  // 1100 in binary
    const b = 10;  // 1010 in binary
    console.log(`${a} & ${b} = ${a & b} (AND)`);
    console.log(`${a} | ${b} = ${a | b} (OR)`);
    console.log(`${a} ^ ${b} = ${a ^ b} (XOR)`);
    console.log(`~${a} = ${~a} (NOT)`);
    console.log("");

    // 枚举示例
    console.log("📋 枚举示例 (Enum example):");
    const today = Weekday.WEDNESDAY;
    console.log(`今天是星期${today}`);
    console.log("");

    // Promise 和 async/await 示例
    console.log("⚡ 异步示例 (Async example):");
    const asyncExample = async () => {
        return new Promise((resolve) => {
            setTimeout(() => resolve("异步操作完成!"), 100);
        });
    };

    asyncExample().then((message) => {
        console.log(message);
        console.log("");

        // 类示例
        console.log("🎯 类示例 (Class example):");
        class Calculator {
            constructor() {
                this.history = [];
            }

            add(a, b) {
                const result = a + b;
                this.history.push(`${a} + ${b} = ${result}`);
                return result;
            }

            getHistory() {
                return [...this.history];
            }
        }

        const calc = new Calculator();
        calc.add(5, 3);
        calc.add(10, 7);
        console.log("计算历史:", calc.getHistory());
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