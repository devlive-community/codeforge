<template>
  <div class="space-y-4">
    <!-- 当前版本和路径选择 -->
    <Label label="语言环境目录">
      <div class="flex gap-2">
        <Input :model-value="executeHome" class="w-full" placeholder="选择语言环境目录路径" :disabled="true"/>
        <Button type="primary"
                :icon-only="true"
                :icon="Folder"
                @click="onSelectDirectory">
        </Button>
      </div>
    </Label>

    <!-- 当前版本信息 -->
    <div v-if="environmentInfo && environmentInfo.current_version" class="p-3 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
      <div class="flex items-center space-x-2">
        <CheckCircle class="w-5 h-5 text-blue-600 dark:text-blue-400"/>
        <div>
          <span class="text-sm font-medium text-blue-900 dark:text-blue-100">当前版本: </span>
          <span class="text-sm font-semibold text-blue-600 dark:text-blue-400">{{ environmentInfo.current_version }}</span>
        </div>
      </div>
    </div>

    <!-- 加载状态 -->
    <div v-if="isLoading" class="flex items-center justify-center py-8">
      <div class="flex items-center space-x-2 text-sm text-gray-600 dark:text-gray-400">
        <svg class="animate-spin h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
        <span>加载中...</span>
      </div>
    </div>

    <!-- 错误信息 -->
    <div v-if="error" class="p-3 bg-red-50 dark:bg-red-900/20 rounded-lg">
      <div class="flex items-center space-x-2 text-red-600 dark:text-red-400">
        <AlertCircle class="w-5 h-5"/>
        <span class="text-sm">{{ error }}</span>
      </div>
    </div>

    <!-- 获取可用版本错误信息 -->
    <div v-if="environmentInfo?.error" class="p-3 bg-yellow-50 dark:bg-yellow-900/20 rounded-lg">
      <div class="flex items-center space-x-2 text-yellow-600 dark:text-yellow-400">
        <AlertCircle class="w-5 h-5"/>
        <span class="text-sm">{{ environmentInfo.error }}</span>
      </div>
    </div>

    <!-- 版本管理 -->
    <div v-if="environmentInfo && !isLoading">
      <Tabs v-model="activeVersionTab" type="card" size="sm" :tabs="versionTabs">
        <!-- 已安装版本 -->
        <template #installed>
          <div v-if="environmentInfo.installed_versions.length > 0" class="space-y-2">
            <div :class="environmentInfo.error ? 'max-h-76' : 'max-h-92'"
                 class="space-y-2 overflow-y-auto pr-1 scrollbar-thin scrollbar-thumb-gray-300 dark:scrollbar-thumb-gray-600 scrollbar-track-transparent">
              <div v-for="version in environmentInfo.installed_versions"
                   :key="version.version"
                   class="flex items-center justify-between p-2 bg-gray-50 dark:bg-gray-800 rounded-lg">
                <div class="flex items-center space-x-2">
                  <CheckCircle class="w-4 h-4 text-green-600 dark:text-green-400"/>
                  <span class="text-sm font-medium text-gray-900 dark:text-gray-100">{{ version.version }}</span>
                  <span v-if="environmentInfo.current_version === version.version"
                        class="px-2 py-0.5 text-xs font-medium bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 rounded">
                    当前
                  </span>
                </div>
                <Button v-if="environmentInfo.current_version !== version.version"
                        type="primary"
                        size="sm"
                        @click="handleSwitchVersion(version.version)">
                  切换
                </Button>
              </div>
            </div>
          </div>
          <div v-else class="p-4 text-center text-sm text-gray-500 dark:text-gray-400">
            暂无已安装版本
          </div>
        </template>

        <!-- 可用版本 -->
        <template #available>
          <div class="space-y-2">
            <!-- 下载进度 -->
            <div v-if="isDownloading && downloadProgress" class="p-3 bg-blue-50 dark:bg-blue-900/20 rounded-lg space-y-2">
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-2">
                  <svg class="animate-spin h-4 w-4 text-blue-600 dark:text-blue-400" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                    <path class="opacity-75" fill="currentColor"
                          d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                  </svg>
                  <span class="text-sm font-medium text-blue-900 dark:text-blue-100">
                    {{ downloadStatusText }}
                  </span>
                </div>
                <span class="text-sm font-semibold text-blue-600 dark:text-blue-400">
                  {{ downloadProgress.percentage.toFixed(1) }}%
                </span>
              </div>
              <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                <div class="bg-blue-600 dark:bg-blue-400 h-2 rounded-full transition-all duration-300"
                     :style="{ width: `${downloadProgress.percentage}%` }">
                </div>
              </div>
            </div>

            <div :class="environmentInfo.error ? 'max-h-76' : 'max-h-92'"
                 class="space-y-2 overflow-y-auto pr-1 scrollbar-thin scrollbar-thumb-gray-300 dark:scrollbar-thumb-gray-600 scrollbar-track-transparent">
              <div v-for="version in availableVersionsToShow"
                   :key="version.version"
                   class="flex items-center justify-between p-2 bg-gray-50 dark:bg-gray-800 rounded-lg">
                <div class="flex flex-col">
                  <div class="flex items-center space-x-2">
                    <span class="text-sm font-medium text-gray-900 dark:text-gray-100">{{ version.version }}</span>
                    <span v-if="version.is_installed"
                          class="px-2 py-0.5 text-xs font-medium bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 rounded">
                      已安装
                    </span>
                  </div>
                  <div class="flex items-center space-x-3 mt-1 text-xs text-gray-500 dark:text-gray-400">
                    <span v-if="version.size">{{ formatSize(version.size) }}</span>
                    <span v-if="version.release_date">{{ formatDate(version.release_date) }}</span>
                  </div>
                </div>
                <Button v-if="!version.is_installed"
                        type="primary"
                        size="sm"
                        :icon="Download"
                        :disabled="isDownloading"
                        @click="handleDownload(version.version)">
                  下载
                </Button>
              </div>
            </div>
          </div>
        </template>
      </Tabs>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { AlertCircle, CheckCircle, Download, Folder } from 'lucide-vue-next'
