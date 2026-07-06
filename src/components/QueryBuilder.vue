<template>
  <div class="inline-block">
    <Tooltip :text="t('qb.title')">
      <button class="inline-flex items-center gap-1 px-2 py-0.5 rounded text-xs text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="openBuilder">
        <Blocks class="w-3.5 h-3.5"/>
      </button>
    </Tooltip>

    <Teleport to="body">
      <div v-if="visible" class="fixed inset-0 z-50 flex items-start justify-center pt-10 px-6 pb-6" @click="visible = false">
        <div class="w-full max-w-[960px] h-[82vh] bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 flex flex-col overflow-hidden" @click.stop>
          <!-- 标题栏 -->
          <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
            <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
              <Blocks class="w-4 h-4 text-gray-400"/>
              <span>{{ t('qb.title') }} · {{ activeLabel() }}</span>
              <span class="text-[11px] text-gray-400 font-normal">{{ t('qb.dragHint') }}</span>
            </div>
            <div class="flex items-center gap-2">
              <button class="inline-flex items-center gap-1 text-xs text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="reset">
                <RotateCcw class="w-3.5 h-3.5"/>{{ t('qb.reset') }}
              </button>
              <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="visible = false">
                <X class="w-4 h-4"/>
              </button>
            </div>
          </div>

          <div v-if="loading" class="flex-1 flex items-center justify-center text-sm text-gray-400">{{ t('qb.loading') }}</div>
          <div v-else-if="error" class="flex-1 flex items-center justify-center text-sm text-red-500 px-6 text-center">{{ error }}</div>
          <div v-else-if="tables.length === 0" class="flex-1 flex items-center justify-center text-sm text-gray-400 px-6 text-center">{{ t('qb.noTables') }}</div>

          <div v-else class="flex-1 flex min-h-0">
            <!-- 左：结构面板（表 + 可拖拽的列） -->
            <div class="w-56 flex-shrink-0 border-r border-gray-200 dark:border-gray-700 flex flex-col min-h-0">
              <div class="px-3 py-1.5 text-[11px] uppercase tracking-wide text-gray-400 flex-shrink-0">{{ t('qb.tables') }}</div>
              <div class="overflow-auto flex-shrink-0 max-h-40 border-b border-gray-100 dark:border-gray-800">
                <button v-for="tb in tables" :key="tb.name"
                        class="w-full text-left px-3 py-1 text-xs flex items-center gap-1.5 cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-800"
                        :class="tb.name === table ? 'bg-blue-50 dark:bg-blue-900/30 text-blue-600 dark:text-blue-300' : 'text-gray-600 dark:text-gray-300'"
                        @click="table = tb.name">
                  <Table2 class="w-3.5 h-3.5 flex-shrink-0"/>
                  <span class="truncate">{{ tb.name }}</span>
                  <span class="ml-auto text-[10px] text-gray-400">{{ tb.columns.length }}</span>
                </button>
              </div>
              <div class="px-3 py-1.5 text-[11px] uppercase tracking-wide text-gray-400 flex-shrink-0">{{ t('qb.columns') }}</div>
              <div class="overflow-auto flex-1 px-2 pb-2 flex flex-col gap-1">
                <div v-for="c in currentCols" :key="c.name"
                     draggable="true"
                     class="flex items-center gap-1.5 px-2 py-1 rounded border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 text-xs cursor-grab active:cursor-grabbing hover:border-blue-400"
                     @dragstart="onDragStart(c.name)" @dragend="onDragEnd">
                  <GripVertical class="w-3 h-3 text-gray-300 flex-shrink-0"/>
                  <span class="truncate">{{ c.name }}</span>
                  <span class="ml-auto text-[10px] text-gray-400 flex-shrink-0">{{ c.type }}</span>
                </div>
              </div>
            </div>

            <!-- 右：落区 + SQL 预览 -->
            <div class="flex-1 min-w-0 overflow-auto p-3 flex flex-col gap-3">
              <!-- SELECT 列 -->
              <div class="rounded border transition-colors" :class="zoneClass('select')"
                   @dragover.prevent="dropHover = 'select'" @dragleave="dropHover = ''" @drop="onDropSelect">
                <div class="px-2.5 py-1 text-[11px] font-medium text-gray-500 flex items-center gap-1">SELECT</div>
                <div class="px-2.5 pb-2 flex flex-wrap gap-1.5 min-h-[28px] items-center">
                  <span v-if="selectedCols.length === 0 && !isDragging" class="text-xs text-gray-400 italic">{{ t('qb.selectEmpty') }}</span>
                  <span v-for="(col, i) in selectedCols" :key="col" class="inline-flex items-center gap-1 px-2 py-0.5 rounded bg-blue-50 dark:bg-blue-900/30 text-blue-600 dark:text-blue-300 text-xs">
                    {{ col }}
                    <button class="hover:text-red-500 cursor-pointer" @click="selectedCols.splice(i, 1)"><X class="w-3 h-3"/></button>
                  </span>
                  <span v-if="isDragging" class="border-2 border-dashed rounded px-2 py-0.5 text-xs pointer-events-none transition-colors"
                        :class="dropHover === 'select' ? 'border-blue-400 text-blue-500 bg-blue-50 dark:bg-blue-900/20' : 'border-gray-300 dark:border-gray-600 text-gray-400'">
                    {{ t('qb.dropHere', { col: draggedCol }) }}
                  </span>
                </div>
              </div>

              <!-- WHERE 条件 -->
              <div class="rounded border transition-colors" :class="zoneClass('where')"
                   @dragover.prevent="dropHover = 'where'" @dragleave="dropHover = ''" @drop="onDropWhere">
                <div class="px-2.5 py-1 text-[11px] font-medium text-gray-500">WHERE</div>
                <div class="px-2.5 pb-2 flex flex-col gap-1.5 min-h-[28px]">
                  <span v-if="wheres.length === 0 && !isDragging" class="text-xs text-gray-400 italic">{{ t('qb.whereEmpty') }}</span>
                  <div v-if="isDragging" class="border-2 border-dashed rounded px-2 py-1 text-xs text-center pointer-events-none transition-colors"
                       :class="dropHover === 'where' ? 'border-blue-400 text-blue-500 bg-blue-50 dark:bg-blue-900/20' : 'border-gray-300 dark:border-gray-600 text-gray-400'">
                    {{ t('qb.dropHere', { col: draggedCol }) }}
                  </div>
                  <div v-for="(w, i) in wheres" :key="i" class="flex items-center gap-1.5">
                    <span class="px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-800 text-xs font-mono flex-shrink-0">{{ w.col }}</span>
                    <select v-model="w.op" class="text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-1.5 py-1 focus:outline-none flex-shrink-0">
                      <option v-for="op in opsFor(w.col)" :key="op" :value="op">{{ opLabel(op) }}</option>
                    </select>
                    <!-- 值编辑区：随运算符/类型自适应 -->
                    <template v-if="w.op !== 'IS NULL' && w.op !== 'IS NOT NULL'">
                      <template v-if="w.op === 'BETWEEN'">
                        <input v-model="w.value" :type="inputType(w.col)" :placeholder="t('qb.value')"
                               class="w-28 min-w-0 text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-2 py-1 focus:outline-none"/>
                        <span class="text-xs text-gray-400">AND</span>
                        <input v-model="w.value2" :type="inputType(w.col)" :placeholder="t('qb.value')"
                               class="w-28 min-w-0 text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-2 py-1 focus:outline-none"/>
                      </template>
                      <select v-else-if="colType(w.col) === 'boolean'" v-model="w.value"
                              class="text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-1.5 py-1 focus:outline-none">
                        <option value="true">TRUE</option>
                        <option value="false">FALSE</option>
                      </select>
                      <input v-else v-model="w.value" :type="w.op === 'IN' ? 'text' : inputType(w.col)"
                             :placeholder="w.op === 'IN' ? t('qb.inPlaceholder') : t('qb.value')"
                             class="flex-1 min-w-0 text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-2 py-1 focus:outline-none"/>
                    </template>
                    <button class="text-gray-400 hover:text-red-500 cursor-pointer p-1 ml-auto flex-shrink-0" @click="wheres.splice(i, 1)"><Trash2 class="w-3.5 h-3.5"/></button>
                  </div>
                </div>
              </div>

              <!-- ORDER BY -->
              <div class="rounded border transition-colors" :class="zoneClass('order')"
                   @dragover.prevent="dropHover = 'order'" @dragleave="dropHover = ''" @drop="onDropOrder">
                <div class="px-2.5 py-1 text-[11px] font-medium text-gray-500">ORDER BY</div>
                <div class="px-2.5 pb-2 flex flex-wrap gap-1.5 min-h-[28px] items-center">
                  <span v-if="orders.length === 0 && !isDragging" class="text-xs text-gray-400 italic">{{ t('qb.orderEmpty') }}</span>
                  <span v-if="isDragging" class="border-2 border-dashed rounded px-2 py-0.5 text-xs pointer-events-none transition-colors"
                        :class="dropHover === 'order' ? 'border-blue-400 text-blue-500 bg-blue-50 dark:bg-blue-900/20' : 'border-gray-300 dark:border-gray-600 text-gray-400'">
                    {{ t('qb.dropHere', { col: draggedCol }) }}
                  </span>
                  <span v-for="(o, i) in orders" :key="o.col" class="inline-flex items-center gap-1 px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-800 text-xs">
                    {{ o.col }}
                    <button class="text-blue-500 hover:underline cursor-pointer font-mono" @click="o.dir = o.dir === 'ASC' ? 'DESC' : 'ASC'">{{ o.dir }}</button>
                    <button class="hover:text-red-500 cursor-pointer" @click="orders.splice(i, 1)"><X class="w-3 h-3"/></button>
                  </span>
                </div>
              </div>

              <!-- LIMIT -->
              <div class="flex items-center gap-2">
                <label class="text-[11px] font-medium text-gray-500">LIMIT</label>
                <input v-model.number="limit" type="number" min="0" class="w-24 text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-2 py-1 focus:outline-none"/>
              </div>

              <!-- SQL 预览 -->
              <div class="mt-auto">
                <div class="text-[11px] font-medium text-gray-500 mb-1">{{ t('qb.preview') }}</div>
                <pre class="text-xs font-mono bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded p-2 whitespace-pre-wrap break-all">{{ sql }}</pre>
              </div>
            </div>
          </div>

          <div class="flex items-center justify-end gap-2 px-4 py-2.5 border-t border-gray-200 dark:border-gray-700 flex-shrink-0">
            <button class="text-xs px-3 py-1.5 rounded text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer disabled:opacity-40" @click="copySql" :disabled="!sql">{{ t('qb.copy') }}</button>
            <button class="text-xs px-3 py-1.5 rounded text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer disabled:opacity-40" @click="doInsert" :disabled="!sql">{{ t('qb.insert') }}</button>
            <button class="text-xs px-3 py-1.5 rounded bg-blue-500 text-white hover:bg-blue-600 disabled:opacity-40 disabled:cursor-not-allowed cursor-pointer" @click="doRun" :disabled="!sql">{{ t('qb.run') }}</button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import {computed, nextTick, ref, watch} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {Blocks, GripVertical, RotateCcw, Table2, Trash2, X} from 'lucide-vue-next'
