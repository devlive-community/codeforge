<template>
  <div class="max-w-6xl mx-auto px-5 py-10 grid grid-cols-1 lg:grid-cols-[200px_minmax(0,1fr)] gap-10">
    <!-- 版本侧栏 -->
    <aside class="lg:sticky lg:top-24 lg:self-start">
      <p class="px-3 text-xs font-semibold uppercase tracking-wider text-slate-400 mb-3">发布日志</p>
      <nav class="space-y-0.5 lg:max-h-[70vh] lg:overflow-y-auto">
        <RouterLink v-for="r in releases" :key="r.version"
                    :to="`/release/${r.version}`"
                    class="block px-3 py-1.5 rounded-lg text-sm text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white hover:bg-slate-100 dark:hover:bg-slate-800/60 transition-colors"
                    active-class="!bg-brand-50 dark:!bg-brand-900/30 !text-brand-700 dark:!text-brand-300 font-medium">
          v{{ r.version }}
        </RouterLink>
      </nav>
    </aside>

    <!-- 正文 -->
    <article class="prose-doc min-w-0 max-w-3xl">
      <slot/>
    </article>
  </div>
</template>

<script setup lang="ts">
import {releases} from '../content/releases'
</script>
