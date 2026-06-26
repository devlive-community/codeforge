<template>
  <!-- Hero -->
  <section class="relative overflow-hidden">
    <div class="absolute inset-0 -z-10 bg-gradient-to-b from-brand-50/60 to-transparent dark:from-brand-900/20"></div>
    <div class="max-w-6xl mx-auto px-4 pt-20 pb-16 text-center">
      <img src="/logo.svg" alt="CodeForge" class="w-20 h-20 mx-auto mb-6"/>
      <h1 class="text-4xl sm:text-5xl font-extrabold tracking-tight text-gray-900 dark:text-gray-100">
        轻量级桌面代码执行器
      </h1>
      <p class="mt-5 max-w-2xl mx-auto text-lg text-gray-600 dark:text-gray-400">
        轻量级、高性能的桌面代码执行器，专为开发者、学生和编程爱好者设计。
      </p>
      <div class="mt-8 flex items-center justify-center gap-3">
        <RouterLink to="/download"
                    class="px-6 py-3 rounded-xl bg-brand-600 hover:bg-brand-700 text-white font-medium shadow-lg shadow-brand-600/20 transition-colors">
          立即下载
        </RouterLink>
        <a href="https://github.com/devlive-community/codeforge" target="_blank" rel="noopener"
           class="px-6 py-3 rounded-xl border border-gray-300 dark:border-gray-700 text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-gray-800 font-medium transition-colors">
          了解更多
        </a>
      </div>
    </div>
  </section>

  <!-- 特性 -->
  <section class="max-w-6xl mx-auto px-4 py-12">
    <div class="text-center mb-12">
      <p class="text-brand-600 dark:text-brand-400 font-semibold">核心优势</p>
      <h2 class="mt-1 text-3xl font-bold text-gray-900 dark:text-gray-100">为什么选择我们</h2>
      <p class="mt-3 text-gray-600 dark:text-gray-400">轻量级、高性能，专为开发者、学生和编程爱好者设计。</p>
    </div>
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <div v-for="f in features" :key="f.title"
           class="p-6 rounded-2xl border border-gray-200 dark:border-gray-800 hover:border-brand-300 dark:hover:border-brand-700 hover:shadow-lg transition-all">
        <div class="w-11 h-11 rounded-xl bg-brand-50 dark:bg-brand-900/40 flex items-center justify-center text-2xl mb-4">{{ f.icon }}</div>
        <h3 class="font-semibold text-lg text-gray-900 dark:text-gray-100">{{ f.title }}</h3>
        <p class="mt-2 text-gray-600 dark:text-gray-400">{{ f.desc }}</p>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
const features = [
  {icon: '⚡', title: '高性能', desc: '可扩展的语言支持系统，运行迅捷。'},
  {icon: '🖱️', title: '操作简洁', desc: '一键运行代码，所见即所得。'},
  {icon: '🧩', title: '功能丰富', desc: '内置编辑、调试、数据库与 Git 工作台。'}
]
</script>
