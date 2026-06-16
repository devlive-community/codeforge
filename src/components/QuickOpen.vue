<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-24" @click="emit('close')">
    <div class="w-[560px] max-w-[90vw] bg-white dark:bg-gray-800 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col max-h-[60vh]"
         @click.stop>
      <div class="flex items-center px-3 border-b border-gray-200 dark:border-gray-700">
        <Search class="w-4 h-4 text-gray-400 flex-shrink-0"/>
        <input ref="inputRef"
               v-model="query"
               class="flex-1 px-2 py-2.5 text-sm bg-transparent focus:outline-none"
               :placeholder="t('dialog.quickOpenPlaceholder')"
               @keydown.down.prevent="move(1)"
               @keydown.up.prevent="move(-1)"
               @keydown.enter.prevent="choose(filtered[activeIndex])"
               @keydown.esc.prevent="emit('close')"/>
      </div>

      <div ref="listRef" class="overflow-y-auto py-1">
        <div v-if="loading" class="px-4 py-6 text-center text-sm text-gray-400">{{ t('dialog.quickOpenLoading') }}</div>
        <div v-else-if="filtered.length === 0" class="px-4 py-6 text-center text-sm text-gray-400">{{ t('dialog.quickOpenEmpty') }}</div>

        <button v-for="(file, i) in filtered"
                :key="file.path"
                :ref="el => setItemRef(el, i)"
                class="w-full flex items-center px-3 py-1.5 text-left cursor-pointer"
                :class="i === activeIndex ? 'bg-blue-100 dark:bg-gray-700' : 'hover:bg-gray-100 dark:hover:bg-gray-700'"
                @click="choose(file)"
                @mousemove="activeIndex = i">
          <FileText class="w-4 h-4 text-gray-400 flex-shrink-0 mr-2"/>
          <span class="text-sm text-gray-800 dark:text-gray-100 truncate">{{ file.name }}</span>
          <span class="ml-2 text-xs text-gray-400 truncate">{{ file.dir }}</span>
          <span v-if="!query && recentSet.has(file.path)" class="ml-auto pl-2 text-[10px] text-blue-500 flex-shrink-0">{{ t('dialog.recent') }}</span>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, nextTick, onMounted, ref, watch} from 'vue'
import {useI18n} from 'vue-i18n'
import {invoke} from '@tauri-apps/api/core'
import {FileText, Search} from 'lucide-vue-next'

const {t} = useI18n()

interface FileItem
{
  path: string
  name: string
  dir: string
  lower: string
}

const props = defineProps<{ rootDir: string; recentPaths?: string[] }>()
const emit = defineEmits<{ select: [path: string]; close: [] }>()

const query = ref('')
const loading = ref(true)
const allFiles = ref<FileItem[]>([])
const activeIndex = ref(0)
const inputRef = ref<HTMLInputElement | null>(null)
const listRef = ref<HTMLElement | null>(null)
const itemRefs: HTMLElement[] = []
const setItemRef = (el: any, i: number) => {
  if (el) itemRefs[i] = el
}

const recentSet = computed(() => new Set(props.recentPaths || []))

const rel = (p: string) => p.startsWith(props.rootDir) ? p.slice(props.rootDir.length + 1) : p

onMounted(async () => {
  inputRef.value?.focus()
  try {
    const files = await invoke<string[]>('list_files', {path: props.rootDir})
    allFiles.value = files.map(p => {
      const relPath = rel(p)
      const name = relPath.split(/[\\/]/).pop() || relPath
      const dir = relPath.slice(0, relPath.length - name.length).replace(/[\\/]$/, '')
      return {path: p, name, dir, lower: relPath.toLowerCase()}
    })
  }
  catch (error) {
    console.error('列出文件失败:', error)
  }
  finally {
    loading.value = false
  }
})

// 空查询时：最近打开的文件优先（按最近顺序），其余随后
const emptyOrdered = computed<FileItem[]>(() => {
  const recent = props.recentPaths || []
  if (!recent.length) {
    return allFiles.value.slice(0, 50)
  }
  const byPath = new Map(allFiles.value.map(f => [f.path, f]))
  const recentItems: FileItem[] = []
  for (const p of recent) {
    const f = byPath.get(p)
    if (f) {
      recentItems.push(f)
      byPath.delete(p)
    }
  }
  return [...recentItems, ...byPath.values()].slice(0, 50)
})

const filtered = computed<FileItem[]>(() => {
  const q = query.value.trim().toLowerCase()
  if (!q) {
    return emptyOrdered.value
  }
  const scored = allFiles.value
      .map(f => {
        const nameLower = f.name.toLowerCase()
        let score = -1
        if (nameLower.startsWith(q)) score = 0
        else if (nameLower.includes(q)) score = 1
        else if (f.lower.includes(q)) score = 2
        return {f, score}
      })
      .filter(x => x.score >= 0)
      .sort((a, b) => a.score - b.score || a.f.name.length - b.f.name.length)
  return scored.slice(0, 50).map(x => x.f)
})

watch(filtered, () => {
  activeIndex.value = 0
})

const move = (delta: number) => {
  const len = filtered.value.length
  if (len === 0) return
  activeIndex.value = (activeIndex.value + delta + len) % len
  nextTick(() => itemRefs[activeIndex.value]?.scrollIntoView({block: 'nearest'}))
}

const choose = (file?: FileItem) => {
  if (file) {
    emit('select', file.path)
    emit('close')
  }
}
</script>
