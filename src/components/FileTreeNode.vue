<template>
  <div>
    <div class="flex items-center py-1 pr-4 cursor-pointer text-sm select-none w-full whitespace-nowrap"
         :class="isActive ? 'bg-blue-100 text-blue-700' : 'text-gray-700 hover:bg-gray-100'"
         :style="{ paddingLeft: `${depth * 12 + 8}px` }"
         @click="onClick"
         @contextmenu.prevent.stop="onContext">
      <ChevronRight v-if="node.is_dir"
                    class="w-3.5 h-3.5 text-gray-400 transition-transform flex-shrink-0"
                    :class="{ 'rotate-90': expanded }"/>
      <span v-else class="w-3.5 h-3.5 flex-shrink-0"></span>

      <component :is="node.is_dir ? (expanded ? FolderOpen : Folder) : File"
                 class="w-4 h-4 mx-1 flex-shrink-0"
                 :class="node.is_dir ? 'text-blue-500' : 'text-gray-400'"/>

      <span>{{ node.name }}</span>
    </div>

    <template v-if="node.is_dir && expanded">
      <FileTreeNode v-for="child in children" :key="child.path" :node="child" :depth="depth + 1"/>
    </template>
  </div>
</template>

<script setup lang="ts">
import {computed, inject, ref, watch, type ComputedRef, type Ref} from 'vue'
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

// 由 Sidebar 提供的注入项
const openFile = inject<(path: string) => void>('treeOpenFile')
const activePath = inject<ComputedRef<string | null>>('treeActivePath')
const contextMenu = inject<(node: FileNode, e: MouseEvent) => void>('treeContextMenu')
const refreshSignal = inject<Ref<number>>('treeRefresh')

const isActive = computed(() => !props.node.is_dir && activePath?.value === props.node.path)

const loadChildren = async () => {
  try {
    children.value = await invoke<FileNode[]>('read_directory_tree', {path: props.node.path})
    loaded.value = true
  }
  catch (error) {
    console.error('读取目录失败:', error)
  }
}

const onClick = async () => {
  if (props.node.is_dir) {
    expanded.value = !expanded.value
    if (expanded.value && !loaded.value) {
      await loadChildren()
    }
  }
  else {
    openFile?.(props.node.path)
  }
}

const onContext = (e: MouseEvent) => {
  contextMenu?.(props.node, e)
}

// 文件系统变化时刷新已展开目录的子项（保留展开状态）
if (refreshSignal) {
  watch(refreshSignal, () => {
    if (props.node.is_dir && expanded.value && loaded.value) {
      loadChildren()
    }
  })
}
</script>
