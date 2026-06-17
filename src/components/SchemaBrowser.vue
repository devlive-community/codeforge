<template>
  <div class="inline-block">
    <button ref="btnRef" class="inline-flex items-center gap-1 px-2 py-0.5 rounded text-xs text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer"
            :title="t('sql.schemaTitle')" @click="toggle">
      <Database class="w-3.5 h-3.5"/>
      <span>{{ t('sql.schema') }}</span>
    </button>

    <Teleport to="body">
      <template v-if="open">
        <div class="fixed inset-0 z-[60]" @click="open = false"/>
        <div class="fixed z-[61] w-72 overflow-auto rounded-md border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 shadow-lg text-xs"
             :style="{left: pos.left + 'px', top: pos.top + 'px', maxHeight: pos.maxH + 'px'}">
          <div class="sticky top-0 z-10 bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
            <div class="flex items-center justify-between px-3 py-2">
              <span class="font-medium text-gray-600 dark:text-gray-300 truncate">{{ t('sql.schema') }} · {{ activeLabel() }}</span>
              <button class="p-0.5 rounded text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer flex-shrink-0" :title="t('sql.refresh')" @click="load">
                <RefreshCw class="w-3.5 h-3.5" :class="loading ? 'animate-spin' : ''"/>
              </button>
            </div>
            <div class="px-2 pb-2">
              <input v-model="filter" type="text" :placeholder="mode === 'databases' ? t('sql.searchDb') : t('sql.searchTable')"
                     class="w-full px-2 py-1 text-xs rounded border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-900 text-gray-700 dark:text-gray-200 focus:outline-none focus:border-blue-400"/>
            </div>
          </div>

          <div v-if="loading" class="px-3 py-4 text-center text-gray-400">{{ t('sql.loading') }}</div>
          <div v-else-if="error" class="px-3 py-3 text-red-500 whitespace-pre-wrap">{{ error }}</div>
          <div v-else-if="mode === 'databases' && databases.length === 0" class="px-3 py-4 text-center text-gray-400">{{ t('sql.noDatabase') }}</div>
          <div v-else-if="mode === 'tables' && tables.length === 0" class="px-3 py-4 text-center text-gray-400">{{ t('sql.noTable') }}</div>

          <!-- 库 → 表 → 字段 -->
          <div v-else-if="mode === 'databases'" class="py-1">
            <div v-for="db in filteredDatabases" :key="db.name">
              <div class="flex items-center gap-1 px-2 py-1 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="expandDb(db)">
                <ChevronRight class="w-3 h-3 text-gray-400 transition-transform flex-shrink-0" :class="db.expanded ? 'rotate-90' : ''"/>
                <Database class="w-3 h-3 text-amber-500 flex-shrink-0"/>
                <span class="flex-1 truncate text-gray-700 dark:text-gray-200" :title="db.name">{{ db.name }}</span>
                <RefreshCw v-if="db.loading" class="w-3 h-3 text-gray-400 animate-spin"/>
              </div>
              <div v-if="db.expanded && db.tables" class="pl-4">
                <template v-for="tbl in matchTables(db.tables)" :key="tbl.name">
                  <div class="group flex items-center gap-1 px-2 py-1 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="toggleTable(db.name + '.' + tbl.name)">
                    <ChevronRight class="w-3 h-3 text-gray-400 transition-transform flex-shrink-0" :class="isOpen(db.name + '.' + tbl.name) ? 'rotate-90' : ''"/>
                    <Table2 class="w-3 h-3 text-blue-500 flex-shrink-0"/>
                    <span class="flex-1 truncate text-gray-700 dark:text-gray-200" :title="tbl.name" @click.stop="emit('insert', tbl.name)">{{ tbl.name }}</span>
                    <span class="text-[10px] text-gray-400">{{ tbl.columns.length }}</span>
                    <button class="opacity-0 group-hover:opacity-100 p-0.5 rounded text-gray-400 hover:text-violet-500 cursor-pointer" :title="t('sql.copyDdl')" @click.stop="copyDdl(tbl.name, db.name)">
                      <Copy class="w-3 h-3"/>
                    </button>
                    <button class="opacity-0 group-hover:opacity-100 p-0.5 rounded text-gray-400 hover:text-emerald-500 cursor-pointer" :title="t('sql.exportCsv')" @click.stop="exportCsv(tbl.name, db.name)">
                      <FileDown class="w-3 h-3"/>
                    </button>
                    <button class="opacity-0 group-hover:opacity-100 p-0.5 rounded text-gray-400 hover:text-blue-500 cursor-pointer" :title="t('sql.preview100')" @click.stop="preview(tbl.name, db.name)">
                      <Play class="w-3 h-3"/>
                    </button>
                  </div>
                  <div v-if="isOpen(db.name + '.' + tbl.name)" class="pl-7 pr-2 pb-1">
                    <div v-for="col in tbl.columns" :key="col.name" class="flex items-center justify-between gap-2 py-0.5 cursor-pointer hover:text-blue-500" :title="t('sql.insertColumn', { name: col.name })" @click="emit('insert', col.name)">
                      <span class="truncate text-gray-600 dark:text-gray-300">{{ col.name }}</span>
                      <span class="text-[10px] text-gray-400 flex-shrink-0">{{ col.type }}</span>
                    </div>
                  </div>
                </template>
                <div v-if="db.tables.length === 0" class="px-2 py-1 text-gray-400">{{ t('sql.noTable') }}</div>
              </div>
            </div>
          </div>

          <!-- 表 → 字段 -->
          <div v-else class="py-1">
            <template v-for="tbl in filteredTables" :key="tbl.name">
              <div class="group flex items-center gap-1 px-2 py-1 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="toggleTable(tbl.name)">
                <ChevronRight class="w-3 h-3 text-gray-400 transition-transform flex-shrink-0" :class="isOpen(tbl.name) ? 'rotate-90' : ''"/>
                <Table2 class="w-3 h-3 text-blue-500 flex-shrink-0"/>
                <span class="flex-1 truncate text-gray-700 dark:text-gray-200" :title="tbl.name" @click.stop="emit('insert', tbl.name)">{{ tbl.name }}</span>
                <span class="text-[10px] text-gray-400">{{ tbl.columns.length }}</span>
                <button class="opacity-0 group-hover:opacity-100 p-0.5 rounded text-gray-400 hover:text-violet-500 cursor-pointer" :title="t('sql.copyDdl')" @click.stop="copyDdl(tbl.name)">
                  <Copy class="w-3 h-3"/>
                </button>
                <button class="opacity-0 group-hover:opacity-100 p-0.5 rounded text-gray-400 hover:text-emerald-500 cursor-pointer" :title="t('sql.exportCsv')" @click.stop="exportCsv(tbl.name)">
                  <FileDown class="w-3 h-3"/>
                </button>
                <button class="opacity-0 group-hover:opacity-100 p-0.5 rounded text-gray-400 hover:text-blue-500 cursor-pointer" :title="t('sql.preview100')" @click.stop="preview(tbl.name)">
                  <Play class="w-3 h-3"/>
                </button>
              </div>
              <div v-if="isOpen(tbl.name)" class="pl-7 pr-2 pb-1">
                <div v-for="col in tbl.columns" :key="col.name" class="flex items-center justify-between gap-2 py-0.5 cursor-pointer hover:text-blue-500" :title="t('sql.insertColumn', { name: col.name })" @click="emit('insert', col.name)">
                  <span class="truncate text-gray-600 dark:text-gray-300">{{ col.name }}</span>
                  <span class="text-[10px] text-gray-400 flex-shrink-0">{{ col.type }}</span>
                </div>
              </div>
            </template>
          </div>
        </div>
      </template>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import {computed, ref, watch} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {ChevronRight, Copy, Database, FileDown, Play, RefreshCw, Table2} from 'lucide-vue-next'
