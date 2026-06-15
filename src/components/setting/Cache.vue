<template>
  <div class="space-y-6">
    <div class="space-y-4">
      <h3 class="text-lg font-medium text-gray-900 dark:text-gray-100">{{ t('settings.cache.title') }}</h3>
      <p class="text-sm text-gray-600 dark:text-gray-400">
        {{ t('settings.cache.desc') }}
      </p>
    </div>

    <!-- 缓存统计 -->
    <div v-if="cacheInfo" class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div class="p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-600 dark:text-gray-400">{{ t('settings.cache.pluginsCache') }}</p>
            <p class="text-2xl font-semibold text-gray-900 dark:text-gray-100 mt-1">
              {{ formatSize(cacheInfo.plugins_cache_size) }}
            </p>
          </div>
          <FolderOpen class="w-8 h-8 text-blue-600 dark:text-blue-400"/>
        </div>
      </div>

      <div class="p-4 bg-green-50 dark:bg-green-900/20 rounded-lg">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-600 dark:text-gray-400">{{ t('settings.cache.totalCache') }}</p>
            <p class="text-2xl font-semibold text-gray-900 dark:text-gray-100 mt-1">
              {{ formatSize(cacheInfo.total_cache_size) }}
            </p>
          </div>
          <HardDrive class="w-8 h-8 text-green-600 dark:text-green-400"/>
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
        <span>{{ t('settings.cache.loading') }}</span>
      </div>
    </div>

    <!-- 错误信息 -->
    <div v-if="error" class="p-3 bg-red-50 dark:bg-red-900/20 rounded-lg">
      <div class="flex items-center space-x-2 text-red-600 dark:text-red-400">
        <AlertCircle class="w-5 h-5"/>
        <span class="text-sm">{{ error }}</span>
      </div>
    </div>

    <!-- 成功信息 -->
    <div v-if="success" class="p-3 bg-green-50 dark:bg-green-900/20 rounded-lg">
      <div class="flex items-center space-x-2 text-green-600 dark:text-green-400">
        <CheckCircle class="w-5 h-5"/>
        <span class="text-sm">{{ success }}</span>
      </div>
    </div>

    <!-- 缓存清理选项 -->
    <div class="space-y-4">
      <div class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-800 rounded-lg">
        <div class="flex-1">
          <h4 class="font-medium text-gray-900 dark:text-gray-100">{{ t('settings.cache.clearPluginsTitle') }}</h4>
          <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">
            {{ t('settings.cache.clearPluginsDesc') }}
          </p>
        </div>
        <Button
          type="danger"
          :icon="Trash2"
          :disabled="isClearing"
          @click="clearPluginsCache">
          {{ t('settings.cache.clear') }}
        </Button>
      </div>

      <div class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-800 rounded-lg">
        <div class="flex-1">
          <h4 class="font-medium text-gray-900 dark:text-gray-100">{{ t('settings.cache.clearAllTitle') }}</h4>
          <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">
            {{ t('settings.cache.clearAllDesc') }}
          </p>
        </div>
        <Button
          type="danger"
          :icon="Trash2"
          :disabled="isClearing"
          @click="clearAllCache">
          {{ t('settings.cache.clear') }}
        </Button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { invoke } from '@tauri-apps/api/core'
import { AlertCircle, CheckCircle, FolderOpen, HardDrive, Trash2 } from 'lucide-vue-next'
import Button from '../../ui/Button.vue'

const {t} = useI18n()

interface CacheInfo {
  plugins_cache_size: number
  total_cache_size: number
}

const cacheInfo = ref<CacheInfo | null>(null)
const isLoading = ref(false)
const isClearing = ref(false)
const error = ref<string | null>(null)
const success = ref<string | null>(null)

// 格式化文件大小
const formatSize = (bytes: number) => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

// 获取缓存信息
const fetchCacheInfo = async () => {
  isLoading.value = true
  error.value = null

  try {
    const info = await invoke<CacheInfo>('get_cache_info')
    cacheInfo.value = info
  }
  catch (e) {
    error.value = e as string
    console.error('获取缓存信息失败:', e)
  }
  finally {
    isLoading.value = false
  }
}

// 清理插件缓存
const clearPluginsCache = async () => {
  isClearing.value = true
  error.value = null
  success.value = null

  try {
    await invoke('clear_plugins_cache')
    success.value = t('settings.cache.clearedPlugins')
    await fetchCacheInfo()

    // 3秒后清除成功信息
    setTimeout(() => {
      success.value = null
    }, 3000)
  }
  catch (e) {
    error.value = e as string
    console.error('清理插件缓存失败:', e)
  }
  finally {
    isClearing.value = false
  }
}

// 清理所有缓存
const clearAllCache = async () => {
  isClearing.value = true
  error.value = null
  success.value = null

  try {
    await invoke('clear_all_cache')
    success.value = t('settings.cache.clearedAll')
    await fetchCacheInfo()

    // 3秒后清除成功信息
    setTimeout(() => {
      success.value = null
    }, 3000)
  }
  catch (e) {
    error.value = e as string
    console.error('清理所有缓存失败:', e)
  }
  finally {
    isClearing.value = false
  }
}

onMounted(async () => {
  await fetchCacheInfo()
})
</script>
