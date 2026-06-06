<template>
  <div class="flex flex-col h-full bg-white dark:bg-gray-900">
    <div class="flex items-center justify-between px-3 py-1.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
      <div class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400">
        <Code class="w-3.5 h-3.5"/>
        <span>XML 视图</span>
        <span v-if="isRunning" class="text-blue-500">运行中…</span>
        <span v-else-if="executionTime" class="text-gray-400">{{ executionTime }} ms</span>
        <span v-if="parseError" class="text-red-500">解析失败</span>
      </div>
      <button class="text-xs text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('clear')">清空</button>
    </div>

    <div class="flex-1 overflow-auto p-2 font-mono text-xs">
      <div v-if="!stable.trim()" class="text-gray-400 px-2 py-4 text-center">运行后在此查看 XML</div>
      <div v-else-if="parseError" class="space-y-2">
        <div class="text-red-500">{{ parseError }}</div>
        <pre class="whitespace-pre-wrap text-gray-600 dark:text-gray-400">{{ stable }}</pre>
      </div>
      <XmlNode v-else-if="root" :node="root" :depth="0"/>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, ref, watch} from 'vue'
import {debounce} from 'lodash-es'
import {Code} from 'lucide-vue-next'
import XmlNode from './XmlNode.vue'

const props = defineProps<{
  output: string
  isRunning: boolean
  executionTime?: number
}>()
const emit = defineEmits<{ clear: [] }>()

// 流式输出防抖，避免逐行重建树
const stable = ref(props.output)
const applyOutput = debounce((v: string) => { stable.value = v }, 200)
watch(() => props.output, (v) => applyOutput(v))

const doc = computed<Document | null>(() => {
  if (!stable.value.trim()) {
    return null
  }
  try {
    return new DOMParser().parseFromString(stable.value, 'application/xml')
  }
  catch {
    return null
  }
})

const parseError = computed(() => {
  const d = doc.value
  if (!d) {
    return ''
  }
  const err = d.querySelector('parsererror')
  return err ? 'XML 解析失败：' + (err.textContent || '').trim() : ''
})

const root = computed<Element | null>(() => {
  if (!doc.value || parseError.value) {
    return null
  }
  return doc.value.documentElement || null
})
</script>