import {useI18n} from 'vue-i18n'
import {useDbConnections} from '../composables/useDbConnections'
import {useToast} from '../plugins/toast'
import {downloadCsv} from '../utils/csv'

const emit = defineEmits<{ insert: [text: string]; preview: [sql: string] }>()
const toast = useToast()
const {t} = useI18n()

const {resolveActiveSource, activeRef, activeLabel} = useDbConnections()

interface Col { name: string; type: string }
interface Tbl { name: string; columns: Col[] }
interface Db { name: string; expanded: boolean; loading: boolean; tables: Tbl[] | null }

const open = ref(false)
const btnRef = ref<HTMLElement>()
const pos = ref({left: 0, top: 0, maxH: 384})
const loading = ref(false)
const error = ref('')
const mode = ref<'tables' | 'databases'>('tables')
const tables = ref<Tbl[]>([])
const databases = ref<Db[]>([])
const expandedKeys = ref<Set<string>>(new Set())
const filter = ref('')

// 按名称/字段过滤
const matchTables = (list: Tbl[]): Tbl[] => {
  const q = filter.value.trim().toLowerCase()
  if (!q) {
    return list
  }
  return list.filter(t => t.name.toLowerCase().includes(q) || t.columns.some(c => c.name.toLowerCase().includes(q)))
}
const filteredTables = computed(() => matchTables(tables.value))
const filteredDatabases = computed(() => {
  const q = filter.value.trim().toLowerCase()
  if (!q) {
    return databases.value
  }
  return databases.value.filter(d => d.name.toLowerCase().includes(q))
})

