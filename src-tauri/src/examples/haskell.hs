-- Haskell示例代码 - CodeForge 代码执行环境

import Data.List (intercalate)

main :: IO ()
main = do
    putStrLn "🎉 欢迎使用 CodeForge!"
    putStrLn "Welcome to CodeForge!"
    putStrLn ""

    putStrLn "========================================="
    putStrLn "          CodeForge Haskell            "
    putStrLn "========================================="
    putStrLn ""

    -- 基本输出示例
    putStrLn "✅ Haskell运行成功! (Haskell is working!)"
    putStrLn "⚡ 这是Haskell程序 (This is Haskell program)"
    putStrLn ""

    -- 变量操作
    let name = "CodeForge"
        version = "Haskell"
        number1 = 10
        number2 = 20
        result = number1 + number2

    putStrLn "🔢 简单计算 (Simple calculation):"
    putStrLn $ show number1 ++ " + " ++ show number2 ++ " = " ++ show result
    putStrLn ""

    -- 字符串操作
    putStrLn "📝 字符串操作 (String operations):"
    putStrLn $ "平台名称 (Platform): " ++ name
    putStrLn $ "语言版本 (Language): " ++ version
    putStrLn $ "完整信息 (Full info): " ++ name ++ " - " ++ version
    putStrLn ""

    -- 列表操作
    putStrLn "🍎 列表示例 (List example):"
    let fruits = ["苹果", "香蕉", "橙子", "葡萄"]
    mapM_ (\(i, fruit) -> putStrLn $ show (i + 1) ++ ". " ++ fruit) (zip [0..] fruits)
    putStrLn ""

    -- 条件判断
    let score = 85
    putStrLn "📊 成绩评估 (Score evaluation):"
    putStrLn $ evaluateScore score
    putStrLn ""

    -- 循环示例
    putStrLn "🔄 循环输出 (Loop output):"
    mapM_ (\i -> putStrLn $ "第 " ++ show i ++ " 次输出 (Output #" ++ show i ++ "): Hello from CodeForge!") [1..5]
    putStrLn ""

    -- 递归示例 (模拟while循环)
    putStrLn "🔁 递归示例 (Recursion example):"
    countDown 3
    putStrLn ""

    putStrLn "🎯 CodeForge Haskell代码执行完成!"
    putStrLn "🎯 CodeForge Haskell execution completed!"
    putStrLn ""
    putStrLn "感谢使用 CodeForge 代码执行环境! 🚀"
    putStrLn "Thank you for using CodeForge! 🚀"

-- 评估分数的函数
evaluateScore :: Int -> String
evaluateScore score
    | score >= 90 = "优秀! (Excellent!)"
    | score >= 80 = "良好! (Good!)"
    | score >= 60 = "及格 (Pass)"
    | otherwise   = "需要努力 (Need improvement)"

-- 递归倒计数函数 (模拟while循环)
countDown :: Int -> IO ()
countDown 0 = return ()
countDown n = do
    putStrLn $ "递归循环: 第 " ++ show (4 - n) ++ " 次"
    countDown (n - 1)