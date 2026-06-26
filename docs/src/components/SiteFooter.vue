<template>
  <footer class="border-t border-slate-200 dark:border-slate-800 mt-24">
    <div class="max-w-6xl mx-auto px-5 py-12 flex flex-col sm:flex-row items-center justify-between gap-5 text-sm text-slate-500 dark:text-slate-400">
      <div class="flex items-center gap-2.5">
        <img src="/logo.svg" alt="CodeForge" class="w-6 h-6"/>
        <span class="font-medium text-slate-700 dark:text-slate-300">CodeForge</span>
        <span class="text-slate-400 dark:text-slate-500">© {{ year }}</span>
      </div>
      <div class="flex items-center gap-6">
        <a class="hover:text-brand-600 dark:hover:text-brand-400 transition-colors" href="https://github.com/devlive-community/codeforge" target="_blank" rel="noopener">GitHub</a>
        <RouterLink class="hover:text-brand-600 dark:hover:text-brand-400 transition-colors" to="/download">下载</RouterLink>
        <RouterLink class="hover:text-brand-600 dark:hover:text-brand-400 transition-colors" to="/release">发布日志</RouterLink>
      </div>
    </div>
  </footer>
</template>

<script setup lang="ts">
const year = new Date().getFullYear()
</script>
