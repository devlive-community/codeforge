// JavaScript (jQuery) 示例代码 - CodeForge 代码执行环境

console.log('🎉 欢迎使用 CodeForge!');
console.log('Welcome to CodeForge!');
console.log('');

console.log('=========================================');
console.log('        CodeForge JavaScript          ');
console.log('=========================================');
console.log('');

// 基本输出示例
console.log('✅ JavaScript 运行成功! (JavaScript is working!)');
console.log('🌟 这是 Node.js 脚本 (This is Node.js script)');
console.log('');

// 变量操作
const name = 'CodeForge';
const version = 'JavaScript';
const number1 = 10;
const number2 = 20;
const result = number1 + number2;

console.log('🔢 简单计算 (Simple calculation):');
console.log(`${number1} + ${number2} = ${result}`);
console.log('');

// 字符串操作
console.log('📝 字符串操作 (String operations):');
console.log(`平台名称 (Platform): ${name}`);
console.log(`语言版本 (Language): ${version}`);
console.log(`完整信息 (Full info): ${name} - ${version}`);
console.log('');

// 循环示例
console.log('🔄 循环输出 (Loop output):');
for (let i = 1; i <= 5; i++) {
    console.log(`第 ${i} 次输出 (Output #${i}): Hello from CodeForge!`);
}
console.log('');

// 数组操作
const fruits = ['苹果', '香蕉', '橙子', '葡萄'];
console.log('🍎 水果列表 (Fruit list):');
fruits.forEach((fruit, index) => {
    console.log(`${index + 1}. ${fruit}`);
});
console.log('');

// 条件判断
const score = 85;
console.log('📊 成绩评估 (Score evaluation):');
if (score >= 90) {
    console.log('优秀! (Excellent!)');
} else if (score >= 80) {
    console.log('良好! (Good!)');
} else if (score >= 60) {
    console.log('及格 (Pass)');
} else {
    console.log('需要努力 (Need improvement)');
}
console.log('');

// undefined 和 null 示例
console.log('🔍 undefined/null 示例 (undefined/null example):');
let optionalValue = 42;
if (optionalValue !== undefined && optionalValue !== null) {
    console.log(`可选值: ${optionalValue} (Optional value: ${optionalValue})`);
} else {
    console.log('值为空 (Value is undefined/null)');
}
console.log('');

// 函数示例
function greetUser(username) {
    return `Hello, ${username}! 👋`;
}

console.log('🎭 函数示例 (Function example):');
const greeting = greetUser('CodeForge用户');
console.log(greeting);
console.log('');

// 箭头函数示例
const addNumbers = (a, b) => a + b;
const multiplyNumbers = (a, b) => {
    return a * b;
};

console.log('⚡ 箭头函数示例 (Arrow function example):');
console.log(`5 + 3 = ${addNumbers(5, 3)}`);
console.log(`6 × 7 = ${multiplyNumbers(6, 7)}`);
console.log('');

// 数组高阶函数示例
console.log('🔧 数组高阶函数示例 (Array higher-order functions):');
const numbers = Array.from({length: 10}, (_, i) => i + 1);
const evenNumbers = numbers.filter(num => num % 2 === 0);
const doubled = numbers.map(num => num * 2);
const sum = numbers.reduce((acc, num) => acc + num, 0);

console.log(`原始数字 (Original): ${numbers.join(', ')}`);
console.log(`偶数 (Even numbers): ${evenNumbers.join(', ')}`);
console.log(`翻倍 (Doubled): ${doubled.join(', ')}`);
console.log(`总和 (Sum): ${sum}`);
console.log('');

// 对象操作示例
console.log('👤 对象示例 (Object example):');
const person = {
    name: '张三',
    age: 25,
    city: '北京',
    introduce() {
        return `我是${this.name}，今年${this.age}岁，住在${this.city}`;
    }
};

console.log(`姓名: ${person.name}, 年龄: ${person.age}, 城市: ${person.city}`);
console.log(person.introduce());
console.log('');