import { useEnvironmentManager } from '../../composables/useEnvironmentManager'
import Label from '../../ui/Label.vue'
import Input from '../../ui/Input.vue'
import Button from '../../ui/Button.vue'
import Tabs from '../../ui/Tabs.vue'

const props = defineProps<{
  language: string
  executeHome: string | null
}>()

const emit = defineEmits<{
  'update:executeHome': [value: string | null]
  'select-directory': []
}>()

const {
  environmentInfo,
  downloadProgress,
  isLoading,
  isDownloading,
  error,
  downloadAndInstall,
  switchVersion
} = useEnvironmentManager(props.language)

const activeVersionTab = ref('installed')
const versionTabs = computed(() => [
  { key: 'installed', label: '已安装版本' },
  { key: 'available', label: '可用版本', disabled: !!environmentInfo.value?.error }
])

// 下载状态文本
const downloadStatusText = computed(() => {
  if (!downloadProgress.value) {
    return ''
  }

  switch (downloadProgress.value.status) {
    case 'downloading':
      return `正在下载 ${downloadProgress.value.version}`
    case 'extracting':
      return `正在解压 ${downloadProgress.value.version}`
    case 'installing':
      return `正在安装 ${downloadProgress.value.version}`
    case 'completed':
      return `安装完成 ${downloadProgress.value.version}`
    case 'failed':
      return `安装失败 ${downloadProgress.value.version}`
    default:
      return ''
  }
})

// 显示所有可用版本（包括已安装的）
const availableVersionsToShow = computed(() => {
  if (!environmentInfo.value) {
    return []
  }
  return environmentInfo.value.available_versions
})

// 格式化文件大小
const formatSize = (bytes: number) => {
  if (bytes === 0) {
    return '0 B'
  }
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

// 格式化日期
const formatDate = (dateString: string) => {
  const date = new Date(dateString)
  return date.toLocaleDateString('zh-CN', { year: 'numeric', month: '2-digit', day: '2-digit' })
}

// 下载版本
const handleDownload = async (version: string) => {
  try {
    const installPath = await downloadAndInstall(version)
    console.log('下载安装成功:', installPath)
    emit('update:executeHome', installPath)
  }
  catch (e) {
    console.error('下载安装失败:', e)
  }
}

// 切换版本
const handleSwitchVersion = async (version: string) => {
  try {
    await switchVersion(version)
    console.log('切换版本成功:', version)

    // 更新 executeHome
    const versionInfo = environmentInfo.value?.installed_versions.find((v: { version: string }) => v.version === version)
    if (versionInfo?.install_path) {
      emit('update:executeHome', versionInfo.install_path)
    }
  }
  catch (e) {
    console.error('切换版本失败:', e)
  }
}

// 选择目录
const onSelectDirectory = () => {
  emit('select-directory')
}
</script>
