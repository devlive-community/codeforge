<template>
  <div class="bg-white border-b border-gray-200 px-4 py-3 flex items-center justify-between">
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
    </div>

    <div class="flex items-center space-x-3">
      <Button type="warning" :icon="CheckCircle" v-if="hasUpdate">
        有新版本
      </Button>

      <!-- 清空输出按钮 -->
      <Button @click="handleClearOutput"
              :disabled="isRunning"
              type="secondary"
              :icon-only="true"
              :icon="Trash2">
      </Button>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, onMounted, ref} from 'vue'
import {CheckCircle, FileCode, Play, Square, Trash2} from 'lucide-vue-next'
import Select from '../ui/Select.vue'
import Button from '../ui/Button.vue'
import {Language} from '../types/app.ts'
import {invoke} from "@tauri-apps/api/core";
import {useUpdateManager} from "../composables/useUpdateManager.ts";

const props = defineProps<{
  isRunning: boolean
  envInstalled: boolean
  supportedLanguages: Language[]
  currentLanguage: string
}>()

const emit = defineEmits<{
  'run-code': []
  'stop-code': []
  'clear-output': []
  'show-settings': []
  'language-change': [language: string]
  'load-example': [content: string]
}>()

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

const handleClearOutput = () => {
  emit('clear-output')
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
