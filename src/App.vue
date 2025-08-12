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
          <CodeEditor v-model="code" class="h-full" :language="currentLanguage"/>
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
    <Settings v-if="showSettings" @close="closeSettings"/>

    <!-- Toast 组件 -->
    <Toast/>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'
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

const toast = useToast()

const {
  code,
  output,
  isRunning,
  isSuccess,
  lastExecutionTime,
  runCode,
  stopCode,
  clearOutput
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
  activeTab,
  closeAbout,
  closeSettings
} = useAppState()

const { initializeEventListeners, cleanupEventListeners } = useEventManager({
  showAbout,
  showSettings,
  output,
  isRunning,
  isSuccess,
  lastExecutionTime,
  currentLanguage,
  toast
})

// 禁用右键菜单
window.addEventListener('contextmenu', (e) => e.preventDefault(), false)

onMounted(async () => {
  await initialize()
  await initializeEventListeners()

  // 触发 app-ready 事件，通知主进程
  window.dispatchEvent(new CustomEvent('app-ready'))
})

onUnmounted(() => {
  cleanupEventListeners()
})
</script>
