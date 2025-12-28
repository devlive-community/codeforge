<template>
  <div>
    <!-- CDN 镜像配置 -->
    <div class="mb-6">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center">
        <Globe class="w-5 h-5 mr-2"/>
        CDN 镜像配置
      </h3>

      <div class="space-y-4">
        <div class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded-lg">
          <div class="flex items-center space-x-3">
            <div class="flex items-center">
              <input id="cdn-enabled"
                     v-model="cdnEnabled"
                     type="checkbox"
                     class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                     @change="handleCdnEnabledChange">
              <label for="cdn-enabled" class="ml-2 text-sm font-medium text-gray-900 dark:text-gray-300 cursor-pointer">
                启用 CDN 镜像加速
              </label>
            </div>
          </div>
          <div class="flex items-center space-x-2">
            <div v-if="cdnEnabled" class="flex items-center text-green-600 dark:text-green-400 text-sm">
              <CheckCircle class="w-4 h-4 mr-1"/>
              已启用
            </div>
            <div v-else class="flex items-center text-gray-500 dark:text-gray-400 text-sm">
              <XCircle class="w-4 h-4 mr-1"/>
              未启用
            </div>
          </div>
        </div>

        <Label label="CDN 基础 URL">
          <Input v-model="cdnBaseUrl"
                 placeholder="https://cdn.global.devlive.top"
                 class="w-full"
                 :disabled="!cdnEnabled"
                 @input="handleCdnBaseUrlChange"/>
        </Label>

        <div class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-700 rounded-lg">
          <div class="flex items-center space-x-3">
            <div class="flex items-center">
              <input
                  id="fallback-enabled"
                  v-model="fallbackEnabled"
                  type="checkbox"
                  :disabled="!cdnEnabled"
                  class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600 disabled:opacity-50 disabled:cursor-not-allowed"
                  @change="handleFallbackEnabledChange">
              <label for="fallback-enabled" class="ml-2 text-sm font-medium text-gray-900 dark:text-gray-300 cursor-pointer">
                CDN 下载失败时自动回退到 GitHub 官方源
              </label>
            </div>
          </div>
          <div class="flex items-center space-x-2">
            <div v-if="fallbackEnabled" class="flex items-center text-green-600 dark:text-green-400 text-sm">
              <CheckCircle class="w-4 h-4 mr-1"/>
              已启用
            </div>
            <div v-else class="flex items-center text-orange-500 dark:text-orange-400 text-sm">
              <AlertCircle class="w-4 h-4 mr-1"/>
              未启用
            </div>
          </div>
        </div>

        <div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4">
          <div class="flex items-start">
            <Info class="w-5 h-5 text-blue-600 dark:text-blue-400 mr-2 mt-0.5 flex-shrink-0"/>
            <div class="text-sm text-blue-800 dark:text-blue-300">
              <p class="font-medium mb-2">CDN 镜像说明</p>
              <ul class="space-y-1 list-disc list-inside">
                <li>CDN 镜像用于加速环境安装包的下载</li>
<!--                <li>URL 格式：<code class="bg-blue-100 dark:bg-blue-800 px-1 py-0.5 rounded">{base_url}/{language}/{version}/{filename}</code></li>-->
<!--                <li>例如：<code class="bg-blue-100 dark:bg-blue-800 px-1 py-0.5 rounded">https://cdn.global.devlive.top/clojure/1.12.4.1582/clojure-tools-1.12.4.1582.tar.gz</code>-->
<!--                </li>-->
                <li>启用自动回退后，CDN 下载失败会自动使用 GitHub 官方源</li>
                <li>关闭自动回退后，CDN 下载失败将直接报错，不会尝试其他源</li>
              </ul>
            </div>
          </div>
        </div>

        <!-- 操作按钮 -->
        <div class="flex gap-3 pt-0.5">
          <Button @click="saveCdnConfig" :disabled="!hasChanges || isSaving" :loading="isSaving" type="primary">
            {{ isSaving ? '保存中...' : '保存配置' }}
          </Button>
          <Button @click="resetCdnConfig" type="secondary">
            重置为默认
          </Button>
          <Button @click="testCdnConnection" :loading="isTesting" :disabled="!cdnEnabled || !cdnBaseUrl || isTesting" type="secondary">
            {{ isTesting ? '测试中...' : '测试连接' }}
          </Button>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { AlertCircle, CheckCircle, Globe, Info, XCircle } from 'lucide-vue-next'
