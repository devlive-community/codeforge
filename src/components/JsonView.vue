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
      <div class="flex items-center gap-1">
        <button class="p-1 rounded cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700"
                :class="mode === 'tree' ? 'text-blue-500' : 'text-gray-400 hover:text-gray-600 dark:hover:text-gray-300'"
                :title="mode === 'tree' ? '切换为文本' : '切换为可视化（树）'"
                @click="mode = mode === 'tree' ? 'text' : 'tree'">
          <Network class="w-3.5 h-3.5"/>
        </button>
        <button v-if="parsed !== undefined" class="p-1 rounded text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" title="复制" @click="copyJson">
          <Copy class="w-3.5 h-3.5"/>
        </button>
        <button class="p-1 rounded text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" title="清空" @click="emit('clear')">
          <Trash2 class="w-3.5 h-3.5"/>
        </button>
      </div>
    </div>

    <!-- 内容 -->
    <div class="flex-1 overflow-auto p-2 font-mono text-xs">
      <div v-if="!stable.trim()" class="text-gray-400 px-2 py-4 text-center">运行后在此查看 JSON</div>
      <!-- 解析失败：显示错误 + 原文 -->
      <div v-else-if="parseError" class="space-y-2">
        <div class="text-red-500">{{ parseError }}</div>
        <pre class="whitespace-pre-wrap text-gray-600 dark:text-gray-400">{{ stable }}</pre>
      </div>
      <!-- 可视化：JSON 树 -->
      <JsonNode v-else-if="mode === 'tree' && parsed !== undefined" :value="parsed" :depth="0"/>
      <!-- 文本：格式化后的 JSON -->
      <pre v-else class="whitespace-pre-wrap text-gray-700 dark:text-gray-300">{{ formatted }}</pre>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, ref, watch} from 'vue'
import {debounce} from 'lodash-es'
import {Braces, Copy, Network, Trash2} from 'lucide-vue-next'
import JsonNode from './JsonNode.vue'
import {useToast} from '../plugins/toast'

const props = defineProps<{
  output: string
  isRunning: boolean
  executionTime?: number
}>()
const emit = defineEmits<{ clear: [] }>()

const toast = useToast()

// 流式输出逐行刷新 output，解析/渲染防抖，避免每行都重建整棵树导致卡死
const stable = ref(props.output)
const applyOutput = debounce((v: string) => { stable.value = v }, 200)
watch(() => props.output, (v) => applyOutput(v))

const parseError = computed(() => {
  if (!stable.value.trim()) {
    return ''
  }
  try {
    JSON.parse(stable.value)
    return ''
  }
  catch (e: any) {
    return 'JSON 解析失败：' + (e?.message || e)
  }
})

const parsed = computed(() => {
  try {
    return JSON.parse(stable.value)
  }
  catch {
    return undefined
  }
})

// 显示模式：text=格式化文本（默认），tree=可视化树
const mode = ref<'text' | 'tree'>('text')

// 格式化后的 JSON 文本
const formatted = computed(() => {
  if (parsed.value === undefined) {
    return stable.value
  }
  try {
    return JSON.stringify(parsed.value, null, 2)
  }
  catch {
    return stable.value
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
