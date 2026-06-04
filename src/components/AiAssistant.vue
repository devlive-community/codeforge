<template>
  <div class="fixed top-0 right-0 bottom-0 w-[400px] max-w-[90vw] bg-white border-l border-gray-200 shadow-xl z-40 flex flex-col">
    <!-- 头部 -->
    <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 flex-shrink-0">
      <div class="flex items-center space-x-2 min-w-0">
        <Sparkles class="w-4 h-4 text-blue-500 flex-shrink-0"/>
        <span class="text-sm font-medium text-gray-700">AI 助手</span>
        <span class="text-xs text-gray-400 truncate">{{ active.model }}</span>
      </div>
      <div class="flex items-center space-x-1 flex-shrink-0">
        <button v-if="messages.length" class="p-1 rounded text-gray-400 hover:text-red-500 hover:bg-gray-100" title="清空对话" @click="clearChat">
          <Trash2 class="w-4 h-4"/>
        </button>
        <button class="p-1 rounded text-gray-400 hover:text-gray-700 hover:bg-gray-100" title="关闭" @click="emit('close')">
          <X class="w-4 h-4"/>
        </button>
      </div>
    </div>

    <!-- 关联状态 -->
    <div class="px-4 py-1 text-xs border-b flex-shrink-0"
         :class="executionId != null ? 'text-gray-500 bg-gray-50 border-gray-200' : 'text-amber-600 bg-amber-50 border-amber-100'">
      {{ executionId != null ? `已关联运行 #${executionId}，对话随该次运行保存` : '临时会话：运行代码后对话才会保存' }}
    </div>

    <!-- 快捷动作 -->
    <div class="flex items-center space-x-2 px-3 py-2 border-b border-gray-100 flex-shrink-0">
      <button class="text-xs px-2 py-1 rounded bg-gray-100 hover:bg-gray-200 text-gray-600 cursor-pointer" @click="quick('解释下面的代码')">解释代码</button>
      <button class="text-xs px-2 py-1 rounded bg-gray-100 hover:bg-gray-200 text-gray-600 cursor-pointer" @click="quick('找出下面代码中的 bug 并给出修复')">找 Bug</button>
      <button class="text-xs px-2 py-1 rounded bg-gray-100 hover:bg-gray-200 text-gray-600 cursor-pointer" @click="quick('优化下面的代码并说明原因')">优化</button>
    </div>

    <!-- 消息列表 -->
    <div ref="listRef" class="flex-1 overflow-y-auto px-3 py-3 space-y-3" @click="onCodeAction">
      <div v-if="messages.length === 0" class="text-center text-sm text-gray-400 mt-10">
        向 AI 提问，或用上方快捷动作处理当前代码
      </div>
      <div v-for="(m, i) in messages" :key="i" class="flex" :class="m.role === 'user' ? 'justify-end' : 'justify-start'">
        <div v-if="m.role === 'user'" class="ai-markdown ai-user max-w-[90%] rounded-lg px-3 py-2 text-sm break-words bg-blue-500 text-white"
             v-html="renderMd(m.content)"></div>
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
import {nextTick, onMounted, onUnmounted, ref, watch} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {listen, type UnlistenFn} from '@tauri-apps/api/event'
import {Sparkles, Trash2, X} from 'lucide-vue-next'
import MarkdownIt from 'markdown-it'
import {useAiConfig} from '../composables/useAiConfig'
import {useAiHistory, type AiMsg} from '../composables/useAiHistory'
import {useToast} from '../plugins/toast'

// html:false 不解析原始 HTML，规避 XSS
const md = new MarkdownIt({html: false, linkify: true, breaks: true})

// 给代码块包一层工具条（复制 / 应用到编辑器）
const defaultFence = md.renderer.rules.fence!.bind(md.renderer.rules)
md.renderer.rules.fence = (tokens, idx, options, env, self) => {
  const rendered = defaultFence(tokens, idx, options, env, self)
  return `<div class="ai-code"><div class="ai-code-bar">`
      + `<button class="ai-code-btn" data-act="copy">复制</button>`
      + `<button class="ai-code-btn" data-act="insert">应用到编辑器</button>`
      + `</div>${rendered}</div>`
}

const renderMd = (text: string) => md.render(text || '')

// 代码块按钮（事件委托）
const onCodeAction = (e: MouseEvent) => {
  const btn = (e.target as HTMLElement).closest('.ai-code-btn') as HTMLElement | null
  if (!btn) {
    return
  }
  const pre = btn.closest('.ai-code')?.querySelector('pre')
  const codeText = pre?.textContent ?? ''
  if (!codeText) {
    return
  }
  if (btn.dataset.act === 'copy') {
    navigator.clipboard.writeText(codeText)
    toast.success('已复制代码')
  }
  else {
    emit('insert-code', codeText)
    toast.success('已应用到编辑器')
  }
}

const props = defineProps<{
  code: string
  language: string
  executionId: number | null
}>()

const emit = defineEmits<{ close: []; 'insert-code': [code: string] }>()

const toast = useToast()
const {active, reload} = useAiConfig()
const {saveConversation, getMessages, deleteConversation} = useAiHistory()

const messages = ref<AiMsg[]>([])
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

const waiting = () => sending.value && (streamingIndex.value < 0 || !messages.value[streamingIndex.value]?.content)

// 载入当前执行对应的对话（无关联则清空）
const loadForExecution = async () => {
  if (props.executionId != null) {
    messages.value = await getMessages(props.executionId)
  }
  else {
    messages.value = []
  }
  scrollToBottom()
}

// 仅在已关联执行时保存
const persist = async () => {
  if (props.executionId == null || messages.value.length === 0) {
    return
  }
  try {
    await saveConversation(props.executionId, messages.value.map(m => ({...m})))
  }
  catch (error) {
    console.error('保存 AI 对话失败:', error)
  }
}

const clearChat = async () => {
  messages.value = []
  if (props.executionId != null) {
    await deleteConversation(props.executionId)
  }
}

onMounted(async () => {
  await loadForExecution()
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

// 切换到不同的执行 → 切换对应对话
watch(() => props.executionId, loadForExecution)

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

  const payloadMessages = messages.value.map(m => ({role: m.role, content: m.content}))

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
    persist()
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

/* 用户消息（蓝底）变体 */
.ai-user code { background: rgba(255, 255, 255, 0.22); }
.ai-user a { color: #fff; }

/* 代码块工具条 */
.ai-code { margin: 0.4rem 0; }
.ai-code .ai-code-bar {
  display: flex;
  justify-content: flex-end;
  gap: 0.25rem;
  padding: 0.2rem 0.3rem;
  background: #0f172a;
  border-top-left-radius: 0.375rem;
  border-top-right-radius: 0.375rem;
}
.ai-code .ai-code-btn {
  font-size: 11px;
  color: #cbd5e1;
  padding: 0.05rem 0.4rem;
  border-radius: 0.25rem;
  cursor: pointer;
}
.ai-code .ai-code-btn:hover { background: rgba(255, 255, 255, 0.12); color: #fff; }
.ai-code pre { margin-top: 0; border-top-left-radius: 0; border-top-right-radius: 0; }
</style>
