<template>
  <div>
    <div class="flex items-center gap-1 px-1 py-0.5 hover:bg-gray-100 dark:hover:bg-gray-800 rounded cursor-default font-mono text-[11px] leading-5"
         :class="expandable ? 'cursor-pointer' : ''"
         :style="{ paddingLeft: `${depth * 12 + 4}px` }"
         @click="toggle">
      <span class="w-3 h-3 flex-shrink-0 flex items-center justify-center text-gray-400">
        <ChevronRight v-if="expandable" class="w-3 h-3 transition-transform" :class="{ 'rotate-90': expanded }"/>
      </span>
      <span class="text-purple-600 dark:text-purple-300 flex-shrink-0">{{ variable.name }}</span>
      <template v-if="variable.value">
        <span class="text-gray-400">:</span>
        <span class="text-gray-700 dark:text-gray-200 break-all min-w-0">{{ variable.value }}</span>
      </template>
    </div>
    <div v-if="expanded">
      <DebugVarNode v-for="(child, i) in children" :key="i" :variable="child" :depth="depth + 1"/>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, ref} from 'vue'
import {ChevronRight} from 'lucide-vue-next'
import {useDebug, type DapVariable} from '../composables/useDebug'

const props = defineProps<{ variable: DapVariable; depth: number }>()
const debug = useDebug()

const expandable = computed(() => props.variable.variablesReference > 0)
const expanded = ref(false)
const children = ref<DapVariable[]>([])
let loaded = false

const toggle = async () => {
  if (!expandable.value) {
    return
  }
  expanded.value = !expanded.value
  if (expanded.value && !loaded) {
    children.value = await debug.requestVariables(props.variable.variablesReference)
    loaded = true
  }
}
</script>