import Button from '../../ui/Button.vue'
import Label from '../../ui/Label.vue'
import { invoke } from '@tauri-apps/api/core'
import { useToast } from '../../plugins/toast'
import Input from "../../ui/Input.vue";

const emit = defineEmits<{
  'settings-changed': [type: string, value: any]
  'error': [message: string]
}>()

const toast = useToast()

// CDN 状态
const cdnEnabled = ref(false)
const cdnBaseUrl = ref('')
const fallbackEnabled = ref(false)
const originalEnabled = ref(false)
const originalBaseUrl = ref('')
const originalFallbackEnabled = ref(false)
const isSaving = ref(false)
const isTesting = ref(false)

// 计算是否有变更
const hasChanges = computed(() => {
  return cdnEnabled.value !== originalEnabled.value ||
      cdnBaseUrl.value !== originalBaseUrl.value ||
      fallbackEnabled.value !== originalFallbackEnabled.value
})

// 加载配置
const loadCdnConfig = async () => {
  try {
    const config = await invoke<any>('get_app_config')
    if (config.environment_mirror) {
      cdnEnabled.value = config.environment_mirror.enabled ?? false
      cdnBaseUrl.value = config.environment_mirror.base_url ?? ''
      fallbackEnabled.value = config.environment_mirror.fallback_enabled ?? false
      originalEnabled.value = cdnEnabled.value
      originalBaseUrl.value = cdnBaseUrl.value
      originalFallbackEnabled.value = fallbackEnabled.value
    }
  }
  catch (error) {
    console.error('加载 CDN 配置失败:', error)
    toast.error('加载 CDN 配置失败: ' + error)
  }
}

// 保存配置
const saveCdnConfig = async () => {
  isSaving.value = true
  try {
    const config = await invoke<any>('get_app_config')
    config.environment_mirror = {
      enabled: cdnEnabled.value,
      base_url: cdnBaseUrl.value,
      fallback_enabled: fallbackEnabled.value
    }

    await invoke('update_app_config', { config })

    originalEnabled.value = cdnEnabled.value
    originalBaseUrl.value = cdnBaseUrl.value
    originalFallbackEnabled.value = fallbackEnabled.value

    toast.success('CDN 配置已保存')
    emit('settings-changed', 'cdn', config.environment_mirror)
  }
  catch (error) {
    console.error('保存 CDN 配置失败:', error)
    toast.error('保存 CDN 配置失败: ' + error)
    emit('error', '保存 CDN 配置失败')
  }
  finally {
    isSaving.value = false
  }
}

// 重置配置
const resetCdnConfig = async () => {
  cdnEnabled.value = true
  cdnBaseUrl.value = 'https://cdn.global.devlive.top'
  fallbackEnabled.value = false
}

// 测试连接
const testCdnConnection = async () => {
  if (!cdnBaseUrl.value) {
    toast.error('请先输入 CDN 基础 URL')
    return
  }

  isTesting.value = true
  try {
    // 简单的连通性测试：尝试访问 CDN URL
    const testUrl = cdnBaseUrl.value.replace(/\/$/, '')
    await fetch(testUrl, {
      method: 'HEAD',
      mode: 'no-cors'
    })

    toast.success('CDN 连接测试成功')
  }
  catch (error) {
    console.error('CDN 连接测试失败:', error)
    toast.warning('无法直接测试 CDN 连接，但这可能是正常的（CORS 限制）。请尝试下载环境包以验证 CDN 是否正常工作。')
  }
  finally {
    isTesting.value = false
  }
}

// 处理启用状态变化
const handleCdnEnabledChange = () => {
  console.log('CDN 启用状态变化:', cdnEnabled.value)
}

// 处理 URL 变化
const handleCdnBaseUrlChange = () => {
  console.log('CDN URL 变化:', cdnBaseUrl.value)
}

// 处理自动回退状态变化
const handleFallbackEnabledChange = () => {
  console.log('CDN 自动回退状态变化:', fallbackEnabled.value)
}

// 生命周期
onMounted(async () => {
  await loadCdnConfig()
})

defineExpose({
  loadCdnConfig,
  saveCdnConfig
})
</script>
