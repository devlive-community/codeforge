<template>
  <div>
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
import { Github, Info } from 'lucide-vue-next'
import Button from '../../ui/Button.vue'
import Label from '../../ui/Label.vue'
import Input from '../../ui/Input.vue'
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
})
</script>
