---
title: 我用 Tauri 2 做了一个能跑 30 多种语言的代码运行器，聊聊插件化架构是怎么扛住的
date: 2026-06-06
author: CodeForge Team
description: 我想要的东西其实很简单：打开就能写、写完一键就能跑、还能顺手把跑出来的 JSON 看明白。最好是个桌面应用，本地、快、不依赖网络。
tags: ['公告', '开始']
---

> 这是一篇实现向的文章。如果你只想看产品，项目在这里：<https://github.com/devlive-community/codeforge>。如果你也在用 Tauri 折腾桌面工具，下面这些架构上的取舍和坑，也许能帮你少走点弯路。

## 起因：我想要一个「代码草稿本」，但没找到顺手的

事情的起点很朴素。

我经常需要快速验证一小段代码——可能是一段 Python 的正则、一个 Go 的并发写法、一段 Rust 的所有权实验。问题是，为了跑这几行临时代码，要么得新建一个项目、配一遍环境，要么开浏览器找在线 playground，但在线的那些通常只支持一两种语言，还动不动连不上。

我想要的东西其实很简单：**打开就能写、写完一键就能跑、还能顺手把跑出来的 JSON 看明白**。最好是个桌面应用，本地、快、不依赖网络。

找了一圈没有完全合手的，于是我自己做了一个，叫 CodeForge。做到现在它支持 30 多种语言的运行，从 Python、Node、Go、Rust 到 Swift、Haskell，甚至 AppleScript。这篇文章我想重点聊的不是功能清单，而是支撑这一切的那个东西——**插件化的语言系统**，以及它在工程上是怎么落地的。

## 第一个决定：为什么是 Tauri 2，而不是 Electron

做桌面应用，绕不开的第一个问题就是用什么壳。

Electron 当然是最省心的选择，生态成熟、文档全、遇到问题基本都能搜到答案。我一开始也确实往那个方向想过。但越想越觉得它有两个我不愿意接受的代价：**包体积**和**内存占用**。一个定位「随手打开的代码草稿本」，如果装完一百多兆、空载就吃掉几百兆内存，那它从第一秒起就背离了我做它的初衷——那种「轻、快、即开即用」的手感，本身就是这个产品的核心体验。

Tauri 2 用系统自带的 WebView 渲染前端，后端是 Rust，最终产物可以做得很小，常驻内存也低得多。对一个主打轻量的工具来说，这个差别不是「优化项」，而是「成立与否」的问题。

代价也是有的，我得诚实地说：

- WebView 在不同平台上行为不完全一致（macOS 的 WKWebView、Windows 的 WebView2、Linux 的 WebKitGTK），偶尔要写平台分支；
- Rust 的学习曲线比 Node 陡，前期开发明显更慢；
- 遇到问题时能搜到的现成答案比 Electron 少很多，不少坑得自己啃文档和 issue。

但对这个项目来说，这些代价换来的轻量和性能，我认为是值的。最终的技术栈是：前端 Vue 3 + TypeScript + CodeMirror 6，后端 Rust + Tauri 2，执行历史、AI 对话、代码片段、应用配置统一进 SQLite。

## 核心难题：30 多种语言，怎么才能不写成 30 个 if-else

支持一种语言的运行很简单：找到解释器/编译器，拼一条命令，起个子进程，把输出抓回来。

但当语言数量往上涨，这种天真的写法会迅速崩坏成一坨巨大的分支判断——「如果是 Python 就这样、如果是 Go 就那样、如果是 Rust 还要先编译再运行……」。每加一种语言都要回头改核心逻辑，每一种语言的特殊性都在污染主流程。这是典型的、会随规模膨胀而失控的设计。

所以从一开始我就把它设计成**插件化**的。但我想强调的是它的灵魂不在「插件」这个词，而在于——**它是配置驱动的**。核心引擎不认识任何具体语言，它只认识一份描述「这门语言该怎么跑」的配置；绝大多数通用逻辑（拼命令、起进程、抓输出、清理）都写在统一的地方，而每个语言插件本身薄得惊人。

