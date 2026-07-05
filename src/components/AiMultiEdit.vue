<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-16 px-6 pb-6" @click="emit('close')">
    <div class="w-full max-w-[980px] max-h-full bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col"
         @click.stop>
      <!-- 标题栏 -->
      <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
          <Sparkles class="w-4 h-4 text-purple-500"/>
          <span>{{ t('aiMulti.title') }}</span>
        </div>
        <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('close')">
          <X class="w-4 h-4"/>
        </button>
      </div>

      <!-- 输入阶段：指令 + 选择参与文件 -->
      <template v-if="stage === 'input'">
        <div class="p-4 flex flex-col gap-3 overflow-auto">
          <textarea v-model="instruction"
                    rows="3"
                    :placeholder="t('aiMulti.instructionPlaceholder')"
                    class="w-full text-sm rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-3 py-2 resize-none focus:outline-none focus:ring-1 focus:ring-blue-400"></textarea>
          <div>
            <div class="text-xs text-gray-500 mb-1.5">{{ t('aiMulti.contextFiles', { n: selectedFiles.length }) }}</div>
            <div class="max-h-[220px] overflow-auto rounded border border-gray-200 dark:border-gray-700 divide-y divide-gray-100 dark:divide-gray-800">
              <label v-for="f in files" :key="f.path"
                     class="flex items-center gap-2 px-3 py-1.5 text-xs cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-800">
                <input type="checkbox" :value="f.path" v-model="picked" class="cursor-pointer"/>
                <span class="font-medium">{{ f.name }}</span>
                <span class="text-gray-400 truncate">{{ f.path }}</span>
              </label>
            </div>
          </div>
        </div>
        <div class="flex items-center justify-end gap-2 px-4 py-2.5 border-t border-gray-200 dark:border-gray-700 flex-shrink-0">
          <button class="text-xs px-3 py-1.5 rounded text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="emit('close')">{{ t('aiMulti.cancel') }}</button>
          <button :disabled="!instruction.trim() || selectedFiles.length === 0 || loading"
                  class="text-xs px-3 py-1.5 rounded bg-blue-500 text-white hover:bg-blue-600 disabled:opacity-40 disabled:cursor-not-allowed cursor-pointer"
                  @click="generate">
            <span v-if="loading" class="inline-flex items-center gap-1"><Sparkles class="w-3 h-3 animate-pulse"/>{{ t('aiMulti.thinking') }}</span>
            <span v-else>{{ t('aiMulti.generate') }}</span>
          </button>
        </div>
      </template>

      <!-- 审查阶段：逐文件差异 + 勾选应用 -->
      <template v-else>
        <div class="flex-1 overflow-auto p-4 flex flex-col gap-3">
          <div v-if="proposals.length === 0" class="text-center text-sm text-gray-400 py-10">{{ t('aiMulti.noChange') }}</div>
          <div v-for="p in proposals" :key="p.path" class="rounded border border-gray-200 dark:border-gray-700 overflow-hidden">
            <div class="flex items-center gap-2 px-3 py-2 bg-gray-50 dark:bg-gray-800 text-xs">
              <input type="checkbox" :value="p.path" v-model="applyPicked" class="cursor-pointer"/>
              <span class="font-medium">{{ p.name }}</span>
              <span class="text-gray-400 truncate flex-1">{{ p.path }}</span>
              <span class="text-green-600 dark:text-green-400">+{{ p.added }}</span>
              <span class="text-red-600 dark:text-red-400">−{{ p.removed }}</span>
              <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="toggle(p.path)">
                <component :is="expanded.has(p.path) ? ChevronDown : ChevronRight" class="w-4 h-4"/>
              </button>
            </div>
            <div v-if="expanded.has(p.path)" class="font-mono text-xs leading-5 max-h-[300px] overflow-auto">
              <div v-for="(row, i) in p.rows" :key="i" class="flex" :class="rowClass(row.type)">
                <span class="w-10 flex-shrink-0 text-right pr-2 select-none text-gray-400 dark:text-gray-500">{{ row.oldNo || '' }}</span>
                <span class="w-10 flex-shrink-0 text-right pr-2 select-none text-gray-400 dark:text-gray-500">{{ row.newNo || '' }}</span>
                <span class="w-4 flex-shrink-0 select-none text-center">{{ sign(row.type) }}</span>
                <span class="flex-1 whitespace-pre-wrap break-all pr-3">{{ row.text }}</span>
              </div>
            </div>
          </div>
        </div>
        <div class="flex items-center justify-between gap-2 px-4 py-2.5 border-t border-gray-200 dark:border-gray-700 flex-shrink-0">
          <button class="text-xs px-3 py-1.5 rounded text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="stage = 'input'">{{ t('aiMulti.back') }}</button>
          <div class="flex items-center gap-2">
            <button class="text-xs px-3 py-1.5 rounded text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="emit('close')">{{ t('aiMulti.cancel') }}</button>
            <button :disabled="applyPicked.length === 0"
                    class="text-xs px-3 py-1.5 rounded bg-blue-500 text-white hover:bg-blue-600 disabled:opacity-40 disabled:cursor-not-allowed cursor-pointer"
                    @click="apply">
              {{ t('aiMulti.apply', { n: applyPicked.length }) }}
            </button>
          </div>
        </div>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {ChevronDown, ChevronRight, Sparkles, X} from 'lucide-vue-next'
import {useI18n} from 'vue-i18n'
import {useAiConfig} from '../composables/useAiConfig'
import {useToast} from '../plugins/toast'

interface InputFile { path: string; name: string; content: string }
interface DiffRow { type: 'same' | 'add' | 'del'; text: string; oldNo?: number; newNo?: number }
interface Proposal { path: string; name: string; original: string; modified: string; rows: DiffRow[]; added: number; removed: number }

