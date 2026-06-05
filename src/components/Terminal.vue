<template>
  <div class="flex flex-col flex-shrink-0 bg-[#1e1e1e] border-t border-gray-200 dark:border-gray-700"
       :style="{ height: `${height}px` }">
    <!-- 顶部拖拽改高度 -->
    <div class="h-1 cursor-row-resize bg-gray-200 dark:bg-gray-700 hover:bg-blue-500 transition-colors flex-shrink-0"
         @mousedown="startResize"></div>

    <!-- 标签栏 -->
    <div class="flex items-stretch bg-gray-100 dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 flex-shrink-0 overflow-x-auto
                [&::-webkit-scrollbar]:hidden [-ms-overflow-style:none] [scrollbar-width:none]">
      <div class="flex items-center gap-2 px-2 text-xs font-medium text-gray-500 dark:text-gray-400 flex-shrink-0">
        <TerminalIcon class="w-3.5 h-3.5"/>
      </div>
      <div v-for="s in sessions"
           :key="s.id"
           class="group flex items-center gap-1.5 pl-3 pr-2 py-1 border-r border-gray-200 dark:border-gray-700 cursor-pointer text-xs flex-shrink-0"
           :class="s.id === activeId ? 'bg-[#1e1e1e] text-gray-200' : 'text-gray-500 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-700'"
           @click="switchTo(s.id)">
        <span>{{ s.title }}</span>
        <span v-if="s.exited" class="text-gray-500">·已结束</span>
        <button class="rounded p-0.5 text-gray-400 hover:text-gray-200 hover:bg-gray-600 opacity-0 group-hover:opacity-100 transition-opacity"
                :class="{ 'opacity-100': s.id === activeId }"
                title="关闭"
                @click.stop="closeSession(s.id)">
          <X class="w-3 h-3"/>
        </button>
      </div>
      <button class="flex items-center justify-center px-2 text-gray-400 hover:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-700 cursor-pointer flex-shrink-0"
              title="新建终端"
              @click="createSession">
        <Plus class="w-3.5 h-3.5"/>
      </button>
      <div class="flex-1"></div>
      <button class="flex items-center justify-center px-2 text-gray-400 hover:text-gray-200 cursor-pointer flex-shrink-0"
              title="关闭终端面板"
              @click="emit('close')">
        <ChevronDown class="w-4 h-4"/>
      </button>
    </div>

    <!-- 各会话的容器：仅显示当前激活的 -->
    <div class="flex-1 overflow-hidden relative">
      <div v-for="s in sessions"
           :key="s.id"
           v-show="s.id === activeId"
           :ref="el => setContainerRef(s.id, el)"
           class="absolute inset-0 px-2 py-1"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {nextTick, onBeforeUnmount, onMounted, ref, watch} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {listen, type UnlistenFn} from '@tauri-apps/api/event'
import {Terminal} from '@xterm/xterm'
import {FitAddon} from '@xterm/addon-fit'
import '@xterm/xterm/css/xterm.css'
import {ChevronDown, Plus, X, Terminal as TerminalIcon} from 'lucide-vue-next'

const props = defineProps<{ rootDir?: string | null }>()
const emit = defineEmits<{ close: [] }>()

interface Sess { id: string; title: string; exited: boolean }

const sessions = ref<Sess[]>([])
const activeId = ref('')
const height = ref(300)

// xterm 实例与监听器放普通 Map（避免被 Vue 代理）
interface TermBundle {
  term: Terminal
  fit: FitAddon
  unlistenOut?: UnlistenFn
  unlistenExit?: UnlistenFn
  observer?: ResizeObserver
}
const terms = new Map<string, TermBundle>()
const containers = new Map<string, HTMLElement>()

let seq = 0
const genId = () => `term-${Date.now()}-${Math.random().toString(36).slice(2, 6)}`
const termTheme = {background: '#1e1e1e', foreground: '#d4d4d4', cursor: '#d4d4d4'}

const setContainerRef = (id: string, el: any) => {
  if (el) {
    containers.set(id, el as HTMLElement)
  }
}

const fitOne = (id: string) => {
  const b = terms.get(id)
  if (!b) {
    return
  }
  try {
    b.fit.fit()
    invoke('terminal_resize', {id, cols: b.term.cols, rows: b.term.rows}).catch(() => {})
  }
  catch {
    // 容器尺寸为 0 时忽略
  }
}

const initXterm = async (id: string) => {
  await nextTick()
  const el = containers.get(id)
  if (!el) {
    return
  }
  const term = new Terminal({
    fontSize: 13,
    fontFamily: 'Menlo, Monaco, "Courier New", monospace',
    cursorBlink: true,
    theme: termTheme
  })
  const fit = new FitAddon()
  term.loadAddon(fit)
  term.open(el)
  const bundle: TermBundle = {term, fit}
  terms.set(id, bundle)
  fitOne(id)

  bundle.unlistenOut = await listen<{ id: string, data: number[] }>('terminal-output', (e) => {
    if (e.payload.id === id) {
      term.write(new Uint8Array(e.payload.data))
    }
  })
  bundle.unlistenExit = await listen<string>('terminal-exit', (e) => {
    if (e.payload === id) {
      const s = sessions.value.find(x => x.id === id)
      if (s) {
        s.exited = true
      }
      term.write('\r\n\x1b[90m[进程已退出]\x1b[0m\r\n')
    }
  })
  term.onData((data) => {
    invoke('terminal_write', {id, data}).catch(() => {})
  })

  try {
    await invoke('terminal_create', {id, cwd: props.rootDir || null, cols: term.cols, rows: term.rows})
  }
  catch (error) {
    term.write(`\r\n\x1b[31m启动终端失败: ${error}\x1b[0m\r\n`)
  }

  bundle.observer = new ResizeObserver(() => fitOne(id))
  bundle.observer.observe(el)
  term.focus()
}

const createSession = () => {
  const id = genId()
  seq++
  sessions.value.push({id, title: `终端 ${seq}`, exited: false})
  activeId.value = id
  initXterm(id)
}

const switchTo = (id: string) => {
  if (id === activeId.value) {
    return
  }
  activeId.value = id
  nextTick(() => {
    fitOne(id)
    terms.get(id)?.term.focus()
  })
}

const disposeSession = (id: string) => {
  const b = terms.get(id)
  if (b) {
    b.observer?.disconnect()
    b.unlistenOut?.()
    b.unlistenExit?.()
    invoke('terminal_kill', {id}).catch(() => {})
    b.term.dispose()
    terms.delete(id)
  }
  containers.delete(id)
}

const closeSession = (id: string) => {
  disposeSession(id)
  const idx = sessions.value.findIndex(s => s.id === id)
  if (idx >= 0) {
    sessions.value.splice(idx, 1)
  }
  if (sessions.value.length === 0) {
    emit('close')
    return
  }
  if (activeId.value === id) {
    const next = sessions.value[Math.min(idx, sessions.value.length - 1)]
    switchTo(next.id)
  }
}

watch(height, () => nextTick(() => fitOne(activeId.value)))

onMounted(() => createSession())

onBeforeUnmount(() => {
  for (const s of sessions.value) {
    disposeSession(s.id)
  }
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
