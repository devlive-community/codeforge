<template>
  <div class="flex flex-col h-full bg-white dark:bg-gray-900">
    <!-- 头部 -->
    <div class="flex items-center justify-between px-3 py-1.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
      <div class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400 min-w-0">
        <Sheet class="w-3.5 h-3.5 flex-shrink-0"/>
        <span>Excel</span>
        <span v-if="isRunning || loading" class="text-blue-500">读取中…</span>
        <span v-else-if="error" class="text-red-500 truncate">{{ error }}</span>
        <span v-else-if="table.rows.length" class="text-gray-400">{{ table.columns.length }} 列 · {{ table.rows.length }} 行</span>
      </div>
      <div class="flex items-center gap-1">
        <div class="flex items-center rounded-md border border-gray-200 dark:border-gray-700 overflow-hidden">
          <button class="p-1 transition-colors" :class="viewMode === 'table' ? 'bg-blue-500 text-white' : 'text-gray-400 hover:text-gray-600 dark:hover:text-gray-300'" title="表格" @click="viewMode = 'table'">
            <Table2 class="w-3.5 h-3.5"/>
          </button>
          <button class="p-1 transition-colors" :class="viewMode === 'chart' ? 'bg-blue-500 text-white' : 'text-gray-400 hover:text-gray-600 dark:hover:text-gray-300'"
                  :disabled="!table.columns.length" :title="table.columns.length ? '图表' : '无数据可绘图'" @click="viewMode = 'chart'">
            <BarChart3 class="w-3.5 h-3.5" :class="!table.columns.length ? 'opacity-40' : ''"/>
          </button>
        </div>
        <button v-if="table.columns.length" class="p-1 rounded text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" title="导出 CSV" @click="exportCsv">
          <FileDown class="w-3.5 h-3.5"/>
        </button>
        <button class="p-1 rounded text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" title="清空" @click="emit('clear')">
          <Trash2 class="w-3.5 h-3.5"/>
        </button>
      </div>
    </div>

    <!-- 工作表标签 -->
    <div v-if="sheets.length > 1" class="flex items-center gap-1 px-2 py-1 border-b border-gray-200 dark:border-gray-700 overflow-x-auto flex-shrink-0">
      <button v-for="s in sheets" :key="s" class="px-2 py-0.5 rounded text-xs whitespace-nowrap cursor-pointer"
              :class="s === activeSheet ? 'bg-blue-500 text-white' : 'text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-700'"
              @click="activeSheet = s">{{ s }}</button>
    </div>

    <!-- 图表 -->
    <div v-if="viewMode === 'chart' && table.columns.length" class="flex-1 min-h-0 overflow-hidden">
      <ChartPanel :columns="table.columns" :rows="table.rows"/>
    </div>
    <!-- 表格（虚拟滚动） -->
    <div v-else-if="!table.columns.length" class="flex-1 overflow-auto p-2 text-xs">
      <div class="text-gray-400 px-2 py-4 text-center">{{ error ? error : '运行 .xlsx / .xls 后在此查看数据' }}</div>
    </div>
    <VirtualTable v-else class="flex-1" :columns="table.columns" :rows="table.rows"/>
  </div>
</template>

<script setup lang="ts">
import {ref, shallowRef, watch} from 'vue'
import {BarChart3, FileDown, Sheet, Table2, Trash2} from 'lucide-vue-next'
import type * as XLSXType from 'xlsx'
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
const loading = ref(false)
const error = ref('')
const sheets = ref<string[]>([])
const activeSheet = ref('')
const workbook = shallowRef<XLSXType.WorkBook | null>(null)
const table = shallowRef<{ columns: string[]; rows: any[][] }>({columns: [], rows: []})

// 动态加载 xlsx（体积大，按需引入）
let XLSX: typeof XLSXType | null = null
const ensureXlsx = async () => {
  if (!XLSX) {
    XLSX = await import('xlsx')
  }
  return XLSX
}

const fmtCell = (v: any) => {
  if (v === null || v === undefined) {
    return ''
  }
  if (v instanceof Date) {
    const p = (n: number) => String(n).padStart(2, '0')
    return `${v.getFullYear()}-${p(v.getMonth() + 1)}-${p(v.getDate())} ${p(v.getHours())}:${p(v.getMinutes())}:${p(v.getSeconds())}`.replace(' 00:00:00', '')
  }
  return v
}

const buildTable = (name: string) => {
  const wb = workbook.value
  if (!wb || !XLSX || !wb.Sheets[name]) {
    table.value = {columns: [], rows: []}
    return
  }
  const aoa = XLSX.utils.sheet_to_json<any[]>(wb.Sheets[name], {header: 1, defval: null, blankrows: false})
  if (aoa.length === 0) {
    table.value = {columns: [], rows: []}
    return
  }
  const columns = (aoa[0] || []).map((c: any, i: number) => (c === null || c === '' ? `列${i + 1}` : String(c)))
  const rows = aoa.slice(1).map(r => columns.map((_c, i) => fmtCell(r[i])))
  table.value = {columns, rows}
}

const loadFile = async (path: string) => {
  const p = path.trim()
  if (!p) {
    workbook.value = null
    sheets.value = []
    table.value = {columns: [], rows: []}
    return
  }
  loading.value = true
  error.value = ''
  try {
    const lib = await ensureXlsx()
    const {readFile} = await import('@tauri-apps/plugin-fs')
    const bytes = await readFile(p)
    const wb = lib.read(bytes, {type: 'array', cellDates: true})
    workbook.value = wb
    sheets.value = wb.SheetNames
    activeSheet.value = wb.SheetNames[0] || ''
    buildTable(activeSheet.value)
  }
  catch (e: any) {
    error.value = '读取失败：' + String(e?.message || e)
    workbook.value = null
    sheets.value = []
    table.value = {columns: [], rows: []}
  }
  finally {
    loading.value = false
  }
}

watch(() => props.output, (v) => loadFile(v), {immediate: true})
watch(activeSheet, (s) => {
  if (s) {
    buildTable(s)
  }
})
watch(() => table.value.columns.length, (n) => {
  if (!n && viewMode.value === 'chart') {
    viewMode.value = 'table'
  }
})

const exportCsv = () => downloadCsv(table.value.columns, table.value.rows, `${activeSheet.value || 'sheet'}-${Date.now()}.csv`)
</script>
