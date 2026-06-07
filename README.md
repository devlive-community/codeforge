<div align="center">
<img src="public/codeforge.svg" width="100" />

<h1 style="margin-top: -10px;">CodeForge</h1>

**一款现代化的桌面端多语言代码编辑器与运行器**

集「文件/项目编辑 · 一键运行 · AI 助手 · Git 集成 · 集成终端 · 结构化数据可视化」于一身，
让你在一个轻量优雅的桌面应用里完成编写、运行、调试与协作。

<p>
  <img alt="version" src="https://img.shields.io/badge/version-26.1.0-blue" />
  <img alt="platform" src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-555" />
  <img alt="stack" src="https://img.shields.io/badge/Tauri%202-Vue%203-FFC131" />
  <img alt="license" src="https://img.shields.io/badge/license-MIT-green" />
</p>
</div>

---

## 📹 演示视频

[下载演示视频](https://devlive-cdn.oss-cn-beijing.aliyuncs.com/applications/codeforge/codeforge.mp4)（点击下载或观看）

> GitHub 不支持直接播放视频，请下载或点击链接查看。

---

## ✨ 核心功能

### 📂 编辑与项目
- **文件树侧栏 + 多标签编辑** —— 打开文件夹，像 IDE 一样浏览、编辑整个项目
- **面包屑路径导航** —— 点击任意层级在系统文件管理器中定位
- **命令面板**（`Cmd/Ctrl + Shift + P`）—— 一处入口直达所有命令
- **快速打开**（`Cmd/Ctrl + P`）—— 模糊匹配 + 最近文件优先
- **符号大纲**（`Cmd/Ctrl + Shift + O`）、**跳转到行**（`Cmd/Ctrl + G`）
- **代码片段** —— 自定义前缀，输入后按 `Tab` 展开（`$0` 为光标落点）
- **会话恢复** —— 重启自动恢复上次的文件夹与标签页
- **深色模式** —— 跟随系统 / 浅色 / 深色，编辑器主题同步切换

### ▶️ 运行与调试
- **一键运行 / 按文件就地运行**，实时流式输出、执行耗时统计
- **运行选中片段**（`Cmd/Ctrl + Shift + Enter`）
- **监听模式** —— 保存后自动重跑
- **运行输入** —— 自定义参数 / stdin / 环境变量，并按文件记忆
- **执行历史** —— 持久化保存，可一键重跑与还原

### 📊 结构化数据可视化
- **JSON / XML / YAML** —— 可折叠**层级树**，以及卡片 + 连线的**关系图**两种可视化
- **SQL** —— 插件式执行器（内存库 / `.sqlite` 文件 / **MySQL**，可在设置中配置连接、运行时选择数据源），结果渲染为**表格**，失败显示具体错误；执行历史与实时运行一致
- **图表可视化** —— SQL 结果一键切换为图表：**拖拽**字段到「维度 / 指标」即可成图，自动识别数值列，支持聚合（求和/计数/平均/最大/最小）、排序、Top N。内置 **柱状图 · 折线图 · 面积图 · 饼图/环形图 · 散点图 · 雷达图 · 漏斗图 · 热力图 · 仪表盘** 9 种（基于 ECharts，配色跟随主题）。组件与数据源解耦，后续 CSV 等本地数据可复用
- **Markdown** —— 实时渲染预览（支持内嵌 HTML，DOMPurify 净化防 XSS）
- **GitHub Actions 工作流** —— 自动识别并渲染为 **Jobs 依赖 DAG 图**（触发事件 → 各 Job → Steps）

### 🤖 AI 助手
- **多服务商** —— Claude (Anthropic) / OpenAI / DeepSeek
- **AI 代码预测** —— 编辑器内幽灵补全，`Tab` 接受
- **解释代码 / 生成测试 / 格式化代码** —— 一键发起，应用前可 diff 预览确认
- **报错分析、自然语言生成代码、生成 Git 提交信息**
- **对话与执行历史绑定** —— 每次执行的 AI 讨论可追溯

### 🔱 Git 集成
- **源代码管理面板** —— 暂存 / 提交 / 推送 / 分支切换，AI 一键生成提交信息
- **文件树状态徽标**（M / A / D / U）
- **编辑器行内差异标记** —— 相对 HEAD 的增 / 改 / 删

### 🔎 搜索与终端
- **文件夹内搜索与替换**（`Cmd/Ctrl + Shift + F`）
- **集成终端** —— 真实 shell、多标签、可拖拽改高度（`` Cmd/Ctrl + ` ``）

---

## 🧩 支持的语言

可运行语言均采用**插件化架构**，每种语言独立实现；JSON / XML / YAML / Markdown / 纯文本为编辑与可视化类型。

<div align="center">
  <img src="public/icons/python.svg" width="48" title="Python 2 / 3" />
  <img src="public/icons/nodejs.svg" width="48" title="Node.js" />
  <img src="public/icons/typescript.svg" width="48" title="TypeScript" />
  <img src="public/icons/javascript-nodejs.svg" width="48" title="JavaScript (Node.js)" />
  <img src="public/icons/javascript-browser.svg" width="48" title="JavaScript (Browser)" />
  <img src="public/icons/javascript-jquery.svg" width="48" title="JavaScript (jQuery)" />
  <img src="public/icons/go.svg" width="48" title="Go" />
  <img src="public/icons/rust.svg" width="48" title="Rust" />
  <img src="public/icons/java.svg" width="48" title="Java" />
  <img src="public/icons/kotlin.svg" width="48" title="Kotlin" />
  <img src="public/icons/scala.svg" width="48" title="Scala" />
  <img src="public/icons/groovy.svg" width="48" title="Groovy" />
  <img src="public/icons/clojure.svg" width="48" title="Clojure" />
  <img src="public/icons/c.svg" width="48" title="C" />
  <img src="public/icons/cpp.svg" width="48" title="C++" />
  <img src="public/icons/objective-c.svg" width="48" title="Objective-C" />
  <img src="public/icons/objective-cpp.svg" width="48" title="Objective-C++" />
  <img src="public/icons/swift.svg" width="48" title="Swift" />
  <img src="public/icons/ruby.svg" width="48" title="Ruby" />
  <img src="public/icons/php.svg" width="48" title="PHP" />
  <img src="public/icons/r.svg" width="48" title="R" />
  <img src="public/icons/lua.svg" width="48" title="Lua" />
  <img src="public/icons/haskell.svg" width="48" title="Haskell" />
  <img src="public/icons/cangjie.svg" width="48" title="Cangjie" />
  <img src="public/icons/shell.svg" width="48" title="Shell" />
  <img src="public/icons/applescript.svg" width="48" title="AppleScript" />
  <img src="public/icons/html.svg" width="48" title="HTML" />
  <img src="public/icons/css.svg" width="48" title="CSS" />
  <img src="public/icons/svg.svg" width="48" title="SVG" />
  <img src="public/icons/sql.svg" width="48" title="SQL" />
  <img src="public/icons/json.svg" width="48" title="JSON" />
  <img src="public/icons/xml.svg" width="48" title="XML" />
  <img src="public/icons/yaml.svg" width="48" title="YAML" />
  <img src="public/icons/markdown.svg" width="48" title="Markdown" />
  <img src="public/icons/text.svg" width="48" title="纯文本" />
</div>

<div align="center">

`Python` · `Node.js` · `TypeScript` · `JavaScript` · `Go` · `Rust` · `Java` · `Kotlin` · `Scala` · `Groovy` · `Clojure` · `C` · `C++` · `Objective-C/C++` · `Swift` · `Ruby` · `PHP` · `R` · `Lua` · `Haskell` · `Cangjie` · `Shell` · `AppleScript` · `SQL` · `HTML` · `CSS` · `SVG` · `JSON` · `XML` · `YAML` · `Markdown` · `Text`

</div>

---

## 🚀 安装与构建

**环境要求：** Node.js 22+ · Rust 1.8+ · pnpm

```bash
# 克隆项目
git clone https://github.com/devlive-community/codeforge.git
cd codeforge

# 安装依赖
pnpm install

# 开发模式
pnpm tauri dev

# 构建应用
pnpm tauri build
```

---

## 🛠 技术栈

| 层 | 技术 |
| --- | --- |
| 前端 | Vue 3 · TypeScript · Tailwind CSS · CodeMirror 6 · ECharts |
| 后端 | Rust · Tauri 2（rusqlite · mysql） |
| 存储 | SQLite（执行历史 / AI 对话 / 代码片段 / 应用配置统一入库） |
| 架构 | 插件化语言支持系统 · 插件式数据库执行器 · 可复用图表组件 |

---

## 🤝 贡献与反馈

欢迎提交 Issue 与 PR：<https://github.com/devlive-community/codeforge/issues>

## 📄 许可证

[MIT License](LICENSE)
