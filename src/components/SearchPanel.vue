<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-20" @click="emit('close')">
    <div class="w-[680px] max-w-[92vw] bg-white dark:bg-gray-800 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col max-h-[70vh]"
         @click.stop>
      <div class="flex items-center px-3 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <Search class="w-4 h-4 text-gray-400 flex-shrink-0"/>
        <input ref="inputRef"
               v-model="query"
               class="flex-1 px-2 py-2.5 text-sm bg-transparent focus:outline-none"
               :placeholder="t('search.placeholder')"
               @input="onInput"
               @keydown.esc.prevent="emit('close')"/>
        <span class="text-xs text-gray-400 flex-shrink-0">
          {{ loading ? t('search.searching') : results.length ? t('search.countSummary', { count: results.length, files: groups.length }) : '' }}
        </span>
      </div>

      <!-- 替换行 -->
      <div class="flex items-center px-3 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <Replace class="w-4 h-4 text-gray-400 flex-shrink-0"/>
        <input v-model="replacement"
               class="flex-1 px-2 py-2.5 text-sm bg-transparent focus:outline-none"
               :placeholder="t('search.replacePlaceholder')"
               @keydown.esc.prevent="emit('close')"/>
        <button class="text-xs px-2 py-1 rounded bg-blue-500 hover:bg-blue-600 disabled:opacity-40 disabled:cursor-not-allowed text-white flex-shrink-0"
                :disabled="replacing || results.length === 0"
                :title="results.length ? t('search.replaceAllTitle', { count: results.length }) : t('search.searchFirst')"
                @click="confirming = true">
          {{ replacing ? t('search.replacing') : t('search.replaceAll') }}
        </button>
      </div>

      <!-- 全部替换确认 -->
      <div v-if="confirming" class="px-3 py-2 border-b border-amber-200 dark:border-amber-800 bg-amber-50 dark:bg-amber-900/20 flex-shrink-0 text-xs">
        <p class="text-amber-700 dark:text-amber-300 mb-2">
          {{ t('search.confirmLine', { count: results.length, query: query.trim(), rep: replacement, files: affectedCount }) }}<br/>
          {{ t('search.confirmWarn') }}
        </p>
        <div class="flex justify-end gap-2">
          <button class="px-2 py-1 rounded text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="confirming = false">{{ t('search.cancel') }}</button>
          <button class="px-2 py-1 rounded bg-red-500 hover:bg-red-600 text-white cursor-pointer" @click="replaceAll">{{ t('search.confirmReplace') }}</button>
        </div>
      </div>

      <div class="flex-1 overflow-y-auto">
        <div v-if="!loading && query && results.length === 0" class="px-4 py-6 text-center text-sm text-gray-400">{{ t('search.empty') }}</div>

        <div v-for="g in groups" :key="g.path">
          <div class="sticky top-0 bg-gray-50 dark:bg-gray-900 px-3 py-1.5 text-xs text-gray-500 dark:text-gray-400 border-b border-gray-100 dark:border-gray-700 flex items-center gap-1">
            <FileText class="w-3.5 h-3.5 text-gray-400"/>
            <span class="font-medium text-gray-700 dark:text-gray-200">{{ g.name }}</span>
            <span class="truncate">{{ g.dir }}</span>
            <span class="ml-auto text-gray-400">{{ g.items.length }}</span>
          </div>
          <button v-for="m in g.items"
                  :key="m.path + ':' + m.line"
                  class="w-full flex items-baseline gap-2 px-3 py-1 text-left text-xs hover:bg-blue-50 dark:hover:bg-gray-700 cursor-pointer"
                  @click="emit('open', m.path, m.line)">
            <span class="text-gray-400 w-10 text-right flex-shrink-0">{{ m.line }}</span>
            <span class="font-mono text-gray-700 dark:text-gray-200 truncate">{{ m.text.trim() }}</span>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, onMounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {FileText, Replace, Search} from 'lucide-vue-next'
