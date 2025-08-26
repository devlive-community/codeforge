#!/bin/bash
# Shell 示例代码 - CodeForge 代码执行环境

echo "🎉 欢迎使用 CodeForge!"
echo "Welcome to CodeForge!"
echo ""

echo "========================================="
echo "           CodeForge Shell            "
echo "========================================="
echo ""

# 基本输出示例
echo "✅ Shell 运行成功! (Shell is working!)"
echo "🐚 这是 Shell 脚本 (This is Shell script)"
echo ""

# 变量操作
name="CodeForge"
version="Shell"
number1=10
number2=20
result=$((number1 + number2))

echo "🔢 简单计算 (Simple calculation):"
echo "$number1 + $number2 = $result"
echo ""

# 字符串操作
echo "📝 字符串操作 (String operations):"
echo "平台名称 (Platform): $name"
echo "语言版本 (Language): $version"
echo "完整信息 (Full info): $name - $version"
echo ""

# 循环示例
echo "🔄 循环输出 (Loop output):"
for i in {1..5}; do
    echo "第 $i 次输出 (Output #$i): Hello from CodeForge!"
done
echo ""

# 数组操作
fruits=("苹果" "香蕉" "橙子" "葡萄")
echo "🍎 水果列表 (Fruit list):"
for i in "${!fruits[@]}"; do
    echo "$((i + 1)). ${fruits[i]}"
done
echo ""

# 条件判断
score=85
echo "📊 成绩评估 (Score evaluation):"
if [ $score -ge 90 ]; then
    echo "优秀! (Excellent!)"
elif [ $score -ge 80 ]; then
    echo "良好! (Good!)"
elif [ $score -ge 60 ]; then
    echo "及格 (Pass)"
else
    echo "需要努力 (Need improvement)"
fi

echo ""
echo "🎯 CodeForge Shell 代码执行完成!"
echo "🎯 CodeForge Shell execution completed!"
echo ""
echo "感谢使用 CodeForge 代码执行环境! 🚀"
echo "Thank you for using CodeForge! 🚀"