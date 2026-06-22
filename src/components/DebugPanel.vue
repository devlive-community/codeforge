<template>
  <div v-if="debug.status.value !== 'inactive'"
       class="fixed top-0 right-0 bottom-0 z-30 w-[300px] bg-white dark:bg-gray-900 dark:text-gray-100 border-l border-gray-200 dark:border-gray-700 shadow-xl flex flex-col">
    <div class="flex items-center justify-between px-3 py-2 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
      <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
        <Bug class="w-4 h-4 text-gray-400"/>
        <span>{{ t('debug.panelTitle') }}</span>
      </div>
      <button class="text-gray-400 hover:text-red-500 cursor-pointer" :title="t('debug.stop')" @click="debug.stopSession()">
        <Square class="w-4 h-4"/>
      </button>
    </div>

    <div class="flex-1 min-h-0 overflow-y-auto">
      <!-- 监视 -->
      <div class="px-3 py-1.5 text-[11px] font-semibold text-gray-500 dark:text-gray-400 flex items-center gap-2">
        <span>{{ t('debug.watch') }}</span>
        <input v-model="watchInput"
               class="flex-1 min-w-0 text-[11px] font-mono border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-1.5 py-0.5 focus:outline-none focus:border-blue-500"
               :placeholder="t('debug.watchPlaceholder')"
               @keydown.enter="addWatch"/>
      </div>
      <div v-for="(w, i) in debug.watches.value" :key="i"
           class="group flex items-start gap-1 px-3 py-0.5 font-mono text-[11px] leading-5 hover:bg-gray-100 dark:hover:bg-gray-800">
        <span class="text-purple-600 dark:text-purple-300 flex-shrink-0">{{ w.expr }}</span>
        <span class="text-gray-400">:</span>
        <span class="text-gray-700 dark:text-gray-200 break-all flex-1">{{ w.value }}</span>
        <button class="text-gray-400 hover:text-red-500 opacity-0 group-hover:opacity-100 cursor-pointer flex-shrink-0" @click="debug.removeWatch(i)">×</button>
      </div>

      <!-- 调用栈 -->
      <div class="px-3 py-1.5 mt-1 text-[11px] font-semibold text-gray-500 dark:text-gray-400 border-t border-gray-100 dark:border-gray-800">{{ t('debug.callStack') }}</div>
      <div v-if="!debug.frames.value.length" class="px-3 py-2 text-[11px] text-gray-400">{{ t('debug.notStopped') }}</div>
      <button v-for="f in debug.frames.value" :key="f.id"
              class="w-full text-left px-3 py-1 text-[11px] font-mono truncate cursor-pointer"
              :class="debug.selectedFrameId.value === f.id ? 'bg-blue-50 dark:bg-blue-900/30' : 'hover:bg-gray-100 dark:hover:bg-gray-800'"
              @click="debug.selectFrame(f.id)">
        <span class="text-gray-800 dark:text-gray-100">{{ f.name }}</span>
        <span class="text-gray-400"> · {{ f.source?.name || '' }}:{{ f.line }}</span>
      </button>

      <!-- 变量 -->
      <div class="px-3 py-1.5 mt-1 text-[11px] font-semibold text-gray-500 dark:text-gray-400 border-t border-gray-100 dark:border-gray-800">{{ t('debug.variables') }}</div>
      <div v-if="!scopes.length" class="px-3 py-2 text-[11px] text-gray-400">—</div>
      <div v-for="scope in scopes" :key="scope.variablesReference" class="pb-1">
        <button class="w-full text-left px-2 py-0.5 text-[11px] font-medium text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 cursor-pointer flex items-center gap-1"
                @click="toggleScope(scope.variablesReference)">
          <ChevronRight class="w-3 h-3 transition-transform" :class="{ 'rotate-90': openScopes.has(scope.variablesReference) }"/>
          {{ scope.name }}
        </button>
        <template v-if="openScopes.has(scope.variablesReference)">
          <DebugVarNode v-for="(v, i) in (scopeVars.get(scope.variablesReference) || [])" :key="i" :variable="v" :depth="1"/>
        </template>
      </div>

      <!-- 断点列表 -->
      <div class="px-3 py-1.5 mt-1 text-[11px] font-semibold text-gray-500 dark:text-gray-400 border-t border-gray-100 dark:border-gray-800">{{ t('debug.breakpoints') }}</div>
      <div v-if="!bps.length" class="px-3 py-1 text-[11px] text-gray-400">—</div>
      <div v-for="b in bps" :key="b.path + ':' + b.line"
           class="group flex items-center gap-1.5 px-3 py-0.5 text-[11px] hover:bg-gray-100 dark:hover:bg-gray-800">
        <span class="text-red-500 flex-shrink-0 cursor-pointer" @click="debug.revealLocation(b.path, b.line)">{{ b.condition ? '◆' : '●' }}</span>
        <span class="truncate text-gray-700 dark:text-gray-200 flex-shrink-0 cursor-pointer" @click="debug.revealLocation(b.path, b.line)">{{ baseName(b.path) }}<span class="text-gray-400">:{{ b.line }}</span></span>
        <input :value="b.condition" class="flex-1 min-w-0 text-[11px] font-mono bg-transparent border border-transparent hover:border-gray-300 dark:hover:border-gray-600 focus:border-blue-500 rounded px-1 focus:outline-none"
               :placeholder="t('debug.conditionPlaceholder')"
               @change="onCondition(b.path, b.line, $event)"/>
        <button class="text-gray-400 hover:text-red-500 opacity-0 group-hover:opacity-100 flex-shrink-0" @click="debug.toggleBreakpoint(b.path, b.line)">×</button>
      </div>

      <!-- 调试控制台输出 -->
      <div class="px-3 py-1.5 mt-1 text-[11px] font-semibold text-gray-500 dark:text-gray-400 border-t border-gray-100 dark:border-gray-800">{{ t('debug.console') }}</div>
      <pre class="px-3 pb-2 text-[11px] font-mono whitespace-pre-wrap break-all"><span
          v-for="(l, i) in debug.consoleLines.value" :key="i" :class="lineClass(l.category)">{{ l.text }}</span></pre>
    </div>

    <!-- REPL 输入 -->
    <div class="flex-shrink-0 border-t border-gray-200 dark:border-gray-700 px-2 py-1.5">
      <input v-model="replInput"
             class="w-full text-[11px] font-mono border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-2 py-1 focus:outline-none focus:border-blue-500"
             :placeholder="t('debug.replPlaceholder')"
             @keydown.enter="submitRepl"/>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, ref, watch} from 'vue'
