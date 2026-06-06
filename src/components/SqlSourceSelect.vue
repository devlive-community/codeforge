<template>
  <div class="flex items-center gap-1">
    <HardDrive class="w-3 h-3 flex-shrink-0 text-gray-400"/>
    <select :value="activeRef"
            class="text-xs bg-transparent border border-gray-300 dark:border-gray-600 rounded px-1.5 py-0.5 max-w-[200px] cursor-pointer focus:outline-none dark:bg-gray-900"
            title="选择运行 SQL 的数据源"
            @change="onSourceChange">
      <option value="memory" class="dark:bg-gray-800">内存数据库</option>
      <option v-for="c in connections" :key="c.id" :value="`conn:${c.id}`" class="dark:bg-gray-800">{{ c.name }}（{{ c.kind }}）</option>
      <option v-if="activeRef.startsWith('file:')" :value="activeRef" class="dark:bg-gray-800">{{ activeLabel() }}（文件）</option>
      <option value="__pickfile__" class="dark:bg-gray-800">选择 SQLite 文件…</option>
    </select>
  </div>
</template>

<script setup lang="ts">
import {open} from '@tauri-apps/plugin-dialog'
import {HardDrive} from 'lucide-vue-next'
import {useDbConnections} from '../composables/useDbConnections'

const {connections, activeRef, setActiveRef, activeLabel} = useDbConnections()

const onSourceChange = async (e: Event) => {
  const target = e.target as HTMLSelectElement
  const v = target.value
  if (v === '__pickfile__') {
    const selected = await open({multiple: false, filters: [{name: 'SQLite', extensions: ['db', 'sqlite', 'sqlite3', 'db3']}]})
    if (typeof selected === 'string') {
      setActiveRef(`file:${selected}`)
    }
    else {
      target.value = activeRef.value
    }
    return
  }
  setActiveRef(v)
}
</script>
