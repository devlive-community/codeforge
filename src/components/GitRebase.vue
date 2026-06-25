<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-16 px-6 pb-6" @click="emit('close')">
    <div class="w-full max-w-[600px] max-h-full bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col"
         @click.stop>
      <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
          <ListOrdered class="w-4 h-4 text-gray-400"/>
          <span>{{ t('git.rebaseTitle') }}</span>
        </div>
        <div class="flex items-center gap-3 text-xs">
          <label class="flex items-center gap-1 text-gray-500 dark:text-gray-400">
            {{ t('git.rebaseCount') }}
            <select v-model.number="count" class="border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-1 py-0.5 focus:outline-none cursor-pointer" @change="load">
              <option v-for="c in [3, 5, 10, 20]" :key="c" :value="c" class="dark:bg-gray-800">{{ c }}</option>
            </select>
          </label>
          <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('close')">
            <X class="w-4 h-4"/>
          </button>
        </div>
      </div>

      <div class="px-4 py-2 text-[11px] text-gray-500 dark:text-gray-400 border-b border-gray-100 dark:border-gray-800 leading-relaxed flex-shrink-0">
        {{ t('git.rebaseHint') }}
      </div>

      <div class="flex-1 overflow-y-auto min-h-[120px]">
        <div v-for="(row, i) in rows" :key="row.hash"
             class="flex items-center gap-2 px-4 py-2 border-b border-gray-100 dark:border-gray-800"
             :class="row.action === 'drop' ? 'opacity-50' : ''">
          <div class="flex flex-col flex-shrink-0">
            <Tooltip :text="t('git.rebaseMoveUp')">
              <button class="text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 disabled:opacity-20 cursor-pointer leading-none" :disabled="i === 0" @click="move(i, -1)">
                <ChevronUp class="w-3.5 h-3.5"/>
              </button>
            </Tooltip>
            <Tooltip :text="t('git.rebaseMoveDown')">
              <button class="text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 disabled:opacity-20 cursor-pointer leading-none" :disabled="i === rows.length - 1" @click="move(i, 1)">
                <ChevronDown class="w-3.5 h-3.5"/>
              </button>
            </Tooltip>
          </div>
          <select v-model="row.action" class="flex-shrink-0 w-32 text-xs border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-1.5 py-1 focus:outline-none cursor-pointer">
            <option value="pick" class="dark:bg-gray-800">{{ t('git.rebaseActionPick') }}</option>
            <option value="squash" :disabled="i === rows.length - 1" class="dark:bg-gray-800">{{ t('git.rebaseActionSquash') }}</option>
            <option value="fixup" :disabled="i === rows.length - 1" class="dark:bg-gray-800">{{ t('git.rebaseActionFixup') }}</option>
            <option value="drop" class="dark:bg-gray-800">{{ t('git.rebaseActionDrop') }}</option>
          </select>
          <span class="font-mono text-[11px] text-gray-400 flex-shrink-0">{{ row.short }}</span>
          <span class="flex-1 truncate text-sm text-gray-800 dark:text-gray-100">{{ row.subject }}</span>
        </div>
      </div>

      <div class="flex items-center justify-between px-4 py-2.5 border-t border-gray-200 dark:border-gray-700 flex-shrink-0">
        <span class="text-[11px] text-gray-400">{{ t('git.rebaseNote') }}</span>
        <Button size="sm" :loading="busy" :disabled="!rows.length" @click="run">{{ t('git.rebaseRun') }}</Button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {ChevronDown, ChevronUp, ListOrdered, X} from 'lucide-vue-next'
import Button from '../ui/Button.vue'
import Tooltip from '../ui/Tooltip.vue'
import {useToast} from '../plugins/toast'
import {useI18n} from 'vue-i18n'

interface Commit { hash: string; short: string; author: string; date: string; subject: string }
interface Row { hash: string; short: string; subject: string; action: string }

const props = defineProps<{ rootDir: string }>()
const emit = defineEmits<{ close: []; changed: [] }>()

const toast = useToast()
const {t} = useI18n()
const count = ref(5)
const rows = ref<Row[]>([])
const base = ref('--root')
const busy = ref(false)

const load = async () => {
  try {
    // 多取一条用于确定 base（编辑范围之外、更旧的那一条）
    const commits = await invoke<Commit[]>('git_log', {root: props.rootDir, limit: count.value + 1, skip: 0, revision: null})
    const editable = commits.slice(0, count.value)
    base.value = commits.length > count.value ? commits[count.value].hash : '--root'
    rows.value = editable.map(c => ({hash: c.hash, short: c.short, subject: c.subject, action: 'pick'}))
  }
  catch (error) {
    toast.error(t('git.rebaseFailed') + ': ' + error)
  }
}

const move = (i: number, dir: number) => {
  const j = i + dir
  if (j < 0 || j >= rows.value.length) {
    return
  }
  const arr = rows.value
  ;[arr[i], arr[j]] = [arr[j], arr[i]]
}

const run = async () => {
  // 最旧（最后一行）不能是 squash/fixup，强制改回 pick
  const last = rows.value[rows.value.length - 1]
  if (last && (last.action === 'squash' || last.action === 'fixup')) {
    last.action = 'pick'
  }
  // 转为旧→新顺序提交给 git
  const todos = [...rows.value].reverse().map(r => ({action: r.action, hash: r.hash}))
  busy.value = true
  try {
    await invoke('git_rebase_interactive', {root: props.rootDir, base: base.value, todos})
    toast.success(t('git.rebaseDone'))
    emit('changed')
    emit('close')
  }
  catch (error) {
    toast.error(t('git.rebaseFailed') + ': ' + error)
    emit('changed')
    emit('close')
  }
  finally {
    busy.value = false
  }
}

onMounted(load)
</script>
