<template>
  <div class="min-h-[60vh] flex items-center justify-center px-5">
    <div class="text-center max-w-2xl">
      <!-- 404 大号文字 -->
      <div class="text-9xl font-extrabold text-transparent bg-clip-text bg-gradient-to-r from-brand-600 to-violet-600 mb-4">
        404
      </div>
      
      <!-- 标题 -->
      <h1 class="text-3xl font-bold text-slate-900 dark:text-white mb-4">
        {{ t('notFound.title') }}
      </h1>
      
      <!-- 描述 -->
      <p class="text-lg text-slate-600 dark:text-slate-400 mb-8">
        {{ t('notFound.description') }}
      </p>
      
      <!-- 操作按钮 -->
      <div class="flex items-center justify-center gap-4">
        <RouterLink to="/" 
                    class="inline-flex items-center gap-2 px-6 py-3 rounded-xl bg-gradient-to-r from-brand-600 to-violet-600 text-white font-semibold hover:opacity-90 transition-opacity">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"/>
          </svg>
          {{ t('notFound.goHome') }}
        </RouterLink>
        
        <button @click="goBack" 
                class="inline-flex items-center gap-2 px-6 py-3 rounded-xl border-2 border-slate-200 dark:border-slate-800 text-slate-700 dark:text-slate-300 font-semibold hover:border-brand-400 dark:hover:border-brand-500 transition-colors">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"/>
          </svg>
          {{ t('notFound.goBack') }}
        </button>
      </div>
      
      <!-- 快捷链接 -->
      <div class="mt-12 pt-8 border-t border-slate-200 dark:border-slate-800">
        <p class="text-sm text-slate-500 dark:text-slate-400 mb-4">{{ t('notFound.quickLinks') }}</p>
        <div class="flex items-center justify-center gap-6 text-sm">
          <RouterLink to="/download" class="text-brand-600 dark:text-brand-400 hover:underline">{{ t('nav.download') }}</RouterLink>
          <RouterLink to="/blog" class="text-brand-600 dark:text-brand-400 hover:underline">{{ t('nav.blog') }}</RouterLink>
          <RouterLink to="/release" class="text-brand-600 dark:text-brand-400 hover:underline">{{ t('nav.releases') }}</RouterLink>
          <a href="https://github.com/devlive-community/codeforge" target="_blank" rel="noopener" class="text-brand-600 dark:text-brand-400 hover:underline">GitHub</a>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {useRouter} from 'vue-router'
import {useI18n} from 'vue-i18n'

const router = useRouter()
const {t} = useI18n()

const goBack = () => {
  if (window.history.length > 1) {
    router.back()
  } else {
    router.push('/')
  }
}
</script>
