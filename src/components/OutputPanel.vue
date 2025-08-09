<template>
  <div class="flex flex-col h-full bg-gray-900 text-green-400">
    <div class="p-2 border-b border-gray-700 flex items-center justify-between">
      <div class="flex items-center space-x-2">
        <Terminal class="w-4 h-4"/>
        <span class="text-sm font-medium">控制台</span>
      </div>
      <div class="flex items-center space-x-3">
        <span v-if="isCopied" class="text-xs text-gray-400">{{ isCopied ? '已复制' : '复制失败' }}</span>

        <!-- 复制按钮 -->
        <button v-if="output && !isRunning"
                @click="copyOutput"
                class="text-gray-400 hover:text-white transition-colors duration-200 p-1 rounded hover:bg-gray-700 cursor-pointer"
                title="复制输出内容">
          <component :is="copyIcon" class="w-3 h-3"/>
        </button>

        <div v-if="executionTime > 0" class="text-xs text-gray-400 flex items-center space-x-1">
          <Clock class="w-3 h-3"/>
          <span>{{ executionTime }} 毫秒</span>
        </div>
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
import { computed, ref } from 'vue'
import { Check, Clock, Copy, Loader, Terminal } from 'lucide-vue-next'

const props = defineProps<{
  output: string
  isRunning: boolean
  isSuccess: boolean
  executionTime: number
}>()

const isCopied = ref(false)

// 动态切换图标
const copyIcon = computed(() => isCopied.value ? Check : Copy)

const copyOutput = async () => {
  if (!props.output) {
    return
  }

  try {
    await navigator.clipboard.writeText(props.output)
    isCopied.value = true

    // 2秒后恢复复制图标
    setTimeout(() => {
      isCopied.value = false
    }, 2000)
  }
  catch (error) {
    console.error('复制失败:', error)

    // 降级方案：使用传统方法复制
    try {
      const textArea = document.createElement('textarea')
      textArea.value = props.output
      document.body.appendChild(textArea)
      textArea.select()
      document.execCommand('copy')
      document.body.removeChild(textArea)

      isCopied.value = true
      setTimeout(() => {
        isCopied.value = false
      }, 2000)
    }
    catch (fallbackError) {
      console.error('降级复制也失败了:', fallbackError)
    }
  }
}
</script>