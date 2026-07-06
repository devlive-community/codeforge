<template>
  <div class="text-xs" :class="fill ? 'h-full flex flex-col min-h-0' : ''">
    <div v-if="!output.trim()" class="text-gray-400 px-2 py-4 text-center">{{ emptyText || t('sql.noResult') }}</div>

    <template v-else-if="result">
      <!-- 错误 -->
      <div v-if="result.error" class="mb-3 px-3 py-2 rounded border border-red-300 dark:border-red-800 bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 font-mono whitespace-pre-wrap flex-shrink-0">
        {{ t('sql.execFailed') }}{{ result.error }}
      </div>

      <!-- 非查询语句的消息 -->
      <div v-for="(m, mi) in result.messages" :key="'m' + mi" class="mb-1 text-green-600 dark:text-green-400 flex-shrink-0">✓ {{ m }}</div>

      <!-- 结果集表格（虚拟滚动）。fillHeight 且单结果集时铺满面板，否则限高 420 便于多结果集堆叠 -->
      <div v-for="(rs, ri) in result.result_sets" :key="ri" :class="fill ? 'flex-1 min-h-0 flex flex-col' : 'mb-4'">
        <div v-if="result.result_sets.length > 1" class="text-[11px] text-gray-400 mb-1 flex-shrink-0">{{ t('sql.resultSet', { n: ri + 1, rows: rs.rows.length }) }}</div>
        <div class="border border-gray-200 dark:border-gray-700 rounded overflow-hidden" :class="fill ? 'flex-1 min-h-0' : ''">
          <VirtualTable :columns="rs.columns" :rows="rs.rows" :show-index="false" :max-height="fill ? undefined : 420"
                        :editable="editable && fill"
                        @edit-cell="(p) => emit('editCell', { column: rs.columns[p.ci], row: p.row, columns: rs.columns, oldValue: p.oldValue, newValue: p.newValue })"/>
        </div>
      </div>

      <div v-if="!result.error && result.result_sets.length === 0 && result.messages.length === 0" class="text-gray-400 px-2 py-4">{{ t('sql.doneNoOutput') }}</div>
    </template>

    <!-- 非 JSON（解析失败）时退回原文 -->
    <pre v-else class="whitespace-pre-wrap font-mono text-gray-600 dark:text-gray-400">{{ output }}</pre>
  </div>
</template>

<script setup lang="ts">
import {computed} from 'vue'
import {useI18n} from 'vue-i18n'
import VirtualTable from './VirtualTable.vue'

const {t} = useI18n()

interface ResultSet { columns: string[]; rows: any[][] }
interface SqlResult { result_sets: ResultSet[]; messages: string[]; error: string | null; elapsed_ms: number }

const props = defineProps<{ output: string; emptyText?: string; fillHeight?: boolean; editable?: boolean }>()
const emit = defineEmits<{ editCell: [payload: { column: string; row: any[]; columns: string[]; oldValue: any; newValue: any }] }>()

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

// 仅在显式要求铺满高度、且为单结果集（无错误）时铺满；多结果集仍堆叠限高
const fill = computed(() => !!props.fillHeight && !!result.value && !result.value.error && result.value.result_sets.length === 1)
</script>