import {useI18n} from 'vue-i18n'
import Tooltip from '../ui/Tooltip.vue'
import {useDbConnections} from '../composables/useDbConnections'
import {kvGetJSON, kvSetJSON} from '../composables/useKvStore'
import {useToast} from '../plugins/toast'
import {columnsSql, groupTables, quoteIdent, type Tbl} from '../utils/dbSchema'

const emit = defineEmits<{ preview: [sql: string]; insert: [sql: string] }>()
const {t} = useI18n()
const toast = useToast()
const {resolveActiveSource, activeLabel, activeRef} = useDbConnections()

type TypeCat = 'number' | 'string' | 'date' | 'boolean' | 'other'

// 各类型可用的运算符（contains/starts/ends 为 LIKE 语义预设）
const OPS_BY_CAT: Record<TypeCat, string[]> = {
  number: ['=', '!=', '>', '>=', '<', '<=', 'BETWEEN', 'IN', 'IS NULL', 'IS NOT NULL'],
  string: ['contains', 'starts', 'ends', '=', '!=', 'LIKE', 'NOT LIKE', 'IN', 'IS NULL', 'IS NOT NULL'],
  date: ['=', '!=', '>', '>=', '<', '<=', 'BETWEEN', 'IS NULL', 'IS NOT NULL'],
  boolean: ['=', 'IS NULL', 'IS NOT NULL'],
  other: ['=', '!=', 'IS NULL', 'IS NOT NULL']
}

