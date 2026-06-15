<template>
  <div class="-mt-1">
    <p class="text-sm text-gray-600 dark:text-gray-300 mb-3">
      管理数据库连接，运行 SQL 时可在输出区选择数据源。密码以明文保存在本地数据库中。
    </p>

    <!-- 主从布局：左侧连接列表 + 右侧表单 -->
    <div class="flex gap-4 items-start">
      <!-- 左：连接列表 -->
      <div class="w-64 flex-shrink-0 space-y-2">
        <Button size="sm" :icon="Plus" class="w-full" :type="editingId ? 'secondary' : 'primary'" @click="resetForm">新增连接</Button>
        <div class="space-y-1 max-h-[55vh] overflow-y-auto pr-0.5">
          <button v-for="c in connections" :key="c.id"
                  class="group w-full text-left rounded-lg border px-2.5 py-2 transition-colors cursor-pointer"
                  :class="editingId === c.id ? 'border-blue-400 bg-blue-50 dark:border-blue-500 dark:bg-blue-900/20' : 'border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-800'"
                  @click="startEdit(c)">
            <div class="flex items-center gap-2">
              <span class="text-[10px] px-1.5 py-0.5 rounded uppercase font-semibold flex-shrink-0"
                    :class="c.kind === 'mysql' ? 'bg-orange-100 dark:bg-orange-900/40 text-orange-600 dark:text-orange-300' : c.kind === 'postgres' ? 'bg-sky-100 dark:bg-sky-900/40 text-sky-600 dark:text-sky-300' : c.kind === 'clickhouse' ? 'bg-yellow-100 dark:bg-yellow-900/40 text-yellow-700 dark:text-yellow-300' : c.kind === 'duckdb' ? 'bg-amber-100 dark:bg-amber-900/40 text-amber-700 dark:text-amber-300' : 'bg-blue-100 dark:bg-blue-900/40 text-blue-600 dark:text-blue-300'">{{ c.kind }}</span>
              <span class="flex-1 truncate text-sm font-medium text-gray-800 dark:text-gray-100">{{ c.name }}</span>
              <Trash2 class="w-3.5 h-3.5 text-gray-400 hover:text-red-500 opacity-0 group-hover:opacity-100 flex-shrink-0" title="删除" @click.stop="remove(c.id)"/>
            </div>
            <div class="text-xs text-gray-400 truncate mt-0.5">
              {{ isFileKind(c.kind) ? (c.file || '内存库') : `${c.user || ''}@${c.host || ''}:${c.port || defaultPortOf(c.kind)}/${c.database || ''}` }}
            </div>
          </button>
          <div v-if="!connections.length" class="text-xs text-gray-400 text-center py-6">还没有连接</div>
        </div>
      </div>

      <!-- 右：表单 -->
      <div class="flex-1 min-w-0 border border-gray-200 dark:border-gray-700 rounded-lg p-4 space-y-4">
        <div class="text-sm font-medium text-gray-700 dark:text-gray-200">{{ editingId ? '编辑连接' : '新增连接' }}</div>

        <div class="grid grid-cols-2 gap-x-4 gap-y-3">
          <Label label="类型">
            <Select v-model="form.kind" :options="kindOptions" class="w-full" :button-classes="['!py-1.5', 'text-sm', 'w-full']"/>
          </Label>
          <Label label="连接名称">
            <Input v-model="form.name" placeholder="例如：本地数据源"/>
          </Label>
        </div>

        <div v-if="!isFileKind(form.kind)" class="grid grid-cols-2 gap-x-4 gap-y-3">
          <Label label="主机">
            <Input v-model="form.host" placeholder="默认 127.0.0.1"/>
          </Label>
          <Label label="端口">
            <Number v-model="form.port" :placeholder="`默认 ${defaultPortOf(form.kind)}`"/>
          </Label>
          <Label label="用户名">
            <Input v-model="form.user" placeholder="用户名"/>
          </Label>
          <Label label="密码">
            <Input v-model="form.password" type="password" placeholder="密码"/>
          </Label>
          <Label label="数据库名" custom-class="col-span-2">
            <Input v-model="form.database" placeholder="数据库名（可选）"/>
          </Label>
        </div>
        <Label v-else :label="form.kind === 'duckdb' ? 'DuckDB 文件' : 'SQLite 文件'">
          <div class="flex gap-2">
            <Input v-model="form.file" class="flex-1" :placeholder="form.kind === 'duckdb' ? '文件路径（留空则用内存库）' : 'SQLite 文件路径'"/>
            <Button size="sm" type="secondary" @click="pickFile">选择文件</Button>
          </div>
        </Label>

        <div class="flex items-center gap-2 pt-1">
          <Button size="sm" :disabled="!canSave" @click="submit">{{ editingId ? '保存修改' : '添加连接' }}</Button>
          <Button size="sm" type="secondary" :disabled="!canTest" :loading="testing" @click="testConnection">测试连接</Button>
          <Button v-if="editingId" size="sm" type="secondary" @click="resetForm">取消</Button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, reactive, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {open} from '@tauri-apps/plugin-dialog'
import {Plus, Trash2} from 'lucide-vue-next'
import Button from '../../ui/Button.vue'
import Select from '../../ui/Select.vue'
import Input from '../../ui/Input.vue'
import Number from '../../ui/Number.vue'
import Label from '../../ui/Label.vue'
import {useDbConnections, type DataSource, type DbConnection} from '../../composables/useDbConnections'
import {useToast} from '../../plugins/toast'

const {connections, add, update, remove} = useDbConnections()
const toast = useToast()

// DuckDB 在 Windows 上未编译(bundled C++ 不兼容部分 MSVC)，故隐藏该选项
const isWindows = /Windows/i.test(navigator.userAgent)
const kindOptions = [
  {value: 'mysql', label: 'MySQL'},
  {value: 'postgres', label: 'PostgreSQL'},
  {value: 'clickhouse', label: 'ClickHouse'},
  {value: 'sqlite', label: 'SQLite'},
  ...(isWindows ? [] : [{value: 'duckdb', label: 'DuckDB'}])
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
