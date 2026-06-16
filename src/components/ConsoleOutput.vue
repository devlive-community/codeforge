<template>
  <div class="flex flex-col h-full bg-gray-900 text-green-400">
    <div class="p-2 border-b border-gray-700 flex items-center justify-between">
      <div class="flex items-center space-x-2">
        <Terminal class="w-4 h-4"/>
        <span class="text-sm font-medium">{{ t('console.title') }}</span>
        <div v-if="isRunning" class="flex items-center space-x-1 text-yellow-400">
          <Loader class="w-3 h-3 animate-spin"/>
          <span class="text-xs">{{ t('console.running') }}</span>
        </div>
      </div>
      <div class="flex items-center space-x-3">
        <span v-if="isCopied" class="text-xs text-gray-400">{{ t('console.copied') }}</span>

        <!-- 复制按钮 -->
        <button v-if="output && !isRunning"
                @click="copyOutput"
                class="text-gray-400 hover:text-white transition-colors duration-200 p-1 rounded hover:bg-gray-700 cursor-pointer"
                :title="t('console.copyTitle')">
          <component :is="copyIcon" class="w-3 h-3"/>
        </button>

        <!-- 清空按钮 -->
        <button v-if="output && !isRunning"
                @click="emit('clear')"
                class="text-gray-400 hover:text-white transition-colors duration-200 p-1 rounded hover:bg-gray-700 cursor-pointer"
                :title="t('console.clearTitle')">
          <Trash2 class="w-3 h-3"/>
        </button>

        <div v-if="executionTime > 0" class="text-xs text-gray-400 flex items-center space-x-1">
          <Clock class="w-3 h-3"/>
          <span>{{ t('console.ms', { n: executionTime }) }}</span>
        </div>
      </div>
    </div>

    <div class="flex-1 overflow-auto" ref="outputContainer">
      <div v-if="isRunning && !output" class="p-4 flex items-center space-x-2 text-yellow-400">
        <Loader class="w-4 h-4 animate-spin"/>
        <span>{{ t('console.executing') }}</span>
      </div>

      <div v-else-if="output" class="p-4">
        <pre :class="['whitespace-pre-wrap text-sm leading-relaxed font-mono', getOutputClass()]" v-html="renderedOutput"></pre>

        <!-- 正在运行时显示光标 -->
        <div v-if="isRunning" class="flex items-center mt-2 text-yellow-400">
          <Loader class="w-4 h-4 animate-spin">█</Loader>
          <span class="ml-2 text-xs">{{ t('console.programRunning') }}</span>
        </div>
      </div>

      <div v-else class="p-4 text-gray-500 flex flex-col items-center justify-center h-full space-y-2 select-none">
        <Terminal class="w-8 h-8"/>
        <p class="text-sm">{{ t('console.noOutput') }}</p>
        <p class="text-xs">{{ t('console.noOutputHint') }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { Check, Clock, Copy, Loader, Terminal, Trash2 } from 'lucide-vue-next'
import { ansiToHtml } from '../utils/ansi'

const props = defineProps<{
  output: string
  isRunning: boolean
  isSuccess: boolean
  executionTime: number
}>()

const emit = defineEmits<{
  clear: []
}>()

const {t} = useI18n()
const isCopied = ref(false)
const outputContainer = ref<HTMLElement>()

// 动态切换图标
const copyIcon = computed(() => isCopied.value ? Check : Copy)

// 根据执行状态和成功状态确定输出样式
// 渲染 ANSI 颜色（自带 HTML 转义）
const renderedOutput = computed(() => ansiToHtml(props.output || ''))

const getOutputClass = () => {
  if (props.isRunning) {
    return 'text-yellow-300' // 运行中显示黄色
  }
  else if (props.isSuccess) {
    return 'text-green-300'  // 成功显示绿色
  }
  else {
    return 'text-red-300'    // 失败显示红色
  }
}

const copyOutput = async () => {
  if (!props.output) {
    return
  }

  try {
    await navigator.clipboard.writeText(props.output)
    isCopied.value = true

    // 2秒后恢复复制图标
    setTimeout(() => {
      isCopied.value = false
    }, 2000)
  }
  catch (error) {
    console.error('复制失败:', error)

    // 降级方案：使用传统方法复制
    try {
      const textArea = document.createElement('textarea')
      textArea.value = props.output
      document.body.appendChild(textArea)
      textArea.select()
      document.execCommand('copy')
      document.body.removeChild(textArea)

      isCopied.value = true
      setTimeout(() => {
        isCopied.value = false
      }, 2000)
    }
    catch (fallbackError) {
      console.error('降级复制也失败了:', fallbackError)
    }
  }
}

// 自动滚动到底部（实时输出时）
watch(() => props.output, async (newOutput, oldOutput) => {
  // 只有在输出增加时才滚动（避免清空时滚动）
  if (newOutput.length > (oldOutput?.length || 0)) {
    await nextTick()
    if (outputContainer.value) {
      outputContainer.value.scrollTop = outputContainer.value.scrollHeight
    }
  }
}, { flush: 'post' })

// 监听运行状态变化，运行开始时也滚动到底部
watch(() => props.isRunning, async (isRunning) => {
  if (isRunning) {
    await nextTick()
    if (outputContainer.value) {
      outputContainer.value.scrollTop = outputContainer.value.scrollHeight
    }
  }
})
</script>