// 由列的声明类型归类
const typeCategory = (type: string): TypeCat => {
  const s = (type || '').toLowerCase()
  if (/bool|\bbit\b|tinyint\(1\)/.test(s)) {
    return 'boolean'
  }
  if (/int|dec|numeric|real|double|float|serial|money|number/.test(s)) {
    return 'number'
  }
  if (/date|time|timestamp|year/.test(s)) {
    return 'date'
  }
  if (/char|text|clob|string|uuid|enum|json/.test(s)) {
    return 'string'
  }
  return 'other'
}

const visible = ref(false)
const loading = ref(false)
const error = ref('')
const tables = ref<Tbl[]>([])

const table = ref('')
const selectedCols = ref<string[]>([])
const wheres = ref<{ col: string; op: string; value: string; value2?: string }[]>([])
const orders = ref<{ col: string; dir: 'ASC' | 'DESC' }[]>([])
const limit = ref(100)

// 按列名取类型分类 / 该列可用运算符 / 输入框类型
const colType = (col: string): TypeCat => typeCategory(currentCols.value.find(c => c.name === col)?.type ?? '')
const opsFor = (col: string) => OPS_BY_CAT[colType(col)]
const inputType = (col: string) => (colType(col) === 'date' ? 'date' : 'text')
const opLabel = (op: string) =>
  op === 'contains' ? t('qb.opContains') : op === 'starts' ? t('qb.opStarts') : op === 'ends' ? t('qb.opEnds') : op
