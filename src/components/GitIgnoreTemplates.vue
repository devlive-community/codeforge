<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-16 px-6 pb-6" @click="emit('close')">
    <div class="w-full max-w-[520px] max-h-full bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col"
         @click.stop>
      <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
          <FileCode2 class="w-4 h-4 text-gray-400"/>
          <span>{{ t('gitignore.title') }}</span>
        </div>
        <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('close')">
          <X class="w-4 h-4"/>
        </button>
      </div>

      <div class="px-4 py-2 text-[11px] text-gray-400 border-b border-gray-100 dark:border-gray-800 flex-shrink-0">{{ t('gitignore.hint') }}</div>

      <div class="flex-1 overflow-y-auto p-2">
        <label v-for="tpl in templates" :key="tpl.id"
               class="flex items-start gap-2 px-2 py-1.5 rounded hover:bg-gray-100 dark:hover:bg-gray-800 cursor-pointer">
          <input type="checkbox" class="mt-0.5 cursor-pointer" :checked="selected.has(tpl.id)" @change="toggle(tpl.id)"/>
          <div class="min-w-0">
            <div class="text-sm text-gray-800 dark:text-gray-100">{{ tpl.label }}</div>
            <div class="text-[11px] text-gray-400 font-mono truncate">{{ firstPatterns(tpl.content) }}</div>
          </div>
        </label>
      </div>

      <div class="flex items-center justify-end gap-2 px-4 py-2.5 border-t border-gray-200 dark:border-gray-700 flex-shrink-0">
        <Button size="sm" type="secondary" @click="emit('close')">{{ t('gitignore.cancel') }}</Button>
        <Button size="sm" :loading="busy" :disabled="!selected.size" @click="apply">{{ t('gitignore.apply') }}</Button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {FileCode2, X} from 'lucide-vue-next'
import Button from '../ui/Button.vue'
import {useToast} from '../plugins/toast'
import {useI18n} from 'vue-i18n'
import {gitignoreTemplates} from '../data/gitignoreTemplates'

const props = defineProps<{ rootDir: string }>()
const emit = defineEmits<{ close: [] }>()

const toast = useToast()
const {t} = useI18n()
const templates = gitignoreTemplates
const selected = ref<Set<string>>(new Set())
const busy = ref(false)

const firstPatterns = (content: string) =>
  content.split('\n').filter(l => l.trim() && !l.trim().startsWith('#')).slice(0, 4).join('  ')

const toggle = (id: string) => {
  const s = new Set(selected.value)
  if (s.has(id)) {
    s.delete(id)
  }
  else {
    s.add(id)
  }
  selected.value = s
}

const apply = async () => {
  busy.value = true
  try {
    for (const tpl of templates) {
      if (selected.value.has(tpl.id)) {
        await invoke('git_ignore_append_block', {root: props.rootDir, content: tpl.content})
      }
    }
    toast.success(t('gitignore.applied'))
    emit('close')
  }
  catch (e) {
    toast.error(t('gitignore.failed') + ': ' + e)
  }
  finally {
    busy.value = false
  }
}
</script>
