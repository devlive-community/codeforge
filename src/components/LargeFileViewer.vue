<template>
  <div class="absolute inset-0 z-10 flex flex-col bg-white dark:bg-gray-900">
    <!-- 顶部信息栏 -->
    <div class="flex items-center justify-between px-4 py-2 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 flex-shrink-0">
      <div class="flex items-center space-x-3 min-w-0">
        <FileText class="w-4 h-4 text-gray-500 flex-shrink-0"/>
        <span class="text-sm font-medium text-gray-700 dark:text-gray-200 truncate">{{ fileName }}</span>
        <span class="text-xs px-1.5 py-0.5 rounded bg-amber-100 text-amber-700 flex-shrink-0">{{ t('largeFile.readonly') }}</span>
        <span class="text-xs text-gray-400 flex-shrink-0">{{ humanSize }} · {{ t('largeFile.lineCount', { n: lineCount.toLocaleString() }) }}</span>
      </div>
      <div class="flex items-center space-x-3 flex-shrink-0">
        <span class="text-xs text-gray-400">{{ rangeText }}</span>
        <button class="p-1 rounded text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-700 cursor-pointer" :title="t('largeFile.close')" @click="emit('close')">
          <X class="w-4 h-4"/>
        </button>
      </div>
    </div>

    <!-- 虚拟滚动区域 -->
    <div ref="scroller" class="flex-1 overflow-auto font-mono text-sm leading-5 bg-white dark:bg-gray-900" @scroll="onScroll">
      <div :style="{ paddingTop: `${topPad}px`, paddingBottom: `${bottomPad}px` }">
        <div v-for="(line, i) in windowLines"
             :key="windowStart + i"
             class="flex"
             :style="{ height: `${lineHeight}px` }">
          <span class="w-16 pr-3 text-right text-gray-400 select-none flex-shrink-0 bg-gray-50 dark:bg-gray-800 border-r border-gray-100 dark:border-gray-700">{{ windowStart + i + 1 }}</span>
          <pre class="pl-3 whitespace-pre text-gray-800 dark:text-gray-200">{{ line }}</pre>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, onMounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {useI18n} from 'vue-i18n'
import {FileText, X} from 'lucide-vue-next'

const {t} = useI18n()

const props = defineProps<{
  filePath: string
  lineCount: number
  sizeBytes: number
}>()

const emit = defineEmits<{ close: [] }>()

const lineHeight = 20
const buffer = 40

const scroller = ref<HTMLElement | null>(null)
const windowStart = ref(0)
const windowLines = ref<string[]>([])
let loading = false
let pending = false

const fileName = computed(() => props.filePath.split(/[\\/]/).pop() || props.filePath)

const humanSize = computed(() => {
  const b = props.sizeBytes
  if (b < 1024) return `${b} B`
  if (b < 1024 * 1024) return `${(b / 1024).toFixed(1)} KB`
  return `${(b / 1024 / 1024).toFixed(1)} MB`
})

const topPad = computed(() => windowStart.value * lineHeight)
const bottomPad = computed(() => Math.max(0, (props.lineCount - windowStart.value - windowLines.value.length) * lineHeight))

const rangeText = computed(() => {
  if (windowLines.value.length === 0) return ''
  return t('largeFile.range', { from: windowStart.value + 1, to: windowStart.value + windowLines.value.length })
})

const visibleCount = () => Math.ceil((scroller.value?.clientHeight || 600) / lineHeight)

const loadWindow = async () => {
  if (!scroller.value) return
  const first = Math.max(0, Math.floor(scroller.value.scrollTop / lineHeight) - buffer)
  const count = visibleCount() + buffer * 2

  // 当前窗口已覆盖可见范围则跳过
  const visibleStart = Math.floor(scroller.value.scrollTop / lineHeight)
  const loadedEnd = windowStart.value + windowLines.value.length
  if (windowLines.value.length
      && visibleStart >= windowStart.value
      && visibleStart + visibleCount() <= loadedEnd) {
    return
  }

  if (loading) {
    pending = true
    return
  }
  loading = true
  try {
    const lines = await invoke<string[]>('read_file_lines', {path: props.filePath, start: first, count})
    windowStart.value = first
    windowLines.value = lines
  }
  catch (error) {
    console.error(t('largeFile.readFailed'), error)
  }
  loading = false
  if (pending) {
    pending = false
    loadWindow()
  }
}

let scrollTimer: any = null
const onScroll = () => {
  clearTimeout(scrollTimer)
  scrollTimer = setTimeout(loadWindow, 60)
}

onMounted(loadWindow)
</script>
