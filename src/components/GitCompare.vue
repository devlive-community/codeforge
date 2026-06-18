<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-12 px-6 pb-6" @click="emit('close')">
    <div class="w-full max-w-[1000px] max-h-full bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col"
         @click.stop>
      <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
          <GitCompareArrows class="w-4 h-4 text-gray-400"/>
          <span>{{ t('git.compareTitle') }}</span>
        </div>
        <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('close')">
          <X class="w-4 h-4"/>
        </button>
      </div>

      <!-- ref 选择 -->
      <div class="flex items-center gap-2 px-4 py-2 border-b border-gray-200 dark:border-gray-700 flex-shrink-0 text-xs">
        <span class="text-gray-400">{{ t('git.compareBase') }}</span>
        <select v-model="base" class="bg-transparent border border-gray-200 dark:border-gray-700 rounded px-1.5 py-0.5 focus:outline-none cursor-pointer dark:bg-gray-900" @change="run">
          <option v-for="b in refs" :key="'b' + b" :value="b" class="dark:bg-gray-800">{{ b }}</option>
        </select>
        <span class="text-gray-400">…</span>
        <span class="text-gray-400">{{ t('git.compareHead') }}</span>
        <select v-model="head" class="bg-transparent border border-gray-200 dark:border-gray-700 rounded px-1.5 py-0.5 focus:outline-none cursor-pointer dark:bg-gray-900" @change="run">
          <option v-for="b in refs" :key="'h' + b" :value="b" class="dark:bg-gray-800">{{ b }}</option>
        </select>
        <span v-if="result" class="ml-2 text-emerald-600 dark:text-emerald-400">{{ t('git.compareAhead', { n: result.ahead }) }}</span>
        <span v-if="result" class="text-red-500">{{ t('git.compareBehind', { n: result.behind }) }}</span>
      </div>

      <section class="flex-1 overflow-auto bg-gray-50 dark:bg-gray-950">
        <pre v-if="result && result.patch.trim()" class="p-3 text-xs leading-relaxed font-mono whitespace-pre-wrap"><span
            v-for="(ln, i) in patchLines" :key="i" :class="lineClass(ln)">{{ ln }}
</span></pre>
        <div v-else class="h-full flex items-center justify-center text-sm text-gray-400">{{ t('git.compareSame') }}</div>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, onMounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {GitCompareArrows, X} from 'lucide-vue-next'
import {useToast} from '../plugins/toast'
import {useI18n} from 'vue-i18n'

interface CompareResult { ahead: number; behind: number; patch: string }

const props = defineProps<{ rootDir: string; branch: string }>()
const emit = defineEmits<{ close: [] }>()

const toast = useToast()
const {t} = useI18n()
const refs = ref<string[]>([])
const base = ref('')
const head = ref('')
const result = ref<CompareResult | null>(null)

const patchLines = computed(() => (result.value?.patch || '').split('\n'))
const lineClass = (ln: string): string => {
  if (ln.startsWith('+') && !ln.startsWith('+++')) return 'text-green-600 dark:text-green-400'
  if (ln.startsWith('-') && !ln.startsWith('---')) return 'text-red-600 dark:text-red-400'
  if (ln.startsWith('@@')) return 'text-cyan-600 dark:text-cyan-400'
  if (ln.startsWith('diff ') || ln.startsWith('index ') || ln.startsWith('+++') || ln.startsWith('---')) return 'text-gray-400'
  return 'text-gray-700 dark:text-gray-300'
}

const run = async () => {
  if (!base.value || !head.value) {
    return
  }
  try {
    result.value = await invoke<CompareResult>('git_compare', {root: props.rootDir, base: base.value, head: head.value})
  }
  catch (error) {
    toast.error(t('git.compareFailed') + ': ' + error)
  }
}

onMounted(async () => {
  try {
    const b = await invoke<{ current: string; branches: string[] }>('git_branches', {root: props.rootDir})
    refs.value = b.branches
    head.value = props.branch || b.current
    // 基线默认取另一个分支（若有）
    base.value = b.branches.find(x => x !== head.value) || head.value
    await run()
  }
  catch (error) {
    toast.error(t('git.compareFailed') + ': ' + error)
  }
})
</script>
