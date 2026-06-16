<template>
  <div class="flex h-full min-h-0 overflow-hidden">
    <!-- 配置侧栏：字段区独立滚动，行/列/值放置区固定在下方始终可见 -->
    <div class="w-52 flex-shrink-0 border-r border-gray-200 dark:border-gray-700 flex flex-col min-h-0 bg-gray-50 dark:bg-gray-800/40 text-xs">
      <div class="flex-1 min-h-0 overflow-y-auto px-3 py-2 border-b border-gray-200 dark:border-gray-700">
        <div class="text-[11px] text-gray-400 mb-1.5">{{ t('sql.fields') }}</div>
        <div class="flex flex-wrap gap-1.5">
          <div v-for="f in columns" :key="f" draggable="true"
               class="inline-flex items-center gap-1 px-2 py-1 rounded border bg-white dark:bg-gray-800 border-gray-200 dark:border-gray-700 hover:border-blue-400 cursor-grab active:cursor-grabbing select-none"
               @dragstart="dragField = f">
            <component :is="numericCols.has(f) ? Hash : Type" class="w-3 h-3" :class="numericCols.has(f) ? 'text-emerald-500' : 'text-amber-500'"/>
            {{ f }}
          </div>
        </div>
      </div>

      <div class="flex-shrink-0 overflow-y-auto max-h-[55%]">
        <div v-for="zone in zones" :key="zone.key" class="px-3 py-2 border-b border-gray-200 dark:border-gray-700"
             @dragover.prevent="dragOver = zone.key" @dragleave="dragOver = ''" @drop.prevent="onDrop(zone.key)">
        <div class="text-[11px] text-gray-400 mb-1.5 flex items-center justify-between">
          <span>{{ zone.label }}</span>
          <Select v-if="zone.key === 'val'" v-model="agg" :options="aggOptions" :button-classes="['!py-0.5', '!px-1.5', 'text-[11px]', '!rounded']" class="w-20"/>
        </div>
        <div class="min-h-[28px] rounded border border-dashed p-1 flex flex-wrap gap-1 transition-colors"
             :class="dragOver === zone.key ? 'border-blue-400 bg-blue-50/50 dark:bg-blue-900/20' : 'border-gray-300 dark:border-gray-600'">
          <span v-for="f in zone.list.value" :key="f" class="inline-flex items-center gap-1 px-2 py-0.5 rounded"
                :class="zone.cls">
            {{ f }}
            <X class="w-3 h-3 cursor-pointer hover:text-red-500" @click="removeFrom(zone.key, f)"/>
          </span>
          <span v-if="!zone.list.value.length" class="text-[11px] text-gray-400 px-1 py-0.5">{{ zone.hint }}</span>
        </div>
        </div>
      </div>
    </div>

    <!-- 透视表 -->
    <div class="flex-1 min-h-0 overflow-auto p-2">
      <table v-if="pivot.colKeys.length" class="text-xs border-collapse">
        <thead>
          <tr>
            <th class="sticky left-0 z-10 bg-gray-100 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 px-2 py-1 text-left text-gray-500 dark:text-gray-400 font-medium whitespace-nowrap">
              {{ rowFields.length ? rowFields.join(' / ') : '' }}
            </th>
            <th v-for="ck in pivot.colKeys" :key="ck" class="bg-gray-100 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 px-2 py-1 text-right text-gray-600 dark:text-gray-300 font-medium whitespace-nowrap">
              {{ ck }}
            </th>
            <th v-if="pivot.colKeys.length > 1" class="bg-gray-100 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 px-2 py-1 text-right text-gray-500 dark:text-gray-400 font-semibold whitespace-nowrap">{{ t('sql.total') }}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="rk in pivot.rowKeys" :key="rk" class="hover:bg-gray-50 dark:hover:bg-gray-800/60">
            <td class="sticky left-0 z-10 bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-700 px-2 py-1 text-gray-700 dark:text-gray-200 whitespace-nowrap">{{ rk }}</td>
            <td v-for="ck in pivot.colKeys" :key="ck" class="border border-gray-200 dark:border-gray-700 px-2 py-1 text-right text-gray-700 dark:text-gray-200 whitespace-nowrap tabular-nums">{{ fmt(pivot.cell(rk, ck)) }}</td>
            <td v-if="pivot.colKeys.length > 1" class="border border-gray-200 dark:border-gray-700 px-2 py-1 text-right font-semibold text-gray-700 dark:text-gray-200 whitespace-nowrap tabular-nums">{{ fmt(pivot.rowTotal(rk)) }}</td>
          </tr>
          <tr v-if="pivot.rowKeys.length > 1" class="bg-gray-50 dark:bg-gray-800/60 font-semibold">
            <td class="sticky left-0 z-10 bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 px-2 py-1 text-gray-500 dark:text-gray-400">{{ t('sql.total') }}</td>
            <td v-for="ck in pivot.colKeys" :key="ck" class="border border-gray-200 dark:border-gray-700 px-2 py-1 text-right text-gray-700 dark:text-gray-200 tabular-nums">{{ fmt(pivot.colTotal(ck)) }}</td>
            <td v-if="pivot.colKeys.length > 1" class="border border-gray-200 dark:border-gray-700 px-2 py-1 text-right text-gray-700 dark:text-gray-200 tabular-nums">{{ fmt(pivot.grandTotal()) }}</td>
          </tr>
        </tbody>
      </table>
      <div v-else class="h-full flex items-center justify-center text-gray-400 text-xs">
        {{ t('sql.pivotHint') }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, ref} from 'vue'