import {useToast} from '../plugins/toast'
import {useI18n} from 'vue-i18n'

interface Match
{
  path: string
  line: number
  text: string
}

const props = defineProps<{ rootDir: string, extraRoots?: string[] }>()
// 主根 + 额外挂载的根，搜索/替换覆盖整个工作区
const allRoots = computed(() => [props.rootDir, ...(props.extraRoots || [])])
const emit = defineEmits<{
  open: [path: string, line: number]
  close: []
  replaced: [paths: string[]]
}>()

const toast = useToast()
const {t} = useI18n()

const query = ref('')
const replacement = ref('')
const replacing = ref(false)
const confirming = ref(false)
const results = ref<Match[]>([])
const affectedCount = computed(() => new Set(results.value.map(m => m.path)).size)
const loading = ref(false)
const inputRef = ref<HTMLInputElement | null>(null)

onMounted(() => inputRef.value?.focus())

// 全部替换：经应用内确认后调用后端，完成后刷新搜索并通知刷新已打开文件
const replaceAll = async () => {
  confirming.value = false
  const q = query.value.trim()
  if (q.length < 2 || results.value.length === 0) {
    return
  }
  const affected = Array.from(new Set(results.value.map(m => m.path)))
  replacing.value = true
  try {
    const summaries = await Promise.all(allRoots.value.map(r =>
        invoke<{ files_changed: number, replacements: number }>('replace_in_files', {
          root: r,
          query: q,
          replacement: replacement.value
        }).catch(() => ({files_changed: 0, replacements: 0}))
    ))
    const summary = summaries.reduce((a, b) => ({
      files_changed: a.files_changed + b.files_changed,
      replacements: a.replacements + b.replacements
    }), {files_changed: 0, replacements: 0})
    toast.success(t('search.replacedSummary', { count: summary.replacements, files: summary.files_changed }))
    emit('replaced', affected)
    await search()
  }
  catch (error) {
    toast.error(t('search.replaceFailed') + ': ' + error)
  }
  finally {
    replacing.value = false
  }
}

// 找到该路径所属的根（多根时取最长匹配，避免嵌套根误判）
const ownerRoot = (p: string) => allRoots.value
    .filter(r => p === r || p.startsWith(r + '/') || p.startsWith(r + '\\'))
    .sort((a, b) => b.length - a.length)[0] || ''
const folderName = (p: string) => p.split(/[\\/]/).filter(Boolean).pop() || p
const rel = (p: string) => {
  const root = ownerRoot(p)
  let r = root && p.startsWith(root) ? p.slice(root.length).replace(/^[\\/]/, '') : p
  // 多根时在相对路径前加上根名，便于区分来自哪个根
  if (allRoots.value.length > 1 && root) r = `${folderName(root)}/${r}`
  return r
}

const groups = computed(() => {
  const map = new Map<string, Match[]>()
  for (const m of results.value) {
    if (!map.has(m.path)) map.set(m.path, [])
    map.get(m.path)!.push(m)
  }
  return Array.from(map.entries()).map(([path, items]) => {
    const relPath = rel(path)
    const name = relPath.split(/[\\/]/).pop() || relPath
    const dir = relPath.slice(0, relPath.length - name.length).replace(/[\\/]$/, '')
    return {path, name, dir, items}
  })
})

let timer: any = null
const search = async () => {
  const q = query.value.trim()
  // 至少 2 个字符，避免在大目录上做无意义的重搜索
  if (q.length < 2) {
    results.value = []
    return
  }
  loading.value = true
  try {
    const perRoot = await Promise.all(
        allRoots.value.map(r => invoke<Match[]>('search_in_files', {root: r, query: q}).catch(() => []))
    )
    results.value = perRoot.flat()
  }
  catch (error) {
    console.error('搜索失败:', error)
    results.value = []
  }
  finally {
    loading.value = false
  }
}

const onInput = () => {
  clearTimeout(timer)
  timer = setTimeout(search, 300)
}
</script>