// 解构赋值示例
console.log('🔍 解构赋值示例 (Destructuring assignment):');
const {name: personName, age: personAge} = person;
const [first, second, ...rest] = fruits;

console.log(`解构对象 (Destructured object): ${personName}, ${personAge}`);
console.log(`解构数组 (Destructured array): ${first}, ${second}, 其他: [${rest.join(', ')}]`);
console.log('');

// Promise 示例
console.log('🎯 Promise 示例 (Promise example):');
const delay = (ms) => new Promise(resolve => setTimeout(resolve, ms));

// 使用 Promise
delay(100)
    .then(() => {
        console.log('Promise 已完成! (Promise completed!)');
        return '异步操作结果';
    })
    .then(result => {
        console.log(`异步结果: ${result}`);
    })
    .catch(error => {
        console.error('Promise 错误:', error);
    });

// async/await 示例
async function asyncExample() {
    try {
        console.log('⏳ 开始异步操作...');
        await delay(50);
        console.log('✅ async/await 完成!');
        return '异步函数结果';
    } catch (error) {
        console.error('async/await 错误:', error);
    }
}

// 立即执行异步函数
(async () => {
    const result = await asyncExample();
    console.log(`异步函数返回: ${result}`);
    console.log('');
})();

// 类和继承示例
console.log('🏗️ 类示例 (Class example):');
class Animal {
    constructor(name, type) {
        this.name = name;
        this.type = type;
    }

    speak() {
        return `${this.name} 发出声音`;
    }
}

class Dog extends Animal {
    constructor(name) {
        super(name, '狗');
    }

    speak() {
        return `${this.name} 汪汪叫`;
    }
}

const dog = new Dog('小黄');
console.log(dog.speak());
console.log('');

// 模块和导出示例（注释版本，因为这是单文件）
console.log('📦 模块概念示例 (Module concept example):');
// export const utilities = {
//     formatDate: (date) => date.toLocaleDateString('zh-CN'),
//     randomNumber: () => Math.floor(Math.random() * 100)
// };

const utilities = {
    formatDate: (date) => date.toLocaleDateString('zh-CN'),
    randomNumber: () => Math.floor(Math.random() * 100)
};

console.log(`当前日期: ${utilities.formatDate(new Date())}`);
console.log(`随机数: ${utilities.randomNumber()}`);
console.log('');

// JSON 操作示例
console.log('📄 JSON 操作示例 (JSON operations):');
const data = {
    users: [
        {id: 1, name: 'Alice', active: true},
        {id: 2, name: 'Bob', active: false},
        {id: 3, name: 'Charlie', active: true}
    ]
};

const jsonString = JSON.stringify(data, null, 2);
console.log('JSON 字符串:');
console.log(jsonString);

const parsedData = JSON.parse(jsonString);
const activeUsers = parsedData.users.filter(user => user.active);
console.log(`活跃用户: ${activeUsers.map(u => u.name).join(', ')}`);
console.log('');

// 错误处理示例
console.log('🚨 错误处理示例 (Error handling):');
try {
    const riskyOperation = () => {
        const random = Math.random();
        if (random < 0.5) {
            throw new Error('随机错误发生了!');
        }
        return '操作成功!';
    };

    const result2 = riskyOperation();
    console.log(result2);
} catch (error) {
    console.log(`捕获错误: ${error.message}`);
} finally {
    console.log('错误处理完成');
}
console.log('');

// 正则表达式示例
console.log('🔤 正则表达式示例 (Regular expressions):');
const text = 'CodeForge 是一个很棒的代码执行环境! Email: contact@codeforge.com';
const emailRegex = /\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b/g;
const emails = text.match(emailRegex);

console.log(`文本: ${text}`);
console.log(`找到的邮箱: ${emails ? emails.join(', ') : '无'}`);
console.log('');

console.log('🎯 CodeForge JavaScript 代码执行完成!');
console.log('🎯 CodeForge JavaScript execution completed!');
console.log('');
console.log('感谢使用 CodeForge 代码执行环境! 🚀');
console.log('Thank you for using CodeForge! 🚀');