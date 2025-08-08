<template>
  <div class="bg-white border-b border-gray-200 px-4 py-3 flex items-center justify-between">
    <div class="flex items-center space-x-3">
      <div class="w-12 h-12 bg-gradient-to-br rounded-lg flex items-center justify-center">
        <img src="/codeforge.svg" alt="CodeForge">
      </div>
      <div>
        <h1 class="text-lg font-bold text-gray-800">CodeForge</h1>
        <p class="text-xs text-gray-500">轻量级、高性能的桌面代码执行器，专为开发者、学生和编程爱好者设计。</p>
      </div>
    </div>

    <div class="flex items-center space-x-3">
      <button @click="$emit('run-code')"
              :disabled="isRunning || !envInstalled"
              :class="['flex items-center space-x-2 px-3 py-1.5 rounded-md font-medium transition-all duration-200 cursor-pointer',
                isRunning || !envInstalled
                  ? 'bg-gray-300 text-gray-500 cursor-not-allowed'
                  : 'btn-success shadow-sm hover:shadow-md'
              ]">
        <component :is="isRunning ? Square : Play" class="w-3 h-3"/>
        <span>{{ isRunning ? '运行中...' : '运行代码' }}</span>
      </button>

      <button @click="$emit('clear-output')" class="btn-danger px-3 py-2.5 rounded-md font-medium transition-all duration-200 cursor-pointer">
        <Trash2 class="w-4 h-4"/>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Play, Square, Trash2 } from 'lucide-vue-next'

defineProps<{
  isRunning: boolean
  envInstalled: boolean
}>()

defineEmits<{
  'run-code': []
  'clear-output': []
  'show-settings': []
}>()
</script>