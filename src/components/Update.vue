<template>
  <Modal v-model:show="isVisible" title="应用更新" size="3xl" :close-on-backdrop="false" :close-on-esc="false" @close="closeUpdate">
    <div class="space-y-6 select-none">
      <!-- 更新图标和状态 -->
      <div class="text-center mb-6">
        <div class="w-20 h-20 rounded-xl flex items-center justify-center mx-auto mb-4 transform transition-all duration-300 hover:scale-105"
             :class="getIconBackgroundClass()">
          <component :is="getStatusIcon()" :class="getIconClass()" class="w-12 h-12"/>
        </div>
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-2 transition-all duration-600 delay-100">
          {{ getStatusTitle() }}
        </h2>
        <p class="text-gray-600 dark:text-gray-400 mb-4 transition-all duration-600 delay-200">
          {{ getStatusDescription() }}
        </p>
      </div>

      <!-- 当前版本和新版本信息 -->
      <div class="mb-6 transition-all duration-600 delay-300">
        <div class="bg-gray-50/80 dark:bg-gray-700/80 backdrop-blur-sm rounded-lg p-4 space-y-3 border border-gray-200/50 dark:border-gray-600/50">
          <div class="flex justify-between items-center transform transition-all duration-200 hover:scale-95">
            <span class="text-sm font-medium text-gray-600 dark:text-gray-300">当前版本</span>
            <span class="text-sm text-gray-900 dark:text-white font-mono bg-gray-100 dark:bg-gray-900/50 px-2 py-1 rounded">{{ currentVersion }}</span>
          </div>
          <div v-if="updateInfo && updateInfo.version" class="flex justify-between items-center transform transition-all duration-200 hover:scale-95">
            <span class="text-sm font-medium text-gray-600 dark:text-gray-300">最新版本</span>
            <span class="text-sm text-gray-900 dark:text-white font-mono bg-green-100 dark:bg-green-900/50 px-2 py-1 rounded">{{ updateInfo.version }}</span>
          </div>
          <div v-if="updateInfo && updateInfo.date" class="flex justify-between items-center transform transition-all duration-200 hover:scale-95">
            <span class="text-sm font-medium text-gray-600 dark:text-gray-300">发布时间</span>
            <span class="text-sm text-gray-900 dark:text-white font-mono">{{ formatDate(updateInfo.date) }}</span>
          </div>
          <div v-if="updateInfo && updateInfo.size" class="flex justify-between items-center transform transition-all duration-200 hover:scale-95">
            <span class="text-sm font-medium text-gray-600 dark:text-gray-300">更新大小</span>
            <span class="text-sm text-gray-900 dark:text-white font-mono">{{ formatSize(updateInfo.size) }}</span>
          </div>
        </div>
      </div>

      <!-- 更新进度 -->
      <div v-if="isUpdating || downloadProgress > 0" class="mb-6 transition-all duration-600 delay-400">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">更新进度</h3>
        <div class="space-y-3">
          <div class="flex justify-between text-sm">
            <span class="text-gray-600 dark:text-gray-300">{{ progressText }}</span>
            <span class="text-gray-900 dark:text-white font-medium">{{ Math.round(downloadProgress) }}%</span>
          </div>
          <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
            <div class="bg-gradient-to-r from-blue-500 to-purple-500 h-2 rounded-full transition-all duration-300 ease-out"
                 :style="{ width: `${downloadProgress}%` }"></div>
          </div>
          <div v-if="downloadSpeed" class="text-xs text-gray-500 dark:text-gray-400 text-center">
            下载速度: {{ formatSpeed(downloadSpeed) }}
          </div>
        </div>
      </div>

      <!-- 更新内容 -->
      <div v-if="updateInfo && updateInfo.notes" class="mb-6 transition-all duration-600 delay-500">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">更新内容</h3>
        <div class="bg-gray-50/50 dark:bg-gray-700/30 rounded-lg p-4 border border-gray-200/30 dark:border-gray-600/30 max-h-40 overflow-y-auto">
          <div class="prose prose-sm dark:prose-invert max-w-none">
            <VueMarkdownIt :source="updateInfo.notes"/>
          </div>
        </div>
      </div>

      <!-- 错误信息 -->
      <div v-if="errorMessage" class="mb-6 transition-all duration-600 delay-300">
        <div class="bg-red-50/80 dark:bg-red-900/20 backdrop-blur-sm rounded-lg p-4 border border-red-200/50 dark:border-red-600/50">
          <div class="flex items-start space-x-3">
            <AlertCircle class="w-5 h-5 text-red-500 flex-shrink-0 mt-0.5"/>
            <div>
              <h4 class="text-sm font-medium text-red-800 dark:text-red-200 mb-1">更新失败</h4>
              <p class="text-sm text-red-700 dark:text-red-300">{{ errorMessage }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <template #footer>
      <div class="flex justify-between items-center  border-gray-200/50 dark:border-gray-600/50">
        <div class="text-xs text-gray-500 dark:text-gray-400">
          <span v-if="lastCheckTime">
            上次检查: {{ formatDate(lastCheckTime) }}
          </span>
        </div>
        <div class="flex space-x-3">
          <Button v-if="!isUpdating && !hasUpdate"
                  @click="checkForUpdates"
                  :disabled="isChecking"
                  class="px-4 py-2 text-sm font-medium text-blue-600 dark:text-blue-400 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-600/50 rounded-lg hover:bg-blue-100 dark:hover:bg-blue-900/40 transition-colors duration-200 disabled:opacity-50">
            {{ isChecking ? '检查中...' : '检查更新' }}
          </Button>

          <Button v-if="hasUpdate && !isUpdating"
                  @click="startUpdate"
                  class="px-4 py-2 text-sm font-medium text-white bg-gradient-to-r from-blue-500 to-purple-500 rounded-lg hover:from-blue-600 hover:to-purple-600 transition-all duration-200 transform hover:scale-105">
            立即更新
          </Button>

          <Button v-if="hasUpdate && !isUpdating" type="secondary" @click="skipUpdate">
            跳过此版本
          </Button>
        </div>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import { nextTick, onMounted } from 'vue'
import { AlertCircle } from 'lucide-vue-next'
import Modal from '../ui/Modal.vue'
import Button from '../ui/Button.vue'
// @ts-ignore
import VueMarkdownIt from 'vue3-markdown-it'
import { useUpdateManager } from '../composables/useUpdateManager'

const emit = defineEmits<{
  close: []
}>()

const {
  // 状态
  isVisible,
  currentVersion,
  isChecking,
  hasUpdate,
  isUpdating,
  downloadProgress,
  downloadSpeed,
  errorMessage,
  lastCheckTime,
  progressText,
  updateInfo,

  // 方法
  checkForUpdates,
  startUpdate,
  skipUpdate,
  loadAppInfo,
  setupUpdateListeners,

  // 计算属性
  getStatusIcon,
  getIconClass,
  getIconBackgroundClass,
  getStatusTitle,
  getStatusDescription,

  // 工具函数
  formatDate,
  formatSize,
  formatSpeed
} = useUpdateManager()

const closeUpdate = () => {
  if (isUpdating.value) {
    return
  }

  isVisible.value = false
  setTimeout(() => {
    emit('close')
  }, 300)
}

onMounted(async () => {
  await loadAppInfo()
  await setupUpdateListeners()

  // 延迟显示动画
  await nextTick()
  setTimeout(() => {
    isVisible.value = true
  }, 50)

  // 自动检查更新
  await checkForUpdates()
})
</script>
