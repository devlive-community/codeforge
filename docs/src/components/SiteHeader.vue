<template>
  <header class="sticky top-0 z-40 border-b border-slate-200/70 dark:border-white/10 bg-white/70 dark:bg-slate-950/70 backdrop-blur-xl">
    <nav class="max-w-6xl mx-auto px-5 h-16 flex items-center gap-8">
      <RouterLink to="/" class="flex items-center gap-2.5 font-semibold text-slate-900 dark:text-white tracking-tight">
        <img src="/logo.svg" alt="CodeForge" class="w-7 h-7"/>
        <span>CodeForge</span>
      </RouterLink>

      <div class="hidden sm:flex items-center gap-1 text-[15px]">
        <RouterLink to="/download" class="px-3 py-2 rounded-lg text-slate-600 dark:text-slate-300 hover:text-slate-900 dark:hover:text-white hover:bg-slate-100 dark:hover:bg-slate-800/60 transition-colors" active-class="!text-brand-600 dark:!text-brand-400">{{ t('nav.download') }}</RouterLink>
        <RouterLink to="/blog" 
                    :class="[
                      'px-3 py-2 rounded-lg text-slate-600 dark:text-slate-300 hover:text-slate-900 dark:hover:text-white hover:bg-slate-100 dark:hover:bg-slate-800/60 transition-colors',
                      route.path.startsWith('/blog') && '!text-brand-600 dark:!text-brand-400'
                    ]">
          {{ t('nav.blog') }}
        </RouterLink>
        <RouterLink to="/release" 
                    :class="[
                      'px-3 py-2 rounded-lg text-slate-600 dark:text-slate-300 hover:text-slate-900 dark:hover:text-white hover:bg-slate-100 dark:hover:bg-slate-800/60 transition-colors',
                      route.path.startsWith('/release') && '!text-brand-600 dark:!text-brand-400'
                    ]">
          {{ t('nav.releases') }}
        </RouterLink>
      </div>

      <div class="ml-auto flex items-center gap-1">
        <a href="https://github.com/devlive-community/codeforge" target="_blank" rel="noopener"
           class="p-2 rounded-lg text-slate-500 hover:text-slate-900 dark:text-slate-400 dark:hover:text-white hover:bg-slate-100 dark:hover:bg-slate-800/60 transition-colors"
           aria-label="GitHub">
          <svg class="w-[18px] h-[18px]" viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 2C6.48 2 2 6.58 2 12.26c0 4.5 2.87 8.32 6.84 9.67.5.1.68-.22.68-.49 0-.24-.01-.87-.01-1.71-2.78.62-3.37-1.37-3.37-1.37-.45-1.18-1.11-1.5-1.11-1.5-.91-.64.07-.62.07-.62 1 .07 1.53 1.06 1.53 1.06.89 1.56 2.34 1.11 2.91.85.09-.66.35-1.11.63-1.37-2.22-.26-4.56-1.14-4.56-5.07 0-1.12.39-2.03 1.03-2.75-.1-.26-.45-1.3.1-2.71 0 0 .84-.27 2.75 1.05A9.36 9.36 0 0 1 12 7.07c.85 0 1.71.12 2.51.34 1.91-1.32 2.75-1.05 2.75-1.05.55 1.41.2 2.45.1 2.71.64.72 1.03 1.63 1.03 2.75 0 3.94-2.34 4.81-4.57 5.06.36.32.68.94.68 1.9 0 1.37-.01 2.48-.01 2.82 0 .27.18.6.69.49A10.02 10.02 0 0 0 22 12.26C22 6.58 17.52 2 12 2z"/>
          </svg>
        </a>
        <LangToggle/>
        <ThemeToggle/>
        <RouterLink to="/download"
                    class="ml-2 hidden sm:inline-flex px-4 py-2 rounded-lg bg-gradient-to-r from-brand-600 to-violet-600 text-white text-sm font-medium shadow-glow hover:opacity-90 transition-opacity">
          {{ t('nav.download') }}
        </RouterLink>
      </div>
    </nav>
  </header>
</template>

<script setup lang="ts">
import {useI18n} from 'vue-i18n'
import {useRoute} from 'vue-router'
import ThemeToggle from './ThemeToggle.vue'
import LangToggle from './LangToggle.vue'

const {t} = useI18n()
const route = useRoute()
</script>
