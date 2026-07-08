<template>
  <div class="fixed inset-0 z-[60] flex items-start justify-center pt-12 px-6 pb-6" @click="emit('close')">
    <div class="w-full max-w-[900px] max-h-full bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 flex flex-col overflow-hidden" @click.stop>
      <!-- 标题栏 -->
      <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200 min-w-0">
          <GitMerge class="w-4 h-4 text-amber-500 flex-shrink-0"/>
          <span class="truncate">{{ t('conflict.title') }} · {{ relPath }}</span>
          <span v-if="!loading" class="text-[11px] text-gray-400 flex-shrink-0">{{ t('conflict.remaining', { n: unresolvedCount }) }}</span>
        </div>
        <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('close')">
          <X class="w-4 h-4"/>
        </button>
      </div>

      <!-- 顶部批量操作 -->
      <div v-if="!loading && conflictCount > 0" class="flex items-center gap-2 px-4 py-1.5 border-b border-gray-100 dark:border-gray-800 flex-shrink-0 text-xs">
        <span class="text-gray-400">{{ t('conflict.allBlocks') }}:</span>
        <button class="px-2 py-0.5 rounded border border-gray-300 dark:border-gray-600 hover:border-blue-400 cursor-pointer" @click="chooseAll('ours')">{{ t('conflict.ours') }}</button>
        <button class="px-2 py-0.5 rounded border border-gray-300 dark:border-gray-600 hover:border-blue-400 cursor-pointer" @click="chooseAll('theirs')">{{ t('conflict.theirs') }}</button>
        <button class="px-2 py-0.5 rounded border border-gray-300 dark:border-gray-600 hover:border-blue-400 cursor-pointer" @click="chooseAll('both')">{{ t('conflict.both') }}</button>
      </div>

      <!-- 冲突块 -->
      <div class="flex-1 overflow-auto p-3 font-mono text-xs">
        <div v-if="loading" class="text-gray-400 p-4">{{ t('conflict.loading') }}</div>
        <div v-else-if="conflictCount === 0" class="text-gray-400 p-4">{{ t('conflict.noConflict') }}</div>
        <template v-else>
          <template v-for="(seg, i) in segments" :key="i">
            <!-- 普通文本 -->
            <pre v-if="seg.type === 'text'" class="whitespace-pre-wrap text-gray-500 dark:text-gray-400 px-2">{{ seg.text }}</pre>
            <!-- 冲突块 -->
            <div v-else class="my-2 rounded border" :class="seg.choice ? 'border-gray-200 dark:border-gray-700' : 'border-amber-400'">
              <div class="flex items-center gap-2 px-2 py-1 bg-gray-50 dark:bg-gray-800 text-[11px]">
                <span :class="seg.choice ? 'text-green-500' : 'text-amber-500'">{{ seg.choice ? t('conflict.resolvedBlock', { pick: pickLabel(seg.choice) }) : t('conflict.pickSide') }}</span>
                <div class="ml-auto flex items-center gap-1">
                  <button class="px-1.5 py-0.5 rounded cursor-pointer" :class="seg.choice === 'ours' ? 'bg-blue-500 text-white' : 'text-blue-500 hover:bg-blue-50 dark:hover:bg-blue-900/20'" @click="seg.choice = 'ours'">{{ t('conflict.ours') }}</button>
                  <button class="px-1.5 py-0.5 rounded cursor-pointer" :class="seg.choice === 'theirs' ? 'bg-blue-500 text-white' : 'text-blue-500 hover:bg-blue-50 dark:hover:bg-blue-900/20'" @click="seg.choice = 'theirs'">{{ t('conflict.theirs') }}</button>
                  <button class="px-1.5 py-0.5 rounded cursor-pointer" :class="seg.choice === 'both' ? 'bg-blue-500 text-white' : 'text-blue-500 hover:bg-blue-50 dark:hover:bg-blue-900/20'" @click="seg.choice = 'both'">{{ t('conflict.both') }}</button>
                </div>
              </div>
              <div class="grid grid-cols-2 divide-x divide-gray-200 dark:divide-gray-700">
                <div :class="{ 'opacity-40': seg.choice === 'theirs' }">
                  <div class="px-2 py-0.5 text-[10px] text-blue-500 bg-blue-50/50 dark:bg-blue-900/10">{{ t('conflict.ourSide') }}</div>
                  <pre class="whitespace-pre-wrap px-2 py-1 text-gray-700 dark:text-gray-200">{{ seg.ours.join('\n') }}</pre>
                </div>
                <div :class="{ 'opacity-40': seg.choice === 'ours' }">
                  <div class="px-2 py-0.5 text-[10px] text-emerald-500 bg-emerald-50/50 dark:bg-emerald-900/10">{{ t('conflict.theirSide') }}</div>
                  <pre class="whitespace-pre-wrap px-2 py-1 text-gray-700 dark:text-gray-200">{{ seg.theirs.join('\n') }}</pre>
                </div>
              </div>
            </div>
          </template>
        </template>
      </div>

      <!-- 底部 -->
      <div class="flex items-center justify-between gap-2 px-4 py-2.5 border-t border-gray-200 dark:border-gray-700 flex-shrink-0">
        <button class="text-xs text-gray-500 hover:text-gray-700 dark:hover:text-gray-300 cursor-pointer" @click="emit('open', relPath)">{{ t('conflict.openInEditor') }}</button>
        <div class="flex items-center gap-2">
          <button class="text-xs px-3 py-1.5 rounded text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="emit('close')">{{ t('conflict.cancel') }}</button>
          <button class="text-xs px-3 py-1.5 rounded bg-blue-500 text-white hover:bg-blue-600 disabled:opacity-40 disabled:cursor-not-allowed cursor-pointer"
                  :disabled="loading || unresolvedCount > 0" @click="saveResolved">
            {{ t('conflict.saveResolved') }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, onMounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {GitMerge, X} from 'lucide-vue-next'
import {useI18n} from 'vue-i18n'
import {useToast} from '../plugins/toast'

const props = defineProps<{ rootDir: string; relPath: string }>()
const emit = defineEmits<{ close: []; resolved: []; open: [path: string] }>()

const {t} = useI18n()
const toast = useToast()

type Choice = 'ours' | 'theirs' | 'both' | null
interface Seg { type: 'text' | 'conflict'; text?: string; ours: string[]; theirs: string[]; choice: Choice }

const loading = ref(true)
const segments = ref<Seg[]>([])

const conflictSegs = computed(() => segments.value.filter(s => s.type === 'conflict'))
const conflictCount = computed(() => conflictSegs.value.length)
const unresolvedCount = computed(() => conflictSegs.value.filter(s => !s.choice).length)

const abs = () => `${props.rootDir}/${props.relPath}`
const pickLabel = (c: Choice) => (c === 'ours' ? t('conflict.ours') : c === 'theirs' ? t('conflict.theirs') : t('conflict.both'))

// 解析带冲突标记的文件为「文本 / 冲突块」序列（兼容 diff3 的 ||||||| base 段）
const parse = (content: string) => {
  const lines = content.split('\n')
  const segs: Seg[] = []
  let buf: string[] = []
  const flushText = () => {
    if (buf.length) {
      segs.push({type: 'text', text: buf.join('\n'), ours: [], theirs: [], choice: null})
      buf = []
    }
  }
  let i = 0
  while (i < lines.length) {
    if (lines[i].startsWith('<<<<<<<')) {
      flushText()
      i++
      const ours: string[] = []
      while (i < lines.length && !lines[i].startsWith('=======') && !lines[i].startsWith('|||||||')) {
        ours.push(lines[i]); i++
      }
      // 跳过 diff3 的 base 段
      if (i < lines.length && lines[i].startsWith('|||||||')) {
        i++
        while (i < lines.length && !lines[i].startsWith('=======')) {
          i++
        }
      }
      i++ // 跳过 =======
      const theirs: string[] = []
      while (i < lines.length && !lines[i].startsWith('>>>>>>>')) {
        theirs.push(lines[i]); i++
      }
      i++ // 跳过 >>>>>>>
      segs.push({type: 'conflict', ours, theirs, choice: null})
    }
    else {
      buf.push(lines[i]); i++
    }
  }
  flushText()
  return segs
}

const chooseAll = (c: Choice) => {
  for (const s of conflictSegs.value) {
    s.choice = c
  }
}

// 按选择重建文件内容
const rebuild = () => {
  const out: string[] = []
  for (const s of segments.value) {
    if (s.type === 'text') {
      out.push(s.text || '')
    }
    else if (s.choice === 'ours') {
      out.push(...s.ours)
    }
    else if (s.choice === 'theirs') {
      out.push(...s.theirs)
    }
    else if (s.choice === 'both') {
      out.push(...s.ours, ...s.theirs)
    }
  }
  return out.join('\n')
}

const saveResolved = async () => {
  if (unresolvedCount.value > 0) {
    return
  }
  try {
    await invoke('write_file_text', {path: abs(), content: rebuild()})
    await invoke('git_stage', {root: props.rootDir, paths: [abs()]})
    toast.success(t('conflict.done'))
    emit('resolved')
    emit('close')
  }
  catch (error) {
    toast.error(t('conflict.saveFailed') + ': ' + error)
  }
}

onMounted(async () => {
  try {
    const content = await invoke<string>('read_file_text', {path: abs(), maxSizeMb: 20})
    segments.value = parse(content)
  }
  catch (error) {
    toast.error(t('conflict.loadFailed') + ': ' + error)
    emit('close')
  }
  finally {
    loading.value = false
  }
})
</script>
