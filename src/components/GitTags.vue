<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-16 px-6 pb-6" @click="emit('close')">
    <div class="w-full max-w-[520px] max-h-full bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col"
         @click.stop>
      <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
          <TagIcon class="w-4 h-4 text-gray-400"/>
          <span>{{ t('git.tagTitle') }}</span>
        </div>
        <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('close')">
          <X class="w-4 h-4"/>
        </button>
      </div>

      <!-- 在 HEAD 打标签 -->
      <div class="p-3 border-b border-gray-200 dark:border-gray-700 flex-shrink-0 space-y-2">
        <div class="flex gap-2">
          <input v-model="name"
                 class="flex-1 text-sm border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-2 py-1 focus:outline-none focus:border-blue-500"
                 :placeholder="t('git.tagNamePlaceholder')"
                 @keydown.enter="create"/>
          <Button size="sm" :loading="busy" :disabled="!name.trim()" @click="create">{{ t('git.tagHead') }}</Button>
        </div>
        <input v-model="message"
               class="w-full text-sm border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-2 py-1 focus:outline-none focus:border-blue-500"
               :placeholder="t('git.tagMessagePlaceholder')"/>
      </div>

      <!-- 标签列表 -->
      <div class="flex-1 overflow-y-auto min-h-[120px]">
        <div v-if="!tags.length" class="px-4 py-10 text-center text-sm text-gray-400">{{ t('git.tagEmpty') }}</div>
        <div v-for="tg in tags" :key="tg"
             class="group flex items-center gap-2 px-4 py-2 border-b border-gray-100 dark:border-gray-800">
          <TagIcon class="w-3.5 h-3.5 text-amber-500 flex-shrink-0"/>
          <span class="flex-1 truncate text-sm text-gray-800 dark:text-gray-100">{{ tg }}</span>
          <button class="text-xs text-blue-500 hover:underline opacity-0 group-hover:opacity-100 cursor-pointer flex-shrink-0" :disabled="busy" @click="checkout(tg)">{{ t('git.tagCheckout') }}</button>
          <button class="text-xs text-red-500 hover:underline opacity-0 group-hover:opacity-100 cursor-pointer flex-shrink-0" :disabled="busy" @click="remove(tg)">{{ t('git.tagDelete') }}</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {Tag as TagIcon, X} from 'lucide-vue-next'
import Button from '../ui/Button.vue'
import {useToast} from '../plugins/toast'
import {useI18n} from 'vue-i18n'

const props = defineProps<{ rootDir: string }>()
const emit = defineEmits<{ close: [] }>()

const toast = useToast()
const {t} = useI18n()
const tags = ref<string[]>([])
const name = ref('')
const message = ref('')
const busy = ref(false)

const load = async () => {
  try {
    tags.value = await invoke<string[]>('git_tags', {root: props.rootDir})
  }
  catch (error) {
    toast.error(t('git.tagFailed') + ': ' + error)
  }
}

const create = async () => {
  if (!name.value.trim()) {
    return
  }
  busy.value = true
  try {
    await invoke('git_tag_create', {root: props.rootDir, name: name.value.trim(), hash: '', message: message.value.trim()})
    name.value = ''
    message.value = ''
    toast.success(t('git.tagCreated'))
    await load()
  }
  catch (error) {
    toast.error(t('git.tagFailed') + ': ' + error)
  }
  finally {
    busy.value = false
  }
}

const checkout = async (tg: string) => {
  busy.value = true
  try {
    await invoke('git_checkout', {root: props.rootDir, branch: tg})
    toast.success(t('git.tagCheckedOut'))
    emit('close')
  }
  catch (error) {
    toast.error(t('git.tagFailed') + ': ' + error)
  }
  finally {
    busy.value = false
  }
}

const remove = async (tg: string) => {
  busy.value = true
  try {
    await invoke('git_tag_delete', {root: props.rootDir, name: tg})
    toast.success(t('git.tagDeleted'))
    await load()
  }
  catch (error) {
    toast.error(t('git.tagFailed') + ': ' + error)
  }
  finally {
    busy.value = false
  }
}

onMounted(load)
</script>
