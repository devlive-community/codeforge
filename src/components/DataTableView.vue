<template>
  <div class="flex flex-col h-full bg-white dark:bg-gray-900">
    <!-- 头部 -->
    <div class="flex items-center justify-between px-3 py-1.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
      <div class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400 min-w-0">
        <Table2 class="w-3.5 h-3.5 flex-shrink-0"/>
        <span>数据表</span>
        <span v-if="isRunning" class="text-blue-500">解析中…</span>
        <span v-else-if="parsed.rows.length" class="text-gray-400">{{ parsed.columns.length }} 列 · {{ parsed.rows.length }} 行</span>
      </div>
      <div class="flex items-center gap-1">
        <div class="flex items-center rounded-md border border-gray-200 dark:border-gray-700 overflow-hidden">
          <button class="p-1 transition-colors" :class="viewMode === 'table' ? 'bg-blue-500 text-white' : 'text-gray-400 hover:text-gray-600 dark:hover:text-gray-300'" title="表格" @click="viewMode = 'table'">
            <Table2 class="w-3.5 h-3.5"/>
          </button>
          <button class="p-1 transition-colors" :class="viewMode === 'chart' ? 'bg-blue-500 text-white' : 'text-gray-400 hover:text-gray-600 dark:hover:text-gray-300'"
                  :disabled="!parsed.columns.length" :title="parsed.columns.length ? '图表' : '无数据可绘图'" @click="viewMode = 'chart'">
            <BarChart3 class="w-3.5 h-3.5" :class="!parsed.columns.length ? 'opacity-40' : ''"/>
          </button>
        </div>
        <button v-if="parsed.columns.length" class="p-1 rounded text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" title="导出 CSV" @click="exportCsv">
          <FileDown class="w-3.5 h-3.5"/>
        </button>
        <button class="p-1 rounded text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" title="清空" @click="emit('clear')">
          <Trash2 class="w-3.5 h-3.5"/>
        </button>
      </div>
    </div>

    <!-- 图表视图 -->
    <div v-if="viewMode === 'chart' && parsed.columns.length" class="flex-1 min-h-0">
      <ChartPanel :columns="parsed.columns" :rows="parsed.rows"/>
    </div>
    <!-- 表格视图（虚拟滚动，支持大文件） -->
    <div v-if="!parsed.columns.length" class="flex-1 overflow-auto p-2 text-xs">
      <div class="text-gray-400 px-2 py-4 text-center">运行后在此查看数据表（支持 CSV / TSV）</div>
    </div>
    <VirtualTable v-else class="flex-1" :columns="parsed.columns" :rows="parsed.rows"/>
  </div>
</template>

<script setup lang="ts">
import {computed, ref, watch} from 'vue'
import {debounce} from 'lodash-es'
import {BarChart3, FileDown, Table2, Trash2} from 'lucide-vue-next'
import ChartPanel from './charts/ChartPanel.vue'
import VirtualTable from './VirtualTable.vue'
import {downloadCsv} from '../utils/csv'

const props = defineProps<{
  output: string
  isRunning: boolean
  executionTime?: number
}>()
const emit = defineEmits<{ clear: [] }>()

const viewMode = ref<'table' | 'chart'>('table')

const stable = ref(props.output)
const applyOutput = debounce((v: string) => { stable.value = v }, 150)
watch(() => props.output, (v) => applyOutput(v))

// 根据首行推断分隔符（制表符优先支持 TSV，否则逗号）
const detectDelimiter = (text: string): string => {
  const firstLine = text.split('\n', 1)[0] || ''
  const tabs = (firstLine.match(/\t/g) || []).length
  const commas = (firstLine.match(/,/g) || []).length
  const semis = (firstLine.match(/;/g) || []).length
  if (tabs > 0 && tabs >= commas) {
    return '\t'
  }
  if (semis > commas) {
    return ';'
  }
  return ','
}

// 支持引号、转义引号("")、字段内换行的分隔解析
const parseDelimited = (text: string, delim: string): string[][] => {
  const rows: string[][] = []
  let field = ''
  let row: string[] = []
  let inQuotes = false
  for (let i = 0; i < text.length; i++) {
    const c = text[i]
    if (inQuotes) {
      if (c === '"') {
        if (text[i + 1] === '"') {
          field += '"'
          i++
        }
        else {
          inQuotes = false
        }
      }
      else {
        field += c
      }
    }
    else if (c === '"') {
      inQuotes = true
    }
    else if (c === delim) {
      row.push(field)
      field = ''
    }
    else if (c === '\n') {
      row.push(field)
      rows.push(row)
      row = []
      field = ''
    }
    else if (c !== '\r') {
      field += c
    }
  }
  if (field.length > 0 || row.length > 0) {
    row.push(field)
    rows.push(row)
  }
  return rows
}

const parsed = computed<{ columns: string[]; rows: string[][] }>(() => {
  const text = stable.value.trim()
  if (!text) {
    return {columns: [], rows: []}
  }
  const delim = detectDelimiter(text)
  const all = parseDelimited(text, delim).filter(r => r.length > 1 || (r.length === 1 && r[0] !== ''))
  if (all.length === 0) {
    return {columns: [], rows: []}
  }
  const columns = all[0].map((c, i) => c.trim() || `列${i + 1}`)
  const rows = all.slice(1).map(r => {
    const out = new Array(columns.length).fill('')
    for (let i = 0; i < columns.length; i++) {
      out[i] = r[i] ?? ''
    }
    return out
  })
  return {columns, rows}
})

const exportCsv = () => downloadCsv(parsed.value.columns, parsed.value.rows, `data-${Date.now()}.csv`)

// 数据为空时回退表格视图
watch(() => parsed.value.columns.length, (n) => {
  if (!n && viewMode.value === 'chart') {
    viewMode.value = 'table'
  }
})
</script>
