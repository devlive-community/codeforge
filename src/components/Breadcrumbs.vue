<template>
  <span class="relative flex items-center text-xs text-gray-500 w-max whitespace-nowrap">
    <template v-for="(seg, i) in segments" :key="i">
      <ChevronRight v-if="i > 0" class="w-3 h-3 mx-0.5 text-gray-300 dark:text-gray-600 flex-shrink-0"/>
      <button class="hover:text-blue-500 cursor-pointer flex-shrink-0"
              :class="{ 'text-blue-500': dropdown.index === i }"
              :title="seg.full"
              @click.stop="toggleDropdown(i, seg, $event)">
        {{ seg.name }}
      </button>
    </template>
    <span v-if="dirty" class="ml-1 text-amber-500 flex-shrink-0" :title="t('breadcrumb.unsaved')">●</span>

    <!-- 段落下拉：列出该层目录内容，便于快速切换同级文件/钻入子目录 -->
    <div v-if="dropdown.index !== null"
         class="fixed inset-0 z-40"
         @click="closeDropdown"
         @contextmenu.prevent="closeDropdown">
      <div class="absolute bg-white dark:bg-gray-800 dark:text-gray-100 rounded-md shadow-lg border border-gray-200 dark:border-gray-700 py-1 text-sm min-w-[180px] max-h-[50vh] overflow-y-auto"
           :style="{ top: `${dropdown.y}px`, left: `${dropdown.x}px` }"
           @click.stop>
        <div v-if="dropdown.loading" class="px-3 py-2 text-gray-400">{{ t('breadcrumb.loading') }}</div>
        <div v-else-if="dropdown.entries.length === 0" class="px-3 py-2 text-gray-400">{{ t('breadcrumb.empty') }}</div>
        <button v-for="e in dropdown.entries"
                :key="e.path"
                class="w-full flex items-center gap-1.5 px-3 py-1 text-left hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer"
                :class="{ 'text-blue-600 dark:text-blue-400': e.path === path }"
                @click="pick(e)">
          <component :is="e.is_dir ? Folder : File" class="w-3.5 h-3.5 flex-shrink-0" :class="e.is_dir ? 'text-blue-500' : 'text-gray-400'"/>
          <span class="truncate">{{ e.name }}</span>
        </button>
      </div>
    </div>
  </span>
</template>

<script setup lang="ts">
import {computed, reactive} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {useI18n} from 'vue-i18n'
import {ChevronRight, File, Folder} from 'lucide-vue-next'

const {t} = useI18n()

const props = defineProps<{
  path: string
  rootDir?: string | null
  dirty?: boolean
}>()
const emit = defineEmits<{ reveal: [path: string]; open: [path: string] }>()

interface Seg { name: string; full: string }
interface Entry { name: string; path: string; is_dir: boolean }

const segments = computed<Seg[]>(() => {
  const p = props.path
  const root = props.rootDir
  // 在已打开文件夹内：以文件夹名为首段，再展开相对层级
  if (root && p.startsWith(root)) {
    const base = root.replace(/[\\/]$/, '')
    const rootName = base.split(/[\\/]/).pop() || base
    const rel = p.slice(root.length).replace(/^[\\/]/, '')
    const parts = rel.split(/[\\/]/).filter(Boolean)
    const segs: Seg[] = [{name: rootName, full: base}]
    let acc = base
    for (const part of parts) {
      acc = `${acc}/${part}`
      segs.push({name: part, full: acc})
    }
    return segs
  }
  // 未打开文件夹：显示绝对路径（过长则保留末 4 段，full 仍为完整路径）
  const parts = p.split(/[\\/]/).filter(Boolean)
  const segs: Seg[] = []
  let acc = ''
  for (const part of parts) {
    acc = `${acc}/${part}`
    segs.push({name: part, full: acc})
  }
  return segs.length > 5 ? segs.slice(-4) : segs
})

const dirOf = (p: string) => p.replace(/[\\/][^\\/]*$/, '')

const dropdown = reactive<{ index: number | null, x: number, y: number, loading: boolean, entries: Entry[] }>({
  index: null, x: 0, y: 0, loading: false, entries: []
})

const isLastSeg = (i: number) => i === segments.value.length - 1

const loadDir = async (dir: string) => {
  dropdown.loading = true
  try {
    const all = await invoke<Entry[]>('read_directory_tree', {path: dir})
    // 目录在前、各自按名排序
    dropdown.entries = [...all].sort((a, b) =>
        a.is_dir === b.is_dir ? a.name.localeCompare(b.name) : (a.is_dir ? -1 : 1))
  }
  catch {
    dropdown.entries = []
  }
  finally {
    dropdown.loading = false
  }
}

const toggleDropdown = (i: number, seg: Seg, e: MouseEvent) => {
  if (dropdown.index === i) {
    closeDropdown()
    return
  }
  const rect = (e.currentTarget as HTMLElement).getBoundingClientRect()
  dropdown.x = rect.left
  dropdown.y = rect.bottom + 2
  dropdown.index = i
  // 文件段落列出其同级（父目录内容）；目录段落列出自身内容
  loadDir(isLastSeg(i) ? dirOf(seg.full) : seg.full)
}

const closeDropdown = () => {
  dropdown.index = null
}

const pick = (e: Entry) => {
  if (e.is_dir) {
    // 钻入子目录：保持下拉打开并切换到该目录内容
    loadDir(e.path)
  }
  else {
    emit('open', e.path)
    closeDropdown()
  }
}
</script>
