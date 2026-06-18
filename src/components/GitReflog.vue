<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-12 px-6 pb-6" @click="emit('close')">
    <div class="w-full max-w-[760px] max-h-full bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col"
         @click.stop>
      <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
          <RotateCcw class="w-4 h-4 text-gray-400"/>
          <span>{{ t('git.reflogTitle') }}</span>
        </div>
        <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('close')">
          <X class="w-4 h-4"/>
        </button>
      </div>

      <div class="flex-1 overflow-y-auto">
        <div v-if="!entries.length" class="px-4 py-10 text-center text-sm text-gray-400">{{ t('git.reflogEmpty') }}</div>
        <div v-for="(e, i) in entries" :key="i"
             class="group flex items-center gap-2 px-4 py-2 border-b border-gray-100 dark:border-gray-800">
          <code class="text-amber-600 dark:text-amber-400 flex-shrink-0">{{ e.short }}</code>
          <code class="text-[11px] text-gray-400 flex-shrink-0 w-24 truncate">{{ e.selector }}</code>
          <span class="flex-1 min-w-0 text-sm text-gray-800 dark:text-gray-200 truncate">{{ e.subject }}</span>
          <span class="text-[11px] text-gray-400 flex-shrink-0">{{ e.date }}</span>
          <button class="text-xs text-red-500 hover:underline opacity-0 group-hover:opacity-100 cursor-pointer flex-shrink-0" :disabled="busy" @click="confirmTarget = e">{{ t('git.restoreHere') }}</button>
        </div>
      </div>
    </div>
  </div>

  <!-- 恢复确认 -->
  <Modal v-if="confirmTarget" :show="true" :title="t('git.restoreHere')" size="sm" @update:show="confirmTarget = null">
    <div class="space-y-3">
      <p class="text-sm text-gray-700 dark:text-gray-300">
        <code class="text-amber-600 dark:text-amber-400">{{ confirmTarget.short }}</code> · {{ confirmTarget.subject }}
      </p>
      <p class="text-xs text-red-500">{{ t('git.restoreConfirm') }}</p>
      <div class="flex justify-end gap-2">
        <Button size="sm" type="secondary" @click="confirmTarget = null">{{ t('git.cancel') }}</Button>
        <Button size="sm" type="danger" @click="restore">{{ t('git.restoreHere') }}</Button>
      </div>
    </div>
  </Modal>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {RotateCcw, X} from 'lucide-vue-next'
import Button from '../ui/Button.vue'
import Modal from '../ui/Modal.vue'
import {useToast} from '../plugins/toast'
import {useI18n} from 'vue-i18n'

interface ReflogEntry { short: string; selector: string; subject: string; date: string }

const props = defineProps<{ rootDir: string }>()
const emit = defineEmits<{ close: []; changed: [] }>()

const toast = useToast()
const {t} = useI18n()
const entries = ref<ReflogEntry[]>([])
const busy = ref(false)
const confirmTarget = ref<ReflogEntry | null>(null)

const load = async () => {
  try {
    entries.value = await invoke<ReflogEntry[]>('git_reflog', {root: props.rootDir, limit: 100})
  }
  catch (error) {
    toast.error(t('git.reflogFailed') + ': ' + error)
  }
}

const restore = async () => {
  const e = confirmTarget.value
  confirmTarget.value = null
  if (!e) {
    return
  }
  busy.value = true
  try {
    await invoke('git_reset', {root: props.rootDir, hash: e.selector, mode: 'hard'})
    toast.success(t('git.restored'))
    await load()
    emit('changed')
  }
  catch (error) {
    toast.error(t('git.resetFailed') + ': ' + error)
  }
  finally {
    busy.value = false
  }
}

onMounted(load)
</script>
