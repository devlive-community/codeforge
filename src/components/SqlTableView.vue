<template>
  <div class="flex flex-col h-full bg-white dark:bg-gray-900">
    <div class="flex items-center justify-between px-3 py-1.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
      <div class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400 min-w-0">
        <Database class="w-3.5 h-3.5 flex-shrink-0"/>
        <span>SQL 结果</span>
        <span v-if="isRunning" class="text-blue-500">运行中…</span>
        <span v-else-if="executionTime" class="text-gray-400">{{ executionTime }} ms</span>
        <!-- 数据源选择 -->
        <SqlSourceSelect/>
      </div>
      <button class="p-1 rounded text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" title="清空" @click="emit('clear')">
        <Trash2 class="w-3.5 h-3.5"/>
      </button>
    </div>

    <div class="flex-1 overflow-auto p-2">
      <SqlResultTable :output="stable" :empty-text="`运行后在此查看 SQL 结果（数据源：${activeLabel()}）`"/>
    </div>
  </div>
</template>

<script setup lang="ts">
import {ref, watch} from 'vue'
import {debounce} from 'lodash-es'
import {Database, Trash2} from 'lucide-vue-next'
import {useDbConnections} from '../composables/useDbConnections'
import SqlSourceSelect from './SqlSourceSelect.vue'
import SqlResultTable from './SqlResultTable.vue'

const props = defineProps<{
  output: string
  isRunning: boolean
  executionTime?: number
}>()
const emit = defineEmits<{ clear: [] }>()

// 流式输出防抖
const stable = ref(props.output)
const applyOutput = debounce((v: string) => { stable.value = v }, 100)
watch(() => props.output, (v) => applyOutput(v))

// 当前数据源名（占位文案用）；选择逻辑由 SqlSourceSelect 组件负责
const {activeLabel} = useDbConnections()
</script>
