<template>
  <div class="h-screen flex flex-col bg-gray-50">
    <AppHeader :is-running="isRunning"
               :env-installed="envInfo.installed"
               :supported-languages="supportedLanguages"
               :current-language="currentLanguage"
               @run-code="runCode"
               @stop-code="stopCode"
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
import { onMounted, onUnmounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, UnlistenFn } from '@tauri-apps/api/event'
import AppHeader from './components/AppHeader.vue'
import CodeEditor from './components/CodeEditor.vue'
import OutputPanel from './components/OutputPanel.vue'
import StatusBar from './components/StatusBar.vue'
import About from './components/About.vue'
import Settings from './components/Settings.vue'
import Toast from './components/Toast.vue'
import { useToast } from './plugins/toast'

interface ExecutionResult
{
  success: boolean
  stdout: string
  stderr: string
  execution_time: number
  timestamp: number
  language: string
}

interface LanguageInfo
{
  installed: boolean
  version: string
  path: string
  language: string
}

interface EnvInfo
{
  installed: boolean
  version: string
  path: string
  language: string
}

interface Language
{
  name: string
  value: string
}

interface CodeOutputEvent
{
  type: 'stdout' | 'stderr'
  content: string
  language: string
}

// 代码模板
const codeTemplates: Record<string, string> = {
  python: `# Welcome to CodeForge!
# Write your Python code here and click Run to execute

print("Hello, CodeForge!")

# Example: Simple calculation
x = 10
y = 20
result = x + y
print(f"The result of {x} + {y} = {result}")

# Example: List operations
numbers = [1, 2, 3, 4, 5]
squared = [n**2 for n in numbers]
print(f"Original: {numbers}")
print(f"Squared: {squared}")`,

  python2: `# Welcome to CodeForge - Python 2!
# Write your Python 2 code here and click Run to execute

print "Hello, CodeForge from Python 2!"

# Example: Simple calculation
x = 10
y = 20
result = x + y
print "The result of %d + %d = %d" % (x, y, result)

# Example: List operations
numbers = [1, 2, 3, 4, 5]
squared = [n**2 for n in numbers]
print "Original:", numbers
print "Squared:", squared`,

  python3: `# Welcome to CodeForge - Python 3!
# Write your Python 3 code here and click Run to execute

print("Hello, CodeForge from Python 3!")

# Example: Simple calculation
x = 10
y = 20
result = x + y
print(f"The result of {x} + {y} = {result}")

# Example: List operations
numbers = [1, 2, 3, 4, 5]
squared = [n**2 for n in numbers]
print(f"Original: {numbers}")
print(f"Squared: {squared}")`
}

const toast = useToast()
const code = ref('')
const currentLanguage = ref('python2')
const output = ref('')
const isRunning = ref(false)
const isSuccess = ref(false)
const lastExecutionTime = ref(0)
const activeTab = ref('output')
const supportedLanguages = ref<Language[]>([])
const showAbout = ref(false)
const showSettings = ref(false)

// 实时输出相关
const realTimeOutput = ref('')
const realTimeStderr = ref('')

// 事件监听器
let unlistenAboutFn: UnlistenFn | null = null
let unlistenSettingsFn: UnlistenFn | null = null
let unlistenOutputFn: UnlistenFn | null = null
let unlistenExecutionStartFn: UnlistenFn | null = null
let unlistenExecutionCompleteFn: UnlistenFn | null = null
let unlistenExecutionStoppedFn: UnlistenFn | null = null
let unlistenExecutionTimeoutFn: UnlistenFn | null = null
let unlistenExecutionErrorFn: UnlistenFn | null = null

const closeAbout = () => {
  showAbout.value = false
}

const closeSettings = () => {
  showSettings.value = false
}

const envInfo = ref<EnvInfo>({
  installed: false,
  version: '检查中...',
  path: '检查中...',
  language: 'python'
})

const getLanguageDisplayName = (languageValue: string) => {
  const language = supportedLanguages.value.find(lang => lang.value === languageValue)
  return language ? language.name : languageValue
}

const refreshEnvInfo = async () => {
  try {
    const info: LanguageInfo = await invoke('get_info', {
      language: currentLanguage.value
    })

    envInfo.value = {
      installed: info.installed,
      version: info.version,
      path: info.path,
      language: info.language
    }
  }
  catch (error) {
    console.error('Error checking Env installation:', error)
    envInfo.value = {
      installed: false,
      version: 'Error',
      path: 'Error',
      language: currentLanguage.value
    }
  }
}

const getSupportedLanguages = async () => {
  try {
    const languages = await invoke<Language[]>('get_supported_languages')
    supportedLanguages.value = languages.map((language) => ({
      name: language.name,
      value: language.value
    }))

    // 设置默认语言
    if (supportedLanguages.value.length > 0 && !currentLanguage.value) {
      currentLanguage.value = supportedLanguages.value[0].value
    }
  }
  catch (error) {
    console.error('Error getting supported languages:', error)
    supportedLanguages.value = []
  }
}

const handleLanguageChange = async (newLanguage: string) => {
  currentLanguage.value = newLanguage

  // 更新代码模板
  code.value = codeTemplates[newLanguage] || `# ${ getLanguageDisplayName(newLanguage) } Code
# Write your code here...

print("Hello from ${ getLanguageDisplayName(newLanguage) }!")`

  // 清空输出
  clearOutput()

  // 刷新环境信息
  await refreshEnvInfo()

  toast.info(`已切换到 ${ getLanguageDisplayName(newLanguage) }`)
}

