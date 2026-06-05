<template>
  <div class="flex flex-col h-full bg-white dark:bg-gray-900">
    <!-- 头部 -->
    <div class="flex items-center justify-between px-3 py-1.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
      <div class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400">
        <Braces class="w-3.5 h-3.5"/>
        <span>JSON 视图</span>
        <span v-if="isRunning" class="text-blue-500">运行中…</span>
        <span v-else-if="executionTime" class="text-gray-400">{{ executionTime }} ms</span>
        <span v-if="parseError" class="text-red-500">解析失败</span>
      </div>
      <div class="flex items-center gap-2">
        <button v-if="parsed !== undefined" class="text-xs text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="copyJson">复制</button>
        <button class="text-xs text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('clear')">清空</button>
      </div>
    </div>

    <!-- 内容 -->
    <div class="flex-1 overflow-auto p-2 font-mono text-xs">
      <div v-if="!output.trim()" class="text-gray-400 px-2 py-4 text-center">运行后在此查看 JSON</div>
      <!-- 解析失败：显示错误 + 原文 -->
      <div v-else-if="parseError" class="space-y-2">
        <div class="text-red-500">{{ parseError }}</div>
        <pre class="whitespace-pre-wrap text-gray-600 dark:text-gray-400">{{ output }}</pre>
      </div>
      <!-- JSON 树 -->
      <JsonNode v-else :value="parsed" :depth="0"/>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed} from 'vue'
import {Braces} from 'lucide-vue-next'
import JsonNode from './JsonNode.vue'
import {useToast} from '../plugins/toast'

const props = defineProps<{
  output: string
  isRunning: boolean
  executionTime?: number
}>()
const emit = defineEmits<{ clear: [] }>()

const toast = useToast()

const parseError = computed(() => {
  if (!props.output.trim()) {
    return ''
  }
  try {
    JSON.parse(props.output)
    return ''
  }
  catch (e: any) {
    return 'JSON 解析失败：' + (e?.message || e)
  }
})

const parsed = computed(() => {
  try {
    return JSON.parse(props.output)
  }
  catch {
    return undefined
  }
})

const copyJson = async () => {
  try {
    await navigator.clipboard.writeText(JSON.stringify(parsed.value, null, 2))
    toast.success('已复制格式化 JSON')
  }
  catch {
    toast.error('复制失败')
  }
}
</script>
