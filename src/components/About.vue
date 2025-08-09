<template>
  <div class="fixed inset-0 backdrop-blur-sm bg-white/20 dark:bg-gray-900/20 flex items-center justify-center z-50 transition-all duration-300 ease-out"
       :class="{ 'opacity-0': !isVisible, 'opacity-100': isVisible }"
       @click.self="$event.preventDefault()">
    <div
        class="bg-white/95 dark:bg-gray-800/95 backdrop-blur-md rounded-xl shadow-2xl w-4xl p-6 transform transition-all duration-300 ease-out border border-white/20 dark:border-gray-700/30"
        :class="{
          'scale-95 opacity-0 translate-y-4': !isVisible,
          'scale-100 opacity-100 translate-y-0': isVisible
        }">
      <!-- 关闭按钮 -->
      <div class="flex justify-end mb-4">
        <button
            class="text-gray-400 hover:cursor-pointer hover:text-gray-600 dark:hover:text-gray-200 transition-all duration-200 hover:scale-110 rounded-full p-1 hover:bg-gray-100 dark:hover:bg-gray-700"
            @click="closeAbout">
          <X class="w-5 h-5"></X>
        </button>
      </div>

      <!-- 应用图标和标题 -->
      <div class="text-center mb-6">
        <div
            class="w-24 h-24 rounded-xl flex items-center justify-center mx-auto mb-4 transform transition-all duration-300 hover:scale-105 hover:rotate-3">
          <img src="/codeforge.svg" alt="CodeForge">
        </div>
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-2 transition-all duration-600 delay-100">CodeForge</h2>
        <p class="text-gray-600 dark:text-gray-400 mb-4 transition-all duration-600 delay-200">CodeForge 是一款轻量级、高性能的桌面代码执行器，专为开发者、学生和编程爱好者设计。</p>
      </div>

      <!-- 版本信息 -->
      <div class="mb-6 transition-all duration-600 delay-300">
        <div class="bg-gray-50/80 dark:bg-gray-700/80 backdrop-blur-sm rounded-lg p-4 space-y-3 border border-gray-200/50 dark:border-gray-600/50">
          <div class="flex justify-between items-center transform transition-all duration-200 hover:scale-95">
            <span class="text-sm font-medium text-gray-600 dark:text-gray-300">版本</span>
            <span class="text-sm text-gray-900 dark:text-white font-mono bg-blue-100 dark:bg-blue-900/50 px-2 py-1 rounded">{{ version }}</span>
          </div>
          <div class="flex justify-between items-center transform transition-all duration-200 hover:scale-95">
            <span class="text-sm font-medium text-gray-600 dark:text-gray-300">构建时间</span>
            <span class="text-sm text-gray-900 dark:text-white font-mono">{{ buildTime }}</span>
          </div>
          <div class="flex justify-between items-center transform transition-all duration-200 hover:scale-95">
            <span class="text-sm font-medium text-gray-600 dark:text-gray-300">平台</span>
            <span class="text-sm text-gray-900 dark:text-white font-mono">{{ platform }}</span>
          </div>
        </div>
      </div>

      <!-- 核心功能 -->
      <div class="mb-6 transition-all duration-600 delay-400">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">核心功能</h3>
        <div class="grid grid-cols-2 gap-3">
          <div v-for="(feature, index) in features"
               class="flex items-center space-x-3 p-3 bg-gray-50/50 dark:bg-gray-700/30 rounded-lg border border-gray-200/30 dark:border-gray-600/30 text-sm text-gray-700 dark:text-gray-300 transform transition-all duration-200 hover:scale-105 hover:bg-blue-50 dark:hover:bg-blue-900/20 hover:border-blue-200 dark:hover:border-blue-600/50 hover:shadow-md"
               :key="index">
            <div class="w-2 h-2 bg-gradient-to-r from-blue-500 to-purple-500 rounded-full flex-shrink-0"></div>
            <span class="font-medium">{{ feature }}</span>
          </div>
        </div>
      </div>

      <!-- 技术栈 -->
      <div class="mb-6 transition-all duration-600 delay-500">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white">技术栈</h3>
        <hr class="border-gray-200/50 dark:border-gray-600/50 my-3" />
        <div class="flex flex-wrap gap-2">
          <span v-for="(tech, index) in techStack"
                class="px-3 py-1 text-xs rounded-full font-medium transform transition-all duration-200 hover:scale-110 hover:-translate-y-1 cursor-pointer"
                :key="index"
                :class="tech.class"
                @click="openTechUrl(tech.url)">
            {{ tech.name }}
          </span>
        </div>
      </div>

      <!-- 底部信息 -->
      <div class="border-t border-gray-200/50 dark:border-gray-600/50 pt-4 transition-all duration-600 delay-600">
        <p class="text-xs text-center text-gray-500 dark:text-gray-400">
          © 2025 Devlive Community. 专为开发者打造的代码执行工具
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { nextTick, onMounted, ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-shell'
import { X } from 'lucide-vue-next'

interface AppInfo
{
  version: string
  build_time: string
  platform: string
  arch: string
}

const platform = ref('Unknown')
const version = ref('1.0.0')
const buildTime = ref('2025-08-09')
const isVisible = ref(false)

const features = [
  '多语言代码执行支持',
  '智能语法高亮系统',
  '实时执行统计分析',
  '现代化用户界面'
]

const techStack = [
  { name: 'Vue 3', class: 'bg-emerald-100 dark:bg-emerald-900/50 text-emerald-800 dark:text-emerald-200', url: 'https://vuejs.org' },
  { name: 'TypeScript', class: 'bg-blue-100 dark:bg-blue-900/50 text-blue-800 dark:text-blue-200', url: 'https://www.typescriptlang.org' },
  { name: 'Tauri', class: 'bg-purple-100 dark:bg-purple-900/50 text-purple-800 dark:text-purple-200', url: 'https://tauri.app' },
  { name: 'Rust', class: 'bg-orange-100 dark:bg-orange-900/50 text-orange-800 dark:text-orange-200', url: 'https://www.rust-lang.org' },
  { name: 'Tailwind CSS', class: 'bg-cyan-100 dark:bg-cyan-900/50 text-cyan-800 dark:text-cyan-200', url: 'https://tailwindcss.com' }
]

const emit = defineEmits<{
  close: []
}>()

const closeAbout = () => {
  isVisible.value = false
  setTimeout(() => {
    emit('close')
  }, 300)
}

const openTechUrl = async (url: string) => {
  try {
    console.log('Opening URL:', url)
    await open(url)
  }
  catch (error) {
    console.error('Failed to open URL:', error)
  }
}

const loadAppInfo = async () => {
  try {
    const appInfo: AppInfo = await invoke('get_app_info')
    version.value = appInfo.version
    buildTime.value = appInfo.build_time
    platform.value = `${ appInfo.platform } (${ appInfo.arch })`
  }
  catch (error) {
    console.error('Failed to get app info:', error)
    // 使用默认值
    platform.value = navigator.platform
  }
}

// 监听组件显示状态，当组件重新显示时重置动画
watch(() => true, () => {
  if (!isVisible.value) {
    nextTick(() => {
      setTimeout(() => {
        isVisible.value = true
      }, 50)
    })
  }
})

onMounted(async () => {
  await loadAppInfo()

  // 延迟显示动画
  await nextTick()
  setTimeout(() => {
    isVisible.value = true
  }, 50)
})
</script>
