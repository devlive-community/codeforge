<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-12 px-6 pb-6" @click="emit('close')">
    <div class="w-full max-w-[1000px] max-h-full bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col"
         @click.stop>
      <!-- 头部 -->
      <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
          <History class="w-4 h-4 text-gray-400"/>
          <span>{{ t('git.history') }}</span>
        </div>
        <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('close')">
          <X class="w-4 h-4"/>
        </button>
      </div>

      <div class="flex-1 min-h-0 grid grid-cols-[300px_1fr]">
        <!-- 提交列表 -->
        <aside class="border-r border-gray-200 dark:border-gray-700 overflow-y-auto" @scroll="onScroll">
          <div v-if="!commits.length && !loading" class="px-4 py-10 text-center text-sm text-gray-400">{{ t('git.logEmpty') }}</div>
          <button v-for="c in commits" :key="c.hash"
                  class="w-full text-left px-3 py-2 border-b border-gray-100 dark:border-gray-800 cursor-pointer"
                  :class="selected?.hash === c.hash ? 'bg-blue-50 dark:bg-blue-900/20' : 'hover:bg-gray-50 dark:hover:bg-gray-800'"
                  @click="select(c)">
            <div class="text-sm text-gray-800 dark:text-gray-100 truncate">{{ c.subject }}</div>
            <div class="mt-0.5 flex items-center gap-2 text-[11px] text-gray-400">
              <code class="text-amber-600 dark:text-amber-400">{{ c.short }}</code>
              <span class="truncate">{{ c.author }}</span>
              <span class="ml-auto flex-shrink-0">{{ c.date }}</span>
            </div>
          </button>
          <div v-if="loading" class="py-3 text-center text-xs text-gray-500">{{ t('git.loadMore') }}…</div>
          <div v-else-if="done && commits.length" class="py-3 text-center text-xs text-gray-400">{{ t('git.allLoaded') }}</div>
        </aside>

        <!-- 提交详情补丁 -->
        <section class="min-w-0 overflow-auto bg-gray-50 dark:bg-gray-950">
          <pre v-if="patch" class="p-3 text-xs leading-relaxed font-mono whitespace-pre-wrap"><span
              v-for="(ln, i) in patchLines" :key="i" :class="lineClass(ln)">{{ ln }}
</span></pre>
          <div v-else class="h-full flex items-center justify-center text-sm text-gray-400">{{ t('git.history') }}</div>
        </section>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, onMounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {History, X} from 'lucide-vue-next'
import {useToast} from '../plugins/toast'
import {useI18n} from 'vue-i18n'

interface GitCommit { hash: string; short: string; author: string; date: string; subject: string }

const props = defineProps<{ rootDir: string }>()
const emit = defineEmits<{ close: [] }>()

const toast = useToast()
const {t} = useI18n()

const PAGE = 50
const commits = ref<GitCommit[]>([])
const loading = ref(false)
const done = ref(false)
const selected = ref<GitCommit | null>(null)
const patch = ref('')

const patchLines = computed(() => patch.value.split('\n'))

// 补丁行着色：+ 绿 / - 红 / @@ 青 / diff·index 灰
const lineClass = (ln: string): string => {
  if (ln.startsWith('+') && !ln.startsWith('+++')) return 'text-green-600 dark:text-green-400'
  if (ln.startsWith('-') && !ln.startsWith('---')) return 'text-red-600 dark:text-red-400'
  if (ln.startsWith('@@')) return 'text-cyan-600 dark:text-cyan-400'
  if (ln.startsWith('diff ') || ln.startsWith('index ') || ln.startsWith('+++') || ln.startsWith('---')) return 'text-gray-400'
  return 'text-gray-700 dark:text-gray-300'
}

const loadMore = async () => {
  if (loading.value || done.value) {
    return
  }
  loading.value = true
  try {
    const batch = await invoke<GitCommit[]>('git_log', {root: props.rootDir, limit: PAGE, skip: commits.value.length})
    commits.value = [...commits.value, ...batch]
    if (batch.length < PAGE) {
      done.value = true
    }
  }
  catch (error) {
    toast.error(t('git.logFailed') + ': ' + error)
    done.value = true
  }
  finally {
    loading.value = false
  }
}

const onScroll = (e: Event) => {
  const el = e.target as HTMLElement
  if (el.scrollHeight - el.scrollTop - el.clientHeight < 80) {
    loadMore()
  }
}

const select = async (c: GitCommit) => {
  selected.value = c
  patch.value = ''
  try {
    patch.value = await invoke<string>('git_show', {root: props.rootDir, hash: c.hash})
  }
  catch (error) {
    toast.error(t('git.showFailed') + ': ' + error)
  }
}

onMounted(async () => {
  await loadMore()
  if (commits.value.length) {
    await select(commits.value[0])
  }
})
</script>
