<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-24" @click="emit('close')">
    <div class="w-[520px] max-w-[90vw] bg-white dark:bg-gray-800 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col max-h-[60vh]"
         @click.stop>
      <div class="flex items-center gap-2 px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 text-sm font-medium text-gray-700 dark:text-gray-200">
        <Layers class="w-4 h-4 text-brand-500"/>
        <span>{{ t('workspaces.title') }}</span>
      </div>

      <!-- 保存当前 -->
      <div class="flex items-center gap-2 px-3 py-2.5 border-b border-gray-200 dark:border-gray-700">
        <input v-model="name"
               class="flex-1 text-sm border border-gray-300 dark:border-gray-600 dark:bg-gray-900 rounded px-2 py-1.5 focus:outline-none focus:border-blue-500"
               :placeholder="t('workspaces.namePlaceholder')"
               @keydown.enter="saveCurrent"/>
        <button class="text-xs px-3 py-1.5 rounded bg-brand-600 text-white hover:bg-brand-700 disabled:opacity-40 disabled:cursor-not-allowed cursor-pointer whitespace-nowrap"
                :disabled="!rootDir || !name.trim()"
                @click="saveCurrent">
          {{ t('workspaces.saveCurrent') }}
        </button>
      </div>

      <!-- 列表 -->
      <div class="flex-1 overflow-y-auto py-1">
        <div v-if="!workspaces.length" class="px-4 py-8 text-center text-sm text-gray-400">{{ t('workspaces.empty') }}</div>
        <div v-for="w in workspaces" :key="w.name"
             class="group flex items-center gap-2 px-3 py-2 hover:bg-gray-100 dark:hover:bg-gray-700">
          <button class="flex-1 min-w-0 text-left cursor-pointer" @click="emit('open', w)">
            <div class="text-sm font-medium text-gray-800 dark:text-gray-100 truncate">{{ w.name }}</div>
            <div class="text-xs text-gray-400 truncate">
              {{ folderName(w.rootDir) }}<span v-if="w.extraRoots.length"> · +{{ w.extraRoots.length }}</span>
            </div>
          </button>
          <button class="p-1 rounded text-gray-400 hover:text-red-500 opacity-0 group-hover:opacity-100 cursor-pointer"
                  :title="t('workspaces.delete')"
                  @click.stop="remove(w.name)">
            <Trash2 class="w-3.5 h-3.5"/>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {ref} from 'vue'
import {useI18n} from 'vue-i18n'
import {Layers, Trash2} from 'lucide-vue-next'
import {useNamedWorkspaces, type NamedWorkspace} from '../composables/useNamedWorkspaces'

const {t} = useI18n()
const props = defineProps<{ rootDir: string | null; extraRoots: string[] }>()
const emit = defineEmits<{ open: [ws: NamedWorkspace]; close: [] }>()

const {workspaces, save, remove} = useNamedWorkspaces()
const name = ref('')

const folderName = (p: string) => p.split(/[\\/]/).filter(Boolean).pop() || p

const saveCurrent = () => {
  if (!props.rootDir || !name.value.trim()) {
    return
  }
  save(name.value, props.rootDir, props.extraRoots)
  name.value = ''
}
</script>
