<template>
  <div class="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 px-4 py-3 flex items-center justify-between">
    <div class="flex items-center space-x-3">
      <Select v-model="selectedLanguage"
              class="w-64"
              searchable
              :options="supportedLanguages as any"
              :disabled="isRunning"
              placeholder="选择语言"
              value-key="value"
              label-key="name"
              @change="handleLanguageChange">
      </Select>

      <!-- 运行/停止按钮 -->
      <Button v-if="!isRunning"
              @click="handleRunCode"
              :disabled="!envInstalled"
              :icon="Play">
        <span>运行代码</span>
      </Button>

      <Button v-else
              @click="handleStopCode"
              type="danger"
              :icon="Square">
        <span>停止执行</span>
      </Button>

      <Button type="info" :icon="FileCode" @click="loadExample">加载示例</Button>

      <Tooltip text="执行历史">
        <Button type="secondary" :icon="History" :icon-only="true" @click="emit('show-history')"/>
      </Tooltip>

      <!-- 打开/保存文件 -->
      <Tooltip text="打开文件">
        <Button type="secondary" :icon="FolderOpen" :icon-only="true" @click="emit('open-file')"/>
      </Tooltip>
      <Tooltip text="保存文件">
        <Button type="secondary" :icon="Save" :icon-only="true" @click="emit('save-file')"/>
      </Tooltip>
      <Tooltip text="AI 助手">
        <Button type="secondary" :icon="Sparkles" :icon-only="true" @click="emit('show-ai')"/>
      </Tooltip>
    </div>

    <div class="flex items-center space-x-3">
      <!-- 侧栏开关 + 布局切换 -->
      <div class="flex items-center bg-gray-100 dark:bg-gray-700 rounded-md p-0.5">
        <Tooltip :text="sidebarVisible ? '隐藏侧栏' : '显示侧栏'">
          <button class="p-1.5 rounded transition-colors cursor-pointer"
                  :class="sidebarVisible ? 'bg-white dark:bg-gray-900 text-blue-600 dark:text-blue-400 shadow-sm' : 'text-gray-500 hover:text-gray-700 dark:hover:text-gray-200'"
                  @click="emit('toggle-sidebar')">
            <PanelLeft class="w-4 h-4"/>
          </button>
        </Tooltip>

        <div class="w-px h-4 bg-gray-300 dark:bg-gray-600 mx-0.5"></div>

        <Tooltip v-for="item in layoutOptions" :key="item.value" :text="item.label">
          <button class="p-1.5 rounded transition-colors cursor-pointer"
                  :class="currentLayout === item.value ? 'bg-white dark:bg-gray-900 text-blue-600 dark:text-blue-400 shadow-sm' : 'text-gray-500 hover:text-gray-700 dark:hover:text-gray-200'"
                  @click="handleLayoutChange(item.value)">
            <component :is="item.icon" class="w-4 h-4"/>
          </button>
        </Tooltip>
      </div>

      <Button type="warning" :icon="CheckCircle" v-if="hasUpdate">
        有新版本
      </Button>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, onMounted, ref} from 'vue'
import {CheckCircle, FileCode, FolderOpen, History, Maximize2, PanelBottom, PanelLeft, PanelRight, Play, Save, Sparkles, Square} from 'lucide-vue-next'
import Select from '../ui/Select.vue'
import Button from '../ui/Button.vue'
import Tooltip from '../ui/Tooltip.vue'
import {Language, LayoutMode} from '../types/app.ts'
import {invoke} from "@tauri-apps/api/core";
import {useUpdateManager} from "../composables/useUpdateManager.ts";

const props = defineProps<{
  isRunning: boolean
  envInstalled: boolean
  supportedLanguages: Language[]
  currentLanguage: string
  currentLayout: LayoutMode
  sidebarVisible: boolean
}>()

const emit = defineEmits<{
  'run-code': []
  'stop-code': []
  'show-settings': []
  'language-change': [language: string]
  'load-example': [content: string]
  'layout-change': [mode: LayoutMode]
  'open-file': []
  'save-file': []
  'show-history': []
  'show-ai': []
  'toggle-sidebar': []
}>()

const layoutOptions: { value: LayoutMode; label: string; icon: any }[] = [
  {value: 'horizontal', label: '左右布局', icon: PanelRight},
  {value: 'vertical', label: '上下布局', icon: PanelBottom},
  {value: 'editor', label: '仅编辑器', icon: Maximize2}
]

const handleLayoutChange = (mode: LayoutMode) => {
  emit('layout-change', mode)
}

const {checkForUpdates} = useUpdateManager()

// 使用计算属性来处理双向绑定
const selectedLanguage = computed({
  get: () => props.currentLanguage,
  set: (value: string) => {
    if (value !== props.currentLanguage) {
      emit('language-change', value)
    }
  }
})
const hasUpdate = ref(false)

// 事件处理函数 - 确保不传递任何参数
const handleRunCode = () => {
  emit('run-code')
}

const handleStopCode = () => {
  emit('stop-code')
}

const checkUpdater = async () => {
  const result = await checkForUpdates()
  if (result) {
    hasUpdate.value = true
  }
}

// 处理 Select 组件的 change 事件
const handleLanguageChange = (value: string) => {
  if (value !== props.currentLanguage) {
    emit('language-change', value)
  }
}

const loadExample = async () => {
  try {
    const example = await invoke<any>('load_example', {language: selectedLanguage.value})
    emit('load-example', example)
  }
  catch (error) {
    console.error('Error loading example:', error)
  }
}

onMounted(async () => {
  await checkUpdater()
})
</script>
