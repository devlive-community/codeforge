# R 示例代码 - CodeForge 代码执行环境

cat("欢迎使用 CodeForge!\n")
cat("Welcome to CodeForge!\n\n")

cat("=========================================\n")
cat("            CodeForge R                  \n")
cat("=========================================\n\n")

# 基本输出示例
cat("R 运行成功! (R is working!)\n")
cat("这是 R 程序 (This is R program)\n\n")

# 变量操作
name <- "CodeForge"
version <- "R"
number1 <- 10
number2 <- 20
result <- number1 + number2

cat("简单计算 (Simple calculation):\n")
cat(sprintf("%d + %d = %d\n", number1, number2, result))
cat("\n")

# 字符串操作
cat("字符串操作 (String operations):\n")
cat(sprintf("平台名称 (Platform): %s\n", name))
cat(sprintf("语言版本 (Language): %s\n", version))
cat(sprintf("完整信息 (Full info): %s - %s\n", name, version))
cat("\n")

# 向量操作
cat("向量示例 (Vector example):\n")
fruits <- c("苹果", "香蕉", "橙子", "葡萄")
for (i in 1:length(fruits)) {
  cat(sprintf("%d. %s\n", i, fruits[i]))
}
cat("\n")

# 数值向量
numbers <- c(1, 2, 3, 4, 5)
cat("数值向量示例 (Numeric vector example):\n")
cat("原向量:", numbers, "\n")
cat("向量和:", sum(numbers), "\n")
cat("向量平均值:", mean(numbers), "\n")
cat("向量长度:", length(numbers), "\n\n")

# 条件判断
score <- 85
cat("成绩评估 (Score evaluation):\n")
if (score >= 90) {
  cat("优秀! (Excellent!)\n")
} else if (score >= 80) {
  cat("良好! (Good!)\n")
} else if (score >= 60) {
  cat("及格 (Pass)\n")
} else {
  cat("需要努力 (Need improvement)\n")
}
cat("\n")

# 循环示例
cat("循环输出 (Loop output):\n")
for (i in 1:5) {
  cat(sprintf("第 %d 次输出 (Output #%d): Hello from CodeForge!\n", i, i))
}
cat("\n")

# while循环
cat("While循环示例 (While loop example):\n")
counter <- 1
while (counter <= 3) {
  cat(sprintf("While循环: 第 %d 次\n", counter))
  counter <- counter + 1
}
cat("\n")

# 函数定义
greet_user <- function(user_name) {
  return(paste("Hello,", user_name, "!"))
}

add_numbers <- function(a, b) {
  return(a + b)
}

fibonacci <- function(n) {
  if (n <= 1) return(n)
  return(fibonacci(n - 1) + fibonacci(n - 2))
}

cat("函数示例 (Function example):\n")
cat(greet_user("CodeForge用户"), "\n")
cat("函数计算 5 + 3 =", add_numbers(5, 3), "\n\n")

# 递归示例
cat("递归示例 (Recursion example):\n")
fib_n <- 7
fib_result <- fibonacci(fib_n)
cat(sprintf("斐波那契数列第%d项: %d\n", fib_n, fib_result))
cat("\n")

# 列表操作
cat("列表示例 (List example):\n")
person_list <- list(
  name = "张三",
  age = 25,
  height = 175.5,
  hobbies = c("读书", "游泳", "编程")
)

cat("姓名:", person_list$name, "\n")
cat("年龄:", person_list$age, "\n")
cat("身高:", person_list$height, "cm\n")
cat("爱好:", paste(person_list$hobbies, collapse = ", "), "\n\n")

# 数据框操作
cat("数据框示例 (Data frame example):\n")
students <- data.frame(
  姓名 = c("张三", "李四", "王五"),
  成绩 = c(85, 92, 78),
  年龄 = c(20, 21, 19),
  stringsAsFactors = FALSE
)

cat("学生数据框:\n")
print(students)
cat("\n")

# 数据框统计
cat("数据框统计 (Data frame statistics):\n")
cat("平均成绩:", mean(students$成绩), "\n")
cat("最高成绩:", max(students$成绩), "\n")
cat("最低成绩:", min(students$成绩), "\n")
cat("成绩标准差:", sd(students$成绩), "\n\n")

# 矩阵操作
cat("矩阵示例 (Matrix example):\n")
matrix_data <- matrix(c(1, 2, 3, 4, 5, 6), nrow = 2, ncol = 3)
cat("矩阵:\n")
print(matrix_data)

