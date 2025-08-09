<template>
  <div class="h-screen flex flex-col bg-gray-50">
    <AppHeader :is-running="isRunning"
               :env-installed="envInfo.installed"
               :supported-languages="supportedLanguages"
               :current-language="currentLanguage"
               @run-code="runCode"
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

    <!-- 通知信息 -->
    <Toast v-if="toast.show"
           :show="toast.show"
           :message="toast.message"
           :type="toast.type"
           :duration="3000"
           :show-progress="true"
           @close="toast.show = false">
    </Toast>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import AppHeader from './components/AppHeader.vue'
import CodeEditor from './components/CodeEditor.vue'
import OutputPanel from './components/OutputPanel.vue'
import StatusBar from './components/StatusBar.vue'
import Toast from './components/Toast.vue'

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

const code = ref('')
const currentLanguage = ref('python')
const output = ref('')
const isRunning = ref(false)
const isSuccess = ref(false)
const lastExecutionTime = ref(0)
const activeTab = ref('output')
const showSettings = ref(false)
const supportedLanguages = ref<Language[]>([])

const envInfo = ref<EnvInfo>({
  installed: false,
  version: '检查中...',
  path: '检查中...',
  language: 'python'
})

const toast = ref({
  show: false,
  message: '',
  type: 'success' as 'success' | 'error' | 'info'
})

const showToast = (message: string, type: 'success' | 'error' | 'info' = 'success') => {
  toast.value = { show: true, message, type }
}

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
  output.value = ''

  // 刷新环境信息
  await refreshEnvInfo()

  showToast(`已切换到 ${ getLanguageDisplayName(newLanguage) }`, 'info')
}

const runCode = async () => {
  if (!envInfo.value.installed) {
    showToast(`${ envInfo.value.language } 环境未安装`, 'error')
    return
  }

  isRunning.value = true
  output.value = ''

  try {
    const result: ExecutionResult = await invoke('execute_code', {
      request: {
        code: code.value,
        language: currentLanguage.value
      }
    })

    lastExecutionTime.value = result.execution_time
    isSuccess.value = result.success

    if (result.success) {
      output.value = result.stdout || '代码执行成功 (无输出)'
      if (result.stderr) {
        output.value += '\n' + result.stderr
      }
      showToast(`代码执行成功，用时 ${ result.execution_time } 毫秒`)
    }
    else {
      output.value = result.stderr || '代码执行失败 (无输出)'
      showToast('代码执行失败，查看输出的错误信息', 'error')
    }
  }
  catch (error) {
    output.value = `代码执行失败: ${ error }`
    showToast('代码执行失败，请检查日志', 'error')
  }
  finally {
    isRunning.value = false
  }
}

const clearOutput = () => {
  output.value = ''
  showToast('输出已清空', 'info')
}

// window.addEventListener("contextmenu", (e) => e.preventDefault(), false);

onMounted(async () => {
  await getSupportedLanguages()

  // 设置初始代码模板
  if (supportedLanguages.value.length > 0) {
    currentLanguage.value = supportedLanguages.value[0].value
    code.value = codeTemplates[currentLanguage.value] || codeTemplates.python
  }
  else {
    code.value = codeTemplates.python
  }

  await refreshEnvInfo()
})
</script>