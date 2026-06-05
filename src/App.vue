<template>
  <div class="h-screen flex flex-col bg-gray-50 dark:bg-gray-900">
    <AppHeader :is-running="isRunning"
               :env-installed="envInfo.installed"
               :supported-languages="supportedLanguages"
               :current-language="currentLanguage"
               :current-layout="layoutMode"
               :sidebar-visible="sidebarVisible"
               @toggle-sidebar="toggleSidebar"
               @run-code="handleRunCode"
               @stop-code="stopCode"
               @language-change="onLanguageChange"
               @layout-change="handleLayoutChange"
               @open-file="handleOpenFileClick"
               @save-file="handleSave"
               @show-history="showHistory = true"
               @show-ai="handleShowAi"
               @show-settings="showSettings = true"
               @load-example="loadExample">
    </AppHeader>

    <!-- 运行输入：参数 + stdin（任何布局/运行前都可填）-->
    <div class="bg-gray-50 dark:bg-gray-900 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
      <button class="w-full flex items-center px-4 py-1 text-xs text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="showRunInput = !showRunInput">
        <ChevronRight class="w-3 h-3 mr-1 transition-transform" :class="{ 'rotate-90': showRunInput }"/>
        运行输入（参数 / stdin / 环境变量）
        <span v-if="!showRunInput && (runArgs || runStdin || runEnv)" class="ml-2 text-blue-500">●</span>
      </button>
      <div v-if="showRunInput" class="px-4 pb-2 space-y-2">
        <div class="flex items-start space-x-3">
          <div class="flex flex-col w-56 flex-shrink-0">
            <label class="text-[11px] text-gray-400 mb-0.5">运行参数</label>
            <input v-model="runArgs" class="text-xs border border-gray-300 rounded px-2 py-1 focus:outline-none focus:border-blue-400" placeholder="空格分隔，如 --port 8080"/>
          </div>
          <div class="flex flex-col flex-1 min-w-0">
            <label class="text-[11px] text-gray-400 mb-0.5">标准输入 (stdin)</label>
            <textarea v-model="runStdin" rows="2" class="w-full text-xs border border-gray-300 rounded px-2 py-1 font-mono resize-none focus:outline-none focus:border-blue-400" placeholder="运行时喂给程序的输入"></textarea>
          </div>
        </div>
        <div class="flex flex-col">
          <label class="text-[11px] text-gray-400 mb-0.5">环境变量</label>
          <input v-model="runEnv" class="text-xs border border-gray-300 rounded px-2 py-1 font-mono focus:outline-none focus:border-blue-400" placeholder="KEY=值，多个用 ; 分隔，如 DEBUG=1;PORT=8080"/>
        </div>
        <label class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400 cursor-pointer select-none">
          <input v-model="watchMode" type="checkbox" class="cursor-pointer"/>
          监听模式：保存后自动运行
        </label>
      </div>
    </div>

    <div class="flex-1 overflow-hidden flex">
      <!-- 左侧文件树侧栏 -->
      <template v-if="sidebarVisible">
        <Sidebar :root-dir="rootDir"
                 :active-path="currentFilePath"
                 :recent-folders="recentFolders"
                 :git-status="gitStatus"
                 class="flex-shrink-0"
                 :style="{ width: `${sidebarWidth}px` }"
                 @open-folder="openFolder"
                 @open-recent="openFolderPath"
                 @open-file="smartOpen"
                 @renamed="(from, to) => updateTabPath(from, to)"
                 @deleted="(p) => detachTabPath(p)"/>
        <!-- 拖拽改变侧栏宽度 -->
        <div class="w-1 bg-gray-200 dark:bg-gray-700 hover:bg-blue-500 cursor-col-resize transition-colors flex-shrink-0"
             @mousedown="startSidebarResize"></div>
      </template>

      <div class="flex-1 overflow-hidden">
      <!-- 编辑器代码片段 -->
      <template v-if="showConsole">
        <ResizablePanels :direction="effectiveDirection" :min-primary="minPrimary" :min-secondary="minSecondary">
          <template #primary>
            <div class="h-full flex flex-col overflow-hidden">
              <EditorTabs :tabs="editorTabs" :active-id="activeTabId" @switch="switchTab" @close="handleCloseTab" @new="handleNewTab"
                          @close-others="closeOthers" @close-right="closeToRight" @move="moveTab" @copy-path="handleCopyPath"/>
              <div v-if="!showViewer" class="bg-gray-100 dark:bg-gray-800 px-4 py-2 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between flex-shrink-0">
                <div class="flex items-center space-x-3">
                  <img :src="`/icons/${currentLanguage.replace(/\d+$/, '')}.svg`" class="w-5 h-5" :alt="currentLanguage"/>
                  <h2 class="text-sm font-medium text-gray-700 dark:text-gray-200">{{ getLanguageDisplayName(currentLanguage) }} 代码编辑器</h2>
                  <span v-if="currentFileName" class="text-xs text-gray-500 flex items-center">
                    · {{ currentFileName }}
                    <span v-if="isDirty" class="ml-1 text-amber-500" title="有未保存的修改">●</span>
                  </span>
                </div>

                <div class="flex items-center space-x-2 text-xs text-gray-500">
                  <span v-if="predicting" class="flex items-center gap-1 text-blue-500">
                    <Sparkles class="w-3 h-3 animate-pulse"/> AI 预测中…
                  </span>
                  <span v-else-if="ghostActive" class="text-blue-500">Tab 接受 · Esc 取消</span>
                  <span><strong>{{ (code || '').length }}</strong> 字符</span>
                  <span><strong>{{ (code || '').split('\n').length }}</strong> 行</span>
                </div>
              </div>
              <div class="flex-1 overflow-hidden relative">
                <CodeEditor v-model="code" class="h-full" :language="currentLanguage" :editor-config="editorConfig" :key="editorConfigKey" @ready="editorView = $event"/>
                <LargeFileViewer v-if="showViewer && viewerFile"
                                 :file-path="viewerFile.path"
                                 :line-count="viewerFile.lineCount"
                                 :size-bytes="viewerFile.sizeBytes"
                                 @close="closeViewer"/>
                <InlineGenerate v-if="showGenerate" :language="currentLanguage" :selection="generateSelection" @insert="insertGeneratedCode" @close="showGenerate = false"/>
              </div>
            </div>
          </template>

          <template #secondary>
            <!-- 输出 -->
            <div class="h-full flex flex-col" :class="effectiveDirection === 'vertical' ? 'border-t border-gray-200' : 'border-l border-gray-200'">
              <!-- 仅编辑器模式下提供收起控制台的入口 -->
              <div v-if="layoutMode === 'editor'" class="bg-gray-100 dark:bg-gray-800 px-4 py-2 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between flex-shrink-0">
                <h2 class="text-sm font-medium text-gray-700 dark:text-gray-200">控制台</h2>
                <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors" title="收起控制台" @click="showConsole = false">
                  <X class="w-4 h-4"/>
                </button>
              </div>

              <ConsoleOutput v-if="consoleType === 'console'"
                             class="flex-1"
                             :output="output"
                             :is-running="isRunning"
                             :is-success="isSuccess"
                             :execution-time="lastExecutionTime"
                             @clear="clearOutput">
              </ConsoleOutput>

              <!-- Web输出组件 -->
              <WebOutput v-else-if="consoleType === 'web'"
                         class="flex-1"
                         :web-content="output"
                         :is-running="isRunning"
                         :execution-time="lastExecutionTime"
                         @clear="clearOutput">
              </WebOutput>
            </div>
          </template>
        </ResizablePanels>
      </template>

      <!-- 仅编辑器：控制台未展开时占满 -->
      <div v-else class="h-full flex flex-col overflow-hidden">
        <EditorTabs :tabs="editorTabs" :active-id="activeTabId" @switch="switchTab" @close="handleCloseTab" @new="handleNewTab"
                          @close-others="closeOthers" @close-right="closeToRight" @move="moveTab" @copy-path="handleCopyPath"/>
        <div v-if="!showViewer" class="bg-gray-100 dark:bg-gray-800 px-4 py-2 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between flex-shrink-0">
          <div class="flex items-center space-x-3">
            <img :src="`/icons/${currentLanguage.replace(/\d+$/, '')}.svg`" class="w-5 h-5" :alt="currentLanguage"/>
            <h2 class="text-sm font-medium text-gray-700 dark:text-gray-200">{{ getLanguageDisplayName(currentLanguage) }} 代码编辑器</h2>
            <span v-if="currentFileName" class="text-xs text-gray-500 flex items-center">
              · {{ currentFileName }}
              <span v-if="isDirty" class="ml-1 text-amber-500" title="有未保存的修改">●</span>
            </span>
          </div>

          <div class="flex items-center space-x-2 text-xs text-gray-500">
            <span v-if="predicting" class="flex items-center gap-1 text-blue-500">
              <Sparkles class="w-3 h-3 animate-pulse"/> AI 预测中…
            </span>
            <span v-else-if="ghostActive" class="text-blue-500">Tab 接受 · Esc 取消</span>
            <span><strong>{{ (code || '').length }}</strong> 字符</span>
            <span><strong>{{ (code || '').split('\n').length }}</strong> 行</span>
          </div>
        </div>
        <div class="flex-1 overflow-hidden relative">
          <CodeEditor v-model="code" class="h-full" :language="currentLanguage" :editor-config="editorConfig" :key="editorConfigKey" @ready="editorView = $event"/>
          <LargeFileViewer v-if="showViewer && viewerFile"
                           :file-path="viewerFile.path"
                           :line-count="viewerFile.lineCount"
                           :size-bytes="viewerFile.sizeBytes"
                           @close="closeViewer"/>
          <InlineGenerate v-if="showGenerate" :language="currentLanguage" :selection="generateSelection" @insert="insertGeneratedCode" @close="showGenerate = false"/>
        </div>
      </div>
      </div>
    </div>

    <!-- 状态栏 -->
    <StatusBar :env-info="envInfo" :is-loading="isLoadingEnvInfo" :execution-time="lastExecutionTime" :code-length="(code || '').length" @check-environment="refreshEnvInfo"/>

    <!-- 关于组件 -->
    <About v-if="showAbout" @close="closeAbout"/>

    <!-- 设置组件 -->
    <Settings v-if="showSettings" @close="onSettingsClose" @settings-changed="handleSettingsChanged"/>

    <!-- 更新组件 -->
    <Update v-if="showUpdate" @close="closeUpdate"/>

    <!-- 执行历史 -->
    <ExecutionHistory v-model:show="showHistory"
                      :supported-languages="supportedLanguages"
                      @restore="restoreHistoryItem"
                      @rerun="rerunHistoryItem"
                      @open-ai="openAiForExecution"/>

    <!-- 运行未保存文件询问 -->
    <Modal v-model:show="showRunPrompt" title="运行未保存的文件" size="sm">
      <div class="space-y-4">
        <p class="text-sm text-gray-700 dark:text-gray-300">
          当前文件 <strong>{{ currentFileName }}</strong> 有未保存的修改，如何运行？
        </p>
        <div class="flex justify-end space-x-2">
          <Button type="secondary" size="sm" @click="showRunPrompt = false">取消</Button>
          <Button type="info" size="sm" @click="promptRunCopy">运行副本(不保存)</Button>
          <Button size="sm" @click="promptSaveAndRun">保存并运行</Button>
        </div>
      </div>
    </Modal>

    <!-- AI 助手 -->
    <AiAssistant v-if="showAi" :code="code" :language="currentLanguage" :execution-id="aiExecutionId" :error-context="aiErrorContext" :initial-prompt="aiInitialPrompt" :root-dir="rootDir" @close="showAi = false" @insert-code="applyAiCode"/>

    <!-- 文件夹内全局搜索 -->
    <SearchPanel v-if="showSearch && rootDir" :root-dir="rootDir" @open="openSearchResult" @replaced="reloadAffectedFiles" @close="showSearch = false"/>

    <!-- 快速打开文件 -->
    <QuickOpen v-if="showQuickOpen && rootDir"
               :root-dir="rootDir"
               @select="smartOpen"
               @close="showQuickOpen = false"/>

    <!-- 命令面板 -->
    <CommandPalette v-if="showCommandPalette"
                    :commands="paletteCommands"
                    @close="showCommandPalette = false"/>

    <!-- 跳转到行 -->
    <GoToLine v-if="showGoToLine"
              :max-line="(code || '').split('\n').length"
              @go="gotoLine"
              @close="showGoToLine = false"/>

    <!-- 符号大纲 -->
    <Outline v-if="showOutline"
             :code="code"
             :language="currentLanguage"
             @go="gotoLine"
             @close="showOutline = false"/>

    <!-- 代码片段管理 -->
    <SnippetManager v-if="showSnippets" @close="showSnippets = false"/>

    <!-- 差异对比：当前 vs 已保存 -->
    <DiffView v-if="showDiff"
              :original="savedContent || ''"
              :modified="code"
              :file-name="currentFileName"
              @close="showDiff = false"/>

    <!-- 实时预览（Markdown / HTML）-->
    <PreviewPanel v-if="showPreview"
                  :content="code"
                  :language="currentLanguage"
                  :file-name="currentFileName"
                  @close="showPreview = false"/>

    <!-- Git 源代码管理 -->
    <GitPanel v-if="showGit && rootDir"
              :root-dir="rootDir"
              @open="smartOpen"
              @refresh="refreshGitStatus"
              @close="showGit = false"/>

    <!-- Toast 组件 -->
    <Toast/>
  </div>
