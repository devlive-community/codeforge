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
    </div>

    <div class="flex items-center space-x-3">
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
import { computed } from 'vue'
import { Play, Square, Trash2 } from 'lucide-vue-next'
import Select from '../ui/Select.vue'
import Button from '../ui/Button.vue'
import { Language } from '../types/app.ts'

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
}>()

// 使用计算属性来处理双向绑定
const selectedLanguage = computed({
  get: () => props.currentLanguage,
  set: (value: string) => {
    if (value !== props.currentLanguage) {
      emit('language-change', value)
    }
  }
})

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

// 处理 Select 组件的 change 事件
const handleLanguageChange = (value: string) => {
  if (value !== props.currentLanguage) {
    emit('language-change', value)
  }
}
</script>
