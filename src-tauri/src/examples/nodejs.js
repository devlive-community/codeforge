// Node.js 示例代码 - CodeForge 代码执行环境

console.log("🎉 欢迎使用 CodeForge!");
console.log("Welcome to CodeForge!");
console.log("");

console.log("=========================================");
console.log("         CodeForge Node.js            ");
console.log("=========================================");
console.log("");

// 基本输出示例
console.log("✅ Node.js 运行成功! (Node.js is working!)");
console.log("🟢 这是 JavaScript 程序 (This is JavaScript program)");
console.log("");

// 变量操作
const name = "CodeForge";
const version = "Node.js";
let number1 = 10;
let number2 = 20;
let result = number1 + number2;

console.log("🔢 简单计算 (Simple calculation):");
console.log(`${number1} + ${number2} = ${result}`);
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
const fruits = ["苹果", "香蕉", "橙子", "葡萄"];
console.log("🍎 水果列表 (Fruit list):");
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

// 对象操作示例
const user = {
    name: "CodeForge用户",
    age: 25,
    skills: ["JavaScript", "Node.js", "React"]
};

console.log("");
console.log("📦 对象操作 (Object operations):");
console.log(`用户名: ${user.name}`);
console.log(`年龄: ${user.age}`);
console.log(`技能: ${user.skills.join(", ")}`);

// 函数示例
function greetUser(name) {
    return `Hello, ${name}! 👋`;
}

// 箭头函数示例
const calculateSquare = (num) => num * num;

console.log("");
console.log("🎭 函数示例 (Function examples):");
const greeting = greetUser("CodeForge用户");
console.log(greeting);
console.log(`5 的平方是: ${calculateSquare(5)}`);

// Promise 示例
const delay = (ms) => new Promise(resolve => setTimeout(resolve, ms));

async function asyncExample() {
    console.log("");
    console.log("⏱️  异步操作示例 (Async operation example):");
    console.log("开始异步操作... (Starting async operation...)");
    await delay(100);
    console.log("异步操作完成! (Async operation completed!)");
}

// 执行异步示例
asyncExample().then(() => {
    console.log("");
    console.log("🎯 CodeForge Node.js 代码执行完成!");
    console.log("🎯 CodeForge Node.js execution completed!");
    console.log("");
    console.log("感谢使用 CodeForge 代码执行环境! 🚀");
    console.log("Thank you for using CodeForge! 🚀");
});