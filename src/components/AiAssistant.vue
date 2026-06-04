<template>
  <div class="fixed top-0 right-0 bottom-0 w-[400px] max-w-[90vw] bg-white border-l border-gray-200 shadow-xl z-40 flex flex-col">
    <!-- 头部 -->
    <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 flex-shrink-0">
      <div class="flex items-center space-x-2">
        <Sparkles class="w-4 h-4 text-blue-500"/>
        <span class="text-sm font-medium text-gray-700">AI 助手</span>
        <span class="text-xs text-gray-400">{{ active.model }}</span>
      </div>
      <button class="text-gray-400 hover:text-gray-700" title="关闭" @click="emit('close')">
        <X class="w-4 h-4"/>
      </button>
    </div>

    <!-- 快捷动作 -->
    <div class="flex items-center space-x-2 px-3 py-2 border-b border-gray-100 flex-shrink-0">
      <button class="text-xs px-2 py-1 rounded bg-gray-100 hover:bg-gray-200 text-gray-600 cursor-pointer" @click="quick('解释下面的代码')">解释代码</button>
      <button class="text-xs px-2 py-1 rounded bg-gray-100 hover:bg-gray-200 text-gray-600 cursor-pointer" @click="quick('找出下面代码中的 bug 并给出修复')">找 Bug</button>
      <button class="text-xs px-2 py-1 rounded bg-gray-100 hover:bg-gray-200 text-gray-600 cursor-pointer" @click="quick('优化下面的代码并说明原因')">优化</button>
      <button v-if="messages.length" class="ml-auto text-xs text-gray-400 hover:text-gray-600 cursor-pointer" @click="messages = []">清空</button>
    </div>

    <!-- 消息列表 -->
    <div ref="listRef" class="flex-1 overflow-y-auto px-3 py-3 space-y-3">
      <div v-if="messages.length === 0" class="text-center text-sm text-gray-400 mt-10">
        向 AI 提问，或用上方快捷动作处理当前代码
      </div>
      <div v-for="(m, i) in messages" :key="i" class="flex" :class="m.role === 'user' ? 'justify-end' : 'justify-start'">
        <div v-if="m.role === 'user'" class="max-w-[90%] rounded-lg px-3 py-2 text-sm whitespace-pre-wrap break-words bg-blue-500 text-white">
          {{ m.content }}
        </div>
        <div v-else-if="m.content" class="ai-markdown max-w-[90%] rounded-lg px-3 py-2 text-sm break-words bg-gray-100 text-gray-800"
             v-html="renderMd(m.content)"></div>
      </div>
      <div v-if="waiting()" class="flex justify-start">
        <div class="bg-gray-100 text-gray-500 rounded-lg px-3 py-2 text-sm">思考中…</div>
      </div>
    </div>

    <!-- 输入 -->
    <div class="border-t border-gray-200 p-2 flex-shrink-0">
      <textarea v-model="input"
                rows="2"
                class="w-full text-sm border border-gray-300 rounded px-2 py-1.5 resize-none focus:outline-none focus:border-blue-400"
                placeholder="输入问题，Enter 发送（Shift+Enter 换行）"
                @keydown.enter.exact.prevent="send()"/>
    </div>
  </div>
</template>

