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
      <!-- 调用栈 -->
      <div class="px-3 py-1.5 text-[11px] font-semibold text-gray-500 dark:text-gray-400 sticky top-0 bg-white dark:bg-gray-900">{{ t('debug.callStack') }}</div>
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
    </div>
  </div>
</template>

<script setup lang="ts">
import {ref, watch} from 'vue'
import {Bug, ChevronRight, Square} from 'lucide-vue-next'
import {useI18n} from 'vue-i18n'
import {useDebug, type DapVariable, type Scope} from '../composables/useDebug'
import DebugVarNode from './DebugVarNode.vue'

const {t} = useI18n()
const debug = useDebug()

const scopes = ref<Scope[]>([])
const openScopes = ref<Set<number>>(new Set())
const scopeVars = ref<Map<number, DapVariable[]>>(new Map())

const loadScopes = async () => {
  scopes.value = []
  openScopes.value = new Set()
  scopeVars.value = new Map()
  const id = debug.selectedFrameId.value
  if (id == null) {
    return
  }
  scopes.value = await debug.requestScopes(id)
  // 默认展开第一个作用域（通常是 Locals）
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

// 选中帧变化时重载作用域/变量
watch(() => debug.selectedFrameId.value, () => loadScopes())
</script>
