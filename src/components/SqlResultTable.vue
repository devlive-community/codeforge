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

      <!-- 结果集表格（虚拟滚动，限高以适配多结果集堆叠） -->
      <div v-for="(rs, ri) in result.result_sets" :key="ri" class="mb-4">
        <div v-if="result.result_sets.length > 1" class="text-[11px] text-gray-400 mb-1">结果集 {{ ri + 1 }} · {{ rs.rows.length }} 行</div>
        <div class="border border-gray-200 dark:border-gray-700 rounded overflow-hidden">
          <VirtualTable :columns="rs.columns" :rows="rs.rows" :show-index="false" :max-height="420"/>
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
import VirtualTable from './VirtualTable.vue'

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
</script>
