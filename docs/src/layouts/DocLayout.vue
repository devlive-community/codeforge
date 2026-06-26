<template>
  <div class="max-w-6xl mx-auto px-4 py-8 grid grid-cols-1 lg:grid-cols-[220px_minmax(0,1fr)] gap-8">
    <!-- 版本侧栏 -->
    <aside class="lg:sticky lg:top-24 lg:self-start">
      <p class="px-3 text-xs font-semibold uppercase tracking-wide text-gray-400 mb-2">发布日志</p>
      <nav class="space-y-0.5">
        <RouterLink v-for="r in releases" :key="r.version"
                    :to="`/release/${r.version}`"
                    class="block px-3 py-1.5 rounded-lg text-sm text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
                    active-class="bg-brand-50 dark:bg-brand-900/30 text-brand-700 dark:text-brand-300 font-medium">
          {{ r.version }}
        </RouterLink>
      </nav>
    </aside>

    <!-- 正文 -->
    <article class="prose-doc min-w-0">
      <slot/>
    </article>
  </div>
</template>

<script setup lang="ts">
import {releases} from '../content/releases'
</script>
