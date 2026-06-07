<template>
  <div class="relative">
    <button class="inline-flex items-center gap-1 px-2 py-0.5 rounded text-xs text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer"
            title="表结构" @click="toggle">
      <Table2 class="w-3.5 h-3.5"/>
      <span>表</span>
    </button>

    <template v-if="open">
      <div class="fixed inset-0 z-30" @click="open = false"/>
      <div class="absolute left-0 top-full mt-1 z-40 w-72 max-h-96 overflow-auto rounded-md border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 shadow-lg text-xs">
        <div class="sticky top-0 flex items-center justify-between px-3 py-2 border-b border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800">
          <span class="font-medium text-gray-600 dark:text-gray-300">表结构 · {{ activeLabel() }}</span>
          <button class="p-0.5 rounded text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" title="刷新" @click="load">
            <RefreshCw class="w-3.5 h-3.5" :class="loading ? 'animate-spin' : ''"/>
          </button>
        </div>

        <div v-if="loading" class="px-3 py-4 text-center text-gray-400">加载中…</div>
        <div v-else-if="error" class="px-3 py-3 text-red-500 whitespace-pre-wrap">{{ error }}</div>
        <div v-else-if="tables.length === 0" class="px-3 py-4 text-center text-gray-400">无表</div>

        <div v-else class="py-1">
          <div v-for="t in tables" :key="t.name">
            <div class="group flex items-center gap-1 px-2 py-1 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="expand(t.name)">
              <ChevronRight class="w-3 h-3 text-gray-400 transition-transform flex-shrink-0" :class="expanded.has(t.name) ? 'rotate-90' : ''"/>
              <Table2 class="w-3 h-3 text-blue-500 flex-shrink-0"/>
              <span class="flex-1 truncate text-gray-700 dark:text-gray-200" :title="t.name" @click.stop="emit('insert', t.name)">{{ t.name }}</span>
              <span class="text-[10px] text-gray-400">{{ t.columns.length }}</span>
              <button class="opacity-0 group-hover:opacity-100 p-0.5 rounded text-gray-400 hover:text-blue-500 cursor-pointer" title="预览前 100 行" @click.stop="preview(t.name)">
                <Play class="w-3 h-3"/>
              </button>
            </div>
            <div v-if="expanded.has(t.name)" class="pl-7 pr-2 pb-1">
              <div v-for="col in t.columns" :key="col.name"
                   class="flex items-center justify-between gap-2 py-0.5 cursor-pointer hover:text-blue-500"
                   :title="`插入列名：${col.name}`" @click="emit('insert', col.name)">
                <span class="truncate text-gray-600 dark:text-gray-300">{{ col.name }}</span>
                <span class="text-[10px] text-gray-400 flex-shrink-0">{{ col.type }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import {ref, watch} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {ChevronRight, Play, RefreshCw, Table2} from 'lucide-vue-next'
import {useDbConnections} from '../composables/useDbConnections'

const emit = defineEmits<{ insert: [text: string]; preview: [sql: string] }>()

const {resolveActiveSource, activeRef, activeLabel} = useDbConnections()

interface Col { name: string; type: string }
interface Tbl { name: string; columns: Col[] }

const open = ref(false)
const loading = ref(false)
const error = ref('')
const tables = ref<Tbl[]>([])
const expanded = ref<Set<string>>(new Set())

const quote = (kind: string, name: string) => (kind === 'mysql' ? `\`${name}\`` : `"${name}"`)

const introspectSql = (kind: string): string => {
  if (kind === 'mysql') {
    return 'SELECT table_name AS tbl, column_name AS col, column_type AS typ '
      + 'FROM information_schema.columns WHERE table_schema = DATABASE() '
      + 'ORDER BY table_name, ordinal_position'
  }
  // sqlite / memory
  return 'SELECT m.name AS tbl, p.name AS col, p.type AS typ '
    + 'FROM sqlite_master m JOIN pragma_table_info(m.name) p '
    + "WHERE m.type IN ('table','view') AND m.name NOT LIKE 'sqlite\\_%' ESCAPE '\\' "
    + 'ORDER BY m.name, p.cid'
}

const load = async () => {
  loading.value = true
  error.value = ''
  try {
    const source = resolveActiveSource()
    const res = await invoke<any>('run_sql', {sql: introspectSql(source.kind), source})
    if (res.error) {
      error.value = res.error
      tables.value = []
      return
    }
    const rs = (res.result_sets || [])[0]
    const map = new Map<string, Tbl>()
    for (const row of rs?.rows || []) {
      const [tbl, col, typ] = [String(row[0]), String(row[1]), String(row[2] ?? '')]
      if (!map.has(tbl)) {
        map.set(tbl, {name: tbl, columns: []})
      }
      map.get(tbl)!.columns.push({name: col, type: typ})
    }
    tables.value = [...map.values()]
  }
  catch (e) {
    error.value = String(e)
    tables.value = []
  }
  finally {
    loading.value = false
  }
}

const toggle = () => {
  open.value = !open.value
  if (open.value) {
    load()
  }
}
const expand = (name: string) => {
  const s = new Set(expanded.value)
  s.has(name) ? s.delete(name) : s.add(name)
  expanded.value = s
}
const preview = (name: string) => {
  const kind = resolveActiveSource().kind
  emit('preview', `SELECT * FROM ${quote(kind, name)} LIMIT 100`)
  open.value = false
}

// 切换数据源时若面板开着则重新加载
watch(activeRef, () => {
  if (open.value) {
    load()
  }
})
</script>
