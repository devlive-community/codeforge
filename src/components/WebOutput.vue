<template>
  <div class="flex flex-col h-full bg-white">
    <!-- 工具栏 -->
    <div class="bg-gray-100 px-4 py-2 border-b border-gray-200 flex items-center justify-between flex-shrink-0">
      <div class="flex items-center space-x-3">
        <Globe class="w-4 h-4 text-blue-600"/>
        <h3 class="text-sm font-medium text-gray-700">Web 预览</h3>
        <div v-if="isRunning" class="flex items-center space-x-1 text-yellow-600">
          <Loader class="w-3 h-3 animate-spin"/>
          <span class="text-xs">渲染中</span>
        </div>
      </div>

      <div class="flex items-center space-x-2">
        <div v-if="executionTime > 0" class="text-xs text-gray-500 flex items-center space-x-1">
          <Clock class="w-3 h-3"/>
          <span>{{ executionTime }} 毫秒</span>
        </div>
      </div>
    </div>

    <!-- 内容区域 -->
    <div class="flex-1 overflow-auto relative">
      <!-- 加载状态 -->
      <div v-if="isRunning && !webContent" class="absolute inset-0 flex items-center justify-center bg-gray-50">
        <div class="text-center">
          <Loader class="w-8 h-8 animate-spin mx-auto mb-3 text-blue-600"/>
          <p class="text-gray-600 text-sm">正在渲染页面...</p>
        </div>
      </div>

      <!-- Web内容iframe -->
      <div v-else-if="webContent" class="h-full">
        <iframe ref="webFrame"
                :srcdoc="processedContent"
                class="w-full h-full border-0"
                sandbox="allow-scripts allow-forms allow-modals allow-popups allow-same-origin"
                @load="onFrameLoad">
        </iframe>
      </div>

      <!-- 空状态 -->
      <div v-else class="absolute inset-0 flex flex-col items-center justify-center text-gray-400">
        <Globe class="w-16 h-16 mb-4"/>
        <h3 class="text-lg font-medium mb-2">没有Web内容</h3>
        <p class="text-sm text-center max-w-sm">
          运行 HTML、CSS 或 JavaScript 代码来查看网页效果
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {ref, watch} from 'vue'
import {Clock, Globe, Loader} from 'lucide-vue-next'

const props = defineProps<{
  webContent?: string
  isRunning: boolean
  executionTime: number
}>()

const webFrame = ref<HTMLIFrameElement>()
const processedContent = ref('')

// 处理本地文件：读取内容并内联
const processWebContent = async (content: string) => {
  if (!content) {
    processedContent.value = ''
    return
  }

  let result = content

  try {
    const {readTextFile} = await import('@tauri-apps/plugin-fs')

    // 处理 script 标签
    const scriptMatches = content.match(/<script[^>]*src="(file:\/\/[^"]*)"[^>]*><\/script>/gi)
    if (scriptMatches) {
      for (const match of scriptMatches) {
        const pathMatch = match.match(/src="(file:\/\/[^"]*)"/)
        if (pathMatch) {
          const filePath = pathMatch[1].replace('file://', '')
          try {
            const fileContent = await readTextFile(filePath)
            const inlineScript = "<script>" + fileContent + "<" + "/script>"
            result = result.replace(match, inlineScript)
          }
          catch (error) {
            console.error("读取文件失败:", error)
            result = result.replace(match, "<!-- 无法读取文件: " + filePath + " -->")
          }
        }
      }
    }

    // 处理 style 标签
    const styleMatches = content.match(/<style[^>]*src="(file:\/\/[^"]*)"[^>]*><\/style>/gi)
    if (styleMatches) {
      for (const match of styleMatches) {
        const pathMatch = match.match(/src="(file:\/\/[^"]*)"/)
        if (pathMatch) {
          const filePath = pathMatch[1].replace('file://', '')
          try {
            const fileContent = await readTextFile(filePath)
            const inlineStyle = "<style>" + fileContent + "</style>"
            result = result.replace(match, inlineStyle)
          }
          catch (error) {
            console.error("读取CSS文件失败:", error)
            result = result.replace(match, "<!-- 无法读取CSS文件: " + filePath + " -->")
          }
        }
      }
    }
  }
  catch (error) {
    console.error('无法导入 Tauri 文件系统插件:', error)
  }

  processedContent.value = result
}

// iframe加载完成事件
const onFrameLoad = () => {
  if (webFrame.value?.contentDocument) {
    const style = webFrame.value.contentDocument.createElement('style')
    style.textContent = `
      body {
        margin: 0 !important;
      }

      /* Webkit 滚动条美化 */
      ::-webkit-scrollbar {
        width: 8px;
        height: 8px;
      }

      ::-webkit-scrollbar-track {
        background: #f1f1f1;
        border-radius: 4px;
      }

      ::-webkit-scrollbar-thumb {
        background: #888;
        border-radius: 4px;
      }

      ::-webkit-scrollbar-thumb:hover {
        background: #555;
      }

      /* Firefox 滚动条美化 */
      * {
        scrollbar-width: thin;
        scrollbar-color: #888 #f1f1f1;
      }
    `
    webFrame.value.contentDocument.head.appendChild(style)
  }
}

// 监听webContent变化
watch(() => props.webContent, (newContent) => {
  if (newContent) {
    processWebContent(newContent)
  }
}, {immediate: true})
</script>
