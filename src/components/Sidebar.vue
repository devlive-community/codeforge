<template>
  <div class="flex flex-col h-full bg-gray-50 dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700">
    <div class="flex items-center justify-between px-3 py-2 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
      <span class="text-xs font-semibold text-gray-600 dark:text-gray-300 truncate uppercase">{{ rootName || t('sidebar.explorer') }}</span>
      <div class="flex items-center space-x-1">
        <button class="p-1 rounded text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-700 cursor-pointer"
                :title="t('sidebar.openFolder')"
                @click="emit('open-folder')">
          <FolderOpen class="w-4 h-4"/>
        </button>
        <button v-if="rootDir"
                class="p-1 rounded text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-700 cursor-pointer"
                :title="t('sidebar.refresh')"
                @click="loadRoot">
          <RefreshCw class="w-4 h-4"/>
        </button>
      </div>
    </div>

    <div class="flex-1 overflow-auto py-1">
      <div v-if="!rootDir" class="px-3 py-6">
        <div class="text-center">
          <p class="text-xs text-gray-400 mb-3">{{ t('sidebar.noFolder') }}</p>
          <Button size="sm" @click="emit('open-folder')">{{ t('sidebar.openFolder') }}</Button>
        </div>

        <div v-if="recentFolders && recentFolders.length" class="mt-6">
          <p class="text-xs font-semibold text-gray-400 mb-1 px-1">{{ t('sidebar.recent') }}</p>
          <button v-for="folder in recentFolders"
                  :key="folder"
                  class="w-full flex items-center space-x-2 px-2 py-1 rounded text-left text-sm text-gray-600 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer"
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
      <div ref="menuRef"
           class="absolute bg-white dark:bg-gray-800 dark:text-gray-100 rounded-md shadow-lg border border-gray-200 dark:border-gray-700 py-1 text-sm min-w-[150px]"
           :style="{ top: `${ctx.y}px`, left: `${ctx.x}px` }"
           @click.stop>
        <template v-if="!ctx.node || ctx.node.is_dir">
          <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="promptCreate('file')">{{ t('sidebar.newFile') }}</button>
          <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="promptCreate('folder')">{{ t('sidebar.newFolder') }}</button>
        </template>
        <template v-if="ctx.node">
          <div v-if="!ctx.node || ctx.node.is_dir" class="border-t border-gray-100 dark:border-gray-700 my-1"></div>
          <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="promptRename">{{ t('sidebar.rename') }}</button>
          <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer text-red-600" @click="confirmDelete">{{ t('sidebar.delete') }}</button>
          <div class="border-t border-gray-100 dark:border-gray-700 my-1"></div>
          <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="reveal">{{ t('sidebar.revealIn', { label: revealLabel }) }}</button>
        </template>
      </div>
    </div>

    <!-- 新建/重命名 输入框 -->
    <Modal v-model:show="nameModal.show" :title="nameModalTitle" size="sm">
      <div class="space-y-4">
        <Input v-model="nameModal.value" class="w-full" :placeholder="nameModalPlaceholder" @keyup.enter="submitName"/>
        <div class="flex justify-end space-x-2">
          <Button type="secondary" size="sm" @click="nameModal.show = false">{{ t('sidebar.cancel') }}</Button>
          <Button size="sm" @click="submitName">{{ t('sidebar.confirm') }}</Button>
        </div>
      </div>
    </Modal>

    <!-- 删除确认 -->
    <Modal v-model:show="deleteModal.show" :title="t('sidebar.confirmDeleteTitle')" size="sm">
      <div class="space-y-4">
        <p class="text-sm text-gray-700 dark:text-gray-300">
          {{ t('sidebar.deleteConfirmPre') }}<strong>{{ deleteModal.node?.name }}</strong>{{ deleteModal.node?.is_dir ? t('sidebar.deleteConfirmDir') : '' }}{{ t('sidebar.deleteConfirmPost') }}
        </p>
        <div class="flex justify-end space-x-2">
          <Button type="secondary" size="sm" @click="deleteModal.show = false">{{ t('sidebar.cancel') }}</Button>
          <Button size="sm" class="bg-red-500 hover:bg-red-600 text-white" @click="doDelete">{{ t('sidebar.delete') }}</Button>
        </div>
      </div>
    </Modal>
  </div>