const runCode = async () => {
  if (!envInfo.value.installed) {
    toast.error(`${ envInfo.value.language } 环境未安装`)
    return
  }

  isRunning.value = true

  // 清空所有输出
  output.value = ''
  realTimeOutput.value = ''
  realTimeStderr.value = ''
  isSuccess.value = false
  lastExecutionTime.value = 0

  try {
    const result: ExecutionResult = await invoke('execute_code', {
      request: {
        code: code.value,
        language: currentLanguage.value
      }
    })

    // 注意：这里不需要手动设置 output，因为实时输出已经通过事件处理了
    lastExecutionTime.value = result.execution_time
    isSuccess.value = result.success

    if (result.success) {
      toast.success(`代码执行成功，用时 ${ result.execution_time } 毫秒`)
    }
    else {
      toast.error('代码执行失败，查看输出的错误信息')
    }
  }
  catch (error) {
    output.value = `代码执行失败: ${ error }`
    toast.error('代码执行失败，请检查日志')
    isRunning.value = false
  }
}

const stopCode = async () => {
  if (!isRunning.value) {
    return
  }

  try {
    const result = await invoke<boolean>('stop_execution', {
      language: currentLanguage.value
    })

    if (result) {
      toast.info('正在停止代码执行...')
    }
    else {
      toast.warning('没有找到正在运行的任务')
    }
  }
  catch (error) {
    console.error('Error stopping execution:', error)
    toast.error('停止执行失败')
  }
}

const clearOutput = () => {
  output.value = ''
  realTimeOutput.value = ''
  realTimeStderr.value = ''
  toast.info('输出已清空')
}

// 处理实时输出
const handleRealtimeOutput = (event: any) => {
  const data: CodeOutputEvent = event.payload

  // 只处理当前语言的输出
  if (data.language !== currentLanguage.value) {
    return
  }

  if (data.type === 'stdout') {
    realTimeOutput.value += data.content + '\n'
  }
  else if (data.type === 'stderr') {
    realTimeStderr.value += data.content + '\n'
  }

  // 合并输出显示
  let combinedOutput = ''
  if (realTimeOutput.value) {
    combinedOutput += realTimeOutput.value
  }
  if (realTimeStderr.value) {
    if (combinedOutput) {
      combinedOutput += '\n'
    }
    combinedOutput += realTimeStderr.value
  }

  output.value = combinedOutput
}

// 处理执行状态事件
const handleExecutionStart = (event: any) => {
  const data = event.payload
  if (data.language === currentLanguage.value) {
    console.log('代码开始执行')
  }
}

const handleExecutionComplete = (event: any) => {
  const data = event.payload
  if (data.language === currentLanguage.value) {
    isRunning.value = false
    isSuccess.value = data.success
    if (data.execution_time) {
      lastExecutionTime.value = data.execution_time
    }
    console.log('代码执行完成')
  }
}

const handleExecutionStopped = (event: any) => {
  const data = event.payload
  if (data.language === currentLanguage.value) {
    isRunning.value = false
    output.value += '\n\n🛑 代码执行已被用户停止'
    toast.warning('代码执行已停止')
    console.log('代码执行已停止')
  }
}

const handleExecutionTimeout = (event: any) => {
  const data = event.payload
  if (data.language === currentLanguage.value) {
    isRunning.value = false
    output.value += '\n\n⚠️ 代码执行超时（30秒）'
    toast.error('代码执行超时')
  }
}

const handleExecutionError = (event: any) => {
  const data = event.payload
  if (data.language === currentLanguage.value) {
    isRunning.value = false
    output.value += `\n\n❌ 执行错误: ${ data.error }`
    toast.error('代码执行出错')
  }
}

// 禁用右键菜单
window.addEventListener('contextmenu', (e) => e.preventDefault(), false)

onMounted(async () => {
  await getSupportedLanguages()
  await refreshEnvInfo()

  // 设置初始代码模板
  if (supportedLanguages.value.length > 0) {
    currentLanguage.value = supportedLanguages.value[0].value
    code.value = codeTemplates[currentLanguage.value] || codeTemplates.python
  }
  else {
    code.value = codeTemplates.python
  }

  // 监听来自 Rust 的各种事件
  unlistenAboutFn = await listen('show-about', () => {
    showAbout.value = true
  })

  unlistenSettingsFn = await listen('show-settings', () => {
    showSettings.value = true
  })

  // 监听实时输出事件
  unlistenOutputFn = await listen('code-output', handleRealtimeOutput)

  // 监听执行状态事件
  unlistenExecutionStartFn = await listen('code-execution-start', handleExecutionStart)
  unlistenExecutionCompleteFn = await listen('code-execution-complete', handleExecutionComplete)
  unlistenExecutionStoppedFn = await listen('code-execution-stopped', handleExecutionStopped)
  unlistenExecutionTimeoutFn = await listen('code-execution-timeout', handleExecutionTimeout)
  unlistenExecutionErrorFn = await listen('code-execution-error', handleExecutionError)

  // 触发 app-ready 事件，通知主进程
  window.dispatchEvent(new CustomEvent('app-ready'))
})

onUnmounted(() => {
  // 清理所有事件监听器
  const listeners = [
    unlistenAboutFn,
    unlistenSettingsFn,
    unlistenOutputFn,
    unlistenExecutionStartFn,
    unlistenExecutionCompleteFn,
    unlistenExecutionStoppedFn,
    unlistenExecutionTimeoutFn,
    unlistenExecutionErrorFn
  ]

  listeners.forEach(listener => {
    if (listener) {
      listener()
    }
  })
})
</script>
