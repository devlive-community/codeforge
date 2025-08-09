<template>
  <div class="bg-white border-b border-gray-200 px-4 py-3 flex items-center justify-between">
    <div class="flex items-center space-x-3">
      <div class="relative">
        <!-- 自定义下拉选择器 -->
        <div class="relative">
          <Select v-model="selectedLanguage"
                  class="w-48"
                  :options="supportedLanguages as any"
                  :disabled="isRunning"
                  placeholder="选择语言"
                  value-key="value"
                  label-key="name"
                  @change="handleLanguageChange">
          </Select>
        </div>
      </div>
    </div>

    <div class="flex items-center space-x-3">
      <Button @click="$emit('run-code')"
              :disabled="isRunning || !envInstalled"
              :icon="isRunning ? Square : Play">
        <span>{{ isRunning ? '运行中...' : '运行代码' }}</span>
      </Button>

      <Button @click="$emit('clear-output')"
              :disabled="isRunning || !envInstalled"
              type="danger"
              :icon-only="true"
              :icon="Trash2">
      </Button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { Play, Square, Trash2 } from 'lucide-vue-next'
import Select from '../ui/Select.vue'
import Button from '../ui/Button.vue'

interface Language
{
  name: string
  value: string
}

const props = defineProps<{
  isRunning: boolean
  envInstalled: boolean
  supportedLanguages: Language[]
  currentLanguage: string
}>()

const emit = defineEmits<{
  'run-code': []
  'clear-output': []
  'show-settings': []
  'language-change': [language: string]
}>()

const selectedLanguage = ref(props.currentLanguage)

// 监听外部语言变化
watch(() => props.currentLanguage, (newLanguage) => {
  selectedLanguage.value = newLanguage
})

const handleLanguageChange = () => {
  emit('language-change', selectedLanguage.value)
}
</script>
