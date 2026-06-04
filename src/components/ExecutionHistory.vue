<template>
  <Modal v-model:show="visible"
         title="执行历史"
         size="5xl"
         :close-on-backdrop="false"
         content-class="h-[66vh] overflow-hidden">
    <div class="h-full grid grid-cols-[280px_1fr] border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden bg-white dark:bg-gray-900">
      <aside class="border-r border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 flex flex-col min-h-0">
        <div class="p-3 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between">
          <div class="text-sm font-medium text-gray-700 dark:text-gray-200">
            共 {{ total }} 条
          </div>
          <Button type="secondary"
                  variant="ghost"
                  size="sm"
                  :icon="RefreshCw"
                  :icon-only="true"
                  title="刷新"
                  :loading="isLoading"
                  @click="reloadHistory"/>
        </div>

        <div v-if="isLoading" class="flex-1 flex items-center justify-center text-sm text-gray-500">
          加载中...
        </div>

        <div v-else-if="history.length === 0" class="flex-1 flex flex-col items-center justify-center text-gray-400 space-y-2">
          <History class="w-10 h-10"/>
          <p class="text-sm">暂无执行历史</p>
        </div>

        <div v-else class="flex-1 overflow-y-auto p-2 space-y-1" @scroll="handleHistoryScroll">
          <button v-for="item in history"
                  :key="item.timestamp + item.language + item.execution_time"
                  class="w-full text-left p-3 rounded-md border transition-colors"
                  :class="selectedItem === item ? 'bg-blue-50 border-blue-200 dark:bg-blue-900/30 dark:border-blue-700' : 'bg-white border-transparent hover:bg-gray-100 dark:bg-gray-900 dark:hover:bg-gray-700'"
                  @click="selectedItem = item">
            <div class="flex items-center justify-between gap-2">
              <span class="text-sm font-medium text-gray-900 dark:text-gray-100 truncate">
                {{ getLanguageDisplayName(item.language) }}
              </span>
              <span class="inline-flex items-center rounded px-1.5 py-0.5 text-xs font-medium"
                    :class="item.success ? 'bg-green-100 text-green-700 dark:bg-green-900/40 dark:text-green-300' : 'bg-red-100 text-red-700 dark:bg-red-900/40 dark:text-red-300'">
                {{ item.success ? '成功' : '失败' }}
              </span>
            </div>
            <div class="mt-2 flex items-center justify-between text-xs text-gray-500 dark:text-gray-400">
              <span class="flex items-center gap-1">
                {{ formatTime(item.timestamp) }}
                <Sparkles v-if="hasAi(item)" class="w-3 h-3 text-blue-500" title="有 AI 对话"/>
              </span>
              <span>{{ item.execution_time }} ms</span>
            </div>
          </button>
          <div v-if="isLoadingMore" class="py-3 text-center text-xs text-gray-500">
            加载更多...
          </div>
          <div v-else-if="!hasMore && history.length > 0" class="py-3 text-center text-xs text-gray-400">
            已加载全部历史
          </div>
        </div>
      </aside>

      <section class="min-w-0 min-h-0 flex flex-col">
        <template v-if="selectedItem">
          <div class="px-4 py-3 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between gap-3">
            <div class="min-w-0">
              <div class="flex items-center gap-2">
                <span class="text-sm font-semibold text-gray-900 dark:text-gray-100">
                  {{ getLanguageDisplayName(selectedItem.language) }}
                </span>
                <span class="text-xs text-gray-500">{{ formatTime(selectedItem.timestamp) }}</span>
              </div>
              <div class="mt-1 text-xs text-gray-500">
                {{ selectedItem.execution_time }} ms · {{ selectedItem.code.length }} 字符
              </div>
            </div>

            <div class="flex items-center gap-2">
              <Button v-if="selectedItem.id != null" type="secondary" variant="outline" size="sm" :icon="Sparkles" @click="openAi">
                {{ hasAi(selectedItem) ? 'AI 对话' : '问 AI' }}
              </Button>
              <Button type="secondary" variant="outline" size="sm" :icon="Copy" @click="copyOutput">
                复制输出
              </Button>
              <Button size="sm" :icon="RotateCcw" @click="restoreSelected">
                恢复代码
              </Button>
            </div>
          </div>

          <div class="flex-1 min-h-0 grid grid-rows-[minmax(0,1fr)_minmax(0,1fr)]">
            <div class="min-h-0 border-b border-gray-200 dark:border-gray-700 flex flex-col">
              <div class="px-4 py-2 bg-gray-50 dark:bg-gray-800 text-xs font-medium text-gray-600 dark:text-gray-300">
                代码
              </div>
              <pre class="flex-1 overflow-auto p-4 text-sm leading-relaxed bg-white dark:bg-gray-950 text-gray-800 dark:text-gray-100 font-mono whitespace-pre-wrap">{{ selectedItem.code }}</pre>
            </div>

            <div class="min-h-0 flex flex-col">
              <div class="px-4 py-2 bg-gray-50 dark:bg-gray-800 text-xs font-medium text-gray-600 dark:text-gray-300">
                输出
              </div>
              <pre class="flex-1 overflow-auto p-4 text-sm leading-relaxed font-mono whitespace-pre-wrap"
                   :class="selectedItem.success ? 'bg-gray-950 text-green-300' : 'bg-gray-950 text-red-300'">{{ selectedOutput }}</pre>
            </div>
          </div>
        </template>

        <div v-else class="h-full flex flex-col items-center justify-center text-gray-400 space-y-2">
          <History class="w-12 h-12"/>
          <p class="text-sm">选择一条历史记录查看详情</p>
        </div>
      </section>
    </div>

    <template #footer>
      <div class="flex items-center justify-between">
        <Button type="danger"
                variant="outline"
                size="sm"
                :icon="Trash2"
                :disabled="total === 0"
                @click="clearHistory">
          清空历史
        </Button>
        <Button type="secondary" size="sm" @click="visible = false">关闭</Button>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Copy, History, RefreshCw, RotateCcw, Sparkles, Trash2 } from 'lucide-vue-next'
