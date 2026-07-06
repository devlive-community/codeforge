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
            <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="visible = false">
              <X class="w-4 h-4"/>
            </button>
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
                     @dragstart="draggedCol = c.name">
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
                <div class="px-2.5 pb-2 flex flex-wrap gap-1.5 min-h-[28px]">
                  <span v-if="selectedCols.length === 0" class="text-xs text-gray-400 italic">{{ t('qb.selectEmpty') }}</span>
                  <span v-for="(col, i) in selectedCols" :key="col" class="inline-flex items-center gap-1 px-2 py-0.5 rounded bg-blue-50 dark:bg-blue-900/30 text-blue-600 dark:text-blue-300 text-xs">
                    {{ col }}
                    <button class="hover:text-red-500 cursor-pointer" @click="selectedCols.splice(i, 1)"><X class="w-3 h-3"/></button>
                  </span>
                </div>
              </div>

              <!-- WHERE 条件 -->
              <div class="rounded border transition-colors" :class="zoneClass('where')"
                   @dragover.prevent="dropHover = 'where'" @dragleave="dropHover = ''" @drop="onDropWhere">
                <div class="px-2.5 py-1 text-[11px] font-medium text-gray-500">WHERE</div>
                <div class="px-2.5 pb-2 flex flex-col gap-1.5 min-h-[28px]">
                  <span v-if="wheres.length === 0" class="text-xs text-gray-400 italic">{{ t('qb.whereEmpty') }}</span>
                  <div v-for="(w, i) in wheres" :key="i" class="flex items-center gap-1.5">
                    <span class="px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-800 text-xs font-mono">{{ w.col }}</span>
                    <select v-model="w.op" class="text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-1.5 py-1 focus:outline-none">
                      <option v-for="op in OPS" :key="op" :value="op">{{ op }}</option>
                    </select>
                    <input v-model="w.value" :disabled="w.op === 'IS NULL' || w.op === 'IS NOT NULL'"
                           :placeholder="t('qb.value')"
                           class="flex-1 min-w-0 text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-2 py-1 focus:outline-none disabled:opacity-40"/>
                    <button class="text-gray-400 hover:text-red-500 cursor-pointer p-1" @click="wheres.splice(i, 1)"><Trash2 class="w-3.5 h-3.5"/></button>
                  </div>
                </div>
              </div>

              <!-- ORDER BY -->
              <div class="rounded border transition-colors" :class="zoneClass('order')"
                   @dragover.prevent="dropHover = 'order'" @dragleave="dropHover = ''" @drop="onDropOrder">
                <div class="px-2.5 py-1 text-[11px] font-medium text-gray-500">ORDER BY</div>
                <div class="px-2.5 pb-2 flex flex-wrap gap-1.5 min-h-[28px] items-center">
                  <span v-if="orders.length === 0" class="text-xs text-gray-400 italic">{{ t('qb.orderEmpty') }}</span>
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
import {computed, ref, watch} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {Blocks, GripVertical, Table2, Trash2, X} from 'lucide-vue-next'
import {useI18n} from 'vue-i18n'
import Tooltip from '../ui/Tooltip.vue'
import {useDbConnections} from '../composables/useDbConnections'
import {useToast} from '../plugins/toast'
import {columnsSql, groupTables, quoteIdent, type Tbl} from '../utils/dbSchema'

const emit = defineEmits<{ preview: [sql: string]; insert: [sql: string] }>()
const {t} = useI18n()
const toast = useToast()
const {resolveActiveSource, activeLabel} = useDbConnections()

const OPS = ['=', '!=', '>', '>=', '<', '<=', 'LIKE', 'IS NULL', 'IS NOT NULL']

const visible = ref(false)
const loading = ref(false)
const error = ref('')
const tables = ref<Tbl[]>([])

const table = ref('')
const selectedCols = ref<string[]>([])
const wheres = ref<{ col: string; op: string; value: string }[]>([])
const orders = ref<{ col: string; dir: 'ASC' | 'DESC' }[]>([])
const limit = ref(100)

// 当前被拖拽的列名 + 悬停中的落区（用于高亮）
const draggedCol = ref('')
const dropHover = ref('')

const currentCols = computed(() => tables.value.find(tb => tb.name === table.value)?.columns ?? [])

// 切表时清空所有落区，避免残留其它表的字段
watch(table, () => {
  selectedCols.value = []
  wheres.value = []
  orders.value = []
})

const zoneClass = (zone: string) =>
  dropHover.value === zone
    ? 'border-blue-400 bg-blue-50/50 dark:bg-blue-900/10'
    : 'border-gray-200 dark:border-gray-700'

const onDropSelect = () => {
  dropHover.value = ''
  const c = draggedCol.value
  if (c && !selectedCols.value.includes(c)) {
    selectedCols.value.push(c)
  }
  draggedCol.value = ''
}
const onDropWhere = () => {
  dropHover.value = ''
  if (draggedCol.value) {
    wheres.value.push({col: draggedCol.value, op: '=', value: ''})
  }
  draggedCol.value = ''
}
const onDropOrder = () => {
  dropHover.value = ''
  const c = draggedCol.value
  if (c && !orders.value.some(o => o.col === c)) {
    orders.value.push({col: c, dir: 'ASC'})
  }
  draggedCol.value = ''
}

const kind = () => resolveActiveSource().kind
const q = (name: string) => quoteIdent(kind(), name)

// 值字面量：LIKE 恒为字符串；其余若为纯数字则原样，否则加引号并转义
const literal = (op: string, v: string): string => {
  const s = String(v ?? '')
  if (op !== 'LIKE' && s.trim() !== '' && !isNaN(Number(s))) {
    return s
  }
  return `'${s.replace(/'/g, "''")}'`
}

const sql = computed(() => {
  if (!table.value) {
    return ''
  }
  const cols = selectedCols.value.length === 0 ? '*' : selectedCols.value.map(q).join(', ')
  let out = `SELECT ${cols} FROM ${q(table.value)}`
  const conds = wheres.value
    .filter(w => w.col)
    .map(w => (w.op === 'IS NULL' || w.op === 'IS NOT NULL')
      ? `${q(w.col)} ${w.op}`
      : `${q(w.col)} ${w.op} ${literal(w.op, w.value)}`)
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
    if (tables.value.length && !tables.value.find(tb => tb.name === table.value)) {
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
