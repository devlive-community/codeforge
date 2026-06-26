<template>
  <button class="px-2.5 py-2 rounded-lg text-sm font-medium text-slate-500 hover:text-slate-900 dark:text-slate-400 dark:hover:text-white hover:bg-slate-100 dark:hover:bg-slate-800/60 transition-colors"
          :aria-label="locale === 'zh' ? 'Switch to English' : '切换到中文'"
          @click="toggle">
    {{ locale === 'zh' ? 'EN' : '中' }}
  </button>
</template>

<script setup lang="ts">
import {useI18n} from 'vue-i18n'
import {setLocale, type Locale} from '../i18n'

const {locale} = useI18n()

const toggle = () => {
  setLocale((locale.value === 'zh' ? 'en' : 'zh') as Locale)
}
</script>
