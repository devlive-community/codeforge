<template>
  <div>
    <div ref="rowEl"
         class="flex items-center py-1 pr-4 cursor-pointer text-sm select-none w-full whitespace-nowrap"
         :class="isActive ? 'bg-blue-100 dark:bg-blue-900/40 text-blue-700 dark:text-blue-300' : 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'"
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

      <span :class="gitBadge ? gitColor : ''">{{ node.name }}</span>
      <span v-if="gitBadge" class="ml-auto pl-2 pr-1 text-xs font-bold flex-shrink-0" :class="gitColor">{{ gitBadge }}</span>
    </div>

    <template v-if="node.is_dir && expanded">
      <FileTreeNode v-for="child in children" :key="child.path" :node="child" :depth="depth + 1"/>
    </template>
  </div>
</template>

<script setup lang="ts">
import {computed, inject, nextTick, onMounted, ref, watch, type ComputedRef, type Ref} from 'vue'
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
const rowEl = ref<HTMLElement | null>(null)

// 由 Sidebar 提供的注入项
const openFile = inject<(path: string) => void>('treeOpenFile')
const activePath = inject<ComputedRef<string | null>>('treeActivePath')
const contextMenu = inject<(node: FileNode, e: MouseEvent) => void>('treeContextMenu')
const refreshSignal = inject<Ref<number>>('treeRefresh')
const collapseSignal = inject<Ref<number>>('treeCollapseAll')
const gitStatus = inject<ComputedRef<Record<string, string>>>('treeGitStatus')
// 「在文件树中定位」：需展开的祖先目录集合 + 目标文件
const revealDirs = inject<Ref<Set<string>>>('treeRevealDirs')
const revealLeaf = inject<Ref<string | null>>('treeRevealLeaf')

const isActive = computed(() => !props.node.is_dir && activePath?.value === props.node.path)

// Git 徽标：文件取自身状态，目录暂不显示
const gitBadge = computed(() => (props.node.is_dir ? '' : gitStatus?.value?.[props.node.path] || ''))
const gitColor = computed(() => {
  switch (gitBadge.value) {
    case 'U': return 'text-green-500'
    case 'A': return 'text-green-600 dark:text-green-400'
    case 'D': return 'text-red-500'
    case 'M': return 'text-amber-500'
    default: return 'text-gray-400'
  }
})

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

// 「折叠文件夹」：收到信号即折叠（子节点已卸载，递归整棵树自然全部折叠）
if (collapseSignal) {
  watch(collapseSignal, () => {
    if (props.node.is_dir) {
      expanded.value = false
    }
  })
}

// 「在文件树中定位」：祖先目录自动展开、目标文件滚动到视野。
// 借助已知的祖先集合，新挂载的子节点会在自身 onMounted 时自查并继续展开，天然适配懒加载树。
const maybeReveal = async () => {
  if (props.node.is_dir && revealDirs?.value?.has(props.node.path)) {
    if (!expanded.value) {
      expanded.value = true
      if (!loaded.value) {
        await loadChildren()
      }
    }
  }
  if (!props.node.is_dir && revealLeaf?.value === props.node.path) {
    await nextTick()
    rowEl.value?.scrollIntoView({block: 'center'})
  }
}
onMounted(maybeReveal)
if (revealDirs) {
  watch(revealDirs, maybeReveal)
}
if (revealLeaf) {
  watch(revealLeaf, maybeReveal)
}
</script>