const defaultOp = (col: string) => (colType(col) === 'string' ? 'contains' : '=')

// 当前被拖拽的列名 + 悬停中的落区（用于高亮）+ 是否拖拽中（显示放置提示块）
const draggedCol = ref('')
const dropHover = ref('')
const isDragging = ref(false)

const currentCols = computed(() => tables.value.find(tb => tb.name === table.value)?.columns ?? [])

// 切表时清空所有落区，避免残留其它表的字段（恢复状态期间不触发）
let restoring = false
watch(table, () => {
  if (restoring) {
    return
  }
  selectedCols.value = []
  wheres.value = []
  orders.value = []
})

// ---- 状态记忆：按数据源分别持久化，重开时恢复 ----
interface QbState {
  table: string
  selectedCols: string[]
  wheres: { col: string; op: string; value: string; value2?: string }[]
  orders: { col: string; dir: 'ASC' | 'DESC' }[]
  limit: number
}
const stateKey = () => `qb-state:${activeRef.value}`
const persist = () => {
  if (restoring || !visible.value || !table.value) {
    return
  }
  kvSetJSON(stateKey(), {
    table: table.value,
    selectedCols: selectedCols.value,
    wheres: wheres.value,
    orders: orders.value,
    limit: limit.value
  })
}
watch([table, selectedCols, wheres, orders, limit], persist, {deep: true})

// 从持久化状态恢复（仅保留当前 schema 中仍存在的表/列）
const restoreState = async () => {
  const saved = kvGetJSON<QbState | null>(stateKey(), null)
  if (!saved || !tables.value.some(tb => tb.name === saved.table)) {
    return false
  }
  const cols = new Set((tables.value.find(tb => tb.name === saved.table)?.columns ?? []).map(c => c.name))
  restoring = true
  table.value = saved.table
  selectedCols.value = (saved.selectedCols || []).filter(c => cols.has(c))
  wheres.value = (saved.wheres || []).filter(w => cols.has(w.col))
  orders.value = (saved.orders || []).filter(o => cols.has(o.col))
  limit.value = saved.limit ?? 100
  await nextTick()
  restoring = false
  return true
}

const reset = () => {
  restoring = true
  selectedCols.value = []
  wheres.value = []
  orders.value = []
  limit.value = 100
  nextTick(() => {
    restoring = false
    persist()
  })
}

const zoneClass = (zone: string) =>
  dropHover.value === zone
    ? 'border-blue-400 bg-blue-50/50 dark:bg-blue-900/10'
    : 'border-gray-200 dark:border-gray-700'

const onDragStart = (col: string) => {
  draggedCol.value = col
  isDragging.value = true
  dropHover.value = ''
}
const onDragEnd = () => {
  isDragging.value = false
  dropHover.value = ''
  draggedCol.value = ''
}
const onDropSelect = () => {
  const c = draggedCol.value
  if (c && !selectedCols.value.includes(c)) {
    selectedCols.value.push(c)
  }
  onDragEnd()
}
const onDropWhere = () => {
  const c = draggedCol.value
  if (c) {
    wheres.value.push({col: c, op: defaultOp(c), value: '', value2: ''})
  }
  onDragEnd()
}
const onDropOrder = () => {
  const c = draggedCol.value
  if (c && !orders.value.some(o => o.col === c)) {
    orders.value.push({col: c, dir: 'ASC'})
  }
  onDragEnd()
}