</template>

<script setup lang="ts">
import {computed, nextTick, onMounted, onUnmounted, ref, watch} from 'vue'
import {debounce} from 'lodash-es'
import {ChevronRight, Code2, CornerDownRight, Eye, FolderOpen, GitBranch, GitCompare, History, ListTree, Maximize2, Monitor, Moon, PanelBottom, PanelLeft, PanelRight, Play, Plus, Save, Search, Settings as SettingsIcon, Sparkles, Sun, X} from 'lucide-vue-next'
import {ExecutionResult, LayoutMode, SplitDirection} from './types/app.ts'
import AppHeader from './components/AppHeader.vue'
import CodeEditor from './components/CodeEditor.vue'
import ConsoleOutput from './components/ConsoleOutput.vue'
import WebOutput from "./components/WebOutput.vue";
import StatusBar from './components/StatusBar.vue'
import About from './components/About.vue'
import Settings from './components/Settings.vue'
import Toast from './components/Toast.vue'
import ResizablePanels from './components/ResizablePanels.vue'
import {useToast} from './plugins/toast'

// Composables
import {useCodeExecution} from './composables/useCodeExecution'
import {useLanguageManager} from './composables/useLanguageManager'
import {useFileManager} from './composables/useFileManager'
import {useLanguageRegistry} from './composables/useLanguageRegistry'
import {useWorkspace} from './composables/useWorkspace'
import EditorTabs from './components/EditorTabs.vue'
import Sidebar from './components/Sidebar.vue'
import LargeFileViewer from './components/LargeFileViewer.vue'
import QuickOpen from './components/QuickOpen.vue'
import CommandPalette, {type PaletteCommand} from './components/CommandPalette.vue'
import DiffView from './components/DiffView.vue'
import PreviewPanel from './components/PreviewPanel.vue'
import GitPanel from './components/GitPanel.vue'
import GoToLine from './components/GoToLine.vue'
import Outline from './components/Outline.vue'
import SnippetManager from './components/SnippetManager.vue'
import {initSnippets} from './composables/useSnippets'
import {kvGet, kvGetJSON, kvSet, kvSetJSON} from './composables/useKvStore'
import {useAiConfig} from './composables/useAiConfig'
import {setGhost, clearGhostIn, ghostActive} from './editor/aiComplete'
import {computeDiffMarkers, setDiffMarkers} from './editor/diffGutter'
import AiAssistant from './components/AiAssistant.vue'
import InlineGenerate from './components/InlineGenerate.vue'
import SearchPanel from './components/SearchPanel.vue'
import {useTheme, type AppTheme} from './composables/useTheme'
import Modal from './ui/Modal.vue'
import Button from './ui/Button.vue'
import ExecutionHistory from './components/ExecutionHistory.vue'
import {open as openDialog} from '@tauri-apps/plugin-dialog'
import {invoke} from '@tauri-apps/api/core'
import {useEventManager} from './composables/useEventManager'
import {useShortcuts} from './composables/useShortcuts'
import {useAppState} from './composables/useAppState'
import {useEditorConfig} from './composables/useEditorConfig'
import Update from './components/Update.vue'

