<template>
  <div>
    <!-- 运行与文件 -->
    <div class="mb-6">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center">
        <Play class="w-5 h-5 mr-2"/>
        运行与文件
      </h3>

      <div class="space-y-4">
        <Label label="运行未保存文件时">
          <Select v-model="runSaveStrategy" class="w-1/3" placeholder="选择运行策略" :options="runSaveStrategyOptions" @change="saveBehaviorConfig"/>
        </Label>

        <Label label="打开文件大小上限 (MB)">
          <Number v-model="maxOpenFileSize" :min="1" :max="200" placeholder="超过该大小将以只读方式查看" @change="saveBehaviorConfig"/>
        </Label>
      </div>
    </div>

    <!-- GitHub 配置 -->
    <div class="mb-6">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center">
        <Github class="w-5 h-5 mr-2"/>
        GitHub 配置
      </h3>

      <div class="space-y-4">
        <Label label="GitHub Token (可选)">
          <Input v-model="githubToken"
                 type="password"
                 placeholder="ghp_xxxxxxxxxxxxxxxxxxxx"
                 class="w-full"
                 @input="handleGithubTokenChange"/>
        </Label>

        <div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4">
          <div class="flex items-start">
            <Info class="w-5 h-5 text-blue-600 dark:text-blue-400 mr-2 mt-0.5 flex-shrink-0"/>
            <div class="text-sm text-blue-800 dark:text-blue-300">
              <p class="font-medium mb-2">GitHub Token 说明</p>
              <ul class="space-y-1 list-disc list-inside">
                <li>用于提高 GitHub API 请求速率限制（从 60次/小时 提升到 5000次/小时）</li>
                <li>在 <a href="https://github.com/settings/tokens" target="_blank" class="underline hover:text-blue-600 dark:hover:text-blue-200">GitHub Settings</a> 创建 Personal Access Token</li>
                <li>Token 不需要任何权限（public access 即可）</li>
                <li>留空则使用未认证模式访问 GitHub API</li>
              </ul>
            </div>
          </div>
        </div>

        <!-- GitHub 操作按钮 -->
        <div class="flex gap-3 pt-0.5">
          <Button @click="saveGithubConfig" :disabled="!hasGithubChanges || isSavingGithub" :loading="isSavingGithub" type="primary">
            {{ isSavingGithub ? '保存中...' : '保存 GitHub 配置' }}
          </Button>
          <Button @click="clearGithubToken" type="secondary">
            清除 Token
          </Button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { Github, Info, Play } from 'lucide-vue-next'
import Button from '../../ui/Button.vue'
import Label from '../../ui/Label.vue'
import Input from '../../ui/Input.vue'
import Number from '../../ui/Number.vue'
import Select from '../../ui/Select.vue'
import { invoke } from '@tauri-apps/api/core'
import { useToast } from '../../plugins/toast'

const emit = defineEmits<{
  'settings-changed': [type: string, value: any]
  'error': [message: string]
}>()

const toast = useToast()

const githubToken = ref('')
const originalGithubToken = ref('')
const isSavingGithub = ref(false)

// 运行与文件行为（存于 config.editor，但属于通用行为）
const runSaveStrategy = ref('auto-save')
const maxOpenFileSize = ref(5)

const runSaveStrategyOptions = [
  {label: '自动保存后运行', value: 'auto-save'},
  {label: '每次询问', value: 'ask'},
  {label: '运行副本(不保存)', value: 'temp-copy'}
]

const loadBehaviorConfig = async () => {
  try {
    const config = await invoke<any>('get_app_config')
    if (config.editor) {
      runSaveStrategy.value = config.editor.run_save_strategy ?? 'auto-save'
      maxOpenFileSize.value = config.editor.max_open_file_size ?? 5
    }
  }
  catch (error) {
    console.error('加载运行配置失败:', error)
  }
}

// 读全量配置 → 改字段 → 写回，避免覆盖其它设置
const saveBehaviorConfig = async () => {
  try {
    const config = await invoke<any>('get_app_config')
    config.editor = {
      ...(config.editor || {}),
      run_save_strategy: runSaveStrategy.value,
      max_open_file_size: maxOpenFileSize.value
    }
    await invoke('update_app_config', {config})
  }
  catch (error) {
    console.error('保存运行配置失败:', error)
    toast.error('保存配置失败: ' + error)
  }
}

const hasGithubChanges = computed(() => {
  return githubToken.value !== originalGithubToken.value
})

const loadGithubConfig = async () => {
  try {
    const config = await invoke<any>('get_app_config')
    if (config.github) {
      githubToken.value = config.github.token ?? ''
      originalGithubToken.value = githubToken.value
    }
  }
  catch (error) {
    console.error('加载 GitHub 配置失败:', error)
  }
}

const saveGithubConfig = async () => {
  isSavingGithub.value = true
  try {
    const config = await invoke<any>('get_app_config')
    config.github = {
      token: githubToken.value
    }

    await invoke('update_app_config', { config })

    originalGithubToken.value = githubToken.value

    toast.success('GitHub 配置已保存')
    emit('settings-changed', 'github', config.github)
  }
  catch (error) {
    console.error('保存 GitHub 配置失败:', error)
    toast.error('保存 GitHub 配置失败: ' + error)
    emit('error', '保存 GitHub 配置失败')
  }
  finally {
    isSavingGithub.value = false
  }
}

const clearGithubToken = async () => {
  githubToken.value = ''
}

const handleGithubTokenChange = () => {
  console.log('GitHub Token 变化')
}

defineExpose({
  loadGithubConfig,
  saveGithubConfig
})

onMounted(async () => {
  await loadGithubConfig()
  await loadBehaviorConfig()
})
</script>
