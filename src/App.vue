<template>
  <div class="h-screen flex flex-col bg-gray-50">
    <AppHeader :is-running="isRunning"
               :env-installed="envInfo.installed"
               :supported-languages="supportedLanguages"
               :current-language="currentLanguage"
               :current-layout="layoutMode"
               @run-code="handleRunCode"
               @stop-code="() => stopCode(currentLanguage)"
               @clear-output="clearOutput"
               @language-change="handleLanguageChange"
               @layout-change="handleLayoutChange"
               @show-settings="showSettings = true"
               @load-example="loadExample">
    </AppHeader>

    <div class="flex-1 overflow-hidden">
      <!-- 编辑器代码片段 -->
      <template v-if="showConsole">
        <ResizablePanels :direction="effectiveDirection" :min-primary="minPrimary" :min-secondary="minSecondary">
          <template #primary>
            <div class="h-full flex flex-col overflow-hidden">
              <div class="bg-gray-100 px-4 py-2 border-b border-gray-200 flex items-center justify-between flex-shrink-0">
                <div class="flex items-center space-x-3">
                  <img :src="`/icons/${currentLanguage.replace(/\d+$/, '')}.svg`" class="w-5 h-5" :alt="currentLanguage"/>
                  <h2 class="text-sm font-medium text-gray-700">{{ getLanguageDisplayName(currentLanguage) }} 代码编辑器</h2>
                </div>

                <div class="flex items-center space-x-2 text-xs text-gray-500">
                  <span><strong>{{ (code || '').length }}</strong> 字符</span>
                  <span><strong>{{ (code || '').split('\n').length }}</strong> 行</span>
                </div>
              </div>
              <div class="flex-1 overflow-hidden">
                <CodeEditor v-model="code" class="h-full" :language="currentLanguage" :editor-config="editorConfig" :key="editorConfigKey"/>
              </div>
            </div>
          </template>

          <template #secondary>
            <!-- 输出 -->
            <div class="h-full flex flex-col" :class="effectiveDirection === 'vertical' ? 'border-t border-gray-200' : 'border-l border-gray-200'">
              <!-- 仅编辑器模式下提供收起控制台的入口 -->
              <div v-if="layoutMode === 'editor'" class="bg-gray-100 px-4 py-2 border-b border-gray-200 flex items-center justify-between flex-shrink-0">
                <h2 class="text-sm font-medium text-gray-700">控制台</h2>
                <button class="text-gray-400 hover:text-gray-600 transition-colors" title="收起控制台" @click="showConsole = false">
                  <X class="w-4 h-4"/>
                </button>
              </div>

              <ConsoleOutput v-if="consoleType === 'console'"
                             class="flex-1"
                             :output="output"
                             :is-running="isRunning"
                             :is-success="isSuccess"
                             :execution-time="lastExecutionTime">
              </ConsoleOutput>

              <!-- Web输出组件 -->
              <WebOutput v-else-if="consoleType === 'web'"
                         class="flex-1"
                         :web-content="output"
                         :is-running="isRunning"
                         :execution-time="lastExecutionTime">
              </WebOutput>
            </div>
          </template>
        </ResizablePanels>
      </template>

      <!-- 仅编辑器：控制台未展开时占满 -->
      <div v-else class="h-full flex flex-col overflow-hidden">
        <div class="bg-gray-100 px-4 py-2 border-b border-gray-200 flex items-center justify-between flex-shrink-0">
          <div class="flex items-center space-x-3">
            <img :src="`/icons/${currentLanguage.replace(/\d+$/, '')}.svg`" class="w-5 h-5" :alt="currentLanguage"/>
            <h2 class="text-sm font-medium text-gray-700">{{ getLanguageDisplayName(currentLanguage) }} 代码编辑器</h2>
          </div>

          <div class="flex items-center space-x-2 text-xs text-gray-500">
            <span><strong>{{ (code || '').length }}</strong> 字符</span>
            <span><strong>{{ (code || '').split('\n').length }}</strong> 行</span>
          </div>
        </div>
        <div class="flex-1 overflow-hidden">
          <CodeEditor v-model="code" class="h-full" :language="currentLanguage" :editor-config="editorConfig" :key="editorConfigKey"/>
        </div>
      </div>
    </div>

    <!-- 状态栏 -->
    <StatusBar :env-info="envInfo" :is-loading="isLoadingEnvInfo" :execution-time="lastExecutionTime" :code-length="(code || '').length" @check-environment="refreshEnvInfo"/>

    <!-- 关于组件 -->
    <About v-if="showAbout" @close="closeAbout"/>

    <!-- 设置组件 -->
    <Settings v-if="showSettings" @close="closeSettings" @settings-changed="handleSettingsChanged"/>

    <!-- 更新组件 -->
    <Update v-if="showUpdate" @close="closeUpdate"/>

    <!-- Toast 组件 -->
    <Toast/>
  </div>
</template>

<script setup lang="ts">
import {computed, onMounted, onUnmounted, ref, watch} from 'vue'
import {X} from 'lucide-vue-next'
import {LayoutMode, SplitDirection} from './types/app.ts'
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
import {useEventManager} from './composables/useEventManager'
import {useAppState} from './composables/useAppState'
import {useEditorConfig} from './composables/useEditorConfig'
import Update from './components/Update.vue'

const toast = useToast()

const {
  code,
  output,
  isRunning,
  isSuccess,
  lastExecutionTime,
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
  handleLanguageChange,
  refreshLanguageList,
  refreshEnvInfo,
  initialize
} = useLanguageManager(code, clearOutput, toast)

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

// 包装运行：仅编辑器模式下点击运行时自动展开控制台
const handleRunCode = () => {
  if (layoutMode.value === 'editor') {
    showConsole.value = true
  }
  runCode(currentLanguage.value, envInfo.value.installed, envInfo.value.language)
}

const handleSettingsChanged = async (config: any) => {
  console.log('主组件接收到设置变更:', config)
  setTimeout(() => {
    editorConfigKey.value++
  }, 50)

  await refreshLanguageList()
}

const loadExample = (content: string) => {
  code.value = content || ''
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
  currentLanguage,
  toast,
  handleRealtimeOutput,
  handleExecutionComplete,
  handleExecutionStopped,
  handleExecutionTimeout,
  handleExecutionError
})

// 禁用右键菜单
window.addEventListener('contextmenu', (e) => e.preventDefault(), false)

onMounted(async () => {
  await initialize()
  await loadEditorConfig()
  await initializeEventListeners()
  consoleType.value = getCurrentConsoleType()

  // 触发 app-ready 事件，通知主进程
  window.dispatchEvent(new CustomEvent('app-ready'))
})

onUnmounted(() => {
  cleanupEventListeners()
})
</script>