const toast = useToast()
const showHistory = ref(false)

const {
  code,
  output,
  isRunning,
  isSuccess,
  lastExecutionTime,
  currentExecutionId,
  runCode,
  stopCode,
  clearOutput,
  handleRealtimeOutput,
  handleExecutionComplete,
  handleExecutionStopped,
  handleExecutionTimeout,
  handleExecutionError
} = useCodeExecution(toast)

const {
  currentLanguage,
  supportedLanguages,
  envInfo,
  isLoadingEnvInfo,
  getLanguageDisplayName,
  getCurrentConsoleType,
  getCurrentPluginConfig,
  handleLanguageChange,
  applyLanguage,
  refreshLanguageList,
  refreshEnvInfo,
  initialize
} = useLanguageManager(code, clearOutput, toast)

// 扩展名 ↔ 语言 注册表
const {build: buildLanguageRegistry, detectLanguage, getCandidates} = useLanguageRegistry()

// 本地文件管理（打开/保存/另存为）
const getDefaultFileName = () => {
  const ext = getCurrentPluginConfig()?.extension || currentLanguage.value || 'txt'
  return `未命名.${ext}`
}

// 文件状态（提升到此层，供文件管理与多标签工作区共享）
const currentFilePath = ref<string | null>(null)
const savedContent = ref<string | null>(null)
const restoreFile = (filePath: string | null, saved: string | null) => {
  currentFilePath.value = filePath
  savedContent.value = saved
}

// 多标签工作区
const {
  tabs: editorTabs,
  activeTabId,
  switchTab,
  newTab,
  closeTab,
  closeOthers,
  closeToRight,
  moveTab,
  updateTabPath,
  detachTabPath,
  isActiveReusableScratch,
  initFirstTab
} = useWorkspace({code, currentLanguage, applyLanguage, currentFilePath, savedContent, restoreFile})

// 打开文件后按扩展名自动切换语言（不改动已载入的内容、不解除文件关联）
const handleFileOpened = (filePath: string) => {
  // 优先保持当前语言：当前引擎已匹配该扩展名时不切换（如已在某 JS 引擎上打开 .js）
  const detected = detectLanguage(filePath, currentLanguage.value)
  if (detected && detected !== currentLanguage.value) {
    applyLanguage(detected)
    // 同扩展名对应多个引擎时，提示可手动切换
    if (getCandidates(filePath).length > 1) {
      toast.info(`该类型可用多个运行引擎，已选「${getLanguageDisplayName(detected)}」，可在下拉手动切换`)
    }
  }
}

const {
  currentFileName,
  isDirty,
  pickFile,
  openPath,
  saveFile,
  saveFileAs,
  resetFile
} = useFileManager({
  code,
  toast,
  getDefaultFileName,
  currentFilePath,
  savedContent,
  // 打开文件时若当前不是空白草稿，则在新标签页打开
  onBeforeLoad: () => {
    if (!isActiveReusableScratch()) {
      newTab({language: currentLanguage.value})
    }
  },
  onOpened: handleFileOpened,
  getMaxFileSizeMb: () => editorConfig.value?.max_open_file_size
})

// 手动切换语言（下拉框）：替换为模板并解除文件关联
const onLanguageChange = (language: string) => {
  handleLanguageChange(language)
  resetFile()
}