import {useI18n} from 'vue-i18n'
import {Hash, Type, X} from 'lucide-vue-next'
import Select from '../ui/Select.vue'

const {t} = useI18n()

const props = defineProps<{
  columns: string[]
  rows: any[][]
}>()

const rowFields = ref<string[]>([])
const colFields = ref<string[]>([])
const valFields = ref<string[]>([])
const agg = ref<'sum' | 'count' | 'avg' | 'min' | 'max'>('sum')
const dragField = ref('')
const dragOver = ref('')

const aggOptions = computed(() => [
  {value: 'sum', label: t('sql.sum')},
  {value: 'count', label: t('sql.count')},
  {value: 'avg', label: t('sql.avg')},
  {value: 'min', label: t('sql.min')},
  {value: 'max', label: t('sql.max')}
])

const zones = computed(() => [
  {key: 'row' as const, label: t('sql.zoneRow'), hint: t('sql.hintRow'), list: rowFields, cls: 'bg-blue-100 dark:bg-blue-900/40 text-blue-700 dark:text-blue-300'},
  {key: 'col' as const, label: t('sql.zoneCol'), hint: t('sql.hintCol'), list: colFields, cls: 'bg-indigo-100 dark:bg-indigo-900/40 text-indigo-700 dark:text-indigo-300'},
  {key: 'val' as const, label: t('sql.zoneVal'), hint: t('sql.hintVal'), list: valFields, cls: 'bg-emerald-100 dark:bg-emerald-900/40 text-emerald-700 dark:text-emerald-300'}
])

// 数值列判定：该列存在可解析为数字的非空值
const numericCols = computed(() => {
  const set = new Set<string>()
  props.columns.forEach((c, i) => {
    if (props.rows.some(r => r[i] !== null && r[i] !== '' && !isNaN(Number(r[i])))) {
      set.add(c)
    }
  })
  return set
})

const listOf = (key: string) => (key === 'row' ? rowFields : key === 'col' ? colFields : valFields)
const removeFrom = (key: string, f: string) => {
  const l = listOf(key)
  l.value = l.value.filter(x => x !== f)
}
const onDrop = (key: string) => {
  const f = dragField.value
  dragOver.value = ''
  if (!f) {
    return
  }
  // 同一字段在各区唯一
  rowFields.value = rowFields.value.filter(x => x !== f)
  colFields.value = colFields.value.filter(x => x !== f)
  valFields.value = valFields.value.filter(x => x !== f)
  if (key === 'val') {
    valFields.value = [f] // 值区单字段
  }
  else {
    listOf(key).value = [...listOf(key).value, f]
  }
  dragField.value = ''
}

const idx = (name: string) => props.columns.indexOf(name)
const keyOf = (row: any[], fields: string[]) => fields.map(f => String(row[idx(f)] ?? '')).join(' / ')

const aggregate = (vals: number[]): number | null => {
  if (agg.value === 'count') {
    return vals.length
  }
  if (!vals.length) {
    return null
  }
  switch (agg.value) {
    case 'sum': return vals.reduce((a, b) => a + b, 0)
    case 'avg': return vals.reduce((a, b) => a + b, 0) / vals.length
    case 'min': return Math.min(...vals)
    case 'max': return Math.max(...vals)
    default: return null
  }
}

const pivot = computed(() => {
  const valField = valFields.value[0]
  if (!valField) {
    return {rowKeys: [] as string[], colKeys: [] as string[], cell: () => null, rowTotal: () => null, colTotal: () => null, grandTotal: () => null}
  }
  const vIdx = idx(valField)
  const rowKeySet = new Set<string>()
  const colKeySet = new Set<string>()
  // 收集每个 (行键,列键) 的数值
  const bucket = new Map<string, number[]>()
  const cellKey = (rk: string, ck: string) => rk + ' ' + ck
  for (const row of props.rows) {
    const rk = rowFields.value.length ? keyOf(row, rowFields.value) : t('sql.all')
    const ck = colFields.value.length ? keyOf(row, colFields.value) : (valField)
    rowKeySet.add(rk)
    colKeySet.add(ck)
    const raw = row[vIdx]
    const num = Number(raw)
    const v = raw === null || raw === '' || isNaN(num) ? null : num
    const k = cellKey(rk, ck)
    if (!bucket.has(k)) {
      bucket.set(k, [])
    }
    // count 统计行数；其余仅计入可解析数值
    if (agg.value === 'count') {
      bucket.get(k)!.push(1)
    }
    else if (v !== null) {
      bucket.get(k)!.push(v)
    }
  }
  const rowKeys = [...rowKeySet].sort()
  const colKeys = [...colKeySet].sort()
  const cell = (rk: string, ck: string) => aggregate(bucket.get(cellKey(rk, ck)) || [])
  const collect = (pred: (rk: string, ck: string) => boolean) => {
    const all: number[] = []
    for (const rk of rowKeys) {
      for (const ck of colKeys) {
        if (pred(rk, ck)) {
          all.push(...(bucket.get(cellKey(rk, ck)) || []))
        }
      }
    }
    return aggregate(all)
  }
  return {
    rowKeys,
    colKeys,
    cell,
    rowTotal: (rk: string) => collect(r => r === rk),
    colTotal: (ck: string) => collect((_r, c) => c === ck),
    grandTotal: () => collect(() => true)
  }
})

const fmt = (v: number | null) => {
  if (v === null || v === undefined) {
    return ''
  }
  return Number.isInteger(v) ? String(v) : v.toFixed(2)
}
</script>
