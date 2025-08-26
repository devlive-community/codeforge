package main

import (
	"fmt"
)

// Go 示例代码 - CodeForge 代码执行环境

func main() {
	fmt.Println("🎉 欢迎使用 CodeForge!")
	fmt.Println("Welcome to CodeForge!")
	fmt.Println("")

	fmt.Println("=========================================")
	fmt.Println("           CodeForge Go               ")
	fmt.Println("=========================================")
	fmt.Println("")

	// 基本输出示例
	fmt.Println("✅ Go 运行成功! (Go is working!)")
	fmt.Println("🐹 这是 Go 程序 (This is Go program)")
	fmt.Println("")

	// 变量操作
	name := "CodeForge"
	version := "Go"
	number1 := 10
	number2 := 20
	result := number1 + number2

	fmt.Println("🔢 简单计算 (Simple calculation):")
	fmt.Printf("%d + %d = %d\n", number1, number2, result)
	fmt.Println("")

	// 字符串操作
	fmt.Println("📝 字符串操作 (String operations):")
	fmt.Printf("平台名称 (Platform): %s\n", name)
	fmt.Printf("语言版本 (Language): %s\n", version)
	fmt.Printf("完整信息 (Full info): %s - %s\n", name, version)
	fmt.Println("")

	// 循环示例
	fmt.Println("🔄 循环输出 (Loop output):")
	for i := 1; i <= 5; i++ {
		fmt.Printf("第 %d 次输出 (Output #%d): Hello from CodeForge!\n", i, i)
	}
	fmt.Println("")

	// 切片操作
	fruits := []string{"苹果", "香蕉", "橙子", "葡萄"}
	fmt.Println("🍎 水果列表 (Fruit list):")
	for i, fruit := range fruits {
		fmt.Printf("%d. %s\n", i+1, fruit)
	}
	fmt.Println("")

	// 条件判断
	score := 85
	fmt.Println("📊 成绩评估 (Score evaluation):")
	if score >= 90 {
		fmt.Println("优秀! (Excellent!)")
	} else if score >= 80 {
		fmt.Println("良好! (Good!)")
	} else if score >= 60 {
		fmt.Println("及格 (Pass)")
	} else {
		fmt.Println("需要努力 (Need improvement)")
	}

	// 指针示例
	value := 42
	ptr := &value
	fmt.Println("")
	fmt.Println("🔍 指针示例 (Pointer example):")
	fmt.Printf("值: %d, 地址: %p (Value: %d, Address: %p)\n", *ptr, ptr, *ptr, ptr)

	// 函数示例
	greeting := greetUser("CodeForge用户")
	fmt.Println("")
	fmt.Println("🎭 函数示例 (Function example):")
	fmt.Println(greeting)

	fmt.Println("")
	fmt.Println("🎯 CodeForge Go 代码执行完成!")
	fmt.Println("🎯 CodeForge Go execution completed!")
	fmt.Println("")
	fmt.Println("感谢使用 CodeForge 代码执行环境! 🚀")
	fmt.Println("Thank you for using CodeForge! 🚀")
}

func greetUser(name string) string {
	return fmt.Sprintf("Hello, %s! 👋", name)
}