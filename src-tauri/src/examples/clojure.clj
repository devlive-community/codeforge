;; Clojure 示例代码 - CodeForge 代码执行环境

(println "🎉 欢迎使用 CodeForge!")
(println "Welcome to CodeForge!")
(println "")

(println "=========================================")
(println "           CodeForge Clojure          ")
(println "=========================================")
(println "")

;; 基本输出示例
(println "✅ Clojure 运行成功! (Clojure is working!)")
(println "🔥 这是 Clojure 脚本 (This is Clojure script)")
(println "")

;; 变量操作
(def name "CodeForge")
(def version "Clojure")
(def number1 10)
(def number2 20)
(def result (+ number1 number2))

(println "🔢 简单计算 (Simple calculation):")
(println (str number1 " + " number2 " = " result))
(println "")

;; 字符串操作
(println "📝 字符串操作 (String operations):")
(println (str "平台名称 (Platform): " name))
(println (str "语言版本 (Language): " version))
(println (str "完整信息 (Full info): " name " - " version))
(println "")

;; 循环示例
(println "🔄 循环输出 (Loop output):")
(doseq [i (range 1 6)]
  (println (str "第 " i " 次输出 (Output #" i "): Hello from CodeForge!")))
(println "")

;; 向量操作
(def fruits ["苹果" "香蕉" "橙子" "葡萄"])
(println "🍎 水果列表 (Fruit list):")
(doseq [[index fruit] (map-indexed vector fruits)]
  (println (str (inc index) ". " fruit)))
(println "")

;; 条件判断
(def score 85)
(println "📊 成绩评估 (Score evaluation):")
(cond
  (>= score 90) (println "优秀! (Excellent!)")
  (>= score 80) (println "良好! (Good!)")
  (>= score 60) (println "及格 (Pass)")
  :else (println "需要努力 (Need improvement)"))

;; nil 类型示例
(def optional-value 42)
(println "")
(println "🔍 nil 类型示例 (nil example):")
(if optional-value
  (println (str "可选值: " optional-value " (Optional value: " optional-value ")"))
  (println "值为空 (Value is nil)"))

;; 函数示例
(defn greet-user [username]
  (str "Hello, " username "! 👋"))

(println "")
(println "🎭 函数示例 (Function example):")
(def greeting (greet-user "CodeForge用户"))
(println greeting)

;; 高阶函数示例
(println "")
(println "🔧 高阶函数示例 (Higher-order function):")
(def numbers (range 1 11))
(def even-numbers (filter even? numbers))
(def doubled (map #(* % 2) numbers))

(println (str "原始数字 (Original): " (clojure.string/join ", " numbers)))
(println (str "偶数 (Even numbers): " (clojure.string/join ", " even-numbers)))
(println (str "翻倍 (Doubled): " (clojure.string/join ", " doubled)))

;; Lambda 表达式示例
(println "")
(println "🎯 Lambda 表达式 (Lambda expression):")
(def calculate #(* %1 %2))
(def product (calculate 6 7))
(println (str "6 × 7 = " product))

;; 数据结构示例
(println "")
(println "📦 数据结构示例 (Data structures):")

;; 哈希映射
(def person {:name "张三" :age 25 :city "北京"})
(println (str "姓名: " (:name person) ", 年龄: " (:age person) ", 城市: " (:city person)))

;; 集合操作
(def numbers-set #{1 2 3 4 5})
(def letters-set #{\a \b \c \d})
(println (str "数字集合: " numbers-set))
(println (str "字母集合: " letters-set))

;; 线程宏示例
(println "")
(println "⛓️ 线程宏示例 (Threading macro):")
(def result2 (->> (range 1 21)
                  (filter #(= 0 (mod % 3)))
                  (map #(str "数字: " %))
                  (take 3)
                  (clojure.string/join " | ")))
(println (str "3的倍数前3个: " result2))

;; 原子和状态管理
(println "")
(println "⚛️ 原子示例 (Atom example):")
(def counter (atom 0))
(swap! counter inc)
(swap! counter inc)
(println (str "计数器值: " @counter))

;; 解构示例
(println "")
(println "🔍 解构示例 (Destructuring):")
(def coords [10 20])
(let [[x y] coords]
  (println (str "坐标 x: " x ", y: " y)))

;; 递归示例
(println "")
(println "🔄 递归示例 (Recursion):")
(defn fibonacci [n]
  (if (<= n 1)
    n
    (+ (fibonacci (- n 1)) (fibonacci (- n 2)))))

(def fib-5 (fibonacci 5))
(println (str "斐波那契数列第5项: " fib-5))

;; 宏示例
(println "")
(println "🎪 宏示例 (Macro example):")
(defmacro when-positive [num & body]
  `(when (pos? ~num)
     ~@body))

(when-positive 5
  (println "数字是正数!")
  (println "执行宏体内容"))

(println "")
(println "🎯 CodeForge Clojure 代码执行完成!")
(println "🎯 CodeForge Clojure execution completed!")
(println "")
(println "感谢使用 CodeForge 代码执行环境! 🚀")
(println "Thank you for using CodeForge! 🚀")