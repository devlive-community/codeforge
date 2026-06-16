<template>
  <Select :model-value="activeRef"
          :options="sourceOptions"
          class="w-48"
          :button-classes="['!py-1', '!px-2.5', 'text-sm', '!rounded-md']"
          :placeholder="t('sql.selectSource')"
          @change="onSourceChange"/>
</template>

<script setup lang="ts">
import {computed} from 'vue'
import {open} from '@tauri-apps/plugin-dialog'
import {useI18n} from 'vue-i18n'
import Select from '../ui/Select.vue'
import {useDbConnections} from '../composables/useDbConnections'

const {t} = useI18n()
const {connections, activeRef, setActiveRef, activeLabel} = useDbConnections()

const sourceOptions = computed(() => {
  const opts: { value: string; label: string }[] = [{value: 'memory', label: t('sql.memoryDb')}]
  for (const c of connections.value) {
    opts.push({value: `conn:${c.id}`, label: `${c.name}（${c.kind}）`})
  }
  if (activeRef.value.startsWith('file:')) {
    opts.push({value: activeRef.value, label: `${activeLabel()}（${t('sql.fileTag')}）`})
  }
  opts.push({value: '__pickfile__', label: t('sql.pickFile')})
  return opts
})

const onSourceChange = async (value: string) => {
  if (value === '__pickfile__') {
    const selected = await open({multiple: false, filters: [{name: 'SQLite', extensions: ['db', 'sqlite', 'sqlite3', 'db3']}]})
    if (typeof selected === 'string') {
      setActiveRef(`file:${selected}`)
    }
    return
  }
  setActiveRef(value)
}
</script>
