<template>
  <!-- Hero -->
  <section class="relative overflow-hidden">
    <!-- 背景装饰 -->
    <div class="pointer-events-none absolute inset-0 -z-10">
      <div class="absolute left-1/2 top-[-12rem] h-[34rem] w-[34rem] -translate-x-1/2 rounded-full bg-brand-400/15 blur-3xl"></div>
      <div class="absolute inset-0 bg-[linear-gradient(to_right,rgba(148,163,184,.07)_1px,transparent_1px),linear-gradient(to_bottom,rgba(148,163,184,.07)_1px,transparent_1px)] bg-[size:40px_40px] [mask-image:radial-gradient(ellipse_at_center,black,transparent_75%)]"></div>
    </div>

    <div class="max-w-6xl mx-auto px-5 pt-20 pb-16 grid lg:grid-cols-2 gap-12 items-center">
      <div>
        <RouterLink v-if="latestRelease" :to="`/release/${latestRelease.version}`"
                    class="inline-flex items-center gap-2 rounded-full border border-slate-200 dark:border-slate-700 bg-white/60 dark:bg-slate-900/60 px-3 py-1 text-sm text-slate-600 dark:text-slate-300 hover:border-brand-300 dark:hover:border-brand-600 transition-colors">
          <span class="w-1.5 h-1.5 rounded-full bg-brand-500"></span>
          v{{ latestRelease.version }} 已发布
          <span class="text-slate-400">→</span>
        </RouterLink>

        <h1 class="mt-6 text-[2.75rem] leading-[1.1] sm:text-6xl font-extrabold tracking-tight text-slate-900 dark:text-white">
          轻量级桌面<br/>代码执行器
        </h1>
        <p class="mt-6 max-w-md text-lg text-slate-600 dark:text-slate-400 leading-relaxed">
          高性能、可扩展的桌面代码执行器，集编辑、运行、调试、数据库与 Git 工作台于一身，专为开发者、学生和编程爱好者打造。
        </p>

        <div class="mt-8 flex flex-wrap items-center gap-3">
          <RouterLink to="/download"
                      class="px-6 py-3 rounded-xl bg-slate-900 dark:bg-white text-white dark:text-slate-900 font-medium shadow-soft hover:opacity-90 transition-opacity">
            立即下载
          </RouterLink>
          <a href="https://github.com/devlive-community/codeforge" target="_blank" rel="noopener"
             class="px-6 py-3 rounded-xl border border-slate-300 dark:border-slate-700 text-slate-700 dark:text-slate-200 font-medium hover:bg-slate-50 dark:hover:bg-slate-800/60 transition-colors">
            在 GitHub 查看
          </a>
        </div>

        <div class="mt-10 flex flex-wrap items-center gap-x-5 gap-y-2 text-sm text-slate-400">
          <span class="text-slate-400/80">支持</span>
          <span v-for="lang in langs" :key="lang" class="text-slate-500 dark:text-slate-400">{{ lang }}</span>
          <span class="text-slate-400/80">等数十种语言</span>
        </div>
      </div>

      <div class="lg:pl-6">
        <CodeWindow/>
      </div>
    </div>
  </section>

  <!-- 特性 -->
  <section class="max-w-6xl mx-auto px-5 py-16">
    <div class="max-w-2xl mb-12">
      <p class="text-sm font-semibold text-brand-600 dark:text-brand-400">核心优势</p>
      <h2 class="mt-2 text-3xl font-bold tracking-tight text-slate-900 dark:text-white">一个应用，覆盖完整开发流</h2>
    </div>
    <div class="grid sm:grid-cols-2 lg:grid-cols-3 gap-5">
      <div v-for="f in features" :key="f.title"
           class="group p-6 rounded-2xl border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900/40 hover:shadow-soft hover:border-slate-300 dark:hover:border-slate-700 transition-all">
        <div class="w-10 h-10 rounded-xl bg-brand-50 dark:bg-brand-900/30 text-brand-600 dark:text-brand-400 flex items-center justify-center" v-html="f.icon"></div>
        <h3 class="mt-4 font-semibold text-lg text-slate-900 dark:text-white">{{ f.title }}</h3>
        <p class="mt-2 text-slate-600 dark:text-slate-400 leading-relaxed">{{ f.desc }}</p>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import CodeWindow from '../components/CodeWindow.vue'
import {latestRelease} from '../content/releases'

const langs = ['Python', 'Rust', 'Go', 'JavaScript', 'Java', 'C/C++']

const i = (path: string) =>
  `<svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">${path}</svg>`

const features = [
  {
    title: '极速运行',
    desc: '一键运行任意语言，可扩展的语言支持系统，启动与执行都很迅捷。',
    icon: i('<path d="M13 2 4.5 13.5H11l-1 8.5 8.5-11.5H12z"/>')
  },
  {
    title: '智能编辑',
    desc: 'LSP 补全、跳转、诊断，符号大纲、面包屑、多光标与代码片段一应俱全。',
    icon: i('<path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/>')
  },
  {
    title: '可视化调试',
    desc: '基于 DAP 的断点调试，变量、调用栈、监视与悬停求值，无需切到命令行。',
    icon: i('<rect x="3" y="11" width="18" height="10" rx="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/>')
  },
  {
    title: '数据库工作台',
    desc: '多数据源连接、SQL 执行与结果导出、ER 图与交互式事务，数据开发更顺手。',
    icon: i('<ellipse cx="12" cy="5" rx="9" ry="3"/><path d="M3 5v14a9 3 0 0 0 18 0V5"/><path d="M3 12a9 3 0 0 0 18 0"/>')
  },
  {
    title: 'Git 工作台',
    desc: '暂存、提交、分支、分支图、变基、cherry-pick、子模块与工作树，全部图形化。',
    icon: i('<circle cx="6" cy="6" r="3"/><circle cx="6" cy="18" r="3"/><circle cx="18" cy="8" r="3"/><path d="M6 9v6"/><path d="M18 11a6 6 0 0 1-6 6H9"/>')
  },
  {
    title: 'AI 辅助',
    desc: '代码预测、解释、生成测试与格式化，让常见任务交给 AI 一步完成。',
    icon: i('<path d="M12 3v4"/><path d="M12 17v4"/><path d="m6 6 1.5 1.5"/><path d="M16.5 16.5 18 18"/><circle cx="12" cy="12" r="4"/>')
  }
]
</script>
