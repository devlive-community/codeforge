<?php
// PHP 示例代码 - CodeForge 代码执行环境

echo "🎉 欢迎使用 CodeForge!\n";
echo "Welcome to CodeForge!\n\n";

echo "=========================================\n";
echo "            CodeForge PHP              \n";
echo "=========================================\n\n";

// 基本输出示例
echo "✅ PHP 运行成功! (PHP is working!)\n";
echo "⚡ 这是 PHP 程序 (This is PHP program)\n\n";

// 变量操作
$name = "CodeForge";
$version = "PHP";
$number1 = 10;
$number2 = 20;
$result = $number1 + $number2;

echo "🔢 简单计算 (Simple calculation):\n";
echo "$number1 + $number2 = $result\n\n";

// 字符串操作
echo "📝 字符串操作 (String operations):\n";
echo "平台名称 (Platform): $name\n";
echo "语言版本 (Language): $version\n";
echo "完整信息 (Full info): $name - $version\n\n";

// 数组操作
echo "🍎 数组示例 (Array example):\n";
$fruits = ["苹果", "香蕉", "橙子", "葡萄"];
foreach ($fruits as $index => $fruit) {
    echo ($index + 1) . ". $fruit\n";
}
echo "\n";

// 关联数组
echo "📋 关联数组示例 (Associative array example):\n";
$scores = [
    "张三" => 85,
    "李四" => 92,
    "王五" => 78
];

foreach ($scores as $student => $score) {
    echo "$student: {$score}分\n";
}
echo "\n";

// 条件判断
$score = 85;
echo "📊 成绩评估 (Score evaluation):\n";
if ($score >= 90) {
    echo "优秀! (Excellent!)\n";
} elseif ($score >= 80) {
    echo "良好! (Good!)\n";
} elseif ($score >= 60) {
    echo "及格 (Pass)\n";
} else {
    echo "需要努力 (Need improvement)\n";
}
echo "\n";

// 循环示例
echo "🔄 循环输出 (Loop output):\n";
for ($i = 1; $i <= 5; $i++) {
    echo "第 $i 次输出 (Output #$i): Hello from CodeForge!\n";
}
echo "\n";

// while循环示例
echo "🔁 While循环示例 (While loop example):\n";
$counter = 1;
while ($counter <= 3) {
    echo "While循环: 第 $counter 次\n";
    $counter++;
}
echo "\n";

// 函数定义和调用
function greetUser($userName) {
    return "Hello, $userName! 👋";
}

function addNumbers($a, $b) {
    return $a + $b;
}

function fibonacci($n) {
    if ($n <= 1) return $n;
    return fibonacci($n - 1) + fibonacci($n - 2);
}

echo "🎭 函数示例 (Function example):\n";
echo greetUser("CodeForge用户") . "\n";
echo "函数计算 5 + 3 = " . addNumbers(5, 3) . "\n\n";

// 递归示例
echo "🔄 递归示例 (Recursion example):\n";
$fibN = 7;
$fibResult = fibonacci($fibN);
echo "斐波那契数列第{$fibN}项: $fibResult\n\n";

// 类和对象示例
echo "👤 类和对象示例 (Class and object example):\n";

class Person {
    private $name;
    private $age;
    private $height;

    public function __construct($name, $age, $height) {
        $this->name = $name;
        $this->age = $age;
        $this->height = $height;
    }

    public function display() {
        return "姓名: {$this->name}, 年龄: {$this->age}, 身高: {$this->height} cm";
    }

    public function getName() {
        return $this->name;
    }

    public function getAge() {
        return $this->age;
    }
}

$person = new Person("张三", 25, 175.5);
echo $person->display() . "\n\n";

// 静态方法示例
class Calculator {
    public static function multiply($a, $b) {
        return $a * $b;
    }

    public static function power($base, $exponent) {
        return pow($base, $exponent);
    }
}

echo "🧮 静态方法示例 (Static method example):\n";
echo "Calculator::multiply(5, 3) = " . Calculator::multiply(5, 3) . "\n";
echo "Calculator::power(2, 8) = " . Calculator::power(2, 8) . "\n\n";

// 字符串函数示例
echo "🔤 字符串函数示例 (String function examples):\n";
$text = "CodeForge PHP Example";
echo "原字符串: $text\n";
echo "长度: " . strlen($text) . "\n";
echo "大写: " . strtoupper($text) . "\n";
echo "小写: " . strtolower($text) . "\n";
echo "反转: " . strrev($text) . "\n";
echo "替换: " . str_replace("PHP", "Programming", $text) . "\n\n";

