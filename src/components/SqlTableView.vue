<template>
  <div class="flex flex-col h-full bg-white dark:bg-gray-900">
    <div class="flex items-center justify-between px-3 py-1.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
      <div class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400">
        <Database class="w-3.5 h-3.5"/>
        <span>SQL 结果</span>
        <span v-if="isRunning" class="text-blue-500">运行中…</span>
        <span v-else-if="executionTime" class="text-gray-400">{{ executionTime }} ms</span>
        <span v-if="resultSets.length" class="text-gray-400">· {{ resultSets.length }} 个结果集</span>
      </div>
      <div class="flex items-center gap-1">
        <button class="p-1 rounded cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700"
                :class="mode === 'raw' ? 'text-blue-500' : 'text-gray-400 hover:text-gray-600 dark:hover:text-gray-300'"
                :title="mode === 'raw' ? '切换为表格' : '查看原始输出'"
                @click="mode = mode === 'raw' ? 'table' : 'raw'">
          <FileText class="w-3.5 h-3.5"/>
        </button>
        <button class="p-1 rounded text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" title="清空" @click="emit('clear')">
          <Trash2 class="w-3.5 h-3.5"/>
        </button>
      </div>
    </div>

    <div class="flex-1 overflow-auto p-2 text-xs">
      <div v-if="!stable.trim()" class="text-gray-400 px-2 py-4 text-center">运行后在此查看 SQL 结果</div>

      <!-- 原始输出 -->
      <pre v-else-if="mode === 'raw'" class="whitespace-pre-wrap font-mono text-gray-700 dark:text-gray-300">{{ stable }}</pre>

      <!-- 表格 -->
      <template v-else-if="resultSets.length">
        <div v-for="(rs, ri) in resultSets" :key="ri" class="mb-4">
          <div v-if="resultSets.length > 1" class="text-[11px] text-gray-400 mb-1">结果集 {{ ri + 1 }} · {{ rs.rows.length }} 行</div>
          <div class="overflow-x-auto border border-gray-200 dark:border-gray-700 rounded">
            <table class="w-full border-collapse">
              <thead>
                <tr class="bg-gray-50 dark:bg-gray-800">
                  <th v-for="c in rs.columns" :key="c" class="text-left font-semibold px-3 py-1.5 border-b border-gray-200 dark:border-gray-700 text-gray-600 dark:text-gray-300 whitespace-nowrap">{{ c }}</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="(row, i) in rs.rows" :key="i" class="hover:bg-gray-50 dark:hover:bg-gray-800/50">
                  <td v-for="c in rs.columns" :key="c" class="px-3 py-1 border-b border-gray-100 dark:border-gray-800 font-mono text-gray-700 dark:text-gray-300 whitespace-nowrap">{{ fmt(row[c]) }}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </template>

      <!-- 无结果集（如 CREATE/INSERT 无输出，或非 JSON 文本） -->
      <div v-else class="text-gray-400 px-2 py-4">执行完成，无结果集</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, ref, watch} from 'vue'
import {debounce} from 'lodash-es'
import {Database, FileText, Trash2} from 'lucide-vue-next'

const props = defineProps<{
  output: string
  isRunning: boolean
  executionTime?: number
}>()
const emit = defineEmits<{ clear: [] }>()

const mode = ref<'table' | 'raw'>('table')

// 流式输出防抖
const stable = ref(props.output)
const applyOutput = debounce((v: string) => { stable.value = v }, 200)
watch(() => props.output, (v) => applyOutput(v))

interface ResultSet { columns: string[]; rows: Record<string, any>[] }

// 从 sqlite3 -json 的输出里提取多个顶层 JSON 数组（多条 SELECT 会输出多个数组）
const resultSets = computed<ResultSet[]>(() => {
  const text = stable.value
  const sets: ResultSet[] = []
  let depth = 0
  let start = -1
  let inStr = false
  let esc = false
  for (let i = 0; i < text.length; i++) {
    const ch = text[i]
    if (inStr) {
      if (esc) esc = false
      else if (ch === '\\') esc = true
      else if (ch === '"') inStr = false
      continue
    }
    if (ch === '"') {
      inStr = true
    }
    else if (ch === '[') {
      if (depth === 0) start = i
      depth++
    }
    else if (ch === ']') {
      depth--
      if (depth === 0 && start >= 0) {
        try {
          const arr = JSON.parse(text.slice(start, i + 1))
          if (Array.isArray(arr) && arr.length && typeof arr[0] === 'object') {
            const columns = Array.from(new Set(arr.flatMap((r: any) => Object.keys(r))))
            sets.push({columns, rows: arr})
          }
        }
        catch {
          // 忽略无法解析的片段
        }
        start = -1
      }
    }
  }
  return sets
})

const fmt = (v: any) => {
  if (v === null || v === undefined) return 'NULL'
  if (typeof v === 'object') return JSON.stringify(v)
  return String(v)
}
</script>