const handleNewTab = () => newTab({language: currentLanguage.value, code: ''})
const handleCloseTab = (id: string) => closeTab(id, {language: currentLanguage.value})

const handleCopyPath = async (path: string) => {
  try {
    await navigator.clipboard.writeText(path)
    toast.success('已复制路径')
  }
  catch (error) {
    toast.error('复制失败: ' + error)
  }
}

// ===== 侧栏 / 文件夹 =====
const rootDir = ref<string | null>(null)
const sidebarVisible = ref(kvGet('sidebar-visible') === 'true')
const sidebarWidth = ref(Number(kvGet('sidebar-width')) || 240)

// 最近打开的文件夹
const RECENT_FOLDERS_KEY = 'recent-folders'
const LAST_ROOT_KEY = 'last-root-dir'
const recentFolders = ref<string[]>(kvGetJSON<string[]>(RECENT_FOLDERS_KEY, []))

// 记住打开的文件夹（去重、置顶、最多 8 个），并记录为上次文件夹
const rememberFolder = (path: string) => {
  const list = [path, ...recentFolders.value.filter(p => p !== path)].slice(0, 8)
  recentFolders.value = list
  kvSetJSON(RECENT_FOLDERS_KEY, list)
  kvSet(LAST_ROOT_KEY, path)
}

const openFolderPath = (path: string) => {
  rootDir.value = path
  sidebarVisible.value = true
  rememberFolder(path)
}

// ===== 标签会话持久化 =====
const SESSION_TABS_KEY = 'session-tabs'

const persistSession = () => {
  const paths = editorTabs.value.map(t => t.filePath).filter((p): p is string => !!p)
  const activePath = editorTabs.value.find(t => t.id === activeTabId.value)?.filePath || null
  kvSetJSON(SESSION_TABS_KEY, {paths, activePath})
}

// 标签集合/文件/激活项变化时持久化（不含正文编辑，避免频繁写入）
watch(
    () => editorTabs.value.map(t => t.filePath || '').join('|') + '#' + activeTabId.value,
    () => persistSession()
)

// 启动时恢复上次打开的文件标签（仅已保存且可读的文本文件）
const restoreSession = async () => {
  const saved = kvGetJSON<{ paths: string[], activePath: string | null } | null>(SESSION_TABS_KEY, null)
  if (!saved || !saved.paths?.length) {
    return
  }

  const limitBytes = (editorConfig.value?.max_open_file_size ?? 5) * 1024 * 1024
  for (const p of saved.paths) {
    try {
      const meta = await invoke<{ size_bytes: number, is_text: boolean }>('get_text_file_meta', {path: p})
      if (meta.is_text && meta.size_bytes <= limitBytes) {
        await openPath(p)
      }
    }
    catch {
      // 跳过已删除/无法读取的文件
    }
  }

  if (saved.activePath) {
    const t = editorTabs.value.find(tab => tab.filePath === saved.activePath)
    if (t) {
      switchTab(t.id)
    }
  }
}

watch(sidebarVisible, (v) => kvSet('sidebar-visible', String(v)))

// 拖拽改变侧栏宽度
let resizeStartX = 0
let resizeStartWidth = 0
const onSidebarResize = (e: MouseEvent) => {
  const w = resizeStartWidth + (e.clientX - resizeStartX)
  sidebarWidth.value = Math.max(160, Math.min(600, w))
}
const stopSidebarResize = () => {
  document.removeEventListener('mousemove', onSidebarResize)
  document.removeEventListener('mouseup', stopSidebarResize)
  document.body.style.userSelect = ''
  document.body.style.cursor = ''
  kvSet('sidebar-width', String(sidebarWidth.value))
}
const startSidebarResize = (e: MouseEvent) => {
  e.preventDefault()
  resizeStartX = e.clientX
  resizeStartWidth = sidebarWidth.value
  document.addEventListener('mousemove', onSidebarResize)
  document.addEventListener('mouseup', stopSidebarResize)
  document.body.style.userSelect = 'none'
  document.body.style.cursor = 'col-resize'
}

const toggleSidebar = () => {
  sidebarVisible.value = !sidebarVisible.value
}

const openFolder = async () => {
  const selected = await openDialog({directory: true, multiple: false})
  if (selected && typeof selected === 'string') {
    openFolderPath(selected)
  }
}

// 只读大文件查看器状态
const showViewer = ref(false)
const viewerFile = ref<{ path: string, lineCount: number, sizeBytes: number } | null>(null)

// 按文件大小决定：可编辑打开 / 只读查看
const smartOpen = async (filePath: string) => {
  try {
    const meta = await invoke<{ size_bytes: number, line_count: number, is_text: boolean }>('get_text_file_meta', {path: filePath})
    if (!meta.is_text) {
      toast.error('不是文本文件，无法打开')
      return
    }

    const limitBytes = (editorConfig.value?.max_open_file_size ?? 5) * 1024 * 1024
    if (meta.size_bytes > limitBytes) {
      // 超过可编辑上限 → 只读查看器
      viewerFile.value = {path: filePath, lineCount: meta.line_count, sizeBytes: meta.size_bytes}
      showViewer.value = true
      return
    }

    // 已打开则切换到对应标签，否则在新标签打开
    const existing = editorTabs.value.find(t => t.filePath === filePath)
    if (existing) {
      switchTab(existing.id)
      return
    }
    await openPath(filePath)
  }
  catch (error) {
    toast.error('打开失败: ' + error)
  }
}

const handleOpenFileClick = async () => {
  const path = await pickFile()
  if (path) {
    await smartOpen(path)
  }
}

// AI 助手抽屉（绑定的执行 id：工具栏打开取最近一次运行，历史面板打开取指定运行）
const showAi = ref(false)
const aiExecutionId = ref<number | null>(null)
// 失败运行的报错上下文，供"分析报错"快捷动作
const aiErrorContext = ref<{ code: string, error: string } | null>(null)
// 打开 AI 时的初始提示（解释/生成测试等一次性动作）
const aiInitialPrompt = ref<string | null>(null)

const combinedOutput = (item: ExecutionResult) =>
    [item.stdout?.trim(), item.stderr?.trim()].filter(Boolean).join('\n\n')

const handleShowAi = () => {
  aiExecutionId.value = currentExecutionId.value
  // 最近一次运行失败则带上报错
  aiErrorContext.value = currentExecutionId.value != null && !isSuccess.value && output.value
      ? {code: code.value, error: output.value}
      : null
  aiInitialPrompt.value = null
  showAi.value = true
}

// 取选中文本，无选中则用整篇代码
const selectedOrAll = (): string => {
  const view = editorView.value
  if (view) {
    const {from, to} = view.state.selection.main
    if (from !== to) {
      return view.state.sliceDoc(from, to)
    }
  }
  return code.value
}

