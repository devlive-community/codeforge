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
      <div v-else class="w-max min-w-full"
           @contextmenu.prevent="onRootContext">
        <FileTreeNode v-for="node in rootNodes" :key="node.path" :node="node" :depth="0"/>
      </div>
    </div>

    <!-- 右键菜单 -->
    <div v-if="ctx.visible" class="fixed inset-0 z-40" @click="closeCtx" @contextmenu.prevent="closeCtx">
      <div class="absolute bg-white rounded-md shadow-lg border border-gray-200 py-1 text-sm min-w-[150px]"
           :style="{ top: `${ctx.y}px`, left: `${ctx.x}px` }"
           @click.stop>
        <template v-if="!ctx.node || ctx.node.is_dir">
          <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 cursor-pointer" @click="promptCreate('file')">新建文件</button>
          <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 cursor-pointer" @click="promptCreate('folder')">新建文件夹</button>
        </template>
        <template v-if="ctx.node">
          <div class="border-t border-gray-100 my-1"></div>
          <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 cursor-pointer" @click="promptRename">重命名</button>
          <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 cursor-pointer text-red-600" @click="confirmDelete">删除</button>
          <div class="border-t border-gray-100 my-1"></div>
          <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 cursor-pointer" @click="reveal">在{{ revealLabel }}中显示</button>
        </template>
      </div>
    </div>

    <!-- 新建/重命名 输入框 -->
    <Modal v-model:show="nameModal.show" :title="nameModalTitle" size="sm">
      <div class="space-y-4">
        <Input v-model="nameModal.value" class="w-full" :placeholder="nameModalPlaceholder" @keyup.enter="submitName"/>
        <div class="flex justify-end space-x-2">
          <Button type="secondary" size="sm" @click="nameModal.show = false">取消</Button>
          <Button size="sm" @click="submitName">确定</Button>
        </div>
      </div>
    </Modal>

    <!-- 删除确认 -->
    <Modal v-model:show="deleteModal.show" title="确认删除" size="sm">
      <div class="space-y-4">
        <p class="text-sm text-gray-700 dark:text-gray-300">
          确定删除 <strong>{{ deleteModal.node?.name }}</strong>{{ deleteModal.node?.is_dir ? '（及其内容）' : '' }}？此操作不可恢复。
        </p>
        <div class="flex justify-end space-x-2">
          <Button type="secondary" size="sm" @click="deleteModal.show = false">取消</Button>
          <Button size="sm" class="bg-red-500 hover:bg-red-600 text-white" @click="doDelete">删除</Button>
        </div>
      </div>
    </Modal>
  </div>
</template>

<script setup lang="ts">
import {computed, onMounted, onUnmounted, provide, reactive, ref, watch} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {listen, type UnlistenFn} from '@tauri-apps/api/event'
import {Folder, FolderOpen, RefreshCw} from 'lucide-vue-next'
import Button from '../ui/Button.vue'
import Modal from '../ui/Modal.vue'
import Input from '../ui/Input.vue'
import FileTreeNode from './FileTreeNode.vue'
import {useToast} from '../plugins/toast'

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

const toast = useToast()

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

// 打开目录后启动文件监听（替换旧监听）
const startWatching = async () => {
  if (!props.rootDir) {
    return
  }
  try {
    await invoke('watch_directory', {path: props.rootDir})
  }
  catch (error) {
    console.error('启动文件监听失败:', error)
  }
}

watch(() => props.rootDir, async () => {
  await loadRoot()
  await startWatching()
}, {immediate: true})

// ===== 刷新信号：变更后通知已展开目录刷新（保留展开状态）=====
const refreshSignal = ref(0)
provide('treeRefresh', refreshSignal)
const triggerRefresh = () => {
  refreshSignal.value++
  loadRoot()
}

// 文件系统变化（外部改动）防抖刷新
let fsTimer: any = null
let unlistenFs: UnlistenFn | null = null
const debouncedRefresh = () => {
  clearTimeout(fsTimer)
  fsTimer = setTimeout(triggerRefresh, 300)
}

