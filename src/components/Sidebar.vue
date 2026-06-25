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
                :title="t('sidebar.addFolder')"
                @click="emit('add-folder')">
          <FolderPlus class="w-4 h-4"/>
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
          <div class="flex flex-col items-center gap-2">
            <Button size="sm" @click="emit('open-folder')">{{ t('sidebar.openFolder') }}</Button>
            <button class="text-xs text-blue-500 hover:underline cursor-pointer" @click="cloneModal.show = true">{{ t('git.clone') }}</button>
          </div>
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
      <template v-else>
        <!-- 主根：单根时不显示标题（保持原样）；多根时也作为可折叠分区，便于折叠后查看其它根 -->
        <div v-if="hasExtra"
             class="sticky top-0 z-10 bg-gray-50 dark:bg-gray-800 flex items-center gap-1 px-2 py-1 cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700"
             @click="primaryCollapsed = !primaryCollapsed">
          <ChevronRight class="w-3 h-3 text-gray-400 transition-transform flex-shrink-0"
                        :class="{'rotate-90': !primaryCollapsed}"/>
          <Folder class="w-3.5 h-3.5 text-blue-500 flex-shrink-0"/>
          <span class="flex-1 truncate text-xs font-semibold uppercase text-gray-600 dark:text-gray-300" :title="rootDir || ''">{{ rootName }}</span>
        </div>
        <div v-show="!hasExtra || !primaryCollapsed"
             class="w-max min-w-full"
             @contextmenu.prevent="onRootContext">
          <FileTreeNode v-for="node in rootNodes" :key="node.path" :node="node" :depth="0"/>
        </div>

        <!-- 额外挂载的根（多根工作区 phase 1） -->
        <div v-for="er in (extraRoots || [])" :key="er">
          <div class="group sticky top-0 z-10 bg-gray-50 dark:bg-gray-800 flex items-center gap-1 px-2 py-1 cursor-pointer border-t border-gray-200 dark:border-gray-700 hover:bg-gray-100 dark:hover:bg-gray-700"
               @click="toggleExtra(er)">
            <ChevronRight class="w-3 h-3 text-gray-400 transition-transform flex-shrink-0"
                          :class="{'rotate-90': !extraCollapsed[er]}"/>
            <Folder class="w-3.5 h-3.5 text-blue-500 flex-shrink-0"/>
            <span class="flex-1 truncate text-xs font-semibold uppercase text-gray-600 dark:text-gray-300" :title="er">{{ folderName(er) }}</span>
            <button class="p-0.5 rounded text-gray-400 hover:text-red-500 opacity-0 group-hover:opacity-100 cursor-pointer"
                    :title="t('sidebar.removeFolder')"
                    @click.stop="emit('remove-root', er)">
              <X class="w-3 h-3"/>
            </button>
          </div>
          <div v-show="!extraCollapsed[er]" class="w-max min-w-full">
            <FileTreeNode v-for="node in (extraNodesMap[er] || [])" :key="node.path" :node="node" :depth="0"/>
          </div>
        </div>
      </template>
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
          <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="copyPath">{{ t('git.copyPath') }}</button>

          <!-- Git 操作 -->
          <template v-if="gitRepo">
            <div class="border-t border-gray-100 dark:border-gray-700 my-1"></div>
            <template v-if="!ctx.node.is_dir">
              <template v-if="ctxChanged">
                <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="gitStage">{{ t('git.stageFile') }}</button>
                <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="gitUnstage">{{ t('git.unstageFile') }}</button>
                <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="gitDiff">{{ t('git.viewDiff') }}</button>
                <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer text-red-600" @click="gitDiscard">{{ t('git.discard') }}</button>
              </template>
              <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="gitBlame">{{ t('git.blameTitle') }}</button>
              <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="gitFileHistory">{{ t('git.fileHistory') }}</button>
              <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="gitIgnore">{{ t('git.ignore') }}</button>
            </template>
            <button v-else class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="gitStageDir">{{ t('git.stageDir') }}</button>
          </template>
          <template v-else-if="ctx.node.is_dir && rootDir">
            <div class="border-t border-gray-100 dark:border-gray-700 my-1"></div>
            <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="gitInit">{{ t('git.init') }}</button>
          </template>
        </template>
      </div>
    </div>

    <!-- Git 改动对比 / Blame / 文件历史（侧栏内触发）-->
    <DiffView v-if="gitView.diff"
              :original="gitView.diff.original"
              :modified="gitView.diff.modified"
              :file-name="gitView.diff.name"
              :title="t('git.diffTitle')"
              :subtitle="t('git.diffSubtitle')"
              @close="gitView.diff = null"/>
    <BlameView v-if="gitView.blame"
               :root-dir="rootDir || ''"
               :rel-path="gitView.blame.rel"
               :file-name="gitView.blame.name"
               @close="gitView.blame = null"/>
    <GitLog v-if="gitView.history"
            :root-dir="rootDir || ''"
            :rel-path="gitView.history.rel"
            :file-name="gitView.history.name"
            @close="gitView.history = null"/>

    <!-- 丢弃改动确认 -->
    <Modal v-model:show="discardGit.show" :title="t('git.discardTitle')" size="sm">
      <div class="space-y-4">
        <p class="text-sm text-gray-700 dark:text-gray-300">{{ t('git.discardConfirm', { file: discardGit.node?.name }) }}</p>
        <div class="flex justify-end gap-2">
          <Button size="sm" type="secondary" @click="discardGit.show = false">{{ t('git.cancel') }}</Button>
          <Button size="sm" type="danger" @click="confirmGitDiscard">{{ t('git.discard') }}</Button>
        </div>
      </div>
    </Modal>

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

    <!-- 克隆仓库 -->
    <Modal v-model:show="cloneModal.show" :title="t('git.cloneTitle')" size="sm">
      <div class="space-y-4">
        <Input v-model="cloneModal.url" class="w-full" :placeholder="t('git.cloneUrlPlaceholder')" @keyup.enter="doClone"/>
        <div class="flex justify-end space-x-2">
          <Button type="secondary" size="sm" @click="cloneModal.show = false">{{ t('sidebar.cancel') }}</Button>
          <Button size="sm" :loading="cloneModal.busy" :disabled="!cloneModal.url.trim()" @click="doClone">{{ t('git.clonePick') }}</Button>
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
import {open as openDialog} from '@tauri-apps/plugin-dialog'
import {listen, type UnlistenFn} from '@tauri-apps/api/event'
import {ChevronRight, Folder, FolderOpen, FolderPlus, RefreshCw, X} from 'lucide-vue-next'
import Button from '../ui/Button.vue'
import Modal from '../ui/Modal.vue'
import Input from '../ui/Input.vue'
import FileTreeNode from './FileTreeNode.vue'
import DiffView from './DiffView.vue'
import BlameView from './BlameView.vue'
import GitLog from './GitLog.vue'
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
  extraRoots?: string[]
  activePath?: string | null
  recentFolders?: string[]
  gitStatus?: Record<string, string>
  gitRepo?: boolean
}>()