// 以一次性提示打开 AI（临时会话，不关联执行）
const openAiWithPrompt = (prompt: string) => {
  aiExecutionId.value = null
  aiErrorContext.value = null
  aiInitialPrompt.value = prompt
  showAi.value = true
}

const explainCode = () => {
  const snippet = selectedOrAll()
  if (!snippet.trim()) {
    toast.info('没有可解释的代码')
    return
  }
  openAiWithPrompt(`请解释下面这段 ${currentLanguage.value} 代码的作用与关键逻辑，用中文简洁说明：\n\n\`\`\`${currentLanguage.value}\n${snippet}\n\`\`\``)
}

const generateTests = () => {
  const snippet = selectedOrAll()
  if (!snippet.trim()) {
    toast.info('没有可生成测试的代码')
    return
  }
  openAiWithPrompt(`请为下面这段 ${currentLanguage.value} 代码生成单元测试，覆盖主要分支与边界情况，使用该语言常用的测试框架，只输出测试代码：\n\n\`\`\`${currentLanguage.value}\n${snippet}\n\`\`\``)
}

const openAiForExecution = (item: ExecutionResult) => {
  aiExecutionId.value = item.id ?? null
  aiErrorContext.value = item.success
      ? null
      : {code: item.code, error: combinedOutput(item) || '(无输出)'}
  showAi.value = true
}

// ===== AI 代码预测（幽灵补全，Tab 接受）=====
const {active: aiActive, reload: reloadAiCfg} = useAiConfig()
const aiCompletion = ref(kvGet('ai-completion') === 'true')

const toggleAiCompletion = () => {
  aiCompletion.value = !aiCompletion.value
  kvSet('ai-completion', String(aiCompletion.value))
  if (!aiCompletion.value) {
    clearGhostIn(editorView.value)
    toast.info('AI 代码预测已关闭')
    return
  }
  // 开启时若未配置 API Key，明确提示（否则预测会静默不工作）
  reloadAiCfg()
  if (!aiActive.value.apiKey) {
    toast.warning('AI 代码预测已开启，但尚未配置 API Key，请在「设置 → AI」中填写')
  }
  else {
    toast.info('AI 代码预测已开启，停顿打字即可看到灰色补全，Tab 接受')
  }
}

// 清洗补全结果：去掉代码块围栏与开头多余换行
const cleanCompletion = (raw: string): string => {
  let s = raw.replace(/^```[a-z]*\n?/i, '').replace(/```\s*$/i, '')
  s = s.replace(/^\n+/, '')
  return s
}

let predictNonce = 0
let predictInflight = 0
// 是否正在请求 AI 预测（用于状态提示）
const predicting = ref(false)
const requestPrediction = async () => {
  if (!aiCompletion.value || isRunning.value) {
    return
  }
  const view = editorView.value
  if (!view) {
    return
  }
  reloadAiCfg()
  if (!aiActive.value.apiKey) {
    return
  }
  const sel = view.state.selection.main
  if (!sel.empty) {
    return
  }
  const pos = sel.head
  const prefix = view.state.sliceDoc(0, pos)
  if (!prefix.trim()) {
    return
  }
  const suffix = view.state.sliceDoc(pos)
  const nonce = ++predictNonce
  predictInflight++
  predicting.value = true
  try {
    const res = await invoke<string>('ai_chat', {
      provider: aiActive.value.provider,
      baseUrl: aiActive.value.baseUrl,
      apiKey: aiActive.value.apiKey,
      model: aiActive.value.model,
      system: '你是代码自动补全引擎。只输出应插入在光标处的后续代码，不要解释、不要重复已有代码、不要使用代码块标记。若无合适补全则输出空。',
      messages: [{
        role: 'user',
        content: `语言：${currentLanguage.value}\n光标前代码：\n${prefix}\n\n光标后代码：\n${suffix}\n\n请仅输出应插入光标处的后续代码：`
      }]
    })
    // 丢弃过期结果或光标已移动的情况
    if (nonce !== predictNonce) {
      return
    }
    const v = editorView.value
    if (!v || v.state.selection.main.head !== pos) {
      return
    }
    const text = cleanCompletion(res)
    if (text) {
      v.dispatch({effects: setGhost.of({text, pos})})
    }
  }
  catch {
    // 静默失败，不打扰编辑
  }
  finally {
    predictInflight--
    if (predictInflight === 0) {
      predicting.value = false
    }
  }
}
const requestPredictionDebounced = debounce(requestPrediction, 600)
watch(code, () => {
  if (aiCompletion.value) {
    requestPredictionDebounced()
  }
})

// 把 AI 代码块应用到编辑器（替换当前内容，可撤销）
const applyAiCode = (codeText: string) => {
  code.value = codeText
}

// 当前 CodeMirror view（用于在光标处插入生成的代码）
const editorView = ref<any>(null)

// AI 自然语言生成 / 选区改写
const showGenerate = ref(false)
const generateSelection = ref('')
const openGenerate = () => {
  const view = editorView.value
  generateSelection.value = view
      ? view.state.sliceDoc(view.state.selection.main.from, view.state.selection.main.to)
      : ''
  showGenerate.value = true
}

// 在光标处插入/替换选区为生成的代码
const insertGeneratedCode = (text: string) => {
  const view = editorView.value
  if (view) {
    const sel = view.state.selection.main
    view.dispatch({
      changes: {from: sel.from, to: sel.to, insert: text},
      selection: {anchor: sel.from + text.length}
    })
    view.focus()
  }
  else {
    code.value = text
  }
}

// 文件夹内全局搜索（Cmd+Shift+F）
const showSearch = ref(false)
const openSearch = () => {
  if (!rootDir.value) {
    toast.info('请先打开文件夹')
    return
  }
  showSearch.value = true
}

const gotoLine = (line: number) => {
  const view = editorView.value
  if (!view) {
    return
  }
  const target = Math.max(1, Math.min(line, view.state.doc.lines))
  const l = view.state.doc.line(target)
  view.dispatch({selection: {anchor: l.from}, scrollIntoView: true})
  view.focus()
}

// 跳转到行（Cmd+G）
const showGoToLine = ref(false)
const openGoToLine = () => {
  showGoToLine.value = true
}

// 符号大纲（Cmd+Shift+O）
const showOutline = ref(false)
const openOutline = () => {
  showOutline.value = true
}

// 代码片段管理
const showSnippets = ref(false)

const openSearchResult = async (path: string, line: number) => {
  showSearch.value = false
  await smartOpen(path)
  await nextTick()
  gotoLine(line)
}

