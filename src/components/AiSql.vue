<template>
  <div class="inline-block">
    <button ref="btnRef" class="inline-flex items-center gap-1 px-2 py-0.5 rounded text-xs text-violet-500 hover:bg-violet-50 dark:hover:bg-violet-900/30 cursor-pointer"
            :title="t('sql.aiTitle')" @click="toggle">
      <Sparkles class="w-3.5 h-3.5"/>
      <span>AI</span>
    </button>

    <Teleport to="body">
      <template v-if="open">
        <div class="fixed inset-0 z-[60]" @click="open = false"/>
        <div class="fixed z-[61] w-80 p-3 rounded-md border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 shadow-lg text-xs"
             :style="{left: pos.left + 'px', top: pos.top + 'px'}">
          <div class="flex items-center gap-1 mb-2 text-gray-600 dark:text-gray-300 font-medium">
            <Sparkles class="w-3.5 h-3.5 text-violet-500"/>{{ t('sql.aiHeading') }}
            <span class="ml-auto text-[10px] text-gray-400">{{ active.model }}</span>
          </div>
          <textarea v-model="prompt" rows="3" :disabled="loading"
                    :placeholder="t('sql.aiPlaceholder')"
                    class="w-full px-2 py-1.5 rounded border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-900 text-gray-700 dark:text-gray-200 resize-none focus:outline-none focus:border-violet-400"
                    @keydown.meta.enter="generate" @keydown.ctrl.enter="generate"/>
          <div v-if="error" class="mt-1 text-red-500 whitespace-pre-wrap">{{ error }}</div>
          <div class="mt-2 flex items-center justify-between">
            <span class="text-[10px] text-gray-400">{{ schemaNote }}</span>
            <button class="inline-flex items-center gap-1 px-3 py-1 rounded bg-violet-500 text-white hover:bg-violet-600 disabled:opacity-50 cursor-pointer"
                    :disabled="loading || !prompt.trim()" @click="generate">
              <RefreshCw v-if="loading" class="w-3 h-3 animate-spin"/>
              <Sparkles v-else class="w-3 h-3"/>
              {{ loading ? t('sql.generating') : t('sql.generate') }}
            </button>
          </div>
        </div>
      </template>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import {ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {RefreshCw, Sparkles} from 'lucide-vue-next'
import {useI18n} from 'vue-i18n'
import {useAiConfig} from '../composables/useAiConfig'
import {useDbConnections} from '../composables/useDbConnections'

const emit = defineEmits<{ generated: [sql: string] }>()
const {t} = useI18n()

const {active} = useAiConfig()
const {resolveActiveSource} = useDbConnections()

const open = ref(false)
const btnRef = ref<HTMLElement>()
const pos = ref({left: 0, top: 0})
const prompt = ref('')
const loading = ref(false)
const error = ref('')
const schemaNote = ref('')

const toggle = () => {
  open.value = !open.value
  if (open.value) {
    const r = btnRef.value?.getBoundingClientRect()
    if (r) {
      pos.value = {left: Math.max(8, Math.min(r.left, window.innerWidth - 328)), top: r.bottom + 4}
    }
    error.value = ''
  }
}

const introspectSql = (kind: string): string => {
  if (kind === 'mysql') {
    return 'SELECT table_name AS t, column_name AS c, column_type AS ty '
      + 'FROM information_schema.columns WHERE table_schema = DATABASE() '
      + 'ORDER BY table_name, ordinal_position'
  }
  return 'SELECT m.name AS t, p.name AS c, p.type AS ty '
    + 'FROM sqlite_master m JOIN pragma_table_info(m.name) p '
    + "WHERE m.type IN ('table','view') AND m.name NOT LIKE 'sqlite\\_%' ESCAPE '\\' "
    + 'ORDER BY m.name, p.cid'
}

// 拉取结构，拼成紧凑文本作为上下文（限表数避免超长）
const fetchSchema = async (source: any): Promise<string> => {
  try {
    const res = await invoke<any>('run_sql', {sql: introspectSql(source.kind), source})
    if (res.error) {
      return ''
    }
    const rows = (res.result_sets || [])[0]?.rows || []
    const map = new Map<string, string[]>()
    for (const r of rows) {
      const t = String(r[0])
      if (!map.has(t)) {
        map.set(t, [])
      }
      map.get(t)!.push(`${r[1]} ${r[2] ?? ''}`.trim())
    }
    const tables = [...map.entries()].slice(0, 60)
    return tables.map(([t, cols]) => `${t}(${cols.join(', ')})`).join('\n')
  }
  catch {
    return ''
  }
}

const stripFences = (s: string): string => {
  return s.trim().replace(/^```(?:sql)?\s*/i, '').replace(/```$/i, '').trim()
}

const generate = async () => {
  if (loading.value || !prompt.value.trim()) {
    return
  }
  if (!active.value.apiKey?.trim()) {
    error.value = t('sql.noApiKey')
    return
  }
  loading.value = true
  error.value = ''
  try {
    const source = resolveActiveSource()
    const dialect = source.kind === 'mysql' ? 'MySQL' : 'SQLite'
    schemaNote.value = t('sql.readingSchema')
    const schema = await fetchSchema(source)
    schemaNote.value = schema ? t('sql.schemaAttached') : t('sql.schemaNone')
    const system = `你是资深数据库工程师。根据「数据库结构」和用户需求，生成可直接执行的 ${dialect} SQL。`
      + '要求：只输出 SQL 本身，不要任何解释，不要使用 Markdown 代码块标记。\n'
      + `数据库结构：\n${schema || '(未提供，请根据需求合理假设表名与字段名)'}`
    const text = await invoke<string>('ai_chat', {
      provider: active.value.provider,
      baseUrl: active.value.baseUrl,
      apiKey: active.value.apiKey,
      model: active.value.model,
      system,
      messages: [{role: 'user', content: prompt.value.trim()}]
    })
    const sql = stripFences(text)
    if (!sql) {
      error.value = t('sql.aiNoContent')
      return
    }
    emit('generated', sql)
    open.value = false
    prompt.value = ''
  }
  catch (e: any) {
    error.value = String(e?.message || e)
  }
  finally {
    loading.value = false
  }
}
</script>