onMounted(async () => {
  unlistenFs = await listen('fs-changed', debouncedRefresh)
})

onUnmounted(() => {
  if (unlistenFs) {
    unlistenFs()
  }
})

// ===== 右键菜单 =====
const ctx = reactive<{ visible: boolean, x: number, y: number, node: FileNode | null }>({
  visible: false, x: 0, y: 0, node: null
})
const openCtx = (node: FileNode | null, e: MouseEvent) => {
  ctx.node = node
  ctx.x = e.clientX
  ctx.y = e.clientY
  ctx.visible = true
}
const closeCtx = () => {
  ctx.visible = false
}
provide('treeContextMenu', (node: FileNode, e: MouseEvent) => openCtx(node, e))
// 在空白处右键 = 对根目录操作
const onRootContext = (e: MouseEvent) => {
  if (props.rootDir) {
    openCtx({name: rootName.value, path: props.rootDir, is_dir: true}, e)
  }
}

const isMac = /Mac/i.test(navigator.platform)
const revealLabel = isMac ? '访达' : (/Win/i.test(navigator.platform) ? '资源管理器' : '文件管理器')

const dirOf = (path: string) => path.replace(/[\\/][^\\/]*$/, '')
const joinPath = (dir: string, name: string) => `${dir}/${name}`

// ===== 新建/重命名 =====
const nameModal = reactive<{ show: boolean, mode: 'file' | 'folder' | 'rename', node: FileNode | null, value: string }>({
  show: false, mode: 'file', node: null, value: ''
})
const nameModalTitle = computed(() => ({file: '新建文件', folder: '新建文件夹', rename: '重命名'})[nameModal.mode])
const nameModalPlaceholder = computed(() => nameModal.mode === 'rename' ? '输入新名称' : '输入名称')

const promptCreate = (kind: 'file' | 'folder') => {
  nameModal.mode = kind
  nameModal.node = ctx.node
  nameModal.value = ''
  nameModal.show = true
  closeCtx()
}
const promptRename = () => {
  nameModal.mode = 'rename'
  nameModal.node = ctx.node
  nameModal.value = ctx.node?.name || ''
  nameModal.show = true
  closeCtx()
}

const submitName = async () => {
  const name = nameModal.value.trim()
  if (!name || !nameModal.node) {
    return
  }
  try {
    if (nameModal.mode === 'rename') {
      const to = joinPath(dirOf(nameModal.node.path), name)
      await invoke('rename_path', {from: nameModal.node.path, to})
      toast.success('已重命名')
    }
    else {
      const target = joinPath(nameModal.node.path, name)
      await invoke(nameModal.mode === 'file' ? 'create_file' : 'create_directory', {path: target})
      toast.success(nameModal.mode === 'file' ? '已新建文件' : '已新建文件夹')
    }
    nameModal.show = false
    triggerRefresh()
  }
  catch (error) {
    toast.error('操作失败: ' + error)
  }
}

// ===== 删除 =====
const deleteModal = reactive<{ show: boolean, node: FileNode | null }>({show: false, node: null})
const confirmDelete = () => {
  deleteModal.node = ctx.node
  deleteModal.show = true
  closeCtx()
}
const doDelete = async () => {
  if (!deleteModal.node) {
    return
  }
  try {
    await invoke('delete_path', {path: deleteModal.node.path})
    toast.success('已删除')
    deleteModal.show = false
    triggerRefresh()
  }
  catch (error) {
    toast.error('删除失败: ' + error)
  }
}

// ===== 在文件管理器中显示 =====
const reveal = async () => {
  if (!ctx.node) {
    return
  }
  try {
    await invoke('reveal_path', {path: ctx.node.path})
  }
  catch (error) {
    toast.error('打开失败: ' + error)
  }
  closeCtx()
}
</script>
