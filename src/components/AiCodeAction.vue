<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-16 px-6 pb-6" @click="emit('close')">
    <div class="w-full max-w-[760px] max-h-full bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col"
         @click.stop>
      <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
          <Sparkles class="w-4 h-4 text-purple-500"/>
          <span>{{ t('aiCode.title.' + action) }}</span>
        </div>
        <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('close')">
          <X class="w-4 h-4"/>
        </button>
      </div>

      <div class="flex-1 min-h-[160px] overflow-auto p-4">
        <div v-if="loading" class="flex items-center gap-2 text-sm text-gray-400">
          <Sparkles class="w-4 h-4 animate-pulse"/>{{ t('aiCode.thinking') }}
        </div>
        <pre v-else class="text-xs font-mono whitespace-pre-wrap break-words text-gray-800 dark:text-gray-100">{{ result }}</pre>
      </div>

      <div class="flex items-center justify-end gap-2 px-4 py-2.5 border-t border-gray-200 dark:border-gray-700 flex-shrink-0">
        <button class="text-xs px-3 py-1.5 rounded text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="emit('close')">{{ t('aiCode.close') }}</button>
        <button v-if="!loading && result" class="text-xs px-3 py-1.5 rounded text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="copy">{{ t('aiCode.copy') }}</button>
        <button v-if="!loading && result && (action === 'refactor' || action === 'fix')" class="text-xs px-3 py-1.5 rounded bg-blue-500 text-white hover:bg-blue-600 cursor-pointer" @click="apply('replace')">{{ t('aiCode.replace') }}</button>
        <button v-if="!loading && result && (action === 'test' || action === 'doc')" class="text-xs px-3 py-1.5 rounded bg-blue-500 text-white hover:bg-blue-600 cursor-pointer" @click="apply('insert')">{{ t('aiCode.insert') }}</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {Sparkles, X} from 'lucide-vue-next'
import {useI18n} from 'vue-i18n'
import {useAiConfig} from '../composables/useAiConfig'
import {useToast} from '../plugins/toast'

const props = defineProps<{ language: string; code: string; action: 'explain' | 'refactor' | 'test' | 'fix' | 'doc'; diagnostics?: string }>()
const emit = defineEmits<{ replace: [code: string]; insert: [code: string]; close: [] }>()

const toast = useToast()
const {t} = useI18n()
const {active, reload} = useAiConfig()

const loading = ref(true)
const result = ref('')

const stripFences = (text: string) => {
  const trimmed = text.trim()
  const m = trimmed.match(/^```[\w+-]*\n([\s\S]*?)\n?```$/)
  return m ? m[1] : trimmed
}

const systemFor = (): string => {
  const lang = props.language
  switch (props.action) {
    case 'explain':
      return `你是资深工程师。用简洁中文解释给定的 ${lang} 代码：作用、关键逻辑、潜在问题。可用简短要点。`
    case 'refactor':
      return `你是代码助手。重构给定的 ${lang} 代码以提升可读性与质量，保持行为不变。只输出重构后的完整代码，不要解释，不要使用 Markdown 代码块标记。`
    case 'test':
      return `你是测试工程师。为给定的 ${lang} 代码生成单元测试。只输出测试代码，不要解释，不要使用 Markdown 代码块标记。`
    case 'fix':
      return `你是代码助手。修复给定 ${lang} 代码中的错误与警告（用户消息附带诊断信息），保持其余行为不变。只输出修复后的完整代码，不要解释，不要使用 Markdown 代码块标记。`
    case 'doc':
      return `你是代码助手。为给定的 ${lang} 代码生成规范的文档注释（如 JSDoc/docstring/rustdoc 等，与语言习惯一致）。只输出注释块本身，不要重复原代码，不要解释，不要使用 Markdown 代码块标记。`
  }
}

// 'fix' 把诊断附在用户消息里
const userContent = (): string =>
  props.action === 'fix' && props.diagnostics
    ? `${props.code}\n\n--- 待修复的诊断 ---\n${props.diagnostics}`
    : props.code

const run = async () => {
  reload()
  if (!active.value.apiKey) {
    toast.error(t('chat.needKey'))
    emit('close')
    return
  }
  loading.value = true
  try {
    const reply = await invoke<string>('ai_chat', {
      provider: active.value.provider,
      baseUrl: active.value.baseUrl,
      apiKey: active.value.apiKey,
      model: active.value.model,
      system: systemFor(),
      messages: [{role: 'user', content: userContent()}]
    })
    result.value = props.action === 'explain' ? reply.trim() : stripFences(reply)
  }
  catch (error) {
    toast.error(t('aiCode.failed') + error)
    emit('close')
  }
  finally {
    loading.value = false
  }
}

const apply = (mode: 'replace' | 'insert') => {
  if (!result.value.trim()) {
    return
  }
  if (mode === 'replace') {
    emit('replace', result.value)
  }
  else {
    emit('insert', result.value)
  }
  emit('close')
}

const copy = async () => {
  try {
    await navigator.clipboard.writeText(result.value)
    toast.success(t('aiCode.copied'))
  }
  catch {
    // 忽略
  }
}

onMounted(run)
</script>
