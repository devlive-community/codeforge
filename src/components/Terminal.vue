<template>
  <div class="fixed left-0 right-0 bottom-0 z-40 flex flex-col bg-[#1e1e1e] border-t border-gray-300 dark:border-gray-700 shadow-2xl"
       :style="{ height: `${height}px` }">
    <!-- 顶部拖拽改高度 + 标题栏 -->
    <div class="h-1 cursor-row-resize bg-transparent hover:bg-blue-500 transition-colors flex-shrink-0"
         @mousedown="startResize"></div>
    <div class="flex items-center justify-between px-3 py-1 bg-gray-100 dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
      <div class="flex items-center gap-2 text-xs font-medium text-gray-600 dark:text-gray-300">
        <TerminalIcon class="w-3.5 h-3.5 text-gray-400"/>
        <span>终端</span>
        <span v-if="exited" class="text-gray-400">（已结束）</span>
      </div>
      <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" title="关闭终端" @click="emit('close')">
        <X class="w-4 h-4"/>
      </button>
    </div>
    <div ref="termEl" class="flex-1 overflow-hidden px-2 py-1"></div>
  </div>
</template>

<script setup lang="ts">
import {nextTick, onBeforeUnmount, onMounted, ref, watch} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {listen, type UnlistenFn} from '@tauri-apps/api/event'
import {Terminal} from '@xterm/xterm'
import {FitAddon} from '@xterm/addon-fit'
import '@xterm/xterm/css/xterm.css'
import {X, Terminal as TerminalIcon} from 'lucide-vue-next'

const props = defineProps<{ rootDir?: string | null }>()
const emit = defineEmits<{ close: [] }>()

const termEl = ref<HTMLElement | null>(null)
const height = ref(300)
const exited = ref(false)

const id = `term-${Date.now()}-${Math.random().toString(36).slice(2, 6)}`
let term: Terminal | null = null
let fit: FitAddon | null = null
let unlistenOut: UnlistenFn | null = null
let unlistenExit: UnlistenFn | null = null
let resizeObserver: ResizeObserver | null = null

// 终端始终使用深色配色（不随应用主题切换）
const termTheme = {background: '#1e1e1e', foreground: '#d4d4d4', cursor: '#d4d4d4'}

const doFit = () => {
  if (!fit || !term) {
    return
  }
  try {
    fit.fit()
    invoke('terminal_resize', {id, cols: term.cols, rows: term.rows}).catch(() => {})
  }
  catch {
    // 容器尺寸为 0 时忽略
  }
}

onMounted(async () => {
  await nextTick()
  if (!termEl.value) {
    return
  }
  term = new Terminal({
    fontSize: 13,
    fontFamily: 'Menlo, Monaco, "Courier New", monospace',
    cursorBlink: true,
    theme: termTheme
  })
  fit = new FitAddon()
  term.loadAddon(fit)
  term.open(termEl.value)
  doFit()

  // 接收 PTY 输出
  unlistenOut = await listen<{ id: string, data: number[] }>('terminal-output', (e) => {
    if (e.payload.id === id && term) {
      term.write(new Uint8Array(e.payload.data))
    }
  })
  unlistenExit = await listen<string>('terminal-exit', (e) => {
    if (e.payload === id) {
      exited.value = true
      term?.write('\r\n\x1b[90m[进程已退出]\x1b[0m\r\n')
    }
  })

  // 用户键入 → 写入 PTY
  term.onData((data) => {
    invoke('terminal_write', {id, data}).catch(() => {})
  })

  // 启动 shell
  try {
    await invoke('terminal_create', {
      id,
      cwd: props.rootDir || null,
      cols: term.cols,
      rows: term.rows
    })
  }
  catch (error) {
    term.write(`\r\n\x1b[31m启动终端失败: ${error}\x1b[0m\r\n`)
  }

  term.focus()

  // 容器尺寸变化时自适应
  resizeObserver = new ResizeObserver(() => doFit())
  resizeObserver.observe(termEl.value)
})

// 面板高度变化后重新 fit
watch(height, () => nextTick(doFit))

onBeforeUnmount(() => {
  resizeObserver?.disconnect()
  unlistenOut?.()
  unlistenExit?.()
  invoke('terminal_kill', {id}).catch(() => {})
  term?.dispose()
})

// ===== 拖拽改高度 =====
let startY = 0
let startH = 0
const onResize = (e: MouseEvent) => {
  const h = startH + (startY - e.clientY)
  height.value = Math.max(140, Math.min(window.innerHeight - 120, h))
}
const stopResize = () => {
  document.removeEventListener('mousemove', onResize)
  document.removeEventListener('mouseup', stopResize)
  document.body.style.userSelect = ''
}
const startResize = (e: MouseEvent) => {
  e.preventDefault()
  startY = e.clientY
  startH = height.value
  document.addEventListener('mousemove', onResize)
  document.addEventListener('mouseup', stopResize)
  document.body.style.userSelect = 'none'
}
</script>
