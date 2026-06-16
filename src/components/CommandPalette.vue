<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-24" @click="emit('close')">
    <div class="w-[560px] max-w-[90vw] bg-white dark:bg-gray-800 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col max-h-[60vh]"
         @click.stop>
      <div class="flex items-center px-3 border-b border-gray-200 dark:border-gray-700">
        <Command class="w-4 h-4 text-gray-400 flex-shrink-0"/>
        <input ref="inputRef"
               v-model="query"
               class="flex-1 px-2 py-2.5 text-sm bg-transparent focus:outline-none"
               :placeholder="t('dialog.commandPlaceholder')"
               @keydown.down.prevent="move(1)"
               @keydown.up.prevent="move(-1)"
               @keydown.enter.prevent="choose(filtered[activeIndex])"
               @keydown.esc.prevent="emit('close')"/>
      </div>

      <div ref="listRef" class="overflow-y-auto py-1">
        <div v-if="filtered.length === 0" class="px-4 py-6 text-center text-sm text-gray-400">{{ t('dialog.commandEmpty') }}</div>

        <button v-for="(cmd, i) in filtered"
                :key="cmd.id"
                :ref="el => setItemRef(el, i)"
                class="w-full flex items-center px-3 py-1.5 text-left cursor-pointer"
                :class="i === activeIndex ? 'bg-blue-100 dark:bg-gray-700' : 'hover:bg-gray-100 dark:hover:bg-gray-700'"
                @click="choose(cmd)"
                @mousemove="activeIndex = i">
          <component :is="cmd.icon || ChevronRight" class="w-4 h-4 text-gray-400 flex-shrink-0 mr-2"/>
          <span class="text-sm text-gray-800 dark:text-gray-100 truncate">{{ cmd.label }}</span>
          <span v-if="cmd.group" class="ml-2 text-xs text-gray-400 truncate">{{ cmd.group }}</span>
          <span v-if="cmd.hint" class="ml-auto pl-3 text-xs text-gray-400 dark:text-gray-500 flex-shrink-0 font-mono">{{ cmd.hint }}</span>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, nextTick, onMounted, ref, watch} from 'vue'
import {useI18n} from 'vue-i18n'
import {ChevronRight, Command} from 'lucide-vue-next'

const {t} = useI18n()

export interface PaletteCommand
{
  id: string
  label: string
  group?: string
  hint?: string
  icon?: any
  run: () => void
}

const props = defineProps<{ commands: PaletteCommand[] }>()
const emit = defineEmits<{ close: [] }>()

const query = ref('')
const activeIndex = ref(0)
const inputRef = ref<HTMLInputElement | null>(null)
const listRef = ref<HTMLElement | null>(null)
const itemRefs: HTMLElement[] = []
const setItemRef = (el: any, i: number) => {
  if (el) itemRefs[i] = el
}

onMounted(() => inputRef.value?.focus())

const filtered = computed<PaletteCommand[]>(() => {
  const q = query.value.trim().toLowerCase()
  if (!q) {
    return props.commands
  }
  return props.commands
      .map(c => {
        const label = c.label.toLowerCase()
        let score = -1
        if (label.startsWith(q)) score = 0
        else if (label.includes(q)) score = 1
        else if (subsequence(q, label)) score = 2
        return {c, score}
      })
      .filter(x => x.score >= 0)
      .sort((a, b) => a.score - b.score)
      .map(x => x.c)
})

// 简单子序列匹配（支持 "ot" 匹配 "open tab" 这类首字母缩写）
const subsequence = (q: string, text: string): boolean => {
  let i = 0
  for (const ch of text) {
    if (ch === q[i]) i++
    if (i === q.length) return true
  }
  return false
}

watch(filtered, () => {
  activeIndex.value = 0
})

const move = (delta: number) => {
  const len = filtered.value.length
  if (len === 0) return
  activeIndex.value = (activeIndex.value + delta + len) % len
  nextTick(() => itemRefs[activeIndex.value]?.scrollIntoView({block: 'nearest'}))
}

const choose = (cmd?: PaletteCommand) => {
  if (cmd) {
    emit('close')
    cmd.run()
  }
}
</script>