const props = defineProps<{ files: InputFile[] }>()
const emit = defineEmits<{ apply: [edits: { path: string; content: string }[]]; close: [] }>()

const toast = useToast()
const {t} = useI18n()
const {active, reload} = useAiConfig()

const stage = ref<'input' | 'review'>('input')
const instruction = ref('')
const loading = ref(false)
const picked = ref<string[]>(props.files.map(f => f.path))
const selectedFiles = computed(() => props.files.filter(f => picked.value.includes(f.path)))

const proposals = ref<Proposal[]>([])
const applyPicked = ref<string[]>([])
const expanded = ref<Set<string>>(new Set())

const toggle = (path: string) => {
  const s = new Set(expanded.value)
  s.has(path) ? s.delete(path) : s.add(path)
  expanded.value = s
}

const stripFences = (text: string) => {
  const trimmed = text.trim()
  const m = trimmed.match(/^```[\w+-]*\n([\s\S]*?)\n?```$/)
  return m ? m[1] : trimmed
}

// 基于 LCS 的逐行差异（与 DiffView 保持一致的编辑器规模算法）
const diffRows = (original: string, modified: string): DiffRow[] => {
  const a = original.split('\n')
  const b = modified.split('\n')
  const n = a.length, m = b.length
  if (n * m > 4_000_000) {
    return [{type: 'del', text: t('aiMulti.tooLarge')}]
  }
  const dp: number[][] = Array.from({length: n + 1}, () => new Array(m + 1).fill(0))
  for (let i = n - 1; i >= 0; i--) {
    for (let j = m - 1; j >= 0; j--) {
      dp[i][j] = a[i] === b[j] ? dp[i + 1][j + 1] + 1 : Math.max(dp[i + 1][j], dp[i][j + 1])
    }
  }
  const out: DiffRow[] = []
  let i = 0, j = 0, oldNo = 0, newNo = 0
  while (i < n && j < m) {
    if (a[i] === b[j]) {
      out.push({type: 'same', text: a[i], oldNo: ++oldNo, newNo: ++newNo}); i++; j++
    }
    else if (dp[i + 1][j] >= dp[i][j + 1]) {
      out.push({type: 'del', text: a[i], oldNo: ++oldNo}); i++
    }
    else {
      out.push({type: 'add', text: b[j], newNo: ++newNo}); j++
    }
  }
  while (i < n) out.push({type: 'del', text: a[i++], oldNo: ++oldNo})
  while (j < m) out.push({type: 'add', text: b[j++], newNo: ++newNo})
  return out
}

const rowClass = (ty: DiffRow['type']) =>
  ty === 'add' ? 'bg-green-50 dark:bg-green-900/25 text-green-800 dark:text-green-300'
    : ty === 'del' ? 'bg-red-50 dark:bg-red-900/25 text-red-800 dark:text-red-300'
      : 'text-gray-700 dark:text-gray-300'
const sign = (ty: DiffRow['type']) => (ty === 'add' ? '+' : ty === 'del' ? '−' : '')

const buildUser = (): string => {
  const parts = [instruction.value.trim()]
  for (const f of selectedFiles.value) {
    parts.push(`\n\n===== FILE: ${f.path} =====\n${f.content}`)
  }
  return parts.join('')
}

const SYSTEM = `你是资深工程师，负责跨多个文件完成用户的修改需求。只修改确有必要的文件，未改动的文件不要出现在结果里。`
  + `每个文件的输入以「===== FILE: <路径> =====」开头，其后是该文件的完整内容。`
  + `必须严格输出 JSON（不要 Markdown 代码块、不要任何解释），格式为：`
  + `{"edits":[{"path":"<与输入完全一致的文件路径>","content":"<该文件完整的新内容>"}]}。`
  + `content 必须是文件的完整新内容而非片段，且只能使用输入中出现过的文件路径。`

const generate = async () => {
  reload()
  if (!active.value.apiKey) {
    toast.error(t('chat.needKey'))
    return
  }
  loading.value = true
  try {
    const reply = await invoke<string>('ai_chat', {
      provider: active.value.provider,
      baseUrl: active.value.baseUrl,
      apiKey: active.value.apiKey,
      model: active.value.model,
      system: SYSTEM,
      messages: [{role: 'user', content: buildUser()}]
    })
    const parsed = JSON.parse(stripFences(reply))
    const edits: { path: string; content: string }[] = Array.isArray(parsed?.edits) ? parsed.edits : []
    const byPath = new Map(props.files.map(f => [f.path, f]))
    const next: Proposal[] = []
    for (const e of edits) {
      const src = byPath.get(e.path)
      // 仅接受输入中存在、且内容确有变化的文件
      if (!src || typeof e.content !== 'string' || e.content === src.content) {
        continue
      }
      const rows = diffRows(src.content, e.content)
      next.push({
        path: src.path,
        name: src.name,
        original: src.content,
        modified: e.content,
        rows,
        added: rows.filter(r => r.type === 'add').length,
        removed: rows.filter(r => r.type === 'del').length
      })
    }
    proposals.value = next
    applyPicked.value = next.map(p => p.path)
    expanded.value = new Set(next.length === 1 ? [next[0].path] : [])
    stage.value = 'review'
  }
  catch (error) {
    toast.error(t('aiMulti.failed') + error)
  }
  finally {
    loading.value = false
  }
}

const apply = () => {
  const edits = proposals.value
    .filter(p => applyPicked.value.includes(p.path))
    .map(p => ({path: p.path, content: p.modified}))
  if (edits.length === 0) {
    return
  }
  emit('apply', edits)
  emit('close')
}
</script>
