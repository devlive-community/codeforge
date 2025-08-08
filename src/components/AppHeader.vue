<template>
  <div class="bg-white border-b border-gray-200 px-4 py-3 flex items-center justify-between">
    <div class="flex items-center space-x-3">
      <div class="relative">
        <!-- 自定义下拉选择器 -->
        <div class="relative">
          <select v-model="selectedLanguage"
                  class="w-32 h-10 backdrop-blur-sm appearance-none bg-white/90 text-gray-800 text-sm font-medium border border-gray-200 rounded-lg px-2.5 pr-8 cursor-pointer focus:outline-none hover:bg-white/95 transition-all duration-200"
                  :class="{ 'opacity-50 cursor-not-allowed': isRunning }"
                  :disabled="isRunning"
                  @change="handleLanguageChange">
            <option v-for="language in supportedLanguages"
                    class="text-gray-800 bg-white py-3 px-3 text-sm font-medium hover:bg-blue-50 hover:text-blue-800 focus:bg-blue-100 focus:text-blue-900 border-b border-gray-100 last:border-b-0"
                    :key="language.value"
                    :value="language.value">
              {{ language.name }}
            </option>
          </select>
        </div>
      </div>
    </div>

    <div class="flex items-center space-x-3">
      <button @click="$emit('run-code')"
              :disabled="isRunning || !envInstalled"
              :class="['flex items-center space-x-2 px-3 py-1.5 rounded-md font-medium transition-all duration-200',
                isRunning || !envInstalled
                  ? 'bg-gray-300 text-gray-500 cursor-not-allowed'
                  : 'btn-success shadow-sm hover:shadow-md cursor-pointer'
              ]">
        <component :is="isRunning ? Square : Play" class="w-3 h-3"/>
        <span>{{ isRunning ? '运行中...' : '运行代码' }}</span>
      </button>

      <button class="btn-danger px-3 py-2.5 rounded-md font-medium transition-all duration-200"
              :disabled="isRunning || !envInstalled"
              :class="[isRunning || !envInstalled ? 'cursor-not-allowed' : 'cursor-pointer']"
              @click="$emit('clear-output')">
        <Trash2 class="w-4 h-4"/>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { Play, Square, Trash2 } from 'lucide-vue-next'

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