import Modal from '../ui/Modal.vue'
import Button from '../ui/Button.vue'
import type { ExecutionResult, Language } from '../types/app'
import { useToast } from '../plugins/toast'

interface ExecutionHistoryPage
{
  items: ExecutionResult[]
  total: number
}

const props = defineProps<{
  show: boolean
  supportedLanguages: Language[]
}>()

const emit = defineEmits<{
  'update:show': [value: boolean]
  restore: [item: ExecutionResult]
  'open-ai': [executionId: number]
}>()

const toast = useToast()
const history = ref<ExecutionResult[]>([])
// 有 AI 对话的执行 id 集合
const aiConversationIds = ref<Set<number>>(new Set())
const hasAi = (item: ExecutionResult) => item.id != null && aiConversationIds.value.has(item.id)
const selectedItem = ref<ExecutionResult | null>(null)
const isLoading = ref(false)
const isLoadingMore = ref(false)
const total = ref(0)
const pageSize = 30

const visible = computed({
  get: () => props.show,
  set: (value: boolean) => emit('update:show', value)
})

const hasMore = computed(() => history.value.length < total.value)

const selectedOutput = computed(() => {
  if (!selectedItem.value) {
    return ''
  }
  const stdout = selectedItem.value.stdout.trim()
  const stderr = selectedItem.value.stderr.trim()
  return [stdout, stderr].filter(Boolean).join('\n\n') || '代码执行成功 (无输出)'
})

const getLanguageDisplayName = (languageValue: string) => {
  return props.supportedLanguages.find(language => language.value === languageValue)?.name || languageValue
}

const formatTime = (timestamp: number) => {
  return new Date(timestamp * 1000).toLocaleString('zh-CN', {
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
}

const loadHistoryPage = async (offset: number) => {
  const page = await invoke<ExecutionHistoryPage>('get_execution_history_page', {
    offset,
    limit: pageSize
  })

  total.value = page.total

  if (offset === 0) {
    history.value = page.items
    selectedItem.value = page.items.length > 0 ? page.items[0] : null
  }
  else {
    history.value = [...history.value, ...page.items]
  }
}

const loadAiConversationIds = async () => {
  try {
    const ids = await invoke<number[]>('list_ai_conversation_ids')
    aiConversationIds.value = new Set(ids)
  }
  catch {
    aiConversationIds.value = new Set()
  }
}

const reloadHistory = async () => {
  isLoading.value = true
  try {
    await loadHistoryPage(0)
    await loadAiConversationIds()
  }
  catch (error) {
    toast.error('加载执行历史失败: ' + error)
  }
  finally {
    isLoading.value = false
  }
}

const loadMoreHistory = async () => {
  if (isLoading.value || isLoadingMore.value || !hasMore.value) {
    return
  }

  isLoadingMore.value = true
  try {
    await loadHistoryPage(history.value.length)
  }
  catch (error) {
    toast.error('加载更多历史失败: ' + error)
  }
  finally {
    isLoadingMore.value = false
  }
}

const handleHistoryScroll = async (event: Event) => {
  const target = event.target as HTMLElement
  const distanceToBottom = target.scrollHeight - target.scrollTop - target.clientHeight
  if (distanceToBottom < 80) {
    await loadMoreHistory()
  }
}

const clearHistory = async () => {
  try {
    await invoke('clear_execution_history')
    history.value = []
    selectedItem.value = null
    total.value = 0
    toast.success('执行历史已清空')
  }
  catch (error) {
    toast.error('清空执行历史失败: ' + error)
  }
}

const restoreSelected = () => {
  if (!selectedItem.value) {
    return
  }
  emit('restore', selectedItem.value)
  visible.value = false
}

const openAi = () => {
  if (selectedItem.value?.id == null) {
    return
  }
  emit('open-ai', selectedItem.value.id)
  visible.value = false
}

const copyOutput = async () => {
  if (!selectedItem.value) {
    return
  }
  try {
    await navigator.clipboard.writeText(selectedOutput.value)
    toast.success('输出已复制')
  }
  catch (error) {
    toast.error('复制失败: ' + error)
  }
}

watch(() => props.show, async (show) => {
  if (show) {
    await reloadHistory()
  }
})
</script>
