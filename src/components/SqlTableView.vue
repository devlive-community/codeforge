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
      <div class="flex items-center gap-1">
        <!-- 分页控件（单条 SELECT 分页拉取时显示）-->
        <div v-if="paging?.active" class="flex items-center gap-1 mr-1 text-xs text-gray-500 dark:text-gray-400">
          <button class="p-1 rounded hover:bg-gray-100 dark:hover:bg-gray-700 disabled:opacity-30 disabled:cursor-not-allowed cursor-pointer"
                  :disabled="paging.offset === 0 || isRunning" title="上一页" @click="emit('prev')">
            <ChevronLeft class="w-3.5 h-3.5"/>
          </button>
          <span>第 {{ pageNo }} 页 · 每页 {{ paging.pageSize }}</span>
          <button class="p-1 rounded hover:bg-gray-100 dark:hover:bg-gray-700 disabled:opacity-30 disabled:cursor-not-allowed cursor-pointer"
                  :disabled="!paging.hasMore || isRunning" title="下一页" @click="emit('next')">
            <ChevronRight class="w-3.5 h-3.5"/>
          </button>
        </div>
        <!-- 表格 / 图表 切换 -->
        <div class="flex items-center rounded-md border border-gray-200 dark:border-gray-700 overflow-hidden">
          <button class="p-1 transition-colors" :class="viewMode === 'table' ? 'bg-blue-500 text-white' : 'text-gray-400 hover:text-gray-600 dark:hover:text-gray-300'" title="表格" @click="viewMode = 'table'">
            <Table2 class="w-3.5 h-3.5"/>
          </button>
          <button class="p-1 transition-colors" :class="viewMode === 'chart' ? 'bg-blue-500 text-white' : 'text-gray-400 hover:text-gray-600 dark:hover:text-gray-300'"
                  :disabled="!firstResultSet" :title="firstResultSet ? '图表' : '无数据可绘图'" @click="viewMode = 'chart'">
            <BarChart3 class="w-3.5 h-3.5" :class="!firstResultSet ? 'opacity-40' : ''"/>
          </button>
        </div>
        <button v-if="firstResultSet" class="p-1 rounded text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" title="导出 CSV" @click="exportCsv">
          <FileDown class="w-3.5 h-3.5"/>
        </button>
        <button class="p-1 rounded text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" title="清空" @click="emit('clear')">
          <Trash2 class="w-3.5 h-3.5"/>
        </button>
      </div>
    </div>

    <!-- 图表视图 -->
    <div v-if="viewMode === 'chart' && firstResultSet" class="flex-1 min-h-0 overflow-hidden">
      <ChartPanel :columns="firstResultSet.columns" :rows="firstResultSet.rows"/>
    </div>
    <!-- 表格视图 -->
    <div v-else class="flex-1 overflow-auto p-2">
      <SqlResultTable :output="stable" :empty-text="`运行后在此查看 SQL 结果（数据源：${activeLabel()}）`"/>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, ref, watch} from 'vue'
import {debounce} from 'lodash-es'
import {BarChart3, ChevronLeft, ChevronRight, Database, FileDown, Table2, Trash2} from 'lucide-vue-next'
import {useDbConnections} from '../composables/useDbConnections'
import SqlSourceSelect from './SqlSourceSelect.vue'
import SqlResultTable from './SqlResultTable.vue'
import ChartPanel from './charts/ChartPanel.vue'
import {downloadCsv} from '../utils/csv'

const props = defineProps<{
  output: string
  isRunning: boolean
  executionTime?: number
  paging?: { active: boolean; offset: number; pageSize: number; hasMore: boolean }
}>()
const emit = defineEmits<{ clear: []; prev: []; next: [] }>()

const viewMode = ref<'table' | 'chart'>('table')

// 当前页码（1 基）
const pageNo = computed(() => (props.paging ? Math.floor(props.paging.offset / props.paging.pageSize) + 1 : 1))

// 流式输出防抖
const stable = ref(props.output)
const applyOutput = debounce((v: string) => { stable.value = v }, 100)
watch(() => props.output, (v) => applyOutput(v))

// 解析首个有数据的结果集，供图表绘制
const firstResultSet = computed<{ columns: string[]; rows: any[][] } | null>(() => {
  if (!stable.value.trim()) {
    return null
  }
  try {
    const r = JSON.parse(stable.value)
    const rs = (r.result_sets || []).find((s: any) => s.columns?.length > 0)
    return rs ? {columns: rs.columns, rows: rs.rows} : null
  }
  catch {
    return null
  }
})

// 结果切换后若图表不可用则回退表格
watch(firstResultSet, (rs) => {
  if (!rs && viewMode.value === 'chart') {
    viewMode.value = 'table'
  }
})

const exportCsv = () => {
  const rs = firstResultSet.value
  if (rs) {
    downloadCsv(rs.columns, rs.rows, `sql-result-${Date.now()}.csv`)
  }
}

// 当前数据源名（占位文案用）；选择逻辑由 SqlSourceSelect 组件负责
const {activeLabel} = useDbConnections()
</script>
