<template>
  <Modal v-model:show="isVisible" title="关于 CodeForge" size="3xl" :close-on-backdrop="false" :close-on-esc="false" @close="closeAbout">
    <div class="space-y-6">
      <!-- 应用图标和标题 -->
      <div class="text-center mb-6">
        <div
            class="w-24 h-24 rounded-xl flex items-center justify-center mx-auto mb-4 transform transition-all duration-300 hover:scale-105 hover:rotate-3">
          <img src="/codeforge.svg" alt="CodeForge">
        </div>
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-2 transition-all duration-600 delay-100">CodeForge</h2>
        <p class="text-gray-600 dark:text-gray-400 mb-4 transition-all duration-600 delay-200">CodeForge 是一款轻量级、高性能的桌面代码执行器，专为开发者、学生和编程爱好者设计。</p>

        <!-- GitHub Actions -->
        <div class="flex justify-center gap-3 mt-4">
          <Button @click="starRepository" type="warning" :icon="Star" class="transition-all duration-200 transform hover:scale-105 hover:shadow-lg">
            给个 Star
          </Button>
          <Button @click="openGitHubRepo" type="secondary" :icon="GithubIcon" class="transition-all duration-200 transform hover:scale-105 hover:shadow-lg">
            查看源码
          </Button>
          <Button @click="reportIssue" type="danger" :icon="Bug" class="transition-all duration-200 transform hover:scale-105 hover:shadow-lg">
            报告问题
          </Button>
        </div>
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
        <hr class="border-gray-200/50 dark:border-gray-600/50 my-3"/>
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
    </div>

    <template #footer>
      <!-- 底部信息 -->
      <div class="border-gray-200/50 dark:border-gray-600/50 pt-4 transition-all duration-600 delay-600">
        <p class="text-xs text-center text-gray-500 dark:text-gray-400">
          © 2025 Devlive Community. 专为开发者打造的代码执行工具
        </p>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import Modal from '../ui/Modal.vue'
import { useAboutModal } from '../composables/useAboutModal'
import Button from '../ui/Button.vue'
import { Bug, GithubIcon, Star } from 'lucide-vue-next'

const emit = defineEmits<{
  close: []
}>()

const {
  isVisible,
  platform,
  version,
  buildTime,
  features,
  techStack,
  openTechUrl,
  openGitHubRepo,
  starRepository,
  reportIssue,
  closeAbout,
  showModal
} = useAboutModal(emit)

onMounted(async () => {
  await showModal()
})
</script>
