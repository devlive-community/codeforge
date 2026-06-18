<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-16 px-6 pb-6" @click="emit('close')">
    <div class="w-full max-w-[980px] max-h-full bg-white dark:bg-gray-800 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col"
         @click.stop>
      <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
          <Rows3 class="w-4 h-4 text-gray-400"/>
          <span>{{ staged ? t('git.hunkUnstageTitle') : t('git.hunkStageTitle') }}</span>
          <span class="text-xs text-gray-400">· {{ relPath }}</span>
        </div>
        <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('close')">
          <X class="w-4 h-4"/>
        </button>
      </div>

      <div class="flex-1 overflow-auto font-mono text-xs leading-5 max-h-[72vh]">
        <div v-if="!hunks.length" class="px-4 py-10 text-center text-sm text-gray-400 font-sans">{{ t('git.hunkEmpty') }}</div>
        <div v-for="(hk, hi) in hunks" :key="hi" class="border-b border-gray-200 dark:border-gray-700">
          <div class="flex items-center justify-between px-3 py-1.5 bg-gray-50 dark:bg-gray-900/40 sticky top-0">
            <span class="text-[11px] text-cyan-600 dark:text-cyan-400 truncate">{{ hk.header }}</span>
            <span class="flex items-center gap-2 flex-shrink-0 ml-2 font-sans">
              <button class="text-xs text-blue-500 hover:underline cursor-pointer disabled:opacity-40"
                      :disabled="busy" @click="apply(hk, true, !!staged)">
                {{ staged ? t('git.hunkUnstage') : t('git.hunkStage') }}
              </button>
              <button v-if="!staged" class="text-xs text-red-500 hover:underline cursor-pointer disabled:opacity-40"
                      :disabled="busy" @click="apply(hk, false, true)">
                {{ t('git.hunkDiscard') }}
              </button>
            </span>
          </div>
          <div v-for="(ln, li) in hk.lines" :key="li" class="flex" :class="lineClass(ln)">
            <span class="flex-1 whitespace-pre-wrap break-all px-3">{{ ln }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {Rows3, X} from 'lucide-vue-next'
import {useToast} from '../plugins/toast'
import {useI18n} from 'vue-i18n'

const props = defineProps<{ rootDir: string; relPath: string; staged?: boolean }>()
const emit = defineEmits<{ close: []; changed: [] }>()

const toast = useToast()
const {t} = useI18n()

interface Hunk { header: string; lines: string[] }

const header = ref<string[]>([])
const hunks = ref<Hunk[]>([])
const busy = ref(false)

const load = async () => {
  try {
    const diff = await invoke<string>('git_file_diff', {root: props.rootDir, relPath: props.relPath, staged: !!props.staged})
    parse(diff)
  }
  catch (error) {
    toast.error(t('git.diffFailed') + ': ' + error)
  }
}

// 把 git diff 输出拆成「文件头」+ 若干 hunk
const parse = (diff: string) => {
  header.value = []
  hunks.value = []
  if (!diff.trim()) {
    return
  }
  const lines = diff.split('\n')
  let cur: Hunk | null = null
  for (const line of lines) {
    if (line.startsWith('@@')) {
      cur = {header: line, lines: []}
      hunks.value.push(cur)
    }
    else if (cur) {
      cur.lines.push(line)
    }
    else {
      header.value.push(line)
    }
  }
  // 去掉每个 hunk 末尾因 split 产生的空串
  for (const h of hunks.value) {
    while (h.lines.length && h.lines[h.lines.length - 1] === '') {
      h.lines.pop()
    }
  }
}

const buildPatch = (hk: Hunk): string => {
  const body = [...header.value, hk.header, ...hk.lines].join('\n')
  return body.endsWith('\n') ? body : body + '\n'
}

const apply = async (hk: Hunk, cached: boolean, reverse: boolean) => {
  busy.value = true
  try {
    await invoke('git_apply_patch', {root: props.rootDir, patch: buildPatch(hk), cached, reverse})
    emit('changed')
    await load()
    if (!hunks.value.length) {
      emit('close')
    }
  }
  catch (error) {
    toast.error(t('git.hunkFailed') + ': ' + error)
  }
  finally {
    busy.value = false
  }
}

const lineClass = (ln: string) =>
    ln.startsWith('+') ? 'bg-green-50 dark:bg-green-900/25 text-green-800 dark:text-green-300'
        : ln.startsWith('-') ? 'bg-red-50 dark:bg-red-900/25 text-red-800 dark:text-red-300'
            : 'text-gray-600 dark:text-gray-300'

onMounted(load)
</script>
