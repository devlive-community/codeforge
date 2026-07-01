<template>
  <div class="max-w-6xl mx-auto px-5 py-16">
    <h1 class="text-4xl font-bold tracking-tight text-slate-900 dark:text-white mb-4">
      {{ t('releaseList.title') }}
    </h1>
    <p class="text-lg text-slate-600 dark:text-slate-400 mb-10">
      {{ t('releaseList.intro') }}
    </p>
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-5">
      <RouterLink v-for="r in releases" :key="r.version" :to="`/release/${r.version}`"
                  class="group relative flex flex-col p-6 rounded-2xl border-2 border-slate-200 dark:border-slate-800 hover:border-brand-400 dark:hover:border-brand-500 bg-gradient-to-br from-white to-slate-50 dark:from-slate-900 dark:to-slate-900/50 hover:shadow-xl hover:shadow-brand-500/10 dark:hover:shadow-brand-500/20 transition-all duration-300 hover:-translate-y-1">
        <!-- 版本号和箭头 -->
        <div class="flex items-center justify-between mb-4">
          <span class="text-2xl font-bold text-slate-900 dark:text-white group-hover:text-brand-600 dark:group-hover:text-brand-400 transition-colors">
            v{{ r.version }}
          </span>
          <span class="flex items-center justify-center w-8 h-8 rounded-full bg-slate-100 dark:bg-slate-800 text-slate-400 group-hover:bg-brand-500 group-hover:text-white transition-all duration-300 group-hover:translate-x-1">
            →
          </span>
        </div>
        
        <!-- 日期 -->
        <div v-if="r.date" class="flex items-center gap-2 text-sm text-slate-500 dark:text-slate-400 mb-3">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
          </svg>
          <span>{{ r.date }}</span>
        </div>
        
        <!-- 描述 -->
        <div v-if="r.description" class="text-sm text-slate-600 dark:text-slate-300 leading-relaxed line-clamp-2 flex-1">
          {{ r.description }}
        </div>
        
        <!-- 装饰渐变 -->
        <div class="absolute inset-0 rounded-2xl bg-gradient-to-br from-brand-500/0 via-brand-500/0 to-brand-500/5 dark:to-brand-500/10 opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none"></div>
      </RouterLink>
    </div>
  </div>
</template>

<script setup lang="ts">
import {useI18n} from 'vue-i18n'
import {releases} from '../content/releases'

const {t} = useI18n()
</script>
