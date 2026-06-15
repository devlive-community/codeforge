<template>
  <div class="space-y-4">
    <div class="text-sm text-gray-600 dark:text-gray-300">
      管理数据库连接，运行 SQL 时可在输出区选择数据源。密码以明文保存在本地数据库中。
    </div>

    <!-- 连接列表 -->
    <div v-if="connections.length" class="border border-gray-200 dark:border-gray-700 rounded divide-y divide-gray-100 dark:divide-gray-700">
      <div v-for="c in connections" :key="c.id" class="flex items-center gap-3 px-3 py-2">
        <span class="text-[10px] px-1.5 py-0.5 rounded uppercase font-semibold"
              :class="c.kind === 'mysql' ? 'bg-orange-100 dark:bg-orange-900/40 text-orange-600 dark:text-orange-300' : c.kind === 'postgres' ? 'bg-sky-100 dark:bg-sky-900/40 text-sky-600 dark:text-sky-300' : c.kind === 'clickhouse' ? 'bg-yellow-100 dark:bg-yellow-900/40 text-yellow-700 dark:text-yellow-300' : c.kind === 'duckdb' ? 'bg-amber-100 dark:bg-amber-900/40 text-amber-700 dark:text-amber-300' : 'bg-blue-100 dark:bg-blue-900/40 text-blue-600 dark:text-blue-300'">{{ c.kind }}</span>
        <div class="flex-1 min-w-0">
          <div class="text-sm font-medium text-gray-800 dark:text-gray-100 truncate">{{ c.name }}</div>
          <div class="text-xs text-gray-400 truncate">
            {{ isFileKind(c.kind) ? (c.file || '内存库') : `${c.user || ''}@${c.host || ''}:${c.port || defaultPortOf(c.kind)}/${c.database || ''}` }}
          </div>
        </div>
        <button class="p-1 text-gray-400 hover:text-blue-500 cursor-pointer" title="编辑" @click="startEdit(c)">
          <Pencil class="w-3.5 h-3.5"/>
        </button>
        <button class="p-1 text-gray-400 hover:text-red-500 cursor-pointer" title="删除" @click="remove(c.id)">
          <Trash2 class="w-3.5 h-3.5"/>
        </button>
      </div>
    </div>
    <div v-else class="text-sm text-gray-400 px-1">还没有连接，在下方添加</div>

    <!-- 表单 -->
    <div class="border border-gray-200 dark:border-gray-700 rounded p-3 space-y-3">
      <div class="flex gap-2">
        <Select v-model="form.kind" :options="kindOptions" class="w-32" :button-classes="['!py-1.5', 'text-sm']"/>
        <input v-model="form.name" placeholder="连接名称" class="flex-1 text-sm border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-2 py-1.5 focus:outline-none focus:border-blue-500"/>
      </div>

      <template v-if="!isFileKind(form.kind)">
        <div class="grid grid-cols-2 gap-2">
          <input v-model="form.host" placeholder="主机（默认 127.0.0.1）" class="text-sm border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-2 py-1.5 focus:outline-none focus:border-blue-500"/>
          <input v-model.number="form.port" type="number" :placeholder="`端口（默认 ${defaultPortOf(form.kind)}）`" class="text-sm border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-2 py-1.5 focus:outline-none focus:border-blue-500"/>
          <input v-model="form.user" placeholder="用户名" class="text-sm border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-2 py-1.5 focus:outline-none focus:border-blue-500"/>
          <input v-model="form.password" type="password" placeholder="密码" class="text-sm border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-2 py-1.5 focus:outline-none focus:border-blue-500"/>
          <input v-model="form.database" placeholder="数据库名" class="col-span-2 text-sm border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-2 py-1.5 focus:outline-none focus:border-blue-500"/>
        </div>
      </template>
      <template v-else>
        <div class="flex gap-2">
          <input v-model="form.file" :placeholder="form.kind === 'duckdb' ? 'DuckDB 文件路径（留空则用内存库）' : 'SQLite 文件路径'" class="flex-1 text-sm border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-2 py-1.5 focus:outline-none focus:border-blue-500"/>
          <Button size="sm" type="secondary" @click="pickFile">选择文件</Button>
        </div>
      </template>

      <div class="flex items-center gap-2">
        <Button size="sm" :disabled="!canSave" @click="submit">{{ editingId ? '保存修改' : '添加连接' }}</Button>
        <Button size="sm" type="secondary" :disabled="!canTest" :loading="testing" @click="testConnection">测试连接</Button>
        <Button v-if="editingId" size="sm" type="secondary" @click="resetForm">取消</Button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, reactive, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {open} from '@tauri-apps/plugin-dialog'
