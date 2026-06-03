<template>
  <div class="flex items-stretch bg-gray-50 border-b border-gray-200 overflow-x-auto flex-shrink-0
              [&::-webkit-scrollbar]:hidden [-ms-overflow-style:none] [scrollbar-width:none]">
    <div v-for="tab in tabs"
         :key="tab.id"
         class="group flex items-center space-x-2 pl-3 pr-2 py-1.5 border-r border-gray-200 cursor-pointer max-w-[200px] flex-shrink-0"
         :class="tab.id === activeId ? 'bg-white text-gray-800' : 'text-gray-500 hover:bg-gray-100'"
         @click="emit('switch', tab.id)"
         @mousedown.middle.prevent="emit('close', tab.id)">
      <img :src="iconUrl(tab.language)" class="w-4 h-4 flex-shrink-0" :alt="tab.language"/>
      <span class="text-xs truncate">{{ title(tab) }}</span>
      <span v-if="isDirty(tab)" class="text-amber-500 text-xs flex-shrink-0" title="未保存">●</span>
      <button class="ml-1 rounded p-0.5 text-gray-400 hover:text-gray-700 hover:bg-gray-200 opacity-0 group-hover:opacity-100 transition-opacity flex-shrink-0"
              :class="{ 'opacity-100': tab.id === activeId }"
              title="关闭"
              @click.stop="emit('close', tab.id)">
        <X class="w-3 h-3"/>
      </button>
    </div>

    <button class="flex items-center justify-center px-2 text-gray-400 hover:text-gray-700 hover:bg-gray-100 cursor-pointer flex-shrink-0"
            title="新建标签页"
            @click="emit('new')">
      <Plus class="w-4 h-4"/>
    </button>
  </div>
</template>

<script setup lang="ts">
import {Plus, X} from 'lucide-vue-next'
import type {WorkspaceTab} from '../composables/useWorkspace'

defineProps<{
  tabs: WorkspaceTab[]
  activeId: string
}>()

const emit = defineEmits<{
  switch: [id: string]
  close: [id: string]
  new: []
}>()

const title = (tab: WorkspaceTab) => {
  if (!tab.filePath) {
    return '未命名'
  }
  return tab.filePath.split(/[\\/]/).pop() || '未命名'
}

const isDirty = (tab: WorkspaceTab) => tab.filePath !== null && tab.code !== tab.savedContent

const iconUrl = (language: string) => `/icons/${language.replace(/\d+$/, '')}.svg`
</script>
