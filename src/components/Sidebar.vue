<template>
  <div class="flex flex-col h-full bg-gray-50 border-r border-gray-200">
    <div class="flex items-center justify-between px-3 py-2 border-b border-gray-200 flex-shrink-0">
      <span class="text-xs font-semibold text-gray-600 truncate uppercase">{{ rootName || '资源管理器' }}</span>
      <div class="flex items-center space-x-1">
        <button class="p-1 rounded text-gray-400 hover:text-gray-700 hover:bg-gray-200 cursor-pointer"
                title="打开文件夹"
                @click="emit('open-folder')">
          <FolderOpen class="w-4 h-4"/>
        </button>
        <button v-if="rootDir"
                class="p-1 rounded text-gray-400 hover:text-gray-700 hover:bg-gray-200 cursor-pointer"
                title="刷新"
                @click="loadRoot">
          <RefreshCw class="w-4 h-4"/>
        </button>
      </div>
    </div>

    <div class="flex-1 overflow-auto py-1">
      <div v-if="!rootDir" class="px-3 py-6">
        <div class="text-center">
          <p class="text-xs text-gray-400 mb-3">未打开文件夹</p>
          <Button size="sm" @click="emit('open-folder')">打开文件夹</Button>
        </div>

        <div v-if="recentFolders && recentFolders.length" class="mt-6">
          <p class="text-xs font-semibold text-gray-400 mb-1 px-1">最近打开</p>
          <button v-for="folder in recentFolders"
                  :key="folder"
                  class="w-full flex items-center space-x-2 px-2 py-1 rounded text-left text-sm text-gray-600 hover:bg-gray-100 cursor-pointer"
                  :title="folder"
                  @click="emit('open-recent', folder)">
            <Folder class="w-4 h-4 text-blue-500 flex-shrink-0"/>
            <span class="truncate">{{ folderName(folder) }}</span>
          </button>
        </div>
      </div>

      <!-- w-max + min-w-full：长文件名时撑出横向滚动，同时高亮铺满整行 -->
      <div v-else class="w-max min-w-full">
        <FileTreeNode v-for="node in rootNodes" :key="node.path" :node="node" :depth="0"/>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, provide, ref, watch} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {Folder, FolderOpen, RefreshCw} from 'lucide-vue-next'
import Button from '../ui/Button.vue'
import FileTreeNode from './FileTreeNode.vue'

interface FileNode
{
  name: string
  path: string
  is_dir: boolean
}

const props = defineProps<{
  rootDir: string | null
  activePath?: string | null
  recentFolders?: string[]
}>()

const emit = defineEmits<{
  'open-folder': []
  'open-recent': [path: string]
  'open-file': [path: string]
}>()

const folderName = (path: string) => path.split(/[\\/]/).filter(Boolean).pop() || path

const rootNodes = ref<FileNode[]>([])

const rootName = computed(() => {
  if (!props.rootDir) {
    return ''
  }
  return props.rootDir.split(/[\\/]/).filter(Boolean).pop() || props.rootDir
})

provide('treeOpenFile', (path: string) => emit('open-file', path))
// 当前激活文件路径（用于文件树高亮选中项）
provide('treeActivePath', computed(() => props.activePath ?? null))

const loadRoot = async () => {
  if (!props.rootDir) {
    rootNodes.value = []
    return
  }
  try {
    rootNodes.value = await invoke<FileNode[]>('read_directory_tree', {path: props.rootDir})
  }
  catch (error) {
    console.error('读取目录失败:', error)
    rootNodes.value = []
  }
}

watch(() => props.rootDir, loadRoot, {immediate: true})
</script>