import {Bug, ChevronRight, Square} from 'lucide-vue-next'
import {useI18n} from 'vue-i18n'
import {useDebug, type DapVariable, type Scope} from '../composables/useDebug'
import DebugVarNode from './DebugVarNode.vue'

const {t} = useI18n()
const debug = useDebug()

const scopes = ref<Scope[]>([])
const openScopes = ref<Set<number>>(new Set())
const scopeVars = ref<Map<number, DapVariable[]>>(new Map())
const watchInput = ref('')
const replInput = ref('')

// 断点列表（随 bpVersion 刷新）
const bps = computed(() => {
  void debug.bpVersion.value
  return debug.allBreakpoints()
})
const baseName = (p: string) => p.split(/[\\/]/).pop() || p
const onCondition = (path: string, line: number, e: Event) => {
  debug.setBreakpointCondition(path, line, (e.target as HTMLInputElement).value.trim())
}

const lineClass = (category: string): string => {
  if (category === 'stderr') return 'text-red-500'
  if (category === 'input') return 'text-blue-500'
  if (category === 'result') return 'text-emerald-600 dark:text-emerald-400'
  return 'text-gray-700 dark:text-gray-300'
}

const loadScopes = async () => {
  scopes.value = []
  openScopes.value = new Set()
  scopeVars.value = new Map()
  const id = debug.selectedFrameId.value
  if (id == null) {
    return
  }
  scopes.value = await debug.requestScopes(id)
  const first = scopes.value[0]
  if (first) {
    await toggleScope(first.variablesReference)
  }
}

const toggleScope = async (ref_: number) => {
  const open = openScopes.value
  if (open.has(ref_)) {
    open.delete(ref_)
  }
  else {
    open.add(ref_)
    if (!scopeVars.value.has(ref_)) {
      scopeVars.value.set(ref_, await debug.requestVariables(ref_))
    }
  }
  openScopes.value = new Set(open)
}

const addWatch = () => {
  debug.addWatch(watchInput.value)
  watchInput.value = ''
}
const submitRepl = () => {
  const v = replInput.value
  replInput.value = ''
  debug.evalRepl(v)
}

watch(() => debug.selectedFrameId.value, () => loadScopes())
</script>
