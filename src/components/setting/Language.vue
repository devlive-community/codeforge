<template>
  <div class="-mt-2">
    <Tabs v-model="activePlugin" type="card" size="md" position="left" :tab-button-class="['w-36']" :tabs="tabsPluginData" @change="handleTabChange">
      <template #[activePlugin]="{ tab }">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center">
          <LanguagesIcon class="w-5 h-5 mr-2"/>
          {{ `语言 [ ${ tab.label } ] 配置` }}
        </h3>

        <Tabs v-model="activeTab" type="card" size="md" :tabs="tabsData">
          <template #general>
            <div class="space-y-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  编译前执行的命令
                </label>
                <div class="flex gap-2">
                  <input v-model="pluginConfig.before_compile"
                         type="text"
                         placeholder="编译前执行的命令"
                         class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-transparent text-sm"/>
                </div>
              </div>

              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  执行的命令
                </label>
                <div class="flex flex-col space-y-1.5">
                  <input v-model="pluginConfig.run_command"
                         type="text"
                         placeholder="执行的命令"
                         class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-transparent text-sm"/>

                  <div class="flex text-sm font-medium text-gray-700 dark:text-gray-300 ml-1 space-x-4">
                    <div class="font-bold">$filename</div>
                    <div>执行的源文件或临时生成的文件</div>
                  </div>
                </div>
              </div>

              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  编译完成后执行的命令
                </label>
                <div class="flex gap-2">
                  <input v-model="pluginConfig.after_compile"
                         type="text"
                         placeholder="编译完成后执行的命令"
                         class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-transparent text-sm"/>
                </div>
              </div>

              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  文件后缀名
                </label>
                <div class="flex gap-2">
                  <input v-model="pluginConfig.extension"
                         type="text"
                         placeholder="输入文件后缀名"
                         class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-transparent text-sm"/>
                </div>
              </div>
            </div>
          </template>

          <template #environment>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                语言环境目录
              </label>
              <div class="flex gap-2">
                <input v-model="pluginConfig.execute_home"
                       type="text"
                       placeholder="选择语言环境目录路径"
                       class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-transparent text-sm"/>

                <Button type="primary"
                        :icon-only="true"
                        :icon="Folder"
                        @click="selectExecuteHome">
                </Button>
              </div>
            </div>
          </template>

          <template #template>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                文件模板
              </label>
              <div class="flex">
                <textarea v-model="pluginConfig.template"
                          placeholder="文件模板"
                          rows="20"
                          class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-transparent text-sm"/>
              </div>
            </div>
          </template>

          <template #advanced>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                超时时间(秒)
              </label>
              <div class="flex gap-2">
                <input v-model="pluginConfig.timeout"
                       type="number"
                       placeholder="超时时间(秒)，默认 30 秒"
                       class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-transparent text-sm"/>
              </div>
            </div>
          </template>
        </Tabs>
      </template>
    </Tabs>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { debounce } from 'lodash-es'
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import { ContainerIcon, FileIcon, Folder, LanguagesIcon, PickaxeIcon, Settings2 } from 'lucide-vue-next'
import Button from '../../ui/Button.vue'
import { useToast } from '../../plugins/toast'
import Tabs from '../../ui/Tabs.vue'
import type PluginConfig from '../../types/plugin'

const emit = defineEmits<{
  'settings-changed': [config: PluginConfig]
  'error': [message: string]
}>()

const toast = useToast()

const activePlugin = ref('')
const tabsPluginData = ref([] as any[])

const activeTab = ref('general')
const tabsData = [
  {
    key: 'general',
    label: '通用配置',
    icon: Settings2
  },
  {
    'key': 'environment',
    'label': '环境配置',
    'icon': ContainerIcon
  },
  {
    key: 'template',
    label: '模板配置',
    icon: FileIcon
  },
  {
    key: 'advanced',
    label: '高级配置',
    icon: PickaxeIcon
  }
]

const globalConfig = ref(null as any)
const pluginConfig = ref<PluginConfig>({
  enabled: false,
  execute_home: '',
  extension: '',
  language: '',
  before_compile: '',
  after_compile: '',
  run_command: '',
  template: '',
  timeout: 30
})

const getSupportedLanguages = async () => {
  try {
    const languages = await invoke<any[]>('get_supported_languages')
    tabsPluginData.value = languages.map((language) => ({
      key: language.value,
      label: language.name,
      svgUrl: `/icons/${ language.value.replace(/\d+$/, '') }.svg`
    }))

    if (tabsPluginData.value.length > 0 && !activePlugin.value) {
      activePlugin.value = tabsPluginData.value[0].key
    }
  }
  catch (error) {
    toast.error('获取支持的语言失败 - 错误信息: ' + error)
    tabsPluginData.value = []
  }
}

const getConfigure = async () => {
  try {
    globalConfig.value = await invoke<any>('get_app_config')

    handleTabChange()
  }
  catch (error) {
    toast.error('获取配置失败 - 错误信息: ' + error)
  }
}

const handleTabChange = () => {
  if (globalConfig.value && globalConfig.value.plugins) {
    pluginConfig.value = globalConfig.value.plugins.find((plugin: any) => plugin.language === activePlugin.value)
  }
}

const selectExecuteHome = async () => {
  try {
    const selected = await openDialog({
      directory: true,
      multiple: false,
      title: '选择语言环境目录'
    })

    if (selected) {
      pluginConfig.value.execute_home = selected as string
    }
  }
  catch (error) {
    console.error('Failed to select directory:', error)
    emit('error', '选择目录失败')
  }
}

const updateGlobalConfig = async (updatedPlugin: PluginConfig) => {
  if (!globalConfig.value || !globalConfig.value.plugins) {
    return
  }

  const pluginIndex = globalConfig.value.plugins.findIndex(
      (plugin: any) => plugin.language === updatedPlugin.language
  )

  if (pluginIndex !== -1) {
    globalConfig.value.plugins[pluginIndex] = { ...updatedPlugin }
  }

  try {
    await invoke('update_app_config', { config: globalConfig.value })
    toast.success(`${ tabsPluginData.value.find((tab: any) => tab.key === updatedPlugin.language).label } 配置已保存`)
    emit('settings-changed', updatedPlugin)
  }
  catch (error) {
    toast.error('保存配置失败 - 错误信息: ' + error)
    emit('error', '保存配置失败')
  }
}

watch(pluginConfig, (newConfig, oldConfig) => {
  if (oldConfig && newConfig.language) {
    // 防抖处理，避免频繁保存
    debounceUpdate(newConfig)
  }
}, {
  deep: true,
  flush: 'post'
})

const debounceUpdate = debounce((config: PluginConfig) => {
  updateGlobalConfig(config)
}, 1000)

onMounted(async () => {
  await getSupportedLanguages()
  await getConfigure()
})
</script>