### 一个语言插件，薄到什么程度

落到 Rust 里，所有语言实现同一个 trait：

```rust
pub trait LanguagePlugin: Send + Sync {
    fn get_language_name(&self) -> &'static str;   // 显示名，如 "Python 3"
    fn get_language_key(&self) -> &'static str;    // 唯一键，如 "python3"
    fn get_file_extension(&self) -> String;        // 扩展名
    fn get_version_args(&self) -> Vec<&'static str>; // 版本探测参数
    fn get_path_command(&self) -> String;          // 探测工具链是否可用
    fn get_default_config(&self) -> PluginConfig;  // 默认配置（核心）
    fn get_default_command(&self) -> String;
    // …还有一组带默认实现的钩子，下面会讲
}
```

真正承载「这门语言怎么跑」的，是它返回的那份 `PluginConfig`：

```rust
pub struct PluginConfig {
    pub enabled: bool,
    pub extension: String,
    pub language: String,
    pub run_command: Option<String>,     // 命令模板，如 "python3 $filename"
    pub before_compile: Option<String>,  // 运行前的准备命令
    pub after_compile: Option<String>,   // 运行后的清理命令
    pub template: Option<String>,        // 新建文件时的初始模板
    pub timeout: Option<u64>,            // 超时（默认 30s）
    pub execute_home: Option<String>,    // 自定义工具链路径
    // …
}
```

于是一个解释型语言的插件，几乎就是「填一张表」。这是 Python 3 插件的全部核心——没有任何执行逻辑，只是声明「我叫什么、扩展名是啥、用 `python3 $filename` 跑」：

```rust
fn get_default_config(&self) -> PluginConfig {
    PluginConfig {
        enabled: true,
        language: "python3".into(),
        extension: "py".into(),
        run_command: Some("python3 $filename".into()),  // $filename 运行时被替换
        before_compile: None,
        after_compile: None,
        template: Some("# 在这里输入 Python 3 代码".into()),
        timeout: Some(30),
        // …
    }
}
```

核心引擎拿到文件后，从配置里取出 `run_command`，把 `$filename` 替换成真实路径，剩下的——起进程、流式抓输出、算耗时——全是**与语言无关**的通用逻辑。加一门解释型语言，本质上就是再填一张这样的表。

### 编译型语言怎么塞进同一套抽象

解释型一步到位，麻烦的是 Rust、C/C++、Java 这类「先编译、再运行」的两步语言。我没有为它们单独造一套机制，而是用了两个朴素但好使的办法。

**第一招：用 shell 的 `&&` 把两步串成一条命令。** Rust 插件就是这么干的——它的运行命令实际展开成：

```text
rustc /path/to/main.rs -o /tmp/main && /tmp/main
```

编译和运行被 `&&` 连成一个整体交给 shell。编译失败，`&&` 短路，编译器的报错就成了本次运行的输出——而这恰恰是「验证代码」场景里用户最想看到的东西，根本不需要单独搞一套错误展示。跑完之后，`after_compile` 配了 `rm -f /tmp/main` 把产物清掉。Windows 上则换成 `cmd /c "rustc ... -o main.exe && main.exe"`，平台差异用 `#[cfg(target_os)]` 分支隔开。

**第二招：`before_compile` / `after_compile` 两个钩子。** Go 插件的 `before_compile` 是 `go mod init temp 2>/dev/null || true`——临时跑一段 Go 片段得先有个 module，这个钩子就替用户把它补上了，`|| true` 保证已经初始化过时不报错。运行命令本身用 `go run $filename`，一步编译加运行。

Java 我直接用了 `java $filename`：Java 11 起支持单文件源码模式，一条命令就能编译加运行，连 `javac` 那一步都省了；再挂一个 `after_compile: rm -f *.class` 兜底清理可能残留的字节码。

你看，编译型语言的全部「特殊性」，最后都收敛成了配置里的几个字符串字段，没有一行渗进核心引擎。

### 这套设计最大的红利：加语言常常不用写代码

