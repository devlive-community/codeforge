<template>
  <div>
    <!-- 外观 -->
    <div class="mb-6">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center">
        <Palette class="w-5 h-5 mr-2"/>
        {{ t('settings.general.appearance') }}
      </h3>
      <div class="space-y-4">
        <Label :label="t('settings.uiLanguage')">
          <Select v-model="locale" class="w-1/3" :options="localeOptions" @change="onLocaleChange"/>
        </Label>
        <Label :label="t('settings.general.theme')">
          <Select v-model="appTheme" class="w-1/3" :options="themeOptions" @change="onThemeChange"/>
        </Label>
      </div>
    </div>

    <!-- 运行与文件 -->
    <div class="mb-6">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center">
        <Play class="w-5 h-5 mr-2"/>
        {{ t('settings.general.runAndFile') }}
      </h3>

      <div class="space-y-4">
        <Label :label="t('settings.general.runUnsaved')">
          <Select v-model="runSaveStrategy" class="w-1/3" :placeholder="t('settings.general.selectStrategy')" :options="runSaveStrategyOptions" @change="saveBehaviorConfig"/>
        </Label>

        <Label :label="t('settings.general.maxFileSize')">
          <Number v-model="maxOpenFileSize" :min="1" :max="200" :placeholder="t('settings.general.maxFileSizeHint')" @change="saveBehaviorConfig"/>
        </Label>
      </div>
    </div>

    <!-- GitHub 配置 -->
    <div class="mb-6">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center">
        <Github class="w-5 h-5 mr-2"/>
        {{ t('settings.general.github') }}
      </h3>

      <div class="space-y-4">
        <Label :label="t('settings.general.githubToken')">
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
              <p class="font-medium mb-2">{{ t('settings.general.githubInfoTitle') }}</p>
              <ul class="space-y-1 list-disc list-inside">
                <li>{{ t('settings.general.githubInfo1') }}</li>
                <li>{{ t('settings.general.githubInfo2Pre') }}<a href="https://github.com/settings/tokens" target="_blank" class="underline hover:text-blue-600 dark:hover:text-blue-200">{{ t('settings.general.githubInfo2Link') }}</a>{{ t('settings.general.githubInfo2Post') }}</li>
                <li>{{ t('settings.general.githubInfo3') }}</li>
                <li>{{ t('settings.general.githubInfo4') }}</li>
              </ul>
            </div>
          </div>
        </div>

        <!-- GitHub 操作按钮 -->
        <div class="flex gap-3 pt-0.5">
          <Button @click="saveGithubConfig" :disabled="!hasGithubChanges || isSavingGithub" :loading="isSavingGithub" type="primary">
            {{ isSavingGithub ? t('settings.general.savingGithub') : t('settings.general.saveGithub') }}
          </Button>
          <Button @click="clearGithubToken" type="secondary">
            {{ t('settings.general.clearToken') }}
          </Button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { Github, Info, Palette, Play } from 'lucide-vue-next'
import Button from '../../ui/Button.vue'
import Label from '../../ui/Label.vue'
import Input from '../../ui/Input.vue'
import Number from '../../ui/Number.vue'
import Select from '../../ui/Select.vue'
import { invoke } from '@tauri-apps/api/core'
import { useI18n } from 'vue-i18n'
import { useToast } from '../../plugins/toast'
import { useTheme, type AppTheme } from '../../composables/useTheme'
import { SUPPORTED_LOCALES, setLocale, getLocale } from '../../i18n'

const emit = defineEmits<{
  'settings-changed': [type: string, value: any]
  'error': [message: string]
}>()

const {t} = useI18n()
const toast = useToast()
const {setTheme} = useTheme()

// 界面语言
const locale = ref(getLocale())
const localeOptions = SUPPORTED_LOCALES
const onLocaleChange = () => setLocale(locale.value)

// 外观主题
const appTheme = ref<AppTheme>('system')
const themeOptions = computed(() => [
  {label: t('settings.theme.system'), value: 'system'},
  {label: t('settings.theme.light'), value: 'light'},
  {label: t('settings.theme.dark'), value: 'dark'}
])

const loadAppearance = async () => {
  try {
    const config = await invoke<any>('get_app_config')
    const t = config?.theme
    appTheme.value = (t === 'light' || t === 'dark') ? t : 'system'
  }
  catch {
    appTheme.value = 'system'
  }
}

const onThemeChange = async () => {
  setTheme(appTheme.value)
  try {
    const config = await invoke<any>('get_app_config')
    config.theme = appTheme.value
    await invoke('update_app_config', {config})
  }
  catch (error) {
    toast.error('保存主题失败: ' + error)
  }
}

const githubToken = ref('')
const originalGithubToken = ref('')
const isSavingGithub = ref(false)

// 运行与文件行为（存于 config.editor，但属于通用行为）
const runSaveStrategy = ref('auto-save')
const maxOpenFileSize = ref(5)

const runSaveStrategyOptions = computed(() => [
  {label: t('settings.runStrategy.autoSave'), value: 'auto-save'},
  {label: t('settings.runStrategy.ask'), value: 'ask'},
  {label: t('settings.runStrategy.tempCopy'), value: 'temp-copy'}
])

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
  await loadAppearance()
})
</script>
