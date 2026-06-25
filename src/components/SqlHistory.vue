<template>
  <div class="inline-block">
    <button ref="btnRef" class="inline-flex items-center gap-1 px-2 py-0.5 rounded text-xs text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer"
            :title="t('sqlHistory.title')" @click="toggle">
      <History class="w-3.5 h-3.5"/>
    </button>

    <Teleport to="body">
      <template v-if="open">
        <div class="fixed inset-0 z-[60]" @click="open = false"/>
        <div class="fixed z-[61] w-[420px] max-h-[60vh] flex flex-col rounded-md border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 shadow-lg text-xs"
             :style="{left: pos.left + 'px', top: pos.top + 'px'}">
          <div class="flex items-center gap-1 px-3 py-2 border-b border-gray-100 dark:border-gray-700 text-gray-600 dark:text-gray-300 font-medium">
            <History class="w-3.5 h-3.5"/>{{ t('sqlHistory.title') }}
            <button v-if="sorted.length" class="ml-auto text-[11px] text-gray-400 hover:text-red-500 cursor-pointer" @click="clearHistory">{{ t('sqlHistory.clear') }}</button>
          </div>
          <div class="flex-1 overflow-y-auto">
            <div v-if="!sorted.length" class="px-3 py-8 text-center text-gray-400">{{ t('sqlHistory.empty') }}</div>
            <div v-for="it in sorted" :key="it.ts"
                 class="group px-3 py-1.5 border-b border-gray-50 dark:border-gray-800 hover:bg-gray-50 dark:hover:bg-gray-900/40">
              <div class="flex items-center gap-1.5">
                <button class="flex-shrink-0 cursor-pointer" :class="it.favorite ? 'text-amber-400' : 'text-gray-300 dark:text-gray-600 hover:text-amber-400'" @click="toggleFavorite(it.ts)">
                  <Star class="w-3.5 h-3.5" :fill="it.favorite ? 'currentColor' : 'none'"/>
                </button>
                <code class="flex-1 min-w-0 truncate text-gray-700 dark:text-gray-200 cursor-pointer" :title="it.sql" @click="emit('load', it.sql); open = false">{{ it.sql }}</code>
                <button class="flex-shrink-0 text-emerald-600 dark:text-emerald-400 opacity-0 group-hover:opacity-100 cursor-pointer" :title="t('sqlHistory.run')" @click="emit('run', it.sql); open = false">
                  <Play class="w-3.5 h-3.5"/>
                </button>
                <button class="flex-shrink-0 text-gray-400 hover:text-red-500 opacity-0 group-hover:opacity-100 cursor-pointer" @click="remove(it.ts)">
                  <X class="w-3.5 h-3.5"/>
                </button>
              </div>
            </div>
          </div>
        </div>
      </template>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import {computed, ref} from 'vue'
import {History, Play, Star, X} from 'lucide-vue-next'
import {useI18n} from 'vue-i18n'
import {useSqlHistory} from '../composables/useSqlHistory'

const emit = defineEmits<{ load: [sql: string]; run: [sql: string] }>()
const {t} = useI18n()
const {items, toggleFavorite, remove, clearHistory} = useSqlHistory()

const open = ref(false)
const btnRef = ref<HTMLElement>()
const pos = ref({left: 0, top: 0})

// 收藏置顶，其余按时间倒序
const sorted = computed(() =>
  [...items.value].sort((a, b) => (Number(b.favorite) - Number(a.favorite)) || (b.ts - a.ts))
)

const toggle = () => {
  open.value = !open.value
  if (open.value) {
    const r = btnRef.value?.getBoundingClientRect()
    if (r) {
      pos.value = {left: Math.max(8, Math.min(r.left, window.innerWidth - 428)), top: r.bottom + 4}
    }
  }
}
</script>