<script setup lang="ts">
import {nextTick, onMounted, onUnmounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {listen, type UnlistenFn} from '@tauri-apps/api/event'
import {Sparkles, X} from 'lucide-vue-next'
import MarkdownIt from 'markdown-it'
import {useAiConfig} from '../composables/useAiConfig'
import {useToast} from '../plugins/toast'

// html:false 不解析原始 HTML，规避 XSS
const md = new MarkdownIt({html: false, linkify: true, breaks: true})
const renderMd = (text: string) => md.render(text || '')

interface Msg
{
  role: 'user' | 'assistant'
  content: string
}

const props = defineProps<{
  code: string
  language: string
}>()

const emit = defineEmits<{ close: [] }>()

const toast = useToast()
const {active, reload} = useAiConfig()

const messages = ref<Msg[]>([])
const input = ref('')
const sending = ref(false)
const streamingIndex = ref(-1)
const listRef = ref<HTMLElement | null>(null)

let currentStreamId = ''
let unlistenDelta: UnlistenFn | null = null

const scrollToBottom = () => {
  nextTick(() => {
    if (listRef.value) {
      listRef.value.scrollTop = listRef.value.scrollHeight
    }
  })
}

// 等待中（首段未到达前显示"思考中"）
const waiting = () => sending.value && (streamingIndex.value < 0 || !messages.value[streamingIndex.value]?.content)

onMounted(async () => {
  unlistenDelta = await listen<{ stream_id: string, delta: string }>('ai-stream-delta', (e) => {
    if (e.payload.stream_id === currentStreamId && streamingIndex.value >= 0) {
      messages.value[streamingIndex.value].content += e.payload.delta
      scrollToBottom()
    }
  })
})

onUnmounted(() => {
  unlistenDelta?.()
})

const send = async (text?: string) => {
  const content = (text ?? input.value).trim()
  if (!content || sending.value) {
    return
  }

  reload()
  if (!active.value.apiKey) {
    toast.error('请先在 设置 → AI 中填写 API Key')
    return
  }

  messages.value.push({role: 'user', content})
  input.value = ''

  // 发给后端的对话（不含占位的空助手消息）
  const payloadMessages = messages.value.map(m => ({role: m.role, content: m.content}))

  // 占位的助手消息，流式追加
  messages.value.push({role: 'assistant', content: ''})
  streamingIndex.value = messages.value.length - 1

  const streamId = crypto.randomUUID()
  currentStreamId = streamId
  sending.value = true
  scrollToBottom()

  try {
    await invoke('ai_chat_stream', {
      streamId,
      provider: active.value.provider,
      baseUrl: active.value.baseUrl,
      apiKey: active.value.apiKey,
      model: active.value.model,
      system: `你是嵌入代码编辑器的编程助手。回答简洁、准确，必要时给出可运行的代码。当前编程语言：${props.language}。`,
      messages: payloadMessages
    })
    if (!messages.value[streamingIndex.value].content) {
      messages.value[streamingIndex.value].content = '(空响应)'
    }
  }
  catch (error) {
    messages.value[streamingIndex.value].content += '\n\n❌ ' + error
  }
  finally {
    sending.value = false
    streamingIndex.value = -1
    scrollToBottom()
  }
}

const quick = (instruction: string) => {
  if (!props.code?.trim()) {
    toast.info('当前编辑器没有代码')
    return
  }
  send(`${instruction}：\n\n\`\`\`${props.language}\n${props.code}\n\`\`\``)
}
</script>

<style>
.ai-markdown p {
  margin: 0.25rem 0;
  line-height: 1.5;
}
.ai-markdown p:first-child { margin-top: 0; }
.ai-markdown p:last-child { margin-bottom: 0; }
.ai-markdown pre {
  background: #1e293b;
  color: #e2e8f0;
  padding: 0.6rem 0.75rem;
  border-radius: 0.375rem;
  overflow-x: auto;
  margin: 0.4rem 0;
  font-size: 12px;
  line-height: 1.45;
}
.ai-markdown code {
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  background: rgba(0, 0, 0, 0.06);
  padding: 0.1rem 0.3rem;
  border-radius: 0.25rem;
  font-size: 0.85em;
}
.ai-markdown pre code {
  background: transparent;
  padding: 0;
  font-size: inherit;
}
.ai-markdown ul, .ai-markdown ol {
  padding-left: 1.2rem;
  margin: 0.3rem 0;
}
.ai-markdown li { margin: 0.15rem 0; }
.ai-markdown a { color: #2563eb; text-decoration: underline; }
.ai-markdown h1, .ai-markdown h2, .ai-markdown h3 {
  font-weight: 600;
  margin: 0.5rem 0 0.3rem;
}
.ai-markdown blockquote {
  border-left: 3px solid #cbd5e1;
  padding-left: 0.6rem;
  color: #64748b;
  margin: 0.3rem 0;
}
.ai-markdown table { border-collapse: collapse; margin: 0.4rem 0; }
.ai-markdown th, .ai-markdown td { border: 1px solid #e2e8f0; padding: 0.25rem 0.5rem; }
</style>
