<template>
  <div class="fixed inset-0 z-50 flex justify-center pt-20" @click="emit('close')">
    <div class="w-[680px] max-w-[92vw] bg-white dark:bg-gray-800 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col max-h-[70vh]"
         @click.stop>
      <div class="flex items-center px-3 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <Search class="w-4 h-4 text-gray-400 flex-shrink-0"/>
        <input ref="inputRef"
               v-model="query"
               class="flex-1 px-2 py-2.5 text-sm bg-transparent focus:outline-none"
               placeholder="在文件夹内搜索…"
               @input="onInput"
               @keydown.esc.prevent="emit('close')"/>
        <span class="text-xs text-gray-400 flex-shrink-0">
          {{ loading ? '搜索中…' : results.length ? `${results.length} 处 / ${groups.length} 文件` : '' }}
        </span>
      </div>

      <div class="flex-1 overflow-y-auto">
        <div v-if="!loading && query && results.length === 0" class="px-4 py-6 text-center text-sm text-gray-400">无匹配结果</div>

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
import {FileText, Search} from 'lucide-vue-next'

interface Match
{
  path: string
  line: number
  text: string
}

const props = defineProps<{ rootDir: string }>()
const emit = defineEmits<{ open: [path: string, line: number]; close: [] }>()

const query = ref('')
const results = ref<Match[]>([])
const loading = ref(false)
const inputRef = ref<HTMLInputElement | null>(null)

onMounted(() => inputRef.value?.focus())

const rel = (p: string) => p.startsWith(props.rootDir) ? p.slice(props.rootDir.length + 1) : p

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
    results.value = await invoke<Match[]>('search_in_files', {root: props.rootDir, query: q})
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