// 全局替换后：刷新涉及到的已打开标签（保留有未保存修改的标签）
const reloadAffectedFiles = async (paths: string[]) => {
  const set = new Set(paths)
  for (const tab of editorTabs.value) {
    if (!tab.filePath || !set.has(tab.filePath)) {
      continue
    }
    // 有未保存修改则不覆盖，避免丢失用户编辑
    const dirty = tab.savedContent !== null && tab.code !== tab.savedContent
    if (dirty) {
      continue
    }
    try {
      const content = await invoke<string>('read_file_text', {
        path: tab.filePath,
        maxSizeMb: editorConfig.value?.max_open_file_size
      })
      if (tab.id === activeTabId.value) {
        code.value = content
        savedContent.value = content
      }
      else {
        tab.code = content
        tab.savedContent = content
      }
    }
    catch {
      // 跳过无法读取的文件
    }
  }
}

// 快速打开（Cmd+P）
const showQuickOpen = ref(false)
const openQuickOpen = () => {
  if (!rootDir.value) {
    toast.info('请先打开文件夹')
    return
  }
  showQuickOpen.value = true
}

// 命令面板（Cmd+Shift+P）
const showCommandPalette = ref(false)
const openCommandPalette = () => {
  showCommandPalette.value = true
}

// 差异对比 / 实时预览
const showDiff = ref(false)
const showPreview = ref(false)
const openDiff = () => {
  if (!currentFilePath.value) {
    toast.info('差异对比需要先打开已保存的文件')
    return
  }
  showDiff.value = true
}
const togglePreview = () => {
  showPreview.value = !showPreview.value
}

// ===== Git 源代码管理 =====
const showGit = ref(false)
const openGit = () => {
  if (!rootDir.value) {
    toast.info('请先打开文件夹')
    return
  }
  showGit.value = true
}

// 文件树徽标用：绝对路径 → 状态字母（M/A/D/U）
const gitStatus = ref<Record<string, string>>({})
const refreshGitStatus = async () => {
  if (!rootDir.value) {
    gitStatus.value = {}
    return
  }
  try {
    const s = await invoke<{ is_repo: boolean, files: { path: string, index: string, worktree: string }[] }>(
        'git_status', {root: rootDir.value}
    )
    const map: Record<string, string> = {}
    if (s.is_repo) {
      for (const f of s.files) {
        const code = f.index === '?'
            ? 'U'
            : (f.worktree.trim() || f.index.trim() || 'M')
        map[`${rootDir.value}/${f.path}`] = code
      }
    }
    gitStatus.value = map
  }
  catch {
    gitStatus.value = {}
  }
  // HEAD 可能因提交/切换分支变化，刷新编辑器行内差异基线
  fetchBaseline()
}

// ===== 编辑器行内差异标记（vs HEAD）=====
// 当前文件在 HEAD 中的内容；null 表示无基线（新文件/非 git/未跟踪），不显示标记
const gitBaseline = ref<string | null>(null)

const fetchBaseline = async () => {
  if (!rootDir.value || !currentFilePath.value || !currentFilePath.value.startsWith(rootDir.value)) {
    gitBaseline.value = null
    applyDiffMarkers()
    return
  }
  const rel = currentFilePath.value.slice(rootDir.value.length + 1)
  try {
    const head = await invoke<{ exists: boolean, content: string }>('git_file_head', {
      root: rootDir.value,
      relPath: rel
    })
    gitBaseline.value = head.exists ? head.content : null
  }
  catch {
    gitBaseline.value = null
  }
  applyDiffMarkers()
}

// 计算并派发标记到编辑器
const applyDiffMarkers = () => {
  const view = editorView.value
  if (!view) {
    return
  }
  const markers = gitBaseline.value === null
      ? {changed: new Map(), deleted: new Set<number>()}
      : computeDiffMarkers(gitBaseline.value, code.value)
  view.dispatch({effects: setDiffMarkers.of(markers)})
}
const applyDiffMarkersDebounced = debounce(applyDiffMarkers, 250)

// 切换文件取新基线；编辑时重算；编辑器重挂时重新派发
watch(currentFilePath, () => fetchBaseline())
watch(code, () => applyDiffMarkersDebounced())
watch(editorView, () => applyDiffMarkers())

// 打开文件夹、保存文件后刷新文件树 Git 徽标与差异基线
watch(rootDir, () => refreshGitStatus(), {immediate: true})
watch(savedContent, () => refreshGitStatus())

const closeViewer = () => {
  showViewer.value = false
  viewerFile.value = null
}

const {
  showAbout,
  showSettings,
  showUpdate,
  closeAbout,
  closeSettings,
  closeUpdate
} = useAppState()

// 编辑器配置管理
const {
  editorConfig,
  loadConfig: loadEditorConfig
} = useEditorConfig()

// 强制刷新 CodeEditor 组件的 key
const editorConfigKey = ref(0)
const consoleType = ref('console')

// ===== 布局管理 =====
// 当前布局模式：horizontal(左右) / vertical(上下) / editor(仅编辑器)
const layoutMode = computed<LayoutMode>(() => editorConfig.value?.layout || 'horizontal')

// 控制台是否展开（仅编辑器模式下运行后才展开）
const showConsole = ref(true)

// 实际分割方向：仅编辑器模式按上次保存的方向弹出
const effectiveDirection = computed<SplitDirection>(() => {
  if (layoutMode.value === 'editor') {
    return editorConfig.value?.last_direction || 'horizontal'
  }
  return layoutMode.value
})

// 不同方向使用不同的最小尺寸
const minPrimary = computed(() => effectiveDirection.value === 'vertical' ? 200 : 400)
const minSecondary = computed(() => effectiveDirection.value === 'vertical' ? 150 : 300)

// 布局模式变化时同步控制台展开状态
watch(layoutMode, (mode) => {
  showConsole.value = mode !== 'editor'
}, {immediate: true})

const handleLayoutChange = (mode: LayoutMode) => {
  if (!editorConfig.value) {
    return
  }
  editorConfig.value.layout = mode
  // 记录最近使用的分割方向，供仅编辑器模式弹出时复用
  if (mode === 'horizontal' || mode === 'vertical') {
    editorConfig.value.last_direction = mode
  }
}

// 运行输入：参数 + stdin + 环境变量
const showRunInput = ref(false)
const runArgs = ref('')
const runStdin = ref('')
const runEnv = ref('')

// 监听模式：保存后自动运行
const watchMode = ref(kvGet('watch-mode') === 'true')
watch(watchMode, (v) => kvSet('watch-mode', String(v)))

// 保存包装：保存后若开启监听模式则自动运行
const handleSave = async () => {
  await saveFile()
  if (watchMode.value && currentFilePath.value && !isDirty.value) {
    handleRunCode()
  }
}