因为「怎么跑一门语言」已经被完整地编码进了 `PluginConfig`（命令模板 + 编译前后钩子 + 扩展名 + 超时），所以新增语言很多时候根本不需要写 Rust——填一份配置就行。我把这个能力直接开放给了用户：内置插件之外，还有一类 `CustomPlugin`，启动时从用户配置里加载，只要语言键不和内置的冲突，用户就能纯靠填配置给 CodeForge 加一门新语言。配置驱动设计在这里兑现了它最大的价值。

## 运行时的几个真实工程细节

把代码跑起来，魔鬼都在细节里。几个我觉得值得一提的：

**工具链探测。** 用户机器上不一定装了对应工具链——没人会为了跑段 Haskell 恰好装了 GHC。所以每个插件都要给出探测自己的方式：`get_version_args` 提供版本参数（Python 是 `--version`、Java 是 `-version`、Go 是 `version`，各不相同），`get_path_command` 给出定位工具的命令——有些直接得很巧，比如 Python 用 `import sys; print(sys.executable)` 反查自己的解释器路径。探测不到就给一句明确的「未检测到 xxx」，而不是甩给用户一坨 `command not found`。

**在配置里设环境变量。** `before_compile` 不只能跑准备命令，还能写 `export KEY=value`（Unix）或 `set KEY=value`（Windows），插件会解析它、跨平台地设进环境，还会帮你展开 `$PATH` / `%PATH%`。这样一些需要特定环境变量才能跑的语言，不用改代码、配置里写一行就解决了。

**多标签并发与流式输出。** 每次运行请求都带一个 `task_id`，用它做事件路由：Rust 端读到一段子进程输出就带着 `task_id` 发事件，前端按 `task_id` 把输出投递到对应的标签页。于是多个标签可以同时各跑各的，输出实时流式刷新、互不串台，而不是憋到进程结束才一次性吐出来。

**「成功但没输出」也是一种结果。** 有个小细节我自己挺得意：运行成功、但 stdout/stderr 都为空时，后执行钩子会填一个 `END-NO-OUTPUT` 的哨兵值。否则用户点了运行、界面一片空白，根本分不清是「跑成功了但本来就没输出」还是「卡住了」。一个哨兵值，把这两种状态明确区分开。

**每语言可配的超时。** 默认 30 秒，但写死在配置里、可按语言调整，避免一段死循环把应用拖住。

## 顺手做的一件事：把结构化数据「看明白」

跑代码经常会吐出 JSON，而一坨压扁的 JSON 是很难读的。

所以我顺手给 JSON / XML / YAML 做了两种可视化：可折叠的**层级树**，以及把字段关系画成卡片 + 连线的**关系图**。Markdown 会实时渲染预览（内嵌 HTML 用 DOMPurify 净化，防 XSS）。还有一个我自己很喜欢的小功能——识别到文件是 GitHub Actions 的 workflow 时，自动把它渲染成 **Jobs 依赖的 DAG 图**：触发事件、各个 Job、它们之间的依赖和每个 Job 的 Steps，一目了然，比对着 YAML 数缩进强多了。

## 写在最后

做到现在，CodeForge 大概是我「最想自己天天用」的一个项目——它解决的就是我自己每天都会遇到的麻烦。配置驱动的插件化在前期看起来有点「过度设计」，但当语言数量从 5 种涨到 30 多种时，它的价值才真正显现：核心一直很干净，加语言变成了一件愉快的、甚至能交给用户自己完成的事。

如果你对实现感兴趣，或者也想要这么一个本地的多语言代码草稿本，欢迎来看看、点个 star、提 issue：

**👉 <https://github.com/devlive-community/codeforge>**

技术栈：Vue 3 + TypeScript + CodeMirror 6（前端）· Rust + Tauri 2（后端）· SQLite（存储）· 配置驱动的插件化语言系统（架构）。

对了，**你最希望它支持哪种语言、或者最想要什么功能？** 评论区告诉我，下一个版本说不定就给你安排上。
