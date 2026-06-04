<template>
  <div class="fixed inset-0 z-50 flex justify-center pt-24" @click="emit('close')">
    <div class="w-[560px] max-w-[90vw] bg-white rounded-lg shadow-2xl border border-gray-200 overflow-hidden"
         @click.stop>
      <div class="flex items-center px-3 border-b border-gray-200">
        <Sparkles class="w-4 h-4 text-blue-500 flex-shrink-0"/>
        <input ref="inputRef"
               v-model="prompt"
               class="flex-1 px-2 py-2.5 text-sm focus:outline-none"
               :placeholder="`用自然语言描述要生成的${language}代码…`"
               :disabled="loading"
               @keydown.enter.prevent="generate"
               @keydown.esc.prevent="emit('close')"/>
      </div>
      <div class="px-3 py-2 flex items-center justify-between">
        <span class="text-xs text-gray-400">
          {{ loading ? '生成中…' : 'Enter 生成，Esc 取消；生成内容将插入光标处' }}
        </span>
        <button class="text-xs px-3 py-1 rounded bg-blue-500 text-white hover:bg-blue-600 disabled:opacity-50 cursor-pointer"
                :disabled="loading || !prompt.trim()"
                @click="generate">
          生成
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {Sparkles} from 'lucide-vue-next'
import {useAiConfig} from '../composables/useAiConfig'
import {useToast} from '../plugins/toast'

const props = defineProps<{ language: string }>()
const emit = defineEmits<{ insert: [code: string]; close: [] }>()

const toast = useToast()
const {active, reload} = useAiConfig()

const prompt = ref('')
const loading = ref(false)
const inputRef = ref<HTMLInputElement | null>(null)

onMounted(() => inputRef.value?.focus())

// 去掉可能的 Markdown 代码围栏
const stripFences = (text: string) => {
  const trimmed = text.trim()
  const m = trimmed.match(/^```[\w+-]*\n([\s\S]*?)\n?```$/)
  return m ? m[1] : trimmed
}

const generate = async () => {
  const content = prompt.value.trim()
  if (!content || loading.value) {
    return
  }

  reload()
  if (!active.value.apiKey) {
    toast.error('请先在 设置 → AI 中填写 API Key')
    return
  }

  loading.value = true
  try {
    const reply = await invoke<string>('ai_chat', {
      provider: active.value.provider,
      baseUrl: active.value.baseUrl,
      apiKey: active.value.apiKey,
      model: active.value.model,
      system: `你是代码生成助手。根据需求生成 ${props.language} 代码。只输出代码本身，不要任何解释，不要使用 Markdown 代码块标记。`,
      messages: [{role: 'user', content}]
    })
    const codeText = stripFences(reply)
    if (codeText) {
      emit('insert', codeText)
      emit('close')
    }
    else {
      toast.error('未生成内容')
    }
  }
  catch (error) {
    toast.error('生成失败: ' + error)
  }
  finally {
    loading.value = false
  }
}
</script>
