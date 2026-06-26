<template>
  <button class="p-2 rounded-lg text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-100 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
          :aria-label="isDark ? '切换到浅色' : '切换到深色'"
          @click="toggle">
    <svg v-if="isDark" class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <circle cx="12" cy="12" r="5"/>
      <path d="M12 1v2M12 21v2M4.2 4.2l1.4 1.4M18.4 18.4l1.4 1.4M1 12h2M21 12h2M4.2 19.8l1.4-1.4M18.4 5.6l1.4-1.4"/>
    </svg>
    <svg v-else class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
    </svg>
  </button>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue'

const isDark = ref(false)

const apply = (dark: boolean) => {
  isDark.value = dark
  document.documentElement.classList.toggle('dark', dark)
}

onMounted(() => {
  const saved = localStorage.getItem('theme')
  const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
  apply(saved ? saved === 'dark' : prefersDark)
})

const toggle = () => {
  apply(!isDark.value)
  localStorage.setItem('theme', isDark.value ? 'dark' : 'light')
}
</script>
