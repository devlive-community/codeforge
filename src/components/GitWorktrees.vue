<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-16 px-6 pb-6" @click="emit('close')">
    <div class="w-full max-w-[600px] max-h-full bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col"
         @click.stop>
      <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
          <TreeDeciduous class="w-4 h-4 text-gray-400"/>
          <span>{{ t('git.worktreeTitle') }}</span>
        </div>
        <div class="flex items-center gap-3 text-xs">
          <button class="text-gray-500 hover:text-gray-700 dark:hover:text-gray-200 cursor-pointer disabled:opacity-40" :disabled="busy" @click="prune">{{ t('git.worktreePrune') }}</button>
          <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('close')">
            <X class="w-4 h-4"/>
          </button>
        </div>
      </div>

      <!-- 新增 worktree -->
      <div class="p-3 border-b border-gray-200 dark:border-gray-700 flex-shrink-0 flex gap-2">
        <input v-model="path" class="flex-1 min-w-0 text-sm border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-2 py-1 focus:outline-none focus:border-blue-500" :placeholder="t('git.worktreePathPlaceholder')"/>
        <input v-model="reference" class="w-40 flex-shrink-0 text-sm border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-2 py-1 focus:outline-none focus:border-blue-500" :placeholder="t('git.worktreeRefPlaceholder')"/>
        <Button size="sm" :loading="busy" :disabled="!path.trim()" @click="add">{{ t('git.worktreeAdd') }}</Button>
      </div>

      <!-- 列表 -->
      <div class="flex-1 overflow-y-auto min-h-[120px]">
        <div v-if="!trees.length" class="px-4 py-10 text-center text-sm text-gray-400">{{ t('git.worktreeEmpty') }}</div>
        <div v-for="wt in trees" :key="wt.path"
             class="group flex items-center gap-2 px-4 py-2 border-b border-gray-100 dark:border-gray-800">
          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-1.5">
              <span class="text-sm font-medium text-gray-800 dark:text-gray-100 truncate">
                {{ wt.branch || (wt.detached ? t('git.worktreeDetached') : (wt.bare ? t('git.worktreeBare') : '')) }}
              </span>
              <span v-if="wt.locked" class="text-[10px] px-1 rounded bg-amber-100 text-amber-700 dark:bg-amber-900/40 dark:text-amber-300 flex-shrink-0">{{ t('git.worktreeLocked') }}</span>
              <span class="font-mono text-[11px] text-gray-400 flex-shrink-0">{{ wt.head }}</span>
            </div>
            <div class="text-[11px] text-gray-400 truncate">{{ wt.path }}</div>
          </div>
          <Tooltip :text="t('git.worktreeReveal')">
            <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 opacity-0 group-hover:opacity-100 cursor-pointer flex-shrink-0" @click="reveal(wt.path)">
              <FolderOpen class="w-3.5 h-3.5"/>
            </button>
          </Tooltip>
          <button class="text-xs text-red-500 hover:underline opacity-0 group-hover:opacity-100 cursor-pointer flex-shrink-0" :disabled="busy" @click="remove(wt.path)">{{ t('git.worktreeRemove') }}</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {FolderOpen, TreeDeciduous, X} from 'lucide-vue-next'
import Button from '../ui/Button.vue'
import Tooltip from '../ui/Tooltip.vue'
import {useToast} from '../plugins/toast'
import {useI18n} from 'vue-i18n'

interface Worktree { path: string; head: string; branch: string; bare: boolean; detached: boolean; locked: boolean }

const props = defineProps<{ rootDir: string }>()
const emit = defineEmits<{ close: [] }>()

const toast = useToast()
const {t} = useI18n()
const trees = ref<Worktree[]>([])
const path = ref('')
const reference = ref('')
const busy = ref(false)

const load = async () => {
  try {
    trees.value = await invoke<Worktree[]>('git_worktrees', {root: props.rootDir})
  }
  catch (error) {
    toast.error(t('git.worktreeFailed') + ': ' + error)
  }
}

const add = async () => {
  if (!path.value.trim()) {
    return
  }
  busy.value = true
  try {
    await invoke('git_worktree_add', {root: props.rootDir, path: path.value.trim(), reference: reference.value.trim()})
    path.value = ''
    reference.value = ''
    toast.success(t('git.worktreeAdded'))
    await load()
  }
  catch (error) {
    toast.error(t('git.worktreeFailed') + ': ' + error)
  }
  finally {
    busy.value = false
  }
}

const remove = async (p: string) => {
  busy.value = true
  try {
    await invoke('git_worktree_remove', {root: props.rootDir, path: p})
    toast.success(t('git.worktreeRemoved'))
    await load()
  }
  catch (error) {
    toast.error(t('git.worktreeFailed') + ': ' + error)
  }
  finally {
    busy.value = false
  }
}

const prune = async () => {
  busy.value = true
  try {
    await invoke('git_worktree_prune', {root: props.rootDir})
    toast.success(t('git.worktreePruned'))
    await load()
  }
  catch (error) {
    toast.error(t('git.worktreeFailed') + ': ' + error)
  }
  finally {
    busy.value = false
  }
}

const reveal = async (p: string) => {
  try {
    await invoke('reveal_path', {path: p})
  }
  catch (error) {
    toast.error(t('git.worktreeFailed') + ': ' + error)
  }
}

onMounted(load)
</script>
