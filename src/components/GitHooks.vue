<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-16 px-6 pb-6" @click="emit('close')">
    <div class="w-full max-w-[640px] max-h-full bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col"
         @click.stop>
      <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
          <Webhook class="w-4 h-4 text-gray-400"/>
          <span>{{ t('git.hooksTitle') }}</span>
        </div>
        <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('close')">
          <X class="w-4 h-4"/>
        </button>
      </div>

      <div class="px-4 py-2 text-[11px] text-gray-500 dark:text-gray-400 border-b border-gray-100 dark:border-gray-800 leading-relaxed flex-shrink-0">
        {{ t('git.hooksHint') }}
      </div>

      <!-- 编辑中 -->
      <template v-if="editing">
        <div class="px-4 py-2 flex items-center gap-2 border-b border-gray-100 dark:border-gray-800 flex-shrink-0">
          <span class="font-mono text-sm text-gray-800 dark:text-gray-100">{{ editing }}</span>
          <label class="ml-auto flex items-center gap-1.5 text-xs text-gray-600 dark:text-gray-300 cursor-pointer">
            <input v-model="execBit" type="checkbox" class="cursor-pointer"/>
            {{ t('git.hookExecutable') }}
          </label>
        </div>
        <textarea v-model="content" spellcheck="false"
                  class="flex-1 min-h-[260px] w-full font-mono text-xs p-3 bg-gray-50 dark:bg-gray-800/60 focus:outline-none resize-none"
                  :placeholder="t('git.hookContentPlaceholder')"/>
        <div class="flex items-center justify-end gap-2 px-4 py-2.5 border-t border-gray-200 dark:border-gray-700 flex-shrink-0">
          <Button size="sm" type="secondary" @click="editing = null">{{ t('git.cancel') }}</Button>
          <Button size="sm" :loading="busy" @click="save">{{ t('git.hookSave') }}</Button>
        </div>
      </template>

      <!-- 列表 -->
      <div v-else class="flex-1 overflow-y-auto min-h-[160px]">
        <div v-for="hk in hooks" :key="hk.name"
             class="group flex items-center gap-2 px-4 py-2 border-b border-gray-100 dark:border-gray-800">
          <span class="flex-1 font-mono text-sm" :class="hk.active ? 'text-gray-800 dark:text-gray-100' : 'text-gray-400'">{{ hk.name }}</span>
          <span v-if="hk.active" class="text-[10px] px-1.5 py-0.5 rounded flex-shrink-0 bg-green-100 text-green-700 dark:bg-green-900/40 dark:text-green-300">{{ t('git.hooksActive') }}</span>
          <span v-if="hk.active && !hk.executable" class="text-[10px] px-1.5 py-0.5 rounded flex-shrink-0 bg-amber-100 text-amber-700 dark:bg-amber-900/40 dark:text-amber-300">{{ t('git.hooksNotExecutable') }}</span>
          <button class="text-xs text-blue-500 hover:underline opacity-0 group-hover:opacity-100 cursor-pointer flex-shrink-0" @click="edit(hk.name)">{{ t('git.hookEdit') }}</button>
          <button v-if="hk.active" class="text-xs text-red-500 hover:underline opacity-0 group-hover:opacity-100 cursor-pointer flex-shrink-0" :disabled="busy" @click="remove(hk.name)">{{ t('git.hookDelete') }}</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {Webhook, X} from 'lucide-vue-next'
import Button from '../ui/Button.vue'
import {useToast} from '../plugins/toast'
import {useI18n} from 'vue-i18n'

interface Hook { name: string; active: boolean; executable: boolean }

const props = defineProps<{ rootDir: string }>()
const emit = defineEmits<{ close: [] }>()

const toast = useToast()
const {t} = useI18n()
const hooks = ref<Hook[]>([])
const editing = ref<string | null>(null)
const content = ref('')
const execBit = ref(true)
const busy = ref(false)

const load = async () => {
  try {
    hooks.value = await invoke<Hook[]>('git_hooks', {root: props.rootDir})
  }
  catch (error) {
    toast.error(t('git.hookFailed') + ': ' + error)
  }
}

const edit = async (name: string) => {
  try {
    const existing = await invoke<string>('git_hook_read', {root: props.rootDir, name})
    content.value = existing || '#!/bin/sh\n'
    execBit.value = true
    editing.value = name
  }
  catch (error) {
    toast.error(t('git.hookFailed') + ': ' + error)
  }
}

const save = async () => {
  if (!editing.value) {
    return
  }
  busy.value = true
  try {
    await invoke('git_hook_save', {root: props.rootDir, name: editing.value, content: content.value, executable: execBit.value})
    toast.success(t('git.hookSaved'))
    editing.value = null
    await load()
  }
  catch (error) {
    toast.error(t('git.hookFailed') + ': ' + error)
  }
  finally {
    busy.value = false
  }
}

const remove = async (name: string) => {
  busy.value = true
  try {
    await invoke('git_hook_delete', {root: props.rootDir, name})
    toast.success(t('git.hookDeleted'))
    await load()
  }
  catch (error) {
    toast.error(t('git.hookFailed') + ': ' + error)
  }
  finally {
    busy.value = false
  }
}

onMounted(load)
</script>
