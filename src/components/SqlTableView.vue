<template>
  <div class="flex flex-col h-full bg-white dark:bg-gray-900">
    <div class="flex items-center justify-between px-3 py-1.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
      <div class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400 min-w-0">
        <Database class="w-3.5 h-3.5 flex-shrink-0"/>
        <span>SQL 结果</span>
        <span v-if="isRunning" class="text-blue-500">运行中…</span>
        <span v-else-if="result && result.elapsed_ms != null" class="text-gray-400">{{ result.elapsed_ms }} ms</span>
        <!-- 数据库连接 -->
        <button class="flex items-center gap-1 px-1.5 py-0.5 rounded border border-gray-200 dark:border-gray-700 hover:border-blue-400 cursor-pointer truncate max-w-[220px]"
                :title="dbPath || '内存数据库'"
                @click="pickDb">
          <HardDrive class="w-3 h-3 flex-shrink-0"/>
          <span class="truncate">{{ dbLabel }}</span>
        </button>
        <button v-if="dbPath" class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" title="重置为内存数据库" @click="resetDb">
          <X class="w-3 h-3"/>
        </button>
      </div>
      <button class="p-1 rounded text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" title="清空" @click="emit('clear')">
        <Trash2 class="w-3.5 h-3.5"/>
      </button>
    </div>

    <div class="flex-1 overflow-auto p-2 text-xs">
      <div v-if="!stable.trim()" class="text-gray-400 px-2 py-4 text-center">运行后在此查看 SQL 结果（数据库：{{ dbLabel }}）</div>

      <template v-else-if="result">
        <!-- 错误 -->
        <div v-if="result.error" class="mb-3 px-3 py-2 rounded border border-red-300 dark:border-red-800 bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 font-mono whitespace-pre-wrap">
          执行失败：{{ result.error }}
        </div>

        <!-- 非查询语句的消息 -->
        <div v-for="(m, mi) in result.messages" :key="'m' + mi" class="mb-1 text-green-600 dark:text-green-400">✓ {{ m }}</div>

        <!-- 结果集表格 -->
        <div v-for="(rs, ri) in result.result_sets" :key="ri" class="mb-4">
          <div v-if="result.result_sets.length > 1" class="text-[11px] text-gray-400 mb-1">结果集 {{ ri + 1 }} · {{ rs.rows.length }} 行</div>
          <div class="overflow-x-auto border border-gray-200 dark:border-gray-700 rounded">
            <table class="w-full border-collapse">
              <thead>
                <tr class="bg-gray-50 dark:bg-gray-800">
                  <th v-for="(c, ci) in rs.columns" :key="ci" class="text-left font-semibold px-3 py-1.5 border-b border-gray-200 dark:border-gray-700 text-gray-600 dark:text-gray-300 whitespace-nowrap">{{ c }}</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="(row, i) in rs.rows" :key="i" class="hover:bg-gray-50 dark:hover:bg-gray-800/50">
                  <td v-for="(_c, ci) in rs.columns" :key="ci" class="px-3 py-1 border-b border-gray-100 dark:border-gray-800 font-mono whitespace-nowrap"
                      :class="row[ci] === null ? 'text-gray-400 italic' : 'text-gray-700 dark:text-gray-300'">{{ fmt(row[ci]) }}</td>
                </tr>
                <tr v-if="rs.rows.length === 0">
                  <td :colspan="rs.columns.length" class="px-3 py-2 text-center text-gray-400">（0 行）</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <div v-if="!result.error && result.result_sets.length === 0 && result.messages.length === 0" class="text-gray-400 px-2 py-4">执行完成，无输出</div>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, ref, watch} from 'vue'
import {debounce} from 'lodash-es'
import {open} from '@tauri-apps/plugin-dialog'
import {Database, HardDrive, Trash2, X} from 'lucide-vue-next'
import {kvGet, kvSet, kvRemove} from '../composables/useKvStore'

const props = defineProps<{
  output: string
  isRunning: boolean
  executionTime?: number
}>()
const emit = defineEmits<{ clear: [] }>()

interface ResultSet { columns: string[]; rows: any[][] }
interface SqlResult { result_sets: ResultSet[]; messages: string[]; error: string | null; elapsed_ms: number }

// 流式（这里是一次性）输出防抖
const stable = ref(props.output)
const applyOutput = debounce((v: string) => { stable.value = v }, 100)
watch(() => props.output, (v) => applyOutput(v))

const result = computed<SqlResult | null>(() => {
  if (!stable.value.trim()) {
    return null
  }
  try {
    return JSON.parse(stable.value)
  }
  catch {
    return null
  }
})

// ===== 数据库连接（内存 / 选择文件）=====
const dbPath = ref<string | null>(kvGet('sql-db-path'))
const dbLabel = computed(() => {
  if (!dbPath.value) return '内存数据库'
  return dbPath.value.split(/[\\/]/).pop() || dbPath.value
})
const pickDb = async () => {
  const selected = await open({
    multiple: false,
    filters: [{name: 'SQLite', extensions: ['db', 'sqlite', 'sqlite3', 'db3']}]
  })
  if (typeof selected === 'string') {
    dbPath.value = selected
    kvSet('sql-db-path', selected)
  }
}
const resetDb = () => {
  dbPath.value = null
  kvRemove('sql-db-path')
}

const fmt = (v: any) => {
  if (v === null || v === undefined) return 'NULL'
  if (typeof v === 'object') return JSON.stringify(v)
  return String(v)
}
</script>
