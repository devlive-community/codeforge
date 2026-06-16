<template>
  <div class="flex flex-col h-full bg-white dark:bg-gray-900">
    <!-- 头部 -->
    <div class="flex items-center justify-between px-3 py-1.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
      <div class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400 min-w-0">
        <Table2 class="w-3.5 h-3.5 flex-shrink-0"/>
        <span>{{ t('view.dataTitle') }}</span>
        <span v-if="isRunning || parsing" class="text-blue-500">{{ t('view.parsing') }}{{ parsing && percent > 0 ? ` ${percent}%` : '' }}…</span>
        <span v-else-if="parsed.rows.length" class="text-gray-400">{{ t('view.colsRows', { cols: parsed.columns.length, rows: parsed.rows.length }) }}</span>
      </div>
      <div class="flex items-center gap-1">
        <div class="flex items-center rounded-md border border-gray-200 dark:border-gray-700 overflow-hidden">
          <button class="p-1 transition-colors" :class="viewMode === 'table' ? 'bg-blue-500 text-white' : 'text-gray-400 hover:text-gray-600 dark:hover:text-gray-300'" :title="t('view.table')" @click="viewMode = 'table'">
            <Table2 class="w-3.5 h-3.5"/>
          </button>
          <button class="p-1 transition-colors" :class="viewMode === 'chart' ? 'bg-blue-500 text-white' : 'text-gray-400 hover:text-gray-600 dark:hover:text-gray-300'"
                  :disabled="!parsed.columns.length" :title="parsed.columns.length ? t('view.chart') : t('view.noChartData')" @click="viewMode = 'chart'">
            <BarChart3 class="w-3.5 h-3.5" :class="!parsed.columns.length ? 'opacity-40' : ''"/>
          </button>
        </div>
        <button v-if="parsed.columns.length" class="p-1 rounded text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" :title="t('view.exportCsv')" @click="exportCsv">
          <FileDown class="w-3.5 h-3.5"/>
        </button>
        <button class="p-1 rounded text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" :title="t('view.clear')" @click="emit('clear')">
          <Trash2 class="w-3.5 h-3.5"/>
        </button>
      </div>
    </div>

    <!-- 图表视图 -->
    <div v-if="viewMode === 'chart' && parsed.columns.length" class="flex-1 min-h-0 overflow-hidden">
      <ChartPanel :columns="parsed.columns" :rows="parsed.rows"/>
    </div>
    <!-- 表格视图（虚拟滚动，支持大文件） -->
    <div v-if="!parsed.columns.length" class="flex-1 overflow-auto p-2 text-xs">
      <div class="text-gray-400 px-2 py-4 text-center">{{ t('view.emptyData') }}</div>
    </div>
    <VirtualTable v-else class="flex-1" :columns="parsed.columns" :rows="parsed.rows"/>
  </div>
</template>

<script setup lang="ts">
import {onBeforeUnmount, ref, shallowRef, watch} from 'vue'
import {useI18n} from 'vue-i18n'
import {debounce} from 'lodash-es'
import {BarChart3, FileDown, Table2, Trash2} from 'lucide-vue-next'
import ChartPanel from './charts/ChartPanel.vue'
import VirtualTable from './VirtualTable.vue'
import {downloadCsv} from '../utils/csv'
import {type DelimitedTable, parseTable} from '../utils/delimited'

const props = defineProps<{
  output: string
  isRunning: boolean
  executionTime?: number
}>()
const emit = defineEmits<{ clear: [] }>()
const {t} = useI18n()

const viewMode = ref<'table' | 'chart'>('table')
// shallowRef：大数组不做深度响应，避免开销
const parsed = shallowRef<DelimitedTable>({columns: [], rows: []})
const parsing = ref(false)
const percent = ref(0)

// 解析放到 Web Worker，超大文件不阻塞 UI；创建失败则主线程兜底
let worker: Worker | null = null
let reqId = 0
type WorkerMsg = { id: number; type: 'progress'; percent: number } | { id: number; type: 'done' } & DelimitedTable
try {
  worker = new Worker(new URL('../workers/delimited.worker.ts', import.meta.url), {type: 'module'})
  worker.onmessage = (e: MessageEvent<WorkerMsg>) => {
    const msg = e.data
    if (msg.id !== reqId) {
      return // 丢弃过期结果
    }
    if (msg.type === 'progress') {
      percent.value = msg.percent
      return
    }
    parsed.value = {columns: msg.columns, rows: msg.rows}
    parsing.value = false
    if (!parsed.value.columns.length && viewMode.value === 'chart') {
      viewMode.value = 'table'
    }
  }
}
catch {
  worker = null
}

const doParse = (text: string) => {
  reqId++
  percent.value = 0
  if (!text.trim()) {
    parsed.value = {columns: [], rows: []}
    parsing.value = false
    return
  }
  if (worker) {
    parsing.value = true
    worker.postMessage({id: reqId, text})
  }
  else {
    parsed.value = parseTable(text)
    if (!parsed.value.columns.length && viewMode.value === 'chart') {
      viewMode.value = 'table'
    }
  }
}

const applyOutput = debounce((v: string) => doParse(v), 150)
watch(() => props.output, (v) => applyOutput(v))
doParse(props.output)

const exportCsv = () => downloadCsv(parsed.value.columns, parsed.value.rows, `data-${Date.now()}.csv`)

onBeforeUnmount(() => worker?.terminate())
</script>