const quote = (kind: string, name: string) => (kind === 'mysql' || kind === 'clickhouse' ? `\`${name}\`` : `"${name}"`)
const esc = (s: string) => s.replace(/'/g, "''")

const tablesSql = (kind: string, db?: string): string => {
  if (kind === 'mysql') {
    return 'SELECT table_name AS tbl, column_name AS col, column_type AS typ '
      + `FROM information_schema.columns WHERE table_schema = '${esc(db || '')}' `
      + 'ORDER BY table_name, ordinal_position'
  }
  if (kind === 'postgres') {
    return 'SELECT table_name AS tbl, column_name AS col, data_type AS typ '
      + 'FROM information_schema.columns '
      + "WHERE table_schema NOT IN ('pg_catalog', 'information_schema') "
      + 'ORDER BY table_name, ordinal_position'
  }
  if (kind === 'clickhouse') {
    return 'SELECT table AS tbl, name AS col, type AS typ '
      + 'FROM system.columns WHERE database = currentDatabase() '
      + 'ORDER BY table, position'
  }
  if (kind === 'duckdb') {
    return 'SELECT table_name AS tbl, column_name AS col, data_type AS typ '
      + 'FROM information_schema.columns '
      + "WHERE table_schema NOT IN ('information_schema', 'pg_catalog') "
      + 'ORDER BY table_name, ordinal_position'
  }
  return 'SELECT m.name AS tbl, p.name AS col, p.type AS typ '
    + 'FROM sqlite_master m JOIN pragma_table_info(m.name) p '
    + "WHERE m.type IN ('table','view') AND m.name NOT LIKE 'sqlite\\_%' ESCAPE '\\' "
    + 'ORDER BY m.name, p.cid'
}

const runRows = async (sql: string): Promise<any[][]> => {
  const source = resolveActiveSource()
  const res = await invoke<any>('run_sql', {sql, source})
  if (res.error) {
    throw new Error(res.error)
  }
  return (res.result_sets || [])[0]?.rows || []
}

const groupTables = (rows: any[][]): Tbl[] => {
  const map = new Map<string, Tbl>()
  for (const row of rows) {
    const tbl = String(row[0])
    if (!map.has(tbl)) {
      map.set(tbl, {name: tbl, columns: []})
    }
    map.get(tbl)!.columns.push({name: String(row[1]), type: String(row[2] ?? '')})
  }
  return [...map.values()]
}

const load = async () => {
  loading.value = true
  error.value = ''
  expandedKeys.value = new Set()
  filter.value = ''
  try {
    const source = resolveActiveSource()
    if (source.kind === 'mysql' && !source.database) {
      // 未指定库：列出数据库，按需展开
      mode.value = 'databases'
      const rows = await runRows(
        'SELECT schema_name FROM information_schema.schemata '
        + "WHERE schema_name NOT IN ('information_schema','mysql','performance_schema','sys') "
        + 'ORDER BY schema_name'
      )
      databases.value = rows.map(r => ({name: String(r[0]), expanded: false, loading: false, tables: null}))
    }
    else {
      mode.value = 'tables'
      const db = source.kind === 'mysql' ? source.database || undefined : undefined
      tables.value = groupTables(await runRows(tablesSql(source.kind, db)))
    }
  }
  catch (e: any) {
    error.value = e?.message || String(e)
    tables.value = []
    databases.value = []
  }
  finally {
    loading.value = false
  }
}

const expandDb = async (db: Db) => {
  if (!db.expanded && db.tables == null) {
    db.loading = true
    try {
      db.tables = groupTables(await runRows(tablesSql('mysql', db.name)))
    }
    catch {
      db.tables = []
    }
    finally {
      db.loading = false
    }
  }
  db.expanded = !db.expanded
}

const isOpen = (key: string) => expandedKeys.value.has(key)
const toggleTable = (key: string) => {
  const s = new Set(expandedKeys.value)
  s.has(key) ? s.delete(key) : s.add(key)
  expandedKeys.value = s
}

const preview = (name: string, db?: string) => {
  const kind = resolveActiveSource().kind
  const qualified = db ? `${quote(kind, db)}.${quote(kind, name)}` : quote(kind, name)
  emit('preview', `SELECT * FROM ${qualified} LIMIT 100`)
  open.value = false
}

const exportCsv = async (name: string, db?: string) => {
  try {
    const source = resolveActiveSource()
    const qualified = db ? `${quote(source.kind, db)}.${quote(source.kind, name)}` : quote(source.kind, name)
    const res = await invoke<any>('run_sql', {sql: `SELECT * FROM ${qualified} LIMIT 100000`, source})
    if (res.error) {
      toast.error(res.error)
      return
    }
    const rs = (res.result_sets || [])[0]
    if (!rs || rs.columns.length === 0) {
      toast.error(t('sql.noExportData'))
      return
    }
    downloadCsv(rs.columns, rs.rows, `${name}-${Date.now()}.csv`)
    toast.success(t('sql.exported', { n: rs.rows.length }))
  }
  catch (e: any) {
    toast.error(t('sql.exportFailed') + String(e?.message || e))
  }
}

const copyDdl = async (name: string, db?: string) => {
  try {
    const source = resolveActiveSource()
    if (source.kind === 'postgres' || source.kind === 'duckdb') {
      toast.info(t('sql.ddlUnsupported', { db: source.kind === 'duckdb' ? 'DuckDB' : 'PostgreSQL' }))
      return
    }
    const qualified = db ? `${quote(source.kind, db)}.${quote(source.kind, name)}` : quote(source.kind, name)
    const sql = source.kind === 'mysql' || source.kind === 'clickhouse'
      ? `SHOW CREATE TABLE ${qualified}`
      : `SELECT sql FROM sqlite_master WHERE name = '${esc(name)}'`
    const rows = await runRows(sql)
    // MySQL: 第 2 列为建表语句；ClickHouse/SQLite: 第 1 列
    const ddl = String((source.kind === 'mysql' ? rows[0]?.[1] : rows[0]?.[0]) ?? '')
    if (!ddl) {
      toast.error(t('sql.noDdl'))
      return
    }
    await navigator.clipboard.writeText(ddl.endsWith(';') ? ddl : ddl + ';')
    toast.success(t('sql.ddlCopied', { name }))
  }
  catch (e: any) {
    toast.error(t('sql.copyFailed') + String(e?.message || e))
  }
}

const toggle = () => {
  open.value = !open.value
  if (open.value) {
    const r = btnRef.value?.getBoundingClientRect()
    if (r) {
      const top = r.bottom + 4
      const left = Math.min(r.left, window.innerWidth - 288 - 8)
      pos.value = {left: Math.max(8, left), top, maxH: Math.max(200, Math.min(380, window.innerHeight - top - 12))}
    }
    load()
  }
}

watch(activeRef, () => {
  if (open.value) {
    load()
  }
})
</script>