cat("矩阵维度:", dim(matrix_data), "\n")
cat("矩阵行数:", nrow(matrix_data), "\n")
cat("矩阵列数:", ncol(matrix_data), "\n\n")

# 数学函数
cat("数学函数示例 (Math function examples):\n")
angle <- 45
radians <- angle * pi / 180
cat(sprintf("sin(%.0f°) = %.4f\n", angle, sin(radians)))
cat(sprintf("cos(%.0f°) = %.4f\n", angle, cos(radians)))
cat("sqrt(16) =", sqrt(16), "\n")
cat("log(10) =", log(10), "\n")
cat("exp(1) =", exp(1), "\n\n")

# 统计函数
cat("统计函数示例 (Statistical function examples):\n")
sample_data <- c(5, 2, 8, 1, 9, 3, 7, 4, 6)
cat("数据:", paste(sample_data, collapse = ", "), "\n")
cat("均值:", mean(sample_data), "\n")
cat("中位数:", median(sample_data), "\n")
cat("标准差:", sd(sample_data), "\n")
cat("方差:", var(sample_data), "\n")
cat("最小值:", min(sample_data), "\n")
cat("最大值:", max(sample_data), "\n")
cat("四分位数:\n")
print(quantile(sample_data))
cat("\n")

# 字符串处理
cat("字符串处理示例 (String processing examples):\n")
text <- "CodeForge R Example"
cat("原字符串:", text, "\n")
cat("字符串长度:", nchar(text), "\n")
cat("大写:", toupper(text), "\n")
cat("小写:", tolower(text), "\n")
cat("子字符串:", substr(text, 1, 9), "\n")
cat("替换:", gsub("R", "Programming", text), "\n\n")

# 逻辑运算
cat("逻辑运算示例 (Logical operations example):\n")
a <- TRUE
b <- FALSE
cat("a =", a, ", b =", b, "\n")
cat("a & b =", a & b, "(AND)\n")
cat("a | b =", a | b, "(OR)\n")
cat("!a =", !a, "(NOT)\n\n")

# 向量化操作
cat("向量化操作示例 (Vectorized operations example):\n")
vec1 <- c(1, 2, 3, 4, 5)
vec2 <- c(2, 4, 6, 8, 10)
cat("向量1:", paste(vec1, collapse = ", "), "\n")
cat("向量2:", paste(vec2, collapse = ", "), "\n")
cat("向量相加:", paste(vec1 + vec2, collapse = ", "), "\n")
cat("向量相乘:", paste(vec1 * vec2, collapse = ", "), "\n")
cat("向量平方:", paste(vec1^2, collapse = ", "), "\n\n")

# 条件筛选
cat("条件筛选示例 (Conditional filtering example):\n")
test_scores <- c(85, 92, 78, 96, 73, 88, 91)
cat("所有成绩:", paste(test_scores, collapse = ", "), "\n")
high_scores <- test_scores[test_scores >= 90]
cat("高分(>=90):", paste(high_scores, collapse = ", "), "\n")
low_scores <- test_scores[test_scores < 80]
cat("低分(<80):", paste(low_scores, collapse = ", "), "\n\n")

# 排序操作
cat("排序操作示例 (Sorting operations example):\n")
unsorted_data <- c(5, 2, 8, 1, 9, 3)
cat("原数据:", paste(unsorted_data, collapse = ", "), "\n")
sorted_asc <- sort(unsorted_data)
cat("升序:", paste(sorted_asc, collapse = ", "), "\n")
sorted_desc <- sort(unsorted_data, decreasing = TRUE)
cat("降序:", paste(sorted_desc, collapse = ", "), "\n\n")

# 因子操作
cat("因子示例 (Factor example):\n")
grades <- c("A", "B", "C", "A", "B", "A", "C", "B")
grade_factor <- factor(grades, levels = c("A", "B", "C"))
cat("原始等级:", paste(grades, collapse = ", "), "\n")
cat("因子等级:", paste(as.character(grade_factor), collapse = ", "), "\n")
cat("等级计数:\n")
print(table(grade_factor))
cat("\n")

