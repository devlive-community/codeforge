<template>
  <h1>发布日志</h1>
  <p>每个版本的更新内容如下，点击查看详情。</p>
  <div class="not-prose mt-6 space-y-2">
    <RouterLink v-for="r in releases" :key="r.version" :to="`/release/${r.version}`"
                class="flex items-center justify-between px-4 py-3 rounded-xl border border-slate-200 dark:border-slate-800 hover:border-brand-300 dark:hover:border-brand-600 hover:shadow-soft transition-all">
      <span class="font-medium text-slate-900 dark:text-white">v{{ r.version }}</span>
      <span class="text-slate-400">→</span>
    </RouterLink>
  </div>
</template>

<script setup lang="ts">
import {releases} from '../content/releases'
</script>