// ===== 按文件记忆运行配置（args/stdin/env）=====
const RUN_CONFIGS_KEY = 'run-configs'
type RunConfig = { args: string, stdin: string, env: string }
const loadRunConfigs = (): Record<string, RunConfig> =>
    kvGetJSON<Record<string, RunConfig>>(RUN_CONFIGS_KEY, {})
// 把当前输入写入指定文件的配置（全空则删除该条）
const saveRunConfig = (path: string) => {
  const map = loadRunConfigs()
  if (!runArgs.value && !runStdin.value && !runEnv.value) {
    delete map[path]
  }
  else {
    map[path] = {args: runArgs.value, stdin: runStdin.value, env: runEnv.value}
  }
  kvSetJSON(RUN_CONFIGS_KEY, map)
}
// 载入指定文件的配置（无则清空）
const loadRunConfig = (path: string | null) => {
  const cfg = path ? loadRunConfigs()[path] : null
  runArgs.value = cfg?.args || ''
  runStdin.value = cfg?.stdin || ''
  runEnv.value = cfg?.env || ''
}

// 切换文件时：保存旧文件输入、载入新文件输入
watch(currentFilePath, (np, op) => {
  if (op) {
    saveRunConfig(op)
  }
  loadRunConfig(np)
})

// 编辑输入时防抖保存到当前文件
const persistRunConfig = debounce(() => {
  if (currentFilePath.value) {
    saveRunConfig(currentFilePath.value)
  }
}, 400)
watch([runArgs, runStdin, runEnv], () => persistRunConfig())

// 解析环境变量文本（KEY=值，按换行或分号分隔）
const parseEnv = (text: string): Record<string, string> => {
  const env: Record<string, string> = {}
  for (const part of text.split(/[\n;]/)) {
    const seg = part.trim()
    if (!seg) continue
    const eq = seg.indexOf('=')
    if (eq > 0) {
      env[seg.slice(0, eq).trim()] = seg.slice(eq + 1).trim()
    }
  }
  return env
}

const buildRunBase = () => {
  const env = parseEnv(runEnv.value)
  return {
    language: currentLanguage.value,
    envInstalled: envInfo.value.installed,
    envLanguage: envInfo.value.language,
    args: runArgs.value.trim() ? runArgs.value.trim().split(/\s+/) : undefined,
    stdin: runStdin.value || undefined,
    env: Object.keys(env).length ? env : undefined
  }
}

// 运行未保存文件的询问弹窗
const showRunPrompt = ref(false)

// 包装运行：仅编辑器模式下点击运行时自动展开控制台；关联文件则按策略就地运行
// 运行选中片段：以选中文本作为临时代码运行（不就地、不关联文件）
const runSelection = () => {
  const view = editorView.value
  if (!view) {
    return
  }
  const {from, to} = view.state.selection.main
  if (from === to) {
    toast.info('请先选中要运行的代码')
    return
  }
  const selected = view.state.sliceDoc(from, to)
  if (layoutMode.value === 'editor') {
    showConsole.value = true
  }
  runCode({...buildRunBase(), codeOverride: selected})
}

const handleRunCode = async () => {
  if (layoutMode.value === 'editor') {
    showConsole.value = true
  }

  // 草稿（无关联文件）：临时目录运行
  if (!currentFilePath.value) {
    runCode(buildRunBase())
    return
  }
  // 无改动：直接就地运行
  if (!isDirty.value) {
    runCode({...buildRunBase(), filePath: currentFilePath.value})
    return
  }

  // 有未保存改动：按设置的策略处理
  const strategy = editorConfig.value?.run_save_strategy || 'auto-save'
  if (strategy === 'temp-copy') {
    runCode(buildRunBase()) // 跑当前未保存内容的临时副本
  }
  else if (strategy === 'ask') {
    showRunPrompt.value = true
  }
  else {
    await saveFile()
    runCode({...buildRunBase(), filePath: currentFilePath.value})
  }
}

const promptSaveAndRun = async () => {
  showRunPrompt.value = false
  await saveFile()
  runCode({...buildRunBase(), filePath: currentFilePath.value})
}

const promptRunCopy = () => {
  showRunPrompt.value = false
  runCode(buildRunBase())
}

// 设置关闭后刷新缓存的编辑器配置与快捷键绑定，使其即时生效
const onSettingsClose = async () => {
  closeSettings()
  await loadEditorConfig()
  reloadShortcuts()
}

const handleSettingsChanged = async (config: any) => {
  console.log('主组件接收到设置变更:', config)
  setTimeout(() => {
    editorConfigKey.value++
  }, 50)

  await refreshLanguageList()
  await buildLanguageRegistry()
}

const loadExample = (content: string) => {
  code.value = content || ''
  // 示例内容不对应任何本地文件，解除文件关联
  resetFile()
}

const restoreHistoryItem = (item: ExecutionResult) => {
  applyLanguage(item.language)
  code.value = item.code || ''
  resetFile()
  clearOutput()
  toast.success('已恢复历史代码')
}

// 从执行历史一键重跑：恢复语言与代码后直接运行
const rerunHistoryItem = async (item: ExecutionResult) => {
  applyLanguage(item.language)
  code.value = item.code || ''
  resetFile()
  // 确保环境信息已更新为该语言再运行
  await refreshEnvInfo()
  handleRunCode()
}

// 监听编辑器配置变化
watch(editorConfig, (newConfig) => {
  if (newConfig) {
    console.log('编辑器配置更新，刷新编辑器组件')
    setTimeout(() => {
      editorConfigKey.value++
    }, 50)
  }
}, {deep: true})

watch(currentLanguage, () => {
  consoleType.value = getCurrentConsoleType()
})

const {initializeEventListeners, cleanupEventListeners} = useEventManager({
  showAbout,
  showSettings,
  showUpdate,
  output,
  isRunning,
  isSuccess,
  lastExecutionTime,
  toast,
  handleRealtimeOutput,
  handleExecutionComplete,
  handleExecutionStopped,
  handleExecutionTimeout,
  handleExecutionError
})

// 禁用右键菜单
window.addEventListener('contextmenu', (e) => e.preventDefault(), false)

// 是否有弹窗/覆盖层打开（打开时不响应全局快捷键）
const isOverlayOpen = () =>
    showSettings.value || showAbout.value || showUpdate.value
    || showHistory.value || showViewer.value || showRunPrompt.value
    || showQuickOpen.value || showGenerate.value || showSearch.value
    || showCommandPalette.value || showDiff.value || showGoToLine.value || showOutline.value || showSnippets.value

// 全局快捷键（绑定可在设置中自定义）
const {matchAction: matchShortcut, reload: reloadShortcuts, getBinding, formatCombo} = useShortcuts()