import {Pencil, Trash2} from 'lucide-vue-next'
import Button from '../../ui/Button.vue'
import Select from '../../ui/Select.vue'
import {useDbConnections, type DataSource, type DbConnection} from '../../composables/useDbConnections'
import {useToast} from '../../plugins/toast'

const {connections, add, update, remove} = useDbConnections()
const toast = useToast()

const kindOptions = [
  {value: 'mysql', label: 'MySQL'},
  {value: 'postgres', label: 'PostgreSQL'},
  {value: 'clickhouse', label: 'ClickHouse'},
  {value: 'sqlite', label: 'SQLite'},
  {value: 'duckdb', label: 'DuckDB'}
]

// 各网络型数据源的默认端口
const defaultPortOf = (kind: string) => (kind === 'postgres' ? 5432 : kind === 'clickhouse' ? 8123 : 3306)
// 文件型数据源（用文件路径而非主机端口）
const isFileKind = (kind: string) => kind === 'sqlite' || kind === 'duckdb'

const editingId = ref<string | null>(null)
const form = reactive<{ kind: 'mysql' | 'postgres' | 'clickhouse' | 'sqlite' | 'duckdb'; name: string; host: string; port: number | null; user: string; password: string; database: string; file: string }>({
  kind: 'mysql', name: '', host: '', port: null, user: '', password: '', database: '', file: ''
})

const canSave = computed(() => {
  if (!form.name.trim()) return false
  if (form.kind === 'sqlite') return !!form.file.trim() // SQLite 必须指定文件；DuckDB 可留空用内存库
  return true
})

// 测试连接：仅 SQLite 必须文件，其余（含 DuckDB 内存库、网络型默认值）均可
const canTest = computed(() => (form.kind === 'sqlite' ? !!form.file.trim() : true))

// 由当前表单构建可执行的数据源（不含名称/id）
const buildSource = (): DataSource => isFileKind(form.kind)
    ? {kind: form.kind, file: form.file.trim()}
    : {
      kind: form.kind,
      host: form.host.trim() || '127.0.0.1',
      port: form.port || defaultPortOf(form.kind),
      user: form.user.trim(),
      password: form.password,
      database: form.database.trim() || undefined
    }

const testing = ref(false)
const testConnection = async () => {
  testing.value = true
  try {
    const res = await invoke<{ error?: string }>('run_sql', {sql: 'SELECT 1', source: buildSource()})
    if (res.error) {
      toast.error('连接失败：' + res.error)
    }
    else {
      toast.success('连接成功')
    }
  }
  catch (e: any) {
    toast.error('连接失败：' + String(e?.message || e))
  }
  finally {
    testing.value = false
  }
}

const resetForm = () => {
  editingId.value = null
  Object.assign(form, {kind: 'mysql', name: '', host: '', port: null, user: '', password: '', database: '', file: ''})
}

const startEdit = (c: DbConnection) => {
  editingId.value = c.id
  Object.assign(form, {
    kind: c.kind === 'sqlite' || c.kind === 'duckdb' || c.kind === 'postgres' || c.kind === 'clickhouse' ? c.kind : 'mysql',
    name: c.name,
    host: c.host || '', port: c.port ?? null, user: c.user || '', password: c.password || '',
    database: c.database || '', file: c.file || ''
  })
}

const pickFile = async () => {
  const selected = await open({multiple: false, filters: [{name: '数据库文件', extensions: ['db', 'sqlite', 'sqlite3', 'db3', 'duckdb', 'ddb']}]})
  if (typeof selected === 'string') {
    form.file = selected
  }
}

const submit = () => {
  if (!canSave.value) return
  const payload = isFileKind(form.kind)
      ? {kind: form.kind, name: form.name.trim(), file: form.file.trim()}
      : {kind: form.kind, name: form.name.trim(), host: form.host.trim() || '127.0.0.1', port: form.port || defaultPortOf(form.kind), user: form.user.trim(), password: form.password, database: form.database.trim()}
  if (editingId.value) {
    update(editingId.value, payload)
  }
  else {
    add(payload)
  }
  resetForm()
}
</script>
