<template>
  <div class="flex flex-col h-full bg-white dark:bg-gray-900">
    <!-- 头部 -->
    <div class="flex items-center justify-between px-3 py-1.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
      <div class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400">
        <Braces class="w-3.5 h-3.5"/>
        <span>{{ t('view.jsonTitle') }}</span>
        <span v-if="isRunning" class="text-blue-500">{{ t('view.running') }}</span>
        <span v-else-if="executionTime" class="text-gray-400">{{ t('view.ms', { n: executionTime }) }}</span>
        <span v-if="parseError" class="text-red-500">{{ t('view.parseFailed') }}</span>
      </div>
      <div class="flex items-center gap-1">
        <button class="p-1 rounded cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700"
                :class="mode === 'graph' ? 'text-blue-500' : 'text-gray-400 hover:text-gray-600 dark:hover:text-gray-300'"
                :title="mode === 'graph' ? t('view.toTree') : t('view.toGraph')"
                @click="mode = mode === 'graph' ? 'tree' : 'graph'">
          <Network class="w-3.5 h-3.5"/>
        </button>
        <button v-if="parsed !== undefined" class="p-1 rounded text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" :title="t('view.copy')" @click="copyJson">
          <Copy class="w-3.5 h-3.5"/>
        </button>
        <button class="p-1 rounded text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" :title="t('view.clear')" @click="emit('clear')">
          <Trash2 class="w-3.5 h-3.5"/>
        </button>
      </div>
    </div>

    <!-- 可视化关系图：独立占满（自带滚动）-->
    <DataGraph v-if="mode === 'graph' && stable.trim() && !parseError && parsed !== undefined"
               :value="parsed" class="flex-1"/>

    <!-- 层级树 / 占位 / 错误 -->
    <div v-else class="flex-1 overflow-auto p-2 font-mono text-xs">
      <div v-if="!stable.trim()" class="text-gray-400 px-2 py-4 text-center">{{ t('view.emptyJson') }}</div>
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
import {Braces, Copy, Network, Trash2} from 'lucide-vue-next'
import JsonNode from './JsonNode.vue'
import DataGraph from './DataGraph.vue'
import {useToast} from '../plugins/toast'
import {useI18n} from 'vue-i18n'

const props = defineProps<{
  output: string
  isRunning: boolean
  executionTime?: number
}>()
const emit = defineEmits<{ clear: [] }>()

const toast = useToast()
const {t} = useI18n()

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
    return t('view.jsonParseFail') + (e?.message || e)
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

// 显示模式：tree=层级树（默认），graph=可视化关系图
const mode = ref<'tree' | 'graph'>('tree')

const copyJson = async () => {
  try {
    await navigator.clipboard.writeText(JSON.stringify(parsed.value, null, 2))
    toast.success(t('view.jsonCopied'))
  }
  catch {
    toast.error(t('view.copyFailed'))
  }
}
</script>
