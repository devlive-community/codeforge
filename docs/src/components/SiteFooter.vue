<template>
  <footer class="border-t border-gray-200 dark:border-gray-800 mt-20">
    <div class="max-w-6xl mx-auto px-4 py-10 flex flex-col sm:flex-row items-center justify-between gap-4 text-sm text-gray-500 dark:text-gray-400">
      <div class="flex items-center gap-2">
        <img src="/logo.svg" alt="CodeForge" class="w-6 h-6"/>
        <span>© {{ year }} CodeForge · All Rights Reserved.</span>
      </div>
      <div class="flex items-center gap-4">
        <a class="hover:text-brand-600 dark:hover:text-brand-400" href="https://github.com/devlive-community/codeforge" target="_blank" rel="noopener">GitHub</a>
        <RouterLink class="hover:text-brand-600 dark:hover:text-brand-400" to="/download">下载</RouterLink>
        <RouterLink class="hover:text-brand-600 dark:hover:text-brand-400" to="/release">发布日志</RouterLink>
      </div>
    </div>
  </footer>
</template>

<script setup lang="ts">
const year = new Date().getFullYear()
</script>
