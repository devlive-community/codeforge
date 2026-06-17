<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-16 px-6 pb-6" @click="emit('close')">
    <div class="w-full max-w-[560px] max-h-full bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col"
         @click.stop>
      <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
          <Cloud class="w-4 h-4 text-gray-400"/>
          <span>{{ t('git.remoteTitle') }}</span>
        </div>
        <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('close')">
          <X class="w-4 h-4"/>
        </button>
      </div>

      <!-- 添加远程 -->
      <div class="p-3 border-b border-gray-200 dark:border-gray-700 flex-shrink-0 space-y-2">
        <div class="flex gap-2">
          <input v-model="name" class="w-32 flex-shrink-0 text-sm border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-2 py-1 focus:outline-none focus:border-blue-500" :placeholder="t('git.remoteNamePlaceholder')"/>
          <input v-model="url" class="flex-1 min-w-0 text-sm border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-2 py-1 focus:outline-none focus:border-blue-500" :placeholder="t('git.remoteUrlPlaceholder')"/>
          <Button size="sm" :loading="busy" :disabled="!name.trim() || !url.trim()" @click="add">{{ t('git.remoteAdd') }}</Button>
        </div>
      </div>

      <!-- 远程列表 -->
      <div class="flex-1 overflow-y-auto min-h-[120px]">
        <div v-if="!remotes.length" class="px-4 py-10 text-center text-sm text-gray-400">{{ t('git.remoteEmpty') }}</div>
        <div v-for="r in remotes" :key="r.name"
             class="group flex items-center gap-2 px-4 py-2 border-b border-gray-100 dark:border-gray-800">
          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium text-gray-800 dark:text-gray-100">{{ r.name }}</div>
            <div class="text-[11px] text-gray-400 truncate">{{ r.url }}</div>
          </div>
          <button v-if="branch" class="text-xs text-blue-500 hover:underline cursor-pointer flex-shrink-0" :disabled="busy" @click="setUpstream(r.name)">{{ t('git.setUpstream') }}</button>
          <button class="text-xs text-red-500 hover:underline opacity-0 group-hover:opacity-100 cursor-pointer flex-shrink-0" :disabled="busy" @click="remove(r.name)">{{ t('git.remoteRemove') }}</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {Cloud, X} from 'lucide-vue-next'
import Button from '../ui/Button.vue'
import {useToast} from '../plugins/toast'
import {useI18n} from 'vue-i18n'

interface Remote { name: string; url: string }

const props = defineProps<{ rootDir: string; branch: string }>()
const emit = defineEmits<{ close: [] }>()

const toast = useToast()
const {t} = useI18n()
const remotes = ref<Remote[]>([])
const name = ref('')
const url = ref('')
const busy = ref(false)

const load = async () => {
  try {
    remotes.value = await invoke<Remote[]>('git_remotes', {root: props.rootDir})
  }
  catch (error) {
    toast.error(t('git.remoteFailed') + ': ' + error)
  }
}

const add = async () => {
  if (!name.value.trim() || !url.value.trim()) {
    return
  }
  busy.value = true
  try {
    await invoke('git_remote_add', {root: props.rootDir, name: name.value.trim(), url: url.value.trim()})
    name.value = ''
    url.value = ''
    toast.success(t('git.remoteAdded'))
    await load()
  }
  catch (error) {
    toast.error(t('git.remoteFailed') + ': ' + error)
  }
  finally {
    busy.value = false
  }
}

const remove = async (n: string) => {
  busy.value = true
  try {
    await invoke('git_remote_remove', {root: props.rootDir, name: n})
    toast.success(t('git.remoteRemoved'))
    await load()
  }
  catch (error) {
    toast.error(t('git.remoteFailed') + ': ' + error)
  }
  finally {
    busy.value = false
  }
}

const setUpstream = async (remote: string) => {
  busy.value = true
  try {
    await invoke('git_set_upstream', {root: props.rootDir, remote, branch: props.branch})
    toast.success(t('git.upstreamSet'))
  }
  catch (error) {
    toast.error(t('git.remoteFailed') + ': ' + error)
  }
  finally {
    busy.value = false
  }
}

onMounted(load)
</script>
