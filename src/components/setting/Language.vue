<template>
  <div class="-mt-2">
    <Tabs v-model="activePlugin"
          type="card"
          size="md"
          position="left"
          :tab-button-class="['w-36']"
          :tabs="tabsPluginData"
          @change="handleTabChange">
      <template #[activePlugin]="{ tab }">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4 flex items-center space-x-2">
          <img :src="`/icons/${activePlugin.replace(/\d+$/, '')}.svg`" class="w-6 h-6" :alt="tab.label"/>
          <span>{{ `语言 [ ${ tab.label } ] 配置` }}</span>
        </h3>

        <Tabs v-model="activeTab"
              type="card"
              size="md"
              :tabs="tabsData"
              :nav-class="['w-full', 'justify-center']">
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
                  <div class="flex text-sm font-medium text-gray-700 dark:text-gray-300 ml-1 space-x-4">
                    <div class="font-bold">$classname</div>
                    <div>执行的类名,如果没有则为空</div>
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
            <div class="w-[98%]">
              <Codemirror v-if="isEditorReady && pluginConfig.template !== undefined"
                          style="width: 102%; height: 380px"
                          v-model="pluginConfig.template"
                          :extensions="currentExtensions"
                          class="flex-1 border border-gray-300 dark:border-gray-600 rounded-md overflow-hidden"/>
              <div v-else class="flex-1 flex items-center justify-center h-64 border border-gray-300 dark:border-gray-600 rounded-md">
                <div class="text-gray-500">加载编辑器中...</div>
              </div>
            </div>
          </template>

          <template #advanced>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                超时时间(秒)
              </label>
              <div class="flex gap-2">
                <Number v-model="pluginConfig.timeout" class="w-1/5" placeholder="超时时间(秒)，默认 30 秒"/>
              </div>
            </div>
          </template>
        </Tabs>
      </template>
    </Tabs>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, onMounted, ref, watch } from 'vue'
import { ContainerIcon, FileIcon, Folder, PickaxeIcon, Settings2 } from 'lucide-vue-next'
import { Codemirror } from 'vue-codemirror'
import Button from '../../ui/Button.vue'
import Tabs from '../../ui/Tabs.vue'
import Number from '../../ui/Number.vue'
import { usePluginConfig } from '../../composables/usePluginConfig'
import type PluginConfig from '../../types/plugin'
import { useCodeMirrorEditor } from '../../composables/useCodeMirrorEditor.ts'

const emit = defineEmits<{
  'settings-changed': [config: PluginConfig]
  'error': [message: string]
}>()

const {
  activePlugin,
  activeTab,
  tabsPluginData,
  pluginConfig,
  handleTabChange,
  selectExecuteHome,
  initializePlugin
} = usePluginConfig(emit)

// 编辑器状态
const isEditorReady = ref(false)
const currentExtensions = ref<any[]>([])

// 创建 computed 来响应式地获取当前语言
const currentLanguage = computed(() => {
  return activePlugin.value || ''
})

// 创建 computed 来响应式地获取模板内容
const templateContent = computed({
  get: () => pluginConfig.value?.template || '',
  set: (value: string) => {
    if (pluginConfig.value) {
      pluginConfig.value.template = value
    }
  }
})

// 使用 useCodeMirrorEditor composable
const {
  initializeEditor,
  getLanguageExtension,
  getThemeExtension
} = useCodeMirrorEditor(
    {
      modelValue: templateContent.value,
      language: currentLanguage.value
    }
)

// 更新扩展的函数
const updateExtensions = async () => {
  const newExtensions = []

  // 添加主题扩展
  const themeExtension = getThemeExtension()
  newExtensions.push(themeExtension)

  // 添加语言扩展
  if (currentLanguage.value) {
    const langExtension = getLanguageExtension(currentLanguage.value)
    if (langExtension) {
      newExtensions.push(langExtension)
    }
  }

  currentExtensions.value = newExtensions

  if (!isEditorReady.value) {
    await nextTick()
    isEditorReady.value = true
  }
}

// 监听语言变化
watch(currentLanguage, async (newLanguage) => {
  console.log('Language changed to:', newLanguage)
  if (newLanguage) {
    await updateExtensions()
  }
}, { immediate: false })

// 监听插件配置变化
watch(() => pluginConfig.value?.template, (newTemplate) => {
  console.log('Template changed:', newTemplate)
}, { immediate: false })

// 标签页数据
const tabsData = [
  {
    key: 'general',
    label: '通用配置',
    icon: Settings2
  },
  {
    key: 'environment',
    label: '环境配置',
    icon: ContainerIcon
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

onMounted(async () => {
  console.log('Component mounted')

  // 先初始化插件配置
  await initializePlugin()
  console.log('Plugin initialized:', {
    activePlugin: activePlugin.value,
    template: pluginConfig.value?.template
  })

  // 再初始化编辑器
  await initializeEditor()
  console.log('Editor initialized')

  // 更新扩展
  await updateExtensions()
  console.log('Extensions updated:', currentExtensions.value)
})
</script>