const shortcutDispatch: Record<string, () => void> = {
  run: () => handleRunCode(),
  runSelection: () => runSelection(),
  quickOpen: () => openQuickOpen(),
  commandPalette: () => openCommandPalette(),
  gotoLine: () => openGoToLine(),
  outline: () => openOutline(),
  searchInFiles: () => openSearch(),
  generate: () => openGenerate(),
  save: () => handleSave(),
  saveAs: () => saveFileAs(),
  open: () => handleOpenFileClick(),
  newTab: () => handleNewTab(),
  closeTab: () => handleCloseTab(activeTabId.value),
  toggleSidebar: () => toggleSidebar()
}

// 切换并持久化外观主题
const applyTheme = async (t: AppTheme) => {
  setAppTheme(t)
  try {
    const config = await invoke<any>('get_app_config')
    config.theme = t
    await invoke('update_app_config', {config})
  }
  catch (error) {
    console.error('保存主题失败:', error)
  }
}

// 命令面板命令列表（含快捷键提示）
const hintOf = (id: string) => formatCombo(getBinding(id))
const paletteCommands = computed<PaletteCommand[]>(() => [
  {id: 'run', label: '运行代码', icon: Play, hint: hintOf('run'), run: () => handleRunCode()},
  {id: 'runSelection', label: '运行选中片段', icon: Play, hint: hintOf('runSelection'), run: () => runSelection()},
  {id: 'watchMode', label: watchMode.value ? '关闭监听模式（保存自动运行）' : '开启监听模式（保存自动运行）', icon: Eye, run: () => { watchMode.value = !watchMode.value }},
  {id: 'aiCompletion', label: aiCompletion.value ? '关闭 AI 代码预测' : '开启 AI 代码预测（Tab 补全）', icon: Sparkles, run: () => toggleAiCompletion()},
  {id: 'open', label: '打开文件', icon: FolderOpen, hint: hintOf('open'), run: () => handleOpenFileClick()},
  {id: 'openFolder', label: '打开文件夹', icon: FolderOpen, run: () => openFolder()},
  {id: 'save', label: '保存文件', icon: Save, hint: hintOf('save'), run: () => saveFile()},
  {id: 'saveAs', label: '另存为', icon: Save, hint: hintOf('saveAs'), run: () => saveFileAs()},
  {id: 'newTab', label: '新建标签', icon: Plus, hint: hintOf('newTab'), run: () => handleNewTab()},
  {id: 'closeTab', label: '关闭标签', icon: X, hint: hintOf('closeTab'), run: () => handleCloseTab(activeTabId.value)},
  {id: 'quickOpen', label: '快速打开文件', icon: Search, hint: hintOf('quickOpen'), run: () => openQuickOpen()},
  {id: 'gotoLine', label: '跳转到行', icon: CornerDownRight, hint: hintOf('gotoLine'), run: () => openGoToLine()},
  {id: 'outline', label: '符号大纲', icon: ListTree, hint: hintOf('outline'), run: () => openOutline()},
  {id: 'snippets', label: '管理代码片段', icon: Code2, run: () => { showSnippets.value = true }},
  {id: 'searchInFiles', label: '在文件夹内搜索', icon: Search, hint: hintOf('searchInFiles'), run: () => openSearch()},
  {id: 'generate', label: 'AI 生成代码', icon: Sparkles, hint: hintOf('generate'), run: () => openGenerate()},
  {id: 'showAi', label: 'AI 助手', icon: Sparkles, run: () => handleShowAi()},
  {id: 'explainCode', label: 'AI 解释代码（选中或全文）', icon: Sparkles, run: () => explainCode()},
  {id: 'generateTests', label: 'AI 生成测试（选中或全文）', icon: Sparkles, run: () => generateTests()},
  {id: 'history', label: '执行历史', icon: History, run: () => { showHistory.value = true }},
  {id: 'diff', label: '差异对比（当前 vs 已保存）', icon: GitCompare, run: () => openDiff()},
  {id: 'preview', label: '实时预览（Markdown / HTML）', icon: Eye, run: () => togglePreview()},
  {id: 'git', label: 'Git 源代码管理', icon: GitBranch, run: () => openGit()},
  {id: 'toggleSidebar', label: '切换侧栏', icon: PanelLeft, hint: hintOf('toggleSidebar'), run: () => toggleSidebar()},
  {id: 'layoutHorizontal', label: '布局：左右', group: '布局', icon: PanelRight, run: () => handleLayoutChange('horizontal')},
  {id: 'layoutVertical', label: '布局：上下', group: '布局', icon: PanelBottom, run: () => handleLayoutChange('vertical')},
  {id: 'layoutEditor', label: '布局：仅编辑器', group: '布局', icon: Maximize2, run: () => handleLayoutChange('editor')},
  {id: 'themeSystem', label: '主题：跟随系统', group: '主题', icon: Monitor, run: () => applyTheme('system')},
  {id: 'themeLight', label: '主题：浅色', group: '主题', icon: Sun, run: () => applyTheme('light')},
  {id: 'themeDark', label: '主题：深色', group: '主题', icon: Moon, run: () => applyTheme('dark')},
  {id: 'settings', label: '打开设置', icon: SettingsIcon, run: () => { showSettings.value = true }}
])

const onGlobalKeydown = (e: KeyboardEvent) => {
  if (isOverlayOpen()) {
    return
  }
  const action = matchShortcut(e)
  if (action && shortcutDispatch[action]) {
    // 捕获阶段拦截：阻止事件到达编辑器（避免 Cmd+Enter 等被插入换行）
    e.preventDefault()
    e.stopPropagation()
    shortcutDispatch[action]()
  }
}

const {init: initTheme, setTheme: setAppTheme} = useTheme()

onMounted(async () => {
  await initTheme()
  await initialize()
  await buildLanguageRegistry()
  // 以当前内容初始化首个标签页
  initFirstTab()
  await loadEditorConfig()
  await initializeEventListeners()
  consoleType.value = getCurrentConsoleType()
  // 从数据库载入代码片段
  initSnippets()

  // 恢复上次打开的文件夹
  const lastRoot = kvGet(LAST_ROOT_KEY)
  if (lastRoot) {
    rootDir.value = lastRoot
  }

  // 恢复上次打开的文件标签
  await restoreSession()

  window.addEventListener('keydown', onGlobalKeydown, true)

  // 触发 app-ready 事件，通知主进程
  window.dispatchEvent(new CustomEvent('app-ready'))
})

onUnmounted(() => {
  cleanupEventListeners()
  window.removeEventListener('keydown', onGlobalKeydown, true)
})
</script>
