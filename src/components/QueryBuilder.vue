<template>
  <div class="inline-block">
    <Tooltip :text="t('qb.title')">
      <button class="inline-flex items-center gap-1 px-2 py-0.5 rounded text-xs text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="openBuilder">
        <Blocks class="w-3.5 h-3.5"/>
      </button>
    </Tooltip>

    <Teleport to="body">
      <div v-if="visible" class="fixed inset-0 z-50 flex items-start justify-center pt-10 px-6 pb-6" @click="visible = false">
        <div class="w-full max-w-[880px] max-h-[85vh] bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 flex flex-col overflow-hidden" @click.stop>
          <!-- 标题栏 -->
          <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
            <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
              <Blocks class="w-4 h-4 text-gray-400"/>
              <span>{{ t('qb.title') }} · {{ activeLabel() }}</span>
            </div>
            <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="visible = false">
              <X class="w-4 h-4"/>
            </button>
          </div>

          <div class="flex-1 overflow-auto p-4 flex flex-col gap-4">
            <div v-if="loading" class="text-sm text-gray-400">{{ t('qb.loading') }}</div>
            <div v-else-if="error" class="text-sm text-red-500">{{ error }}</div>
            <div v-else-if="tables.length === 0" class="text-sm text-gray-400">{{ t('qb.noTables') }}</div>

            <template v-else>
              <!-- 表 -->
              <div class="flex items-center gap-2">
                <label class="text-xs text-gray-500 w-16 flex-shrink-0">{{ t('qb.table') }}</label>
                <select v-model="table" class="text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-2 py-1 min-w-[220px] focus:outline-none">
                  <option v-for="tb in tables" :key="tb.name" :value="tb.name">{{ tb.name }}</option>
                </select>
              </div>

              <!-- 列 -->
              <div class="flex gap-2">
                <label class="text-xs text-gray-500 w-16 flex-shrink-0 pt-1">{{ t('qb.columns') }}</label>
                <div class="flex-1">
                  <label class="inline-flex items-center gap-1 text-xs mb-1 cursor-pointer">
                    <input type="checkbox" v-model="selectAll" class="cursor-pointer"/> {{ t('qb.allColumns') }}
                  </label>
                  <div v-if="!selectAll" class="flex flex-wrap gap-x-3 gap-y-1 max-h-28 overflow-auto p-2 rounded border border-gray-200 dark:border-gray-700">
                    <label v-for="c in currentCols" :key="c.name" class="inline-flex items-center gap-1 text-xs cursor-pointer">
                      <input type="checkbox" :value="c.name" v-model="pickedCols" class="cursor-pointer"/>
                      {{ c.name }}<span class="text-gray-400">{{ c.type }}</span>
                    </label>
                  </div>
                </div>
              </div>

              <!-- 条件 WHERE -->
              <div class="flex gap-2">
                <label class="text-xs text-gray-500 w-16 flex-shrink-0 pt-1">{{ t('qb.where') }}</label>
                <div class="flex-1 flex flex-col gap-1.5">
                  <div v-for="(w, i) in wheres" :key="i" class="flex items-center gap-1.5">
                    <select v-model="w.col" class="text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-2 py-1 focus:outline-none">
                      <option value="">{{ t('qb.pickColumn') }}</option>
                      <option v-for="c in currentCols" :key="c.name" :value="c.name">{{ c.name }}</option>
                    </select>
                    <select v-model="w.op" class="text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-1.5 py-1 focus:outline-none">
                      <option v-for="op in OPS" :key="op" :value="op">{{ op }}</option>
                    </select>
                    <input v-model="w.value" :disabled="w.op === 'IS NULL' || w.op === 'IS NOT NULL'"
                           :placeholder="t('qb.value')"
                           class="flex-1 text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-2 py-1 focus:outline-none disabled:opacity-40"/>
                    <button class="text-gray-400 hover:text-red-500 cursor-pointer p-1" @click="wheres.splice(i, 1)"><Trash2 class="w-3.5 h-3.5"/></button>
                  </div>
                  <button class="self-start inline-flex items-center gap-1 text-xs text-blue-500 hover:underline cursor-pointer" @click="wheres.push({ col: '', op: '=', value: '' })">
                    <Plus class="w-3.5 h-3.5"/>{{ t('qb.addCondition') }}
                  </button>
                </div>
              </div>

              <!-- 排序 + 限制 -->
              <div class="flex items-center gap-2 flex-wrap">
                <label class="text-xs text-gray-500 w-16 flex-shrink-0">{{ t('qb.orderBy') }}</label>
                <select v-model="orderCol" class="text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-2 py-1 focus:outline-none">
                  <option value="">{{ t('qb.noOrder') }}</option>
                  <option v-for="c in currentCols" :key="c.name" :value="c.name">{{ c.name }}</option>
                </select>
                <select v-if="orderCol" v-model="orderDir" class="text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-1.5 py-1 focus:outline-none">
                  <option value="ASC">ASC</option>
                  <option value="DESC">DESC</option>
                </select>
                <label class="text-xs text-gray-500 ml-3">{{ t('qb.limit') }}</label>
                <input v-model.number="limit" type="number" min="0" class="w-24 text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-2 py-1 focus:outline-none"/>
              </div>

              <!-- SQL 预览 -->
              <div>
                <div class="text-xs text-gray-500 mb-1">{{ t('qb.preview') }}</div>
                <pre class="text-xs font-mono bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded p-2 whitespace-pre-wrap break-all">{{ sql }}</pre>
              </div>
            </template>
          </div>

          <div class="flex items-center justify-end gap-2 px-4 py-2.5 border-t border-gray-200 dark:border-gray-700 flex-shrink-0">
            <button class="text-xs px-3 py-1.5 rounded text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="copySql" :disabled="!sql">{{ t('qb.copy') }}</button>
            <button class="text-xs px-3 py-1.5 rounded text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="doInsert" :disabled="!sql">{{ t('qb.insert') }}</button>
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
import {Blocks, Plus, Trash2, X} from 'lucide-vue-next'
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
const selectAll = ref(true)
const pickedCols = ref<string[]>([])
const wheres = ref<{ col: string; op: string; value: string }[]>([])
const orderCol = ref('')
const orderDir = ref<'ASC' | 'DESC'>('ASC')
const limit = ref(100)

const currentCols = computed(() => tables.value.find(tb => tb.name === table.value)?.columns ?? [])

// 切表时重置列/条件/排序，避免残留其它表的字段
watch(table, () => {
  pickedCols.value = []
  wheres.value = []
  orderCol.value = ''
})

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
  const cols = selectAll.value || pickedCols.value.length === 0 ? '*' : pickedCols.value.map(q).join(', ')
  let out = `SELECT ${cols} FROM ${q(table.value)}`
  const conds = wheres.value
    .filter(w => w.col)
    .map(w => (w.op === 'IS NULL' || w.op === 'IS NOT NULL')
      ? `${q(w.col)} ${w.op}`
      : `${q(w.col)} ${w.op} ${literal(w.op, w.value)}`)
  if (conds.length) {
    out += ' WHERE ' + conds.join(' AND ')
  }
  if (orderCol.value) {
    out += ` ORDER BY ${q(orderCol.value)} ${orderDir.value}`
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
