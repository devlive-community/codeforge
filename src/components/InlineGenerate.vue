<template>
  <div class="absolute left-1/2 -translate-x-1/2 top-3 w-[92%] max-w-[640px] z-20 bg-white rounded-lg shadow-2xl border border-gray-200 overflow-hidden flex flex-col max-h-[80%]">
    <!-- 需求输入 -->
    <div class="flex items-center px-3 border-b border-gray-200 flex-shrink-0">
      <Sparkles class="w-4 h-4 text-blue-500 flex-shrink-0"/>
      <span v-if="isEdit" class="ml-1.5 text-[11px] px-1.5 py-0.5 rounded bg-blue-100 text-blue-600 flex-shrink-0">改写选中</span>
      <input ref="inputRef"
             v-model="prompt"
             class="flex-1 px-2 py-2.5 text-sm focus:outline-none"
             :placeholder="isEdit ? '描述如何修改选中的代码…' : `用自然语言描述要生成的${language}代码…`"
             :disabled="loading"
             @keydown.enter.prevent="generate"
             @keydown.esc.prevent="emit('close')"/>
      <button class="text-xs px-3 py-1 rounded bg-blue-500 text-white hover:bg-blue-600 disabled:opacity-50 cursor-pointer flex-shrink-0"
              :disabled="loading || !prompt.trim()"
              @click="generate">
        {{ result === null ? '生成' : '重新生成' }}
      </button>
    </div>

    <div v-if="loading" class="px-3 py-3 text-xs text-gray-400 flex-shrink-0">生成中…</div>

    <!-- 生成结果：可编辑，确认后才插入 -->
    <template v-else-if="result !== null">
      <div class="px-3 py-1 text-[11px] text-gray-400 border-b border-gray-100 flex-shrink-0">
        生成结果（可编辑后插入）
      </div>
      <textarea v-model="result"
                class="flex-1 min-h-[120px] w-full font-mono text-xs leading-relaxed px-3 py-2 resize-none focus:outline-none"
                spellcheck="false"
                @keydown.esc.prevent="emit('close')"></textarea>
      <div class="flex items-center justify-end gap-2 px-3 py-2 border-t border-gray-200 flex-shrink-0">
        <button class="text-xs px-3 py-1 rounded text-gray-600 hover:bg-gray-100 cursor-pointer" @click="emit('close')">取消</button>
        <button class="text-xs px-3 py-1 rounded bg-blue-500 text-white hover:bg-blue-600 cursor-pointer" @click="confirm">插入到编辑器</button>
      </div>
    </template>

    <div v-else class="px-3 py-1.5 text-[11px] text-gray-400 flex-shrink-0">
      Enter 生成结果，确认后再插入到光标处；Esc 取消
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, onMounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {Sparkles} from 'lucide-vue-next'
import {useAiConfig} from '../composables/useAiConfig'
import {useToast} from '../plugins/toast'

const props = defineProps<{ language: string; selection?: string }>()
const emit = defineEmits<{ insert: [code: string]; close: [] }>()

const toast = useToast()
const {active, reload} = useAiConfig()

const isEdit = computed(() => !!props.selection?.trim())
const prompt = ref('')
const loading = ref(false)
const result = ref<string | null>(null)
const inputRef = ref<HTMLInputElement | null>(null)

onMounted(() => inputRef.value?.focus())

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

  const system = isEdit.value
      ? `你是代码助手。根据要求修改给定的 ${props.language} 代码。只输出修改后的完整代码，不要任何解释，不要使用 Markdown 代码块标记。`
      : `你是代码生成助手。根据需求生成 ${props.language} 代码。只输出代码本身，不要任何解释，不要使用 Markdown 代码块标记。`
  const userContent = isEdit.value
      ? `修改要求：${content}\n\n原代码：\n${props.selection}`
      : content

  loading.value = true
  try {
    const reply = await invoke<string>('ai_chat', {
      provider: active.value.provider,
      baseUrl: active.value.baseUrl,
      apiKey: active.value.apiKey,
      model: active.value.model,
      system,
      messages: [{role: 'user', content: userContent}]
    })
    result.value = stripFences(reply)
  }
  catch (error) {
    toast.error('生成失败: ' + error)
  }
  finally {
    loading.value = false
  }
}

const confirm = () => {
  if (result.value && result.value.trim()) {
    emit('insert', result.value)
    emit('close')
  }
}
</script>
