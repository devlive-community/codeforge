<template>
  <div class="flex flex-col h-full bg-white dark:bg-gray-900">
    <div class="flex items-center justify-between px-3 py-1.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
      <div class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400">
        <FileText class="w-3.5 h-3.5"/>
        <span>YAML 视图</span>
        <span v-if="isRunning" class="text-blue-500">运行中…</span>
        <span v-else-if="executionTime" class="text-gray-400">{{ executionTime }} ms</span>
        <span v-if="parseError" class="text-red-500">解析失败</span>
      </div>
      <button class="text-xs text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('clear')">清空</button>
    </div>

    <div class="flex-1 overflow-auto p-2 font-mono text-xs">
      <div v-if="!stable.trim()" class="text-gray-400 px-2 py-4 text-center">运行后在此查看 YAML</div>
      <div v-else-if="parseError" class="space-y-2">
        <div class="text-red-500">{{ parseError }}</div>
        <pre class="whitespace-pre-wrap text-gray-600 dark:text-gray-400">{{ stable }}</pre>
      </div>
      <JsonNode v-else-if="parsed !== undefined" :value="parsed" :depth="0"/>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, ref, watch} from 'vue'
import {debounce} from 'lodash-es'
import {FileText} from 'lucide-vue-next'
import yaml from 'js-yaml'
import JsonNode from './JsonNode.vue'

const props = defineProps<{
  output: string
  isRunning: boolean
  executionTime?: number
}>()
const emit = defineEmits<{ clear: [] }>()

// 流式输出防抖
const stable = ref(props.output)
const applyOutput = debounce((v: string) => { stable.value = v }, 200)
watch(() => props.output, (v) => applyOutput(v))

// YAML 解析为 JS 对象，复用 JSON 树渲染（一次解析，派生 parsed/parseError）
const parseResult = computed<{ value: any; error: string }>(() => {
  if (!stable.value.trim()) {
    return {value: undefined, error: ''}
  }
  try {
    return {value: yaml.load(stable.value), error: ''}
  }
  catch (e: any) {
    return {value: undefined, error: 'YAML 解析失败：' + (e?.reason || e?.message || e)}
  }
})
const parsed = computed(() => parseResult.value.value)
const parseError = computed(() => parseResult.value.error)
</script>
