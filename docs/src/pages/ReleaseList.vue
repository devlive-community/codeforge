<template>
  <h1>{{ t('releaseList.title') }}</h1>
  <p>{{ t('releaseList.intro') }}</p>
  <div class="not-prose mt-6 space-y-2">
    <RouterLink v-for="r in releases" :key="r.version" :to="`/release/${r.version}`"
                class="flex items-center justify-between px-4 py-3 rounded-xl border border-slate-200 dark:border-white/10 hover:border-brand-300 dark:hover:border-brand-500/60 hover:shadow-soft transition-all">
      <span class="font-medium text-slate-900 dark:text-white">v{{ r.version }}</span>
      <span class="text-slate-400">→</span>
    </RouterLink>
  </div>
</template>

<script setup lang="ts">
import {useI18n} from 'vue-i18n'
import {releases} from '../content/releases'

const {t} = useI18n()
</script>