</template>

<script setup lang="ts">
import {computed, nextTick, onMounted, onUnmounted, provide, reactive, ref, watch} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {listen, type UnlistenFn} from '@tauri-apps/api/event'
import {Folder, FolderOpen, RefreshCw} from 'lucide-vue-next'
import Button from '../ui/Button.vue'
import Modal from '../ui/Modal.vue'
import Input from '../ui/Input.vue'
import FileTreeNode from './FileTreeNode.vue'
import {useToast} from '../plugins/toast'
import {useI18n} from 'vue-i18n'

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
  gitStatus?: Record<string, string>
}>()

const emit = defineEmits<{
  'open-folder': []
  'open-recent': [path: string]
  'open-file': [path: string]
  'renamed': [from: string, to: string]
  'deleted': [path: string]
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
const {t} = useI18n()

provide('treeOpenFile', (path: string) => emit('open-file', path))
// 当前激活文件路径（用于文件树高亮选中项）
provide('treeActivePath', computed(() => props.activePath ?? null))
// Git 状态映射（绝对路径 → 状态字母），供文件树徽标
provide('treeGitStatus', computed(() => props.gitStatus ?? {}))

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
const menuRef = ref<HTMLElement | null>(null)
const openCtx = async (node: FileNode | null, e: MouseEvent) => {
  ctx.node = node
  ctx.x = e.clientX
  ctx.y = e.clientY
  ctx.visible = true

  // 渲染后测量并钳制进视口，避免靠近边缘时显示不全
  await nextTick()
  const el = menuRef.value
  if (el) {
    const rect = el.getBoundingClientRect()
    if (ctx.x + rect.width > window.innerWidth) {
      ctx.x = Math.max(4, window.innerWidth - rect.width - 4)
    }
    if (ctx.y + rect.height > window.innerHeight) {
      ctx.y = Math.max(4, window.innerHeight - rect.height - 4)
    }
  }
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
const revealLabel = computed(() => isMac ? t('sidebar.revealFinder') : (/Win/i.test(navigator.platform) ? t('sidebar.revealExplorer') : t('sidebar.revealManager')))

const dirOf = (path: string) => path.replace(/[\\/][^\\/]*$/, '')
const joinPath = (dir: string, name: string) => `${dir}/${name}`

// ===== 新建/重命名 =====
const nameModal = reactive<{ show: boolean, mode: 'file' | 'folder' | 'rename', node: FileNode | null, value: string }>({
  show: false, mode: 'file', node: null, value: ''
})
const nameModalTitle = computed(() => ({file: t('sidebar.newFile'), folder: t('sidebar.newFolder'), rename: t('sidebar.rename')})[nameModal.mode])
const nameModalPlaceholder = computed(() => nameModal.mode === 'rename' ? t('sidebar.renamePlaceholder') : t('sidebar.namePlaceholder'))

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
      emit('renamed', nameModal.node.path, to)
      toast.success(t('sidebar.renamed'))
    }
    else {
      const target = joinPath(nameModal.node.path, name)
      await invoke(nameModal.mode === 'file' ? 'create_file' : 'create_directory', {path: target})
      toast.success(nameModal.mode === 'file' ? t('sidebar.createdFile') : t('sidebar.createdFolder'))
    }
    nameModal.show = false
    triggerRefresh()
  }
  catch (error) {
    toast.error(t('sidebar.opFailed') + ': ' + error)
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
    emit('deleted', deleteModal.node.path)
    toast.success(t('sidebar.deleted'))
    deleteModal.show = false
    triggerRefresh()
  }
  catch (error) {
    toast.error(t('sidebar.deleteFailed') + ': ' + error)
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
    toast.error(t('sidebar.openFailed') + ': ' + error)
  }
  closeCtx()
}
</script>
