<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-16 px-6 pb-6" @click="emit('close')">
    <div class="w-full max-w-[560px] max-h-full bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col"
         @click.stop>
      <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
          <Archive class="w-4 h-4 text-gray-400"/>
          <span>{{ t('git.stashTitle') }}</span>
        </div>
        <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('close')">
          <X class="w-4 h-4"/>
        </button>
      </div>

      <!-- 储藏当前改动 -->
      <div class="p-3 border-b border-gray-200 dark:border-gray-700 flex-shrink-0 flex gap-2">
        <input v-model="message"
               class="flex-1 text-sm border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-2 py-1 focus:outline-none focus:border-blue-500"
               :placeholder="t('git.stashMessagePlaceholder')"
               @keydown.enter="push"/>
        <Button size="sm" :loading="busy" @click="push">{{ t('git.stashPush') }}</Button>
      </div>

      <!-- 储藏列表 -->
      <div class="flex-1 overflow-y-auto min-h-[120px]">
        <div v-if="!list.length" class="px-4 py-10 text-center text-sm text-gray-400">{{ t('git.stashEmpty') }}</div>
        <div v-for="s in list" :key="s.reference"
             class="group flex items-center gap-2 px-4 py-2 border-b border-gray-100 dark:border-gray-800">
          <div class="flex-1 min-w-0">
            <div class="text-sm text-gray-800 dark:text-gray-100 truncate">{{ s.message }}</div>
            <code class="text-[11px] text-gray-400">{{ s.reference }}</code>
          </div>
          <button class="text-xs text-blue-500 hover:underline cursor-pointer flex-shrink-0" :disabled="busy" @click="pop(s.reference)">{{ t('git.stashPop') }}</button>
          <button class="text-xs text-red-500 hover:underline cursor-pointer flex-shrink-0" :disabled="busy" @click="drop(s.reference)">{{ t('git.stashDrop') }}</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {Archive, X} from 'lucide-vue-next'
import Button from '../ui/Button.vue'
import {useToast} from '../plugins/toast'
import {useI18n} from 'vue-i18n'

interface StashEntry { reference: string; message: string }

const props = defineProps<{ rootDir: string }>()
const emit = defineEmits<{ close: []; changed: [] }>()

const toast = useToast()
const {t} = useI18n()

const list = ref<StashEntry[]>([])
const message = ref('')
const busy = ref(false)

const load = async () => {
  try {
    list.value = await invoke<StashEntry[]>('git_stash_list', {root: props.rootDir})
  }
  catch (error) {
    toast.error(t('git.stashFailed') + ': ' + error)
  }
}

const push = async () => {
  busy.value = true
  try {
    await invoke('git_stash_push', {root: props.rootDir, message: message.value.trim()})
    message.value = ''
    toast.success(t('git.stashed'))
    await load()
    emit('changed')
  }
  catch (error) {
    toast.error(t('git.stashFailed') + ': ' + error)
  }
  finally {
    busy.value = false
  }
}

const pop = async (reference: string) => {
  busy.value = true
  try {
    await invoke('git_stash_pop', {root: props.rootDir, reference})
    toast.success(t('git.stashPopped'))
    await load()
    emit('changed')
  }
  catch (error) {
    toast.error(t('git.stashFailed') + ': ' + error)
  }
  finally {
    busy.value = false
  }
}

const drop = async (reference: string) => {
  busy.value = true
  try {
    await invoke('git_stash_drop', {root: props.rootDir, reference})
    toast.success(t('git.stashDropped'))
    await load()
  }
  catch (error) {
    toast.error(t('git.stashFailed') + ': ' + error)
  }
  finally {
    busy.value = false
  }
}

onMounted(load)
</script>
