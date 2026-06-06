<template>
  <div class="text-xs">
    <div v-if="!output.trim()" class="text-gray-400 px-2 py-4 text-center">{{ emptyText || '无结果' }}</div>

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

    <!-- 非 JSON（解析失败）时退回原文 -->
    <pre v-else class="whitespace-pre-wrap font-mono text-gray-600 dark:text-gray-400">{{ output }}</pre>
  </div>
</template>

<script setup lang="ts">
import {computed} from 'vue'

interface ResultSet { columns: string[]; rows: any[][] }
interface SqlResult { result_sets: ResultSet[]; messages: string[]; error: string | null; elapsed_ms: number }

const props = defineProps<{ output: string; emptyText?: string }>()

const result = computed<SqlResult | null>(() => {
  if (!props.output.trim()) {
    return null
  }
  try {
    return JSON.parse(props.output)
  }
  catch {
    return null
  }
})

const fmt = (v: any) => {
  if (v === null || v === undefined) return 'NULL'
  if (typeof v === 'object') return JSON.stringify(v)
  return String(v)
}
</script>
