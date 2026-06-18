<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-12 px-6 pb-6" @click="emit('close')">
    <div class="w-full max-w-[1100px] max-h-full bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col"
         @click.stop>
      <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200 min-w-0">
          <GitCommitHorizontal class="w-4 h-4 text-gray-400 flex-shrink-0"/>
          <span>{{ t('git.blameTitle') }}</span>
          <span class="text-xs text-gray-400 truncate">· {{ fileName }}</span>
        </div>
        <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('close')">
          <X class="w-4 h-4"/>
        </button>
      </div>

      <div class="flex-1 overflow-auto font-mono text-xs leading-5">
        <div v-if="loading" class="px-4 py-10 text-center text-gray-400">…</div>
        <div v-for="(l, i) in lines" :key="i" class="flex hover:bg-gray-50 dark:hover:bg-gray-800/60">
          <span class="flex-shrink-0 w-56 px-2 truncate text-gray-400 border-r border-gray-100 dark:border-gray-800 select-none"
                :title="`${l.short} · ${l.author} · ${l.date}`">
            <code class="text-amber-600 dark:text-amber-400">{{ l.short }}</code>
            <span class="ml-1.5">{{ l.author }}</span>
            <span class="ml-1.5 text-gray-300 dark:text-gray-600">{{ l.date }}</span>
          </span>
          <span class="flex-shrink-0 w-12 px-2 text-right text-gray-300 dark:text-gray-600 select-none">{{ i + 1 }}</span>
          <pre class="flex-1 px-2 whitespace-pre text-gray-800 dark:text-gray-200">{{ l.content }}</pre>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {GitCommitHorizontal, X} from 'lucide-vue-next'
import {useToast} from '../plugins/toast'
import {useI18n} from 'vue-i18n'

interface BlameLine { short: string; author: string; date: string; content: string }

const props = defineProps<{ rootDir: string; relPath: string; fileName: string }>()
const emit = defineEmits<{ close: [] }>()

const toast = useToast()
const {t} = useI18n()
const lines = ref<BlameLine[]>([])
const loading = ref(true)

onMounted(async () => {
  try {
    lines.value = await invoke<BlameLine[]>('git_blame', {root: props.rootDir, relPath: props.relPath})
  }
  catch (error) {
    toast.error(t('git.blameFailed') + ': ' + error)
    emit('close')
  }
  finally {
    loading.value = false
  }
})
</script>
