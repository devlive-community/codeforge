<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-16 px-6 pb-6" @click="emit('close')">
    <div class="w-full max-w-[560px] max-h-full bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col"
         @click.stop>
      <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
          <ListChecks class="w-4 h-4 text-gray-400"/>
          <span>{{ t('task.title') }}</span>
        </div>
        <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('close')">
          <X class="w-4 h-4"/>
        </button>
      </div>

      <!-- 新增任务 -->
      <div class="p-3 border-b border-gray-200 dark:border-gray-700 flex-shrink-0 flex gap-2">
        <input v-model="label" class="w-36 flex-shrink-0 text-sm border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-2 py-1 focus:outline-none focus:border-blue-500" :placeholder="t('task.labelPlaceholder')"/>
        <input v-model="command" class="flex-1 min-w-0 text-sm font-mono border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-2 py-1 focus:outline-none focus:border-blue-500" :placeholder="t('task.commandPlaceholder')" @keydown.enter="add"/>
        <Button size="sm" :disabled="!label.trim() || !command.trim()" @click="add">{{ t('task.add') }}</Button>
      </div>

      <!-- 任务列表 -->
      <div class="flex-1 overflow-y-auto min-h-[120px]">
        <div v-if="!tasks.length" class="px-4 py-10 text-center text-sm text-gray-400">{{ t('task.empty') }}</div>
        <div v-for="(tk, i) in tasks" :key="i"
             class="group flex items-center gap-2 px-4 py-2 border-b border-gray-100 dark:border-gray-800">
          <button class="text-emerald-600 dark:text-emerald-400 hover:text-emerald-500 cursor-pointer flex-shrink-0" :title="t('task.run')" @click="run(tk)">
            <Play class="w-4 h-4"/>
          </button>
          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium text-gray-800 dark:text-gray-100 truncate">{{ tk.label }}</div>
            <div class="text-[11px] text-gray-400 font-mono truncate">{{ tk.command }}</div>
          </div>
          <button class="text-xs text-red-500 hover:underline opacity-0 group-hover:opacity-100 cursor-pointer flex-shrink-0" @click="remove(i)">{{ t('task.delete') }}</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {ListChecks, Play, X} from 'lucide-vue-next'
import Button from '../ui/Button.vue'
import {kvGetJSON, kvSetJSON} from '../composables/useKvStore'
import {useI18n} from 'vue-i18n'

interface Task { label: string; command: string }

const props = defineProps<{ rootDir: string }>()
const emit = defineEmits<{ close: []; run: [command: string] }>()

const {t} = useI18n()
const key = () => `tasks:${props.rootDir}`
const tasks = ref<Task[]>([])
const label = ref('')
const command = ref('')

const load = () => {
  tasks.value = kvGetJSON<Task[]>(key(), [])
}
const persist = () => kvSetJSON(key(), tasks.value)

const add = () => {
  if (!label.value.trim() || !command.value.trim()) {
    return
  }
  tasks.value = [...tasks.value, {label: label.value.trim(), command: command.value.trim()}]
  persist()
  label.value = ''
  command.value = ''
}

const remove = (i: number) => {
  tasks.value = tasks.value.filter((_, idx) => idx !== i)
  persist()
}

const run = (tk: Task) => {
  emit('run', tk.command)
  emit('close')
}

onMounted(load)
</script>
