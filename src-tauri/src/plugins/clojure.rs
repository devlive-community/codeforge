use super::{LanguagePlugin, PluginConfig};
use std::vec;

pub struct ClojurePlugin;

impl LanguagePlugin for ClojurePlugin {
    fn get_order(&self) -> i32 {
        11
    }

    fn get_language_name(&self) -> &'static str {
        "Clojure"
    }

    fn get_language_key(&self) -> &'static str {
        "clojure"
    }

    fn get_file_extension(&self) -> String {
        self.get_config()
            .map(|config| config.extension.clone())
            .unwrap_or_else(|| "clj".to_string())
    }

    fn get_version_args(&self) -> Vec<&'static str> {
        vec!["-e", "(println (clojure-version))"]
    }

    fn get_path_command(&self) -> String {
        "which clojure".to_string()
    }

    fn get_default_config(&self) -> PluginConfig {
        PluginConfig {
            enabled: true,
            language: String::from("clojure"),
            before_compile: None,
            extension: String::from("clj"),
            execute_home: None,
            run_command: Some(String::from("clojure $filename")),
            after_compile: None,
            template: Some(String::from(
                ";; Clojure 示例代码 - CodeForge 代码执行环境\n\n(println \"🎉 欢迎使用 CodeForge!\")\n(println \"Welcome to CodeForge!\")\n(println \"\")\n\n(println \"=========================================\")\n(println \"           CodeForge Clojure          \")\n(println \"=========================================\")\n(println \"\")\n\n;; 基本输出示例\n(println \"✅ Clojure 运行成功! (Clojure is working!)\")\n(println \"🔥 这是 Clojure 脚本 (This is Clojure script)\")\n(println \"\")\n\n;; 变量操作\n(def name \"CodeForge\")\n(def version \"Clojure\")\n(def number1 10)\n(def number2 20)\n(def result (+ number1 number2))\n\n(println \"🔢 简单计算 (Simple calculation):\")\n(println (str number1 \" + \" number2 \" = \" result))\n(println \"\")\n\n;; 字符串操作\n(println \"📝 字符串操作 (String operations):\")\n(println (str \"平台名称 (Platform): \" name))\n(println (str \"语言版本 (Language): \" version))\n(println (str \"完整信息 (Full info): \" name \" - \" version))\n(println \"\")\n\n;; 循环示例\n(println \"🔄 循环输出 (Loop output):\")\n(doseq [i (range 1 6)]\n  (println (str \"第 \" i \" 次输出 (Output #\" i \"): Hello from CodeForge!\")))\n(println \"\")\n\n;; 向量操作\n(def fruits [\"苹果\" \"香蕉\" \"橙子\" \"葡萄\"])\n(println \"🍎 水果列表 (Fruit list):\")\n(doseq [[index fruit] (map-indexed vector fruits)]\n  (println (str (inc index) \". \" fruit)))\n(println \"\")\n\n;; 条件判断\n(def score 85)\n(println \"📊 成绩评估 (Score evaluation):\")\n(cond\n  (>= score 90) (println \"优秀! (Excellent!)\")\n  (>= score 80) (println \"良好! (Good!)\")\n  (>= score 60) (println \"及格 (Pass)\")\n  :else (println \"需要努力 (Need improvement)\"))\n\n;; nil 类型示例\n(def optional-value 42)\n(println \"\")\n(println \"🔍 nil 类型示例 (nil example):\")\n(if optional-value\n  (println (str \"可选值: \" optional-value \" (Optional value: \" optional-value \")\"))\n  (println \"值为空 (Value is nil)\"))\n\n;; 函数示例\n(defn greet-user [username]\n  (str \"Hello, \" username \"! 👋\"))\n\n(println \"\")\n(println \"🎭 函数示例 (Function example):\")\n(def greeting (greet-user \"CodeForge用户\"))\n(println greeting)\n\n;; 高阶函数示例\n(println \"\")\n(println \"🔧 高阶函数示例 (Higher-order function):\")\n(def numbers (range 1 11))\n(def even-numbers (filter even? numbers))\n(def doubled (map #(* % 2) numbers))\n\n(println (str \"原始数字 (Original): \" (clojure.string/join \", \" numbers)))\n(println (str \"偶数 (Even numbers): \" (clojure.string/join \", \" even-numbers)))\n(println (str \"翻倍 (Doubled): \" (clojure.string/join \", \" doubled)))\n\n;; Lambda 表达式示例\n(println \"\")\n(println \"🎯 Lambda 表达式 (Lambda expression):\")\n(def calculate #(* %1 %2))\n(def product (calculate 6 7))\n(println (str \"6 × 7 = \" product))\n\n;; 数据结构示例\n(println \"\")\n(println \"📦 数据结构示例 (Data structures):\")\n\n;; 哈希映射\n(def person {:name \"张三\" :age 25 :city \"北京\"})\n(println (str \"姓名: \" (:name person) \", 年龄: \" (:age person) \", 城市: \" (:city person)))\n\n;; 集合操作\n(def numbers-set #{1 2 3 4 5})\n(def letters-set #{\\a \\b \\c \\d})\n(println (str \"数字集合: \" numbers-set))\n(println (str \"字母集合: \" letters-set))\n\n;; 线程宏示例\n(println \"\")\n(println \"⛓️ 线程宏示例 (Threading macro):\")\n(def result2 (->> (range 1 21)\n                  (filter #(= 0 (mod % 3)))\n                  (map #(str \"数字: \" %))\n                  (take 3)\n                  (clojure.string/join \" | \")))\n(println (str \"3的倍数前3个: \" result2))\n\n;; 原子和状态管理\n(println \"\")\n(println \"⚛️ 原子示例 (Atom example):\")\n(def counter (atom 0))\n(swap! counter inc)\n(swap! counter inc)\n(println (str \"计数器值: \" @counter))\n\n;; 解构示例\n(println \"\")\n(println \"🔍 解构示例 (Destructuring):\")\n(def coords [10 20])\n(let [[x y] coords]\n  (println (str \"坐标 x: \" x \", y: \" y)))\n\n;; 递归示例\n(println \"\")\n(println \"🔄 递归示例 (Recursion):\")\n(defn fibonacci [n]\n  (if (<= n 1)\n    n\n    (+ (fibonacci (- n 1)) (fibonacci (- n 2)))))\n\n(def fib-5 (fibonacci 5))\n(println (str \"斐波那契数列第5项: \" fib-5))\n\n;; 宏示例\n(println \"\")\n(println \"🎪 宏示例 (Macro example):\")\n(defmacro when-positive [num & body]\n  `(when (pos? ~num)\n     ~@body))\n\n(when-positive 5\n  (println \"数字是正数!\")\n  (println \"执行宏体内容\"))\n\n(println \"\")\n(println \"🎯 CodeForge Clojure 代码执行完成!\")\n(println \"🎯 CodeForge Clojure execution completed!\")\n(println \"\")\n(println \"感谢使用 CodeForge 代码执行环境! 🚀\")\n(println \"Thank you for using CodeForge! 🚀\")",
            )),
            timeout: Some(45),
        }
    }

    fn get_default_command(&self) -> String {
        self.get_config()
            .and_then(|config| config.run_command.clone())
            .unwrap_or_else(|| "clojure".to_string())
    }
}
