<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-16 px-6 pb-6" @click="emit('close')">
    <div class="w-full max-w-[560px] max-h-full bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col"
         @click.stop>
      <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
          <Boxes class="w-4 h-4 text-gray-400"/>
          <span>{{ t('git.submoduleTitle') }}</span>
        </div>
        <div class="flex items-center gap-3 text-xs">
          <button class="text-gray-500 hover:text-gray-700 dark:hover:text-gray-200 cursor-pointer disabled:opacity-40" :disabled="busy" @click="sync">{{ t('git.submoduleSync') }}</button>
          <button class="text-blue-500 hover:underline cursor-pointer disabled:opacity-40" :disabled="busy || !subs.length" @click="update('')">{{ t('git.submoduleUpdateAll') }}</button>
          <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('close')">
            <X class="w-4 h-4"/>
          </button>
        </div>
      </div>

      <div class="flex-1 overflow-y-auto min-h-[120px]">
        <div v-if="!subs.length" class="px-4 py-10 text-center text-sm text-gray-400">{{ t('git.submoduleEmpty') }}</div>
        <div v-for="sm in subs" :key="sm.path"
             class="group flex items-center gap-2 px-4 py-2 border-b border-gray-100 dark:border-gray-800">
          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium text-gray-800 dark:text-gray-100 truncate">{{ sm.path }}</div>
            <div class="text-[11px] text-gray-400 truncate font-mono">{{ sm.hash.slice(0, 8) }}<span v-if="sm.describe"> · {{ sm.describe }}</span></div>
          </div>
          <span class="text-[10px] px-1.5 py-0.5 rounded flex-shrink-0" :class="stateClass(sm.state)">{{ stateLabel(sm.state) }}</span>
          <button class="text-xs text-blue-500 hover:underline opacity-0 group-hover:opacity-100 cursor-pointer flex-shrink-0" :disabled="busy" @click="update(sm.path)">{{ t('git.submoduleUpdate') }}</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {Boxes, X} from 'lucide-vue-next'
import {useToast} from '../plugins/toast'
import {useI18n} from 'vue-i18n'

interface Submodule { path: string; hash: string; state: string; describe: string }

const props = defineProps<{ rootDir: string }>()
const emit = defineEmits<{ close: [] }>()

const toast = useToast()
const {t} = useI18n()
const subs = ref<Submodule[]>([])
const busy = ref(false)

const load = async () => {
  try {
    subs.value = await invoke<Submodule[]>('git_submodules', {root: props.rootDir})
  }
  catch (error) {
    toast.error(t('git.submoduleFailed') + ': ' + error)
  }
}

const update = async (path: string) => {
  busy.value = true
  try {
    await invoke('git_submodule_update', {root: props.rootDir, path})
    toast.success(t('git.submoduleUpdated'))
    await load()
  }
  catch (error) {
    toast.error(t('git.submoduleFailed') + ': ' + error)
  }
  finally {
    busy.value = false
  }
}

const sync = async () => {
  busy.value = true
  try {
    await invoke('git_submodule_sync', {root: props.rootDir})
    toast.success(t('git.submoduleSynced'))
    await load()
  }
  catch (error) {
    toast.error(t('git.submoduleFailed') + ': ' + error)
  }
  finally {
    busy.value = false
  }
}

const stateLabel = (s: string) => t('git.submoduleState' + s.charAt(0).toUpperCase() + s.slice(1))
const stateClass = (s: string) =>
    s === 'uninitialized' ? 'bg-gray-100 text-gray-500 dark:bg-gray-800 dark:text-gray-400'
        : s === 'modified' ? 'bg-amber-100 text-amber-700 dark:bg-amber-900/40 dark:text-amber-300'
            : s === 'conflict' ? 'bg-red-100 text-red-700 dark:bg-red-900/40 dark:text-red-300'
                : 'bg-green-100 text-green-700 dark:bg-green-900/40 dark:text-green-300'

onMounted(load)
</script>