// 数组函数示例
echo "📊 数组函数示例 (Array function examples):\n";
$numbers = [5, 2, 8, 1, 9, 3];
echo "原数组: " . implode(", ", $numbers) . "\n";
sort($numbers);
echo "排序后: " . implode(", ", $numbers) . "\n";
echo "数组长度: " . count($numbers) . "\n";
echo "数组和: " . array_sum($numbers) . "\n";
echo "最大值: " . max($numbers) . "\n";
echo "最小值: " . min($numbers) . "\n\n";

// 数学函数示例
echo "📐 数学函数示例 (Math function examples):\n";
$angle = 45;
$radians = deg2rad($angle);
echo "sin({$angle}°) = " . round(sin($radians), 4) . "\n";
echo "cos({$angle}°) = " . round(cos($radians), 4) . "\n";
echo "sqrt(16) = " . sqrt(16) . "\n";
echo "round(3.14159, 2) = " . round(3.14159, 2) . "\n";
echo "rand(1, 10) = " . rand(1, 10) . "\n\n";

// 日期和时间示例
echo "📅 日期时间示例 (Date and time examples):\n";
echo "当前时间: " . date('Y-m-d H:i:s') . "\n";
echo "当前时间戳: " . time() . "\n";
echo "格式化日期: " . date('Y年m月d日') . "\n";
echo "星期: " . date('l') . "\n\n";

// JSON操作示例
echo "📋 JSON操作示例 (JSON operations):\n";
$data = [
    "name" => "CodeForge",
    "language" => "PHP",
    "features" => ["易学", "功能强大", "开源"]
];
$jsonString = json_encode($data, JSON_UNESCAPED_UNICODE);
echo "编码JSON: $jsonString\n";

$decodedData = json_decode($jsonString, true);
echo "解码JSON: ";
print_r($decodedData);
echo "\n";

// 文件操作示例（模拟）
echo "📁 文件操作示例 (File operations simulation):\n";
$filename = "example.txt";
$content = "这是一个PHP文件操作示例";
echo "模拟写入文件 '$filename': $content\n";
echo "模拟文件大小: " . strlen($content) . " 字节\n\n";

// 异常处理示例
echo "⚠️ 异常处理示例 (Exception handling):\n";
try {
    $divisor = 0;
    if ($divisor == 0) {
        throw new Exception("除数不能为零");
    }
    $result = 10 / $divisor;
    echo "结果: $result\n";
} catch (Exception $e) {
    echo "捕获异常: " . $e->getMessage() . "\n";
} finally {
    echo "finally块总是执行\n";
}
echo "\n";

// 正则表达式示例
echo "🔍 正则表达式示例 (Regular expression examples):\n";
$email = "test@codeforge.com";
$pattern = '/^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/';
if (preg_match($pattern, $email)) {
    echo "邮箱验证 '$email': 有效\n";
} else {
    echo "邮箱验证 '$email': 无效\n";
}

$text = "CodeForge 2025 是最好的代码执行平台!";
preg_match_all('/\d+/', $text, $matches);
echo "提取数字: " . implode(", ", $matches[0]) . "\n\n";

// 闭包示例
echo "🔗 闭包示例 (Closure examples):\n";
$multiplier = function($x, $y) {
    return $x * $y;
};

$greeter = function($userName) {
    return "Hello, $userName!";
};

echo "闭包乘法 5 * 3 = " . $multiplier(5, 3) . "\n";
echo "闭包问候: " . $greeter('PHP') . "\n\n";

// 命名空间示例（模拟）
echo "📦 命名空间概念示例 (Namespace concept example):\n";
echo "假设命名空间: CodeForge\\Utils\\Calculator\n";
echo "完全限定名: \\CodeForge\\Utils\\Calculator::add()\n\n";

// 魔术方法示例
echo "✨ 魔术方法示例 (Magic method examples):\n";

class MagicDemo {
    private $data = [];

    public function __get($name) {
        return $this->data[$name] ?? null;
    }

    public function __set($name, $value) {
        $this->data[$name] = $value;
    }

    public function __toString() {
        return "MagicDemo对象包含: " . implode(", ", array_keys($this->data));
    }
}