const kind = () => resolveActiveSource().kind
const q = (name: string) => quoteIdent(kind(), name)

const strLit = (s: string) => `'${String(s ?? '').replace(/'/g, "''")}'`
// 按类型生成字面量：布尔→1/0，数字→原样，日期/字符串→带引号转义
const litByCat = (cat: TypeCat, v: string): string => {
  const s = String(v ?? '')
  if (cat === 'boolean') {
    return s === 'true' || s === '1' ? '1' : '0'
  }
  if (cat === 'number' && s.trim() !== '' && !isNaN(Number(s))) {
    return s
  }
  return strLit(s)
}

// 单个 WHERE 条件 → SQL 片段（无效条件返回空串，由上层过滤）
const condSql = (w: { col: string; op: string; value: string; value2?: string }): string => {
  const col = q(w.col)
  const cat = colType(w.col)
  switch (w.op) {
    case 'IS NULL':
    case 'IS NOT NULL':
      return `${col} ${w.op}`
    case 'BETWEEN':
      return (w.value.trim() && (w.value2 ?? '').trim())
        ? `${col} BETWEEN ${litByCat(cat, w.value)} AND ${litByCat(cat, w.value2 || '')}`
        : ''
    case 'IN': {
      const items = w.value.split(',').map(s => s.trim()).filter(Boolean).map(v => litByCat(cat, v))
      return items.length ? `${col} IN (${items.join(', ')})` : ''
    }
    case 'contains':
      return w.value.trim() ? `${col} LIKE ${strLit('%' + w.value + '%')}` : ''
    case 'starts':
      return w.value.trim() ? `${col} LIKE ${strLit(w.value + '%')}` : ''
    case 'ends':
      return w.value.trim() ? `${col} LIKE ${strLit('%' + w.value)}` : ''
    case 'LIKE':
    case 'NOT LIKE':
      return w.value.trim() ? `${col} ${w.op} ${strLit(w.value)}` : ''
    default:
      return w.value.trim() ? `${col} ${w.op} ${litByCat(cat, w.value)}` : ''
  }
}

const sql = computed(() => {
  if (!table.value) {
    return ''
  }
  const cols = selectedCols.value.length === 0 ? '*' : selectedCols.value.map(q).join(', ')
  let out = `SELECT ${cols} FROM ${q(table.value)}`
  const conds = wheres.value.filter(w => w.col).map(condSql).filter(Boolean)
  if (conds.length) {
    out += ' WHERE ' + conds.join(' AND ')
  }
  if (orders.value.length) {
    out += ' ORDER BY ' + orders.value.map(o => `${q(o.col)} ${o.dir}`).join(', ')
  }
  if (limit.value && limit.value > 0) {
    out += ` LIMIT ${limit.value}`
  }
  return out
})

const load = async () => {
  loading.value = true
  error.value = ''
  try {
    const source = resolveActiveSource()
    const db = source.kind === 'mysql' ? source.database || undefined : undefined
    const res = await invoke<any>('run_sql', {sql: columnsSql(source.kind, db), source})
    if (res.error) {
      throw new Error(res.error)
    }
    tables.value = groupTables((res.result_sets || [])[0]?.rows || [])
    // 优先恢复上次内容；无可恢复且当前表无效时回退到第一张表
    const restored = await restoreState()
    if (!restored && tables.value.length && !tables.value.find(tb => tb.name === table.value)) {
      table.value = tables.value[0].name
    }
  }
  catch (e: any) {
    error.value = e?.message || String(e)
    tables.value = []
  }
  finally {
    loading.value = false
  }
}

const openBuilder = () => {
  visible.value = true
  load()
}

const copySql = async () => {
  if (!sql.value) {
    return
  }
  try {
    await navigator.clipboard.writeText(sql.value)
    toast.success(t('qb.copied'))
  }
  catch { /* 忽略 */ }
}
const doInsert = () => {
  if (!sql.value) {
    return
  }
  emit('insert', sql.value)
  visible.value = false
}
const doRun = () => {
  if (!sql.value) {
    return
  }
  emit('preview', sql.value)
  visible.value = false
}
</script>