# 缺失值处理
cat("缺失值处理示例 (Missing value handling example):\n")
data_with_na <- c(1, 2, NA, 4, 5, NA, 7)
cat("包含NA的数据:", paste(data_with_na, collapse = ", "), "\n")
cat("是否为NA:", paste(is.na(data_with_na), collapse = ", "), "\n")
clean_data <- data_with_na[!is.na(data_with_na)]
cat("清理后的数据:", paste(clean_data, collapse = ", "), "\n")
cat("NA数量:", sum(is.na(data_with_na)), "\n\n")

# apply函数族示例
cat("apply函数族示例 (apply family example):\n")
test_matrix <- matrix(1:12, nrow = 3, ncol = 4)
cat("测试矩阵:\n")
print(test_matrix)
cat("行求和 (Row sums):", apply(test_matrix, 1, sum), "\n")
cat("列求和 (Column sums):", apply(test_matrix, 2, sum), "\n\n")

# lapply示例
cat("lapply示例 (lapply example):\n")
number_list <- list(a = 1:5, b = 6:10, c = 11:15)
list_means <- lapply(number_list, mean)
cat("各组平均值:\n")
print(list_means)
cat("\n")

# 随机数生成
cat("随机数生成示例 (Random number generation example):\n")
set.seed(123)  # 设置随机种子以获得可重复结果
random_normal <- rnorm(5, mean = 0, sd = 1)
cat("正态分布随机数:", paste(round(random_normal, 2), collapse = ", "), "\n")
random_uniform <- runif(5, min = 0, max = 10)
cat("均匀分布随机数:", paste(round(random_uniform, 2), collapse = ", "), "\n")
random_integers <- sample(1:100, 5)
cat("随机整数:", paste(random_integers, collapse = ", "), "\n\n")

# 日期和时间
cat("日期时间示例 (Date and time example):\n")
current_date <- Sys.Date()
current_time <- Sys.time()
cat("当前日期:", as.character(current_date), "\n")
cat("当前时间:", as.character(current_time), "\n")
cat("格式化日期:", format(current_date, "%Y年%m月%d日"), "\n\n")

# 简单的数据分析
cat("数据分析示例 (Data analysis example):\n")
analysis_data <- c(23, 45, 56, 78, 32, 67, 89, 12, 34, 56, 78, 90, 23, 45)
cat("分析数据:", paste(analysis_data, collapse = ", "), "\n")
cat("描述性统计:\n")
cat("  样本数:", length(analysis_data), "\n")
cat("  均值:", round(mean(analysis_data), 2), "\n")
cat("  中位数:", median(analysis_data), "\n")
cat("  标准差:", round(sd(analysis_data), 2), "\n")
cat("  最小值:", min(analysis_data), "\n")
cat("  最大值:", max(analysis_data), "\n")
cat("  范围:", max(analysis_data) - min(analysis_data), "\n\n")

# 相关性分析
cat("相关性分析示例 (Correlation analysis example):\n")
x_values <- c(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
y_values <- c(2, 4, 6, 8, 10, 12, 14, 16, 18, 20)
correlation <- cor(x_values, y_values)
cat("x值:", paste(x_values, collapse = ", "), "\n")
cat("y值:", paste(y_values, collapse = ", "), "\n")
cat("相关系数:", round(correlation, 4), "\n\n")

# 线性回归示例
cat("线性回归示例 (Linear regression example):\n")
# 使用更现实的数据，添加一些噪声
set.seed(42)
x_reg <- c(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
y_reg <- 2 * x_reg + 1 + rnorm(10, mean = 0, sd = 0.5)  # 添加噪声
lm_model <- lm(y_reg ~ x_reg)
cat("x值:", paste(x_reg, collapse = ", "), "\n")
cat("y值:", paste(round(y_reg, 2), collapse = ", "), "\n")
cat("回归摘要:\n")
cat("系数:\n")
print(coef(lm_model))
cat("R平方:", round(summary(lm_model)$r.squared, 4), "\n\n")

# 环境信息
cat("环境信息示例 (Environment info example):\n")
cat("R版本:", R.version.string, "\n")
cat("平台:", R.version$platform, "\n")
cat("工作目录:", getwd(), "\n")
cat("搜索路径长度:", length(search()), "\n\n")

# 内存使用
cat("内存使用示例 (Memory usage example):\n")
cat("对象数量:", length(ls()), "\n")
memory_info <- gc()
cat("内存清理完成\n\n")

cat("CodeForge R 代码执行完成!\n")
cat("CodeForge R execution completed!\n\n")
cat("感谢使用 CodeForge 代码执行环境!\n")
cat("Thank you for using CodeForge!\n")