<template>
  <div class="flex flex-col h-full bg-white dark:bg-gray-900">
    <div class="flex items-center justify-between px-3 py-1.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
      <div class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400">
        <Code class="w-3.5 h-3.5"/>
        <span>{{ t('view.xmlTitle') }}</span>
        <span v-if="isRunning" class="text-blue-500">{{ t('view.running') }}</span>
        <span v-else-if="executionTime" class="text-gray-400">{{ t('view.ms', { n: executionTime }) }}</span>
        <span v-if="parseError" class="text-red-500">{{ t('view.parseFailed') }}</span>
      </div>
      <div class="flex items-center gap-1">
        <button class="p-1 rounded cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700"
                :class="mode === 'graph' ? 'text-blue-500' : 'text-gray-400 hover:text-gray-600 dark:hover:text-gray-300'"
                :title="mode === 'graph' ? t('view.toTree') : t('view.toGraph')"
                @click="mode = mode === 'graph' ? 'tree' : 'graph'">
          <Network class="w-3.5 h-3.5"/>
        </button>
        <button class="p-1 rounded text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" :title="t('view.clear')" @click="emit('clear')">
          <Trash2 class="w-3.5 h-3.5"/>
        </button>
      </div>
    </div>

    <!-- 可视化关系图 -->
    <DataGraph v-if="mode === 'graph' && stable.trim() && !parseError && graphData !== null"
               :value="graphData" :root-title="rootTag" class="flex-1"/>

    <!-- 层级树 / 占位 / 错误 -->
    <div v-else class="flex-1 overflow-auto p-2 font-mono text-xs">
      <div v-if="!stable.trim()" class="text-gray-400 px-2 py-4 text-center">{{ t('view.emptyXml') }}</div>
      <div v-else-if="parseError" class="space-y-2">
        <div class="text-red-500">{{ parseError }}</div>
        <pre class="whitespace-pre-wrap text-gray-600 dark:text-gray-400">{{ stable }}</pre>
      </div>
      <XmlNode v-else-if="root" :node="root" :depth="0"/>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, ref, watch} from 'vue'
import {useI18n} from 'vue-i18n'
import {debounce} from 'lodash-es'
import {Code, Network, Trash2} from 'lucide-vue-next'
import XmlNode from './XmlNode.vue'
import DataGraph from './DataGraph.vue'

const props = defineProps<{
  output: string
  isRunning: boolean
  executionTime?: number
}>()
const emit = defineEmits<{ clear: [] }>()
const {t} = useI18n()

// 流式输出防抖，避免逐行重建树
const stable = ref(props.output)
const applyOutput = debounce((v: string) => { stable.value = v }, 200)
watch(() => props.output, (v) => applyOutput(v))

const doc = computed<Document | null>(() => {
  if (!stable.value.trim()) {
    return null
  }
  try {
    return new DOMParser().parseFromString(stable.value, 'application/xml')
  }
  catch {
    return null
  }
})

const parseError = computed(() => {
  const d = doc.value
  if (!d) {
    return ''
  }
  const err = d.querySelector('parsererror')
  return err ? t('view.xmlParseFail') + (err.textContent || '').trim() : ''
})

const root = computed<Element | null>(() => {
  if (!doc.value || parseError.value) {
    return null
  }
  return doc.value.documentElement || null
})

// 显示模式：tree=层级树（默认），graph=可视化关系图
const mode = ref<'tree' | 'graph'>('tree')

// 把 XML 元素转为普通对象，复用 DataGraph（属性加 @ 前缀，同名子标签聚合为数组）
const xmlToData = (el: Element): any => {
  const obj: Record<string, any> = {}
  for (const a of Array.from(el.attributes)) {
    obj['@' + a.name] = a.value
  }
  const children = Array.from(el.children)
  if (children.length === 0) {
    const text = (el.textContent || '').trim()
    if (Object.keys(obj).length === 0) {
      return text
    }
    if (text) {
      obj['#text'] = text
    }
    return obj
  }
  const groups: Record<string, Element[]> = {}
  for (const c of children) {
    (groups[c.tagName] ||= []).push(c)
  }
  for (const [tag, els] of Object.entries(groups)) {
    obj[tag] = els.length === 1 ? xmlToData(els[0]) : els.map(xmlToData)
  }
  return obj
}

const graphData = computed(() => (root.value ? xmlToData(root.value) : null))
const rootTag = computed(() => root.value?.tagName || 'ROOT')
</script>
