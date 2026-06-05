<template>
  <div class="fixed inset-0 z-50 flex justify-center pt-16 px-6 pb-6" @click="emit('close')">
    <div class="w-full max-w-[980px] bg-white dark:bg-gray-800 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col"
         @click.stop>
      <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
          <GitCompare class="w-4 h-4 text-gray-400"/>
          <span>差异对比</span>
          <span v-if="fileName" class="text-xs text-gray-400">· {{ fileName }}</span>
        </div>
        <div class="flex items-center gap-3 text-xs">
          <span class="text-green-600 dark:text-green-400">+{{ added }}</span>
          <span class="text-red-600 dark:text-red-400">−{{ removed }}</span>
          <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" title="关闭" @click="emit('close')">
            <X class="w-4 h-4"/>
          </button>
        </div>
      </div>

      <div class="flex items-center px-4 py-1.5 text-xs text-gray-400 border-b border-gray-100 dark:border-gray-700 flex-shrink-0">
        已保存（红） → 当前（绿）
      </div>

      <div class="flex-1 overflow-auto font-mono text-xs leading-5 max-h-[70vh]">
        <div v-if="rows.length === 0" class="px-4 py-10 text-center text-sm text-gray-400">没有差异，内容一致</div>
        <div v-for="(row, i) in rows" :key="i"
             class="flex"
             :class="rowClass(row.type)">
          <span class="w-10 flex-shrink-0 text-right pr-2 select-none text-gray-400 dark:text-gray-500">{{ row.oldNo || '' }}</span>
          <span class="w-10 flex-shrink-0 text-right pr-2 select-none text-gray-400 dark:text-gray-500">{{ row.newNo || '' }}</span>
          <span class="w-4 flex-shrink-0 select-none text-center">{{ sign(row.type) }}</span>
          <span class="flex-1 whitespace-pre-wrap break-all pr-3">{{ row.text }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed} from 'vue'
import {GitCompare, X} from 'lucide-vue-next'

const props = defineProps<{
  original: string
  modified: string
  fileName?: string | null
}>()
const emit = defineEmits<{ close: [] }>()

interface DiffRow { type: 'same' | 'add' | 'del'; text: string; oldNo?: number; newNo?: number }

// 基于 LCS 的逐行差异（适用于编辑器规模的文本）
const rows = computed<DiffRow[]>(() => {
  const a = props.original.split('\n')
  const b = props.modified.split('\n')
  const n = a.length, m = b.length

  // 超大文件不做精细 diff，避免 O(n*m) 卡顿
  if (n * m > 4_000_000) {
    return [{type: 'del' as const, text: '（文件过大，无法显示逐行差异）'}]
  }

  const dp: number[][] = Array.from({length: n + 1}, () => new Array(m + 1).fill(0))
  for (let i = n - 1; i >= 0; i--) {
    for (let j = m - 1; j >= 0; j--) {
      dp[i][j] = a[i] === b[j] ? dp[i + 1][j + 1] + 1 : Math.max(dp[i + 1][j], dp[i][j + 1])
    }
  }

  const out: DiffRow[] = []
  let i = 0, j = 0, oldNo = 0, newNo = 0
  while (i < n && j < m) {
    if (a[i] === b[j]) {
      out.push({type: 'same', text: a[i], oldNo: ++oldNo, newNo: ++newNo})
      i++; j++
    }
    else if (dp[i + 1][j] >= dp[i][j + 1]) {
      out.push({type: 'del', text: a[i], oldNo: ++oldNo})
      i++
    }
    else {
      out.push({type: 'add', text: b[j], newNo: ++newNo})
      j++
    }
  }
  while (i < n) out.push({type: 'del', text: a[i++], oldNo: ++oldNo})
  while (j < m) out.push({type: 'add', text: b[j++], newNo: ++newNo})

  // 全部相同则视为无差异
  return out.every(r => r.type === 'same') ? [] : out
})

const added = computed(() => rows.value.filter(r => r.type === 'add').length)
const removed = computed(() => rows.value.filter(r => r.type === 'del').length)

const rowClass = (t: DiffRow['type']) =>
    t === 'add' ? 'bg-green-50 dark:bg-green-900/25 text-green-800 dark:text-green-300'
        : t === 'del' ? 'bg-red-50 dark:bg-red-900/25 text-red-800 dark:text-red-300'
            : 'text-gray-700 dark:text-gray-300'

const sign = (t: DiffRow['type']) => (t === 'add' ? '+' : t === 'del' ? '−' : '')
</script>
