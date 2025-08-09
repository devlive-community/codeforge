<template>
  <div class="flex flex-col h-full bg-gray-900 text-green-400">
    <div class="p-2 border-b border-gray-700 flex items-center justify-between">
      <div class="flex items-center space-x-2">
        <Terminal class="w-4 h-4"/>
        <span class="text-sm font-medium">输出</span>
      </div>
      <div v-if="executionTime > 0" class="text-xs text-gray-400 flex items-center space-x-1">
        <Clock class="w-3 h-3"/>
        <span>{{ executionTime }} 毫秒</span>
      </div>
    </div>

    <div class="flex-1 overflow-auto">
      <div v-if="isRunning" class="p-4 flex items-center space-x-2 text-yellow-400">
        <Loader class="w-4 h-4 animate-spin"/>
        <span>执行代码中...</span>
      </div>

      <div v-else-if="output" class="p-4">
        <pre :class="['whitespace-pre-wrap text-sm leading-relaxed', isSuccess ? 'text-green-300' : 'text-red-300']">{{ output }}</pre>
      </div>

      <div v-else class="p-4 text-gray-500 flex flex-col items-center justify-center h-full space-y-2 select-none">
        <Terminal class="w-8 h-8"/>
        <p class="text-sm">没有输出</p>
        <p class="text-xs">可以尝试运行一些代码</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Clock, Loader, Terminal } from 'lucide-vue-next'

defineProps<{
  output: string
  isRunning: boolean
  isSuccess: boolean
  executionTime: number
}>()
</script>