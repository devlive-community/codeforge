<template>
  <div class="h-screen flex flex-col bg-gray-50">
    <AppHeader :is-running="isRunning"
               :env-installed="envInfo.installed"
               :supported-languages="supportedLanguages"
               :current-language="currentLanguage"
               @run-code="() => runCode(currentLanguage, envInfo.installed, envInfo.language)"
               @stop-code="() => stopCode(currentLanguage)"
               @clear-output="clearOutput"
               @language-change="handleLanguageChange"
               @show-settings="showSettings = true">
    </AppHeader>

    <div class="flex-1 flex overflow-hidden">
      <!-- 代码编辑器 -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <div class="bg-gray-100 px-4 py-2 border-b border-gray-200 flex items-center justify-between flex-shrink-0">
          <h2 class="text-sm font-medium text-gray-700">{{ getLanguageDisplayName(currentLanguage) }} 代码编辑器</h2>
          <div class="text-xs text-gray-500">
            <strong>{{ code.length }}</strong> 字符, <strong>{{ code.split('\n').length }}</strong> 行
          </div>
        </div>
        <div class="flex-1 overflow-hidden">
          <CodeEditor v-model="code" class="h-full" :language="currentLanguage" :editor-config="editorConfig" :key="editorConfigKey"/>
        </div>
      </div>

      <!-- 输出 -->
      <div class="w-2/5 flex flex-col border-l border-gray-200">
        <OutputPanel v-if="activeTab === 'output'"
                     class="flex-1"
                     :output="output"
                     :is-running="isRunning"
                     :is-success="isSuccess"
                     :execution-time="lastExecutionTime">
        </OutputPanel>
      </div>
    </div>

    <!-- 状态栏 -->
    <StatusBar :env-info="envInfo" :execution-time="lastExecutionTime" :code-length="code.length"/>

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
import { onMounted, onUnmounted, ref, watch } from 'vue'
import AppHeader from './components/AppHeader.vue'
import CodeEditor from './components/CodeEditor.vue'
import OutputPanel from './components/OutputPanel.vue'
import StatusBar from './components/StatusBar.vue'
import About from './components/About.vue'
import Settings from './components/Settings.vue'
import Toast from './components/Toast.vue'
import { useToast } from './plugins/toast'

// Composables
import { useCodeExecution } from './composables/useCodeExecution'
import { useLanguageManager } from './composables/useLanguageManager'
import { useEventManager } from './composables/useEventManager'
import { useAppState } from './composables/useAppState'
import { useEditorConfig } from './composables/useEditorConfig'
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
  getLanguageDisplayName,
  handleLanguageChange,
  initialize
} = useLanguageManager(code, clearOutput, toast)

const {
  showAbout,
  showSettings,
  showUpdate,
  activeTab,
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

// 处理设置变更
const handleSettingsChanged = (config: any) => {
  console.log('主组件接收到设置变更:', config)
  // 延迟一点点再刷新，减少闪烁
  setTimeout(() => {
    editorConfigKey.value++
  }, 50)
}

// 监听编辑器配置变化
watch(editorConfig, (newConfig) => {
  if (newConfig) {
    console.log('编辑器配置更新，刷新编辑器组件')
    setTimeout(() => {
      editorConfigKey.value++
    }, 50)
  }
}, { deep: true })

const { initializeEventListeners, cleanupEventListeners } = useEventManager({
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

  // 触发 app-ready 事件，通知主进程
  window.dispatchEvent(new CustomEvent('app-ready'))
})

onUnmounted(() => {
  cleanupEventListeners()
})
</script>