$magic = new MagicDemo();
$magic->name = "测试";
$magic->value = 123;
echo "魔术属性: " . $magic->name . "\n";
echo "魔术字符串: " . $magic . "\n\n";

// 类型检查示例
echo "🔍 类型检查示例 (Type checking examples):\n";
$var1 = 42;
$var2 = "Hello";
$var3 = [1, 2, 3];
$var4 = new stdClass();

echo "is_int(42): " . (is_int($var1) ? 'true' : 'false') . "\n";
echo "is_string('Hello'): " . (is_string($var2) ? 'true' : 'false') . "\n";
echo "is_array([1,2,3]): " . (is_array($var3) ? 'true' : 'false') . "\n";
echo "is_object(stdClass): " . (is_object($var4) ? 'true' : 'false') . "\n";
echo "gettype(42): " . gettype($var1) . "\n";
echo "gettype('Hello'): " . gettype($var2) . "\n";
echo "gettype([1,2,3]): " . gettype($var3) . "\n";
echo "gettype(stdClass): " . gettype($var4) . "\n\n";

// 变量函数示例
echo "🔧 变量函数示例 (Variable function examples):\n";
$funcName = 'strtoupper';
$text = 'hello world';
echo "变量函数调用 $funcName('$text'): " . $funcName($text) . "\n\n";

// 引用示例
echo "🔗 引用示例 (Reference examples):\n";
$originalValue = 10;
$referenceValue = &$originalValue;
$referenceValue = 20;
echo "原始值: $originalValue (通过引用修改)\n";
echo "引用值: $referenceValue\n\n";

// 常量示例
echo "📌 常量示例 (Constants examples):\n";
define('SITE_NAME', 'CodeForge');
define('VERSION', '1.0.0');
echo "常量 SITE_NAME: " . SITE_NAME . "\n";
echo "常量 VERSION: " . VERSION . "\n";
echo "PHP版本: " . PHP_VERSION . "\n";
echo "操作系统: " . PHP_OS . "\n\n";

// 超全局变量示例
echo "🌐 超全局变量示例 (Superglobal variables examples):\n";
echo "服务器信息模拟:\n";
echo "- HTTP_HOST: localhost\n";
echo "- REQUEST_METHOD: GET\n";
echo "- PHP_SELF: /index.php\n\n";

// 包含和继承示例概念
echo "📚 面向对象概念示例 (OOP concepts):\n";

abstract class Animal {
    protected $name;

    public function __construct($name) {
        $this->name = $name;
    }

    abstract public function makeSound();

    public function getName() {
        return $this->name;
    }
}

class Dog extends Animal {
    public function makeSound() {
        return "汪汪!";
    }
}

class Cat extends Animal {
    public function makeSound() {
        return "喵喵!";
    }
}

$dog = new Dog("小黄");
$cat = new Cat("小白");

echo "狗 " . $dog->getName() . " 说: " . $dog->makeSound() . "\n";
echo "猫 " . $cat->getName() . " 说: " . $cat->makeSound() . "\n\n";

// 接口示例
echo "🔌 接口示例 (Interface examples):\n";

interface Flyable {
    public function fly();
}

class Bird implements Flyable {
    public function fly() {
        return "鸟儿在天空飞翔!";
    }
}

$bird = new Bird();
echo $bird->fly() . "\n\n";

// 性能测试示例
echo "⏱️ 性能测试示例 (Performance test example):\n";
$startTime = microtime(true);

// 模拟一些计算
$sum = 0;
for ($i = 0; $i < 100000; $i++) {
    $sum += $i;
}

$endTime = microtime(true);
$executionTime = ($endTime - $startTime) * 1000;

echo "计算结果: $sum\n";
echo "执行时间: " . round($executionTime, 2) . " 毫秒\n\n";

// 内存使用示例
echo "💾 内存使用示例 (Memory usage example):\n";
echo "内存使用量: " . number_format(memory_get_usage()) . " 字节\n";
echo "内存峰值: " . number_format(memory_get_peak_usage()) . " 字节\n\n";

echo "🎯 CodeForge PHP 代码执行完成!\n";
echo "🎯 CodeForge PHP execution completed!\n\n";
echo "感谢使用 CodeForge 代码执行环境! 🚀\n";
echo "Thank you for using CodeForge! 🚀\n";
?>