const emit = defineEmits<{
  'open-folder': []
  'add-folder': []
  'remove-root': [path: string]
  'open-recent': [path: string]
  'open-file': [path: string]
  'renamed': [from: string, to: string]
  'deleted': [path: string]
  'git-refresh': []
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

// ===== 多根工作区（phase 1）：额外挂载的根，Git/搜索仍走主根 =====
const hasExtra = computed(() => !!(props.extraRoots && props.extraRoots.length))
const primaryCollapsed = ref(false)
const extraNodesMap = reactive<Record<string, FileNode[]>>({})
const extraCollapsed = reactive<Record<string, boolean>>({})
const toggleExtra = (path: string) => {
  extraCollapsed[path] = !extraCollapsed[path]
}
const loadExtraRoots = async () => {
  for (const er of props.extraRoots || []) {
    if (!(er in extraNodesMap)) {
      try {
        extraNodesMap[er] = await invoke<FileNode[]>('read_directory_tree', {path: er})
        await invoke('watch_directory', {path: er}).catch(() => {})
      }
      catch {
        extraNodesMap[er] = []
      }
    }
  }
  // 清理已移除的根
  for (const k of Object.keys(extraNodesMap)) {
    if (!(props.extraRoots || []).includes(k)) {
      delete extraNodesMap[k]
    }
  }
}
watch(() => props.extraRoots, () => loadExtraRoots(), {immediate: true, deep: true})

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

// ===== Git 操作（文件树右键）=====
const relOf = (abs: string) => {
  const root = props.rootDir || ''
  return abs.startsWith(root) ? abs.slice(root.length).replace(/^[\\/]/, '') : abs
}
// 当前右键文件是否有改动（gitStatus 中存在条目）
const ctxChanged = computed(() => !!(ctx.node && props.gitStatus?.[ctx.node.path]))
// Git 浮层：改动对比 / Blame / 文件历史
const gitView = reactive<{
  diff: { original: string; modified: string; name: string } | null
  blame: { rel: string; name: string } | null
  history: { rel: string; name: string } | null
}>({diff: null, blame: null, history: null})
const discardGit = reactive<{ show: boolean, node: FileNode | null }>({show: false, node: null})

const copyPath = async () => {
  if (!ctx.node) {
    return
  }
  try {
    await navigator.clipboard.writeText(ctx.node.path)
    toast.success(t('git.pathCopied'))
  }
  catch {
    /* ignore */
  }
  closeCtx()
}

const gitStage = async () => {
  if (!ctx.node) {
    return
  }
  try {
    await invoke('git_stage', {root: props.rootDir, paths: [ctx.node.path]})
    toast.success(t('git.staged'))
    emit('git-refresh')
  }
  catch (error) {
    toast.error(t('git.stageFailed') + ': ' + error)
  }
  closeCtx()
}

const gitStageDir = gitStage

const gitUnstage = async () => {
  if (!ctx.node) {
    return
  }
  try {
    await invoke('git_unstage', {root: props.rootDir, paths: [ctx.node.path]})
    toast.success(t('git.unstaged'))
    emit('git-refresh')
  }
  catch (error) {
    toast.error(t('git.unstageFailed') + ': ' + error)
  }
  closeCtx()
}

const gitDiff = async () => {
  const node = ctx.node
  closeCtx()
  if (!node) {
    return
  }
  const rel = relOf(node.path)
  try {
    const head = await invoke<{ exists: boolean; content: string }>('git_file_head', {root: props.rootDir, relPath: rel})
    let work = ''
    try {
      work = await invoke<string>('read_file_text', {path: node.path})
    }
    catch {
      work = ''
    }
    gitView.diff = {original: head.exists ? head.content : '', modified: work, name: rel}
  }
  catch (error) {
    toast.error(t('git.diffFailed') + ': ' + error)
  }
}

const gitDiscard = () => {
  discardGit.node = ctx.node
  discardGit.show = true
  closeCtx()
}
const confirmGitDiscard = async () => {
  const node = discardGit.node
  discardGit.show = false
  if (!node) {
    return
  }
  try {
    // 'U'（未跟踪）丢弃即删除，其余恢复到 HEAD
    if (props.gitStatus?.[node.path] === 'U') {
      await invoke('delete_path', {path: node.path})
      emit('deleted', node.path)
    }
    else {
      await invoke('git_discard', {root: props.rootDir, paths: [node.path]})
    }
    toast.success(t('git.discarded'))
    emit('git-refresh')
    triggerRefresh()
  }
  catch (error) {
    toast.error(t('git.discardFailed') + ': ' + error)
  }
}

const gitBlame = () => {
  if (!ctx.node) {
    return
  }
  const rel = relOf(ctx.node.path)
  gitView.blame = {rel, name: ctx.node.name}
  closeCtx()
}

const gitFileHistory = () => {
  if (!ctx.node) {
    return
  }
  const rel = relOf(ctx.node.path)
  gitView.history = {rel, name: ctx.node.name}
  closeCtx()
}

const gitIgnore = async () => {
  if (!ctx.node) {
    return
  }
  try {
    await invoke('git_ignore_add', {root: props.rootDir, pattern: relOf(ctx.node.path)})
    toast.success(t('git.ignored'))
    emit('git-refresh')
  }
  catch (error) {
    toast.error(t('git.ignoreFailed') + ': ' + error)
  }
  closeCtx()
}

const gitInit = async () => {
  try {
    await invoke('git_init', {root: props.rootDir})
    toast.success(t('git.initDone'))
    emit('git-refresh')
  }
  catch (error) {
    toast.error(t('git.initFailed') + ': ' + error)
  }
  closeCtx()
}

// ===== 克隆仓库 =====
const cloneModal = reactive<{ show: boolean, url: string, busy: boolean }>({show: false, url: '', busy: false})
const doClone = async () => {
  const url = cloneModal.url.trim()
  if (!url) {
    return
  }
  // 选择克隆到的父目录
  const dir = await openDialog({directory: true, multiple: false})
  if (typeof dir !== 'string') {
    return
  }
  cloneModal.busy = true
  try {
    const repoPath = await invoke<string>('git_clone', {url, dir})
    toast.success(t('git.cloned'))
    cloneModal.show = false
    cloneModal.url = ''
    // 打开克隆出的仓库目录
    emit('open-recent', repoPath)
  }
  catch (error) {
    toast.error(t('git.cloneFailed') + ': ' + error)
  }
  finally {
    cloneModal.busy = false
  }
}
</script>
