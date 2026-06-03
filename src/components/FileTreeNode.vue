<template>
  <div>
    <div class="flex items-center py-1 pr-2 cursor-pointer hover:bg-gray-100 text-sm text-gray-700 select-none"
         :style="{ paddingLeft: `${depth * 12 + 8}px` }"
         @click="onClick">
      <ChevronRight v-if="node.is_dir"
                    class="w-3.5 h-3.5 text-gray-400 transition-transform flex-shrink-0"
                    :class="{ 'rotate-90': expanded }"/>
      <span v-else class="w-3.5 h-3.5 flex-shrink-0"></span>

      <component :is="node.is_dir ? (expanded ? FolderOpen : Folder) : File"
                 class="w-4 h-4 mx-1 flex-shrink-0"
                 :class="node.is_dir ? 'text-blue-500' : 'text-gray-400'"/>

      <span class="truncate">{{ node.name }}</span>
    </div>

    <template v-if="node.is_dir && expanded">
      <FileTreeNode v-for="child in children" :key="child.path" :node="child" :depth="depth + 1"/>
    </template>
  </div>
</template>

<script setup lang="ts">
import {inject, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {ChevronRight, File, Folder, FolderOpen} from 'lucide-vue-next'

interface FileNode
{
  name: string
  path: string
  is_dir: boolean
}

const props = defineProps<{
  node: FileNode
  depth: number
}>()

const expanded = ref(false)
const children = ref<FileNode[]>([])
const loaded = ref(false)

// 由 Sidebar 提供的“打开文件”处理函数
const openFile = inject<(path: string) => void>('treeOpenFile')

const onClick = async () => {
  if (props.node.is_dir) {
    expanded.value = !expanded.value
    if (expanded.value && !loaded.value) {
      try {
        children.value = await invoke<FileNode[]>('read_directory_tree', {path: props.node.path})
        loaded.value = true
      }
      catch (error) {
        console.error('读取目录失败:', error)
      }
    }
  }
  else {
    openFile?.(props.node.path)
  }
}
</script>
