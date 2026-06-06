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
      <div class="flex items-center gap-1">
        <button v-if="isWorkflow"
                class="p-1 rounded cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700"
                :class="mode === 'workflow' ? 'text-amber-500' : 'text-gray-400 hover:text-gray-600 dark:hover:text-gray-300'"
                title="GitHub Actions 工作流图"
                @click="mode = mode === 'workflow' ? 'tree' : 'workflow'">
          <Workflow class="w-3.5 h-3.5"/>
        </button>
        <button class="p-1 rounded cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700"
                :class="mode === 'graph' ? 'text-blue-500' : 'text-gray-400 hover:text-gray-600 dark:hover:text-gray-300'"
                :title="mode === 'graph' ? '切换为层级树' : '切换为可视化关系图'"
                @click="mode = mode === 'graph' ? 'tree' : 'graph'">
          <Network class="w-3.5 h-3.5"/>
        </button>
        <button class="p-1 rounded text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" title="清空" @click="emit('clear')">
          <Trash2 class="w-3.5 h-3.5"/>
        </button>
      </div>
    </div>

    <!-- GitHub Actions 工作流图 -->
    <WorkflowGraph v-if="mode === 'workflow' && isWorkflow && !parseError"
                   :workflow="parsed" class="flex-1"/>

    <!-- 可视化关系图 -->
    <DataGraph v-else-if="mode === 'graph' && stable.trim() && !parseError && parsed !== undefined"
               :value="parsed" class="flex-1"/>

    <!-- 层级树 / 占位 / 错误 -->
    <div v-else class="flex-1 overflow-auto p-2 font-mono text-xs">
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
import {FileText, Network, Trash2, Workflow} from 'lucide-vue-next'
import yaml from 'js-yaml'
import JsonNode from './JsonNode.vue'
import DataGraph from './DataGraph.vue'
import WorkflowGraph from './WorkflowGraph.vue'

const props = defineProps<{
  output: string
  isRunning: boolean
  executionTime?: number
}>()
const emit = defineEmits<{ clear: [] }>()

// 显示模式：tree=层级树，graph=关系图，workflow=GitHub Actions 工作流图
const mode = ref<'tree' | 'graph' | 'workflow'>('tree')

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

// 检测是否为 GitHub Actions 工作流（含 on 触发 + jobs）
const isWorkflow = computed(() => {
  const v = parsed.value
  return !!(v && typeof v === 'object' && v.on !== undefined && v.jobs && typeof v.jobs === 'object')
})

// 是工作流时默认用工作流图模式
watch(isWorkflow, (yes) => {
  if (yes && mode.value === 'tree') {
    mode.value = 'workflow'
  }
  else if (!yes && mode.value === 'workflow') {
    mode.value = 'tree'
  }
}, {immediate: true})
</script>
