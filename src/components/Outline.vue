<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-24" @click="emit('close')">
    <div class="w-[520px] max-w-[90vw] bg-white dark:bg-gray-800 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col max-h-[60vh]"
         @click.stop>
      <div class="flex items-center px-3 border-b border-gray-200 dark:border-gray-700">
        <ListTree class="w-4 h-4 text-gray-400 flex-shrink-0"/>
        <input ref="inputRef"
               v-model="query"
               class="flex-1 px-2 py-2.5 text-sm bg-transparent focus:outline-none"
               :placeholder="t('dialog.outlinePlaceholder')"
               @keydown.down.prevent="move(1)"
               @keydown.up.prevent="move(-1)"
               @keydown.enter.prevent="choose(filtered[activeIndex])"
               @keydown.esc.prevent="emit('close')"/>
        <span class="text-xs text-gray-400 flex-shrink-0">{{ symbols.length }}</span>
      </div>

      <div ref="listRef" class="overflow-y-auto py-1">
        <div v-if="symbols.length === 0" class="px-4 py-6 text-center text-sm text-gray-400">{{ t('dialog.outlineNoSymbols') }}</div>
        <div v-else-if="filtered.length === 0" class="px-4 py-6 text-center text-sm text-gray-400">{{ t('dialog.outlineEmpty') }}</div>

        <button v-for="(s, i) in filtered"
                :key="s.line + ':' + s.name"
                :ref="el => setItemRef(el, i)"
                class="w-full flex items-center px-3 py-1.5 text-left cursor-pointer"
                :class="i === activeIndex ? 'bg-blue-100 dark:bg-gray-700' : 'hover:bg-gray-100 dark:hover:bg-gray-700'"
                @click="choose(s)"
                @mousemove="activeIndex = i">
          <span class="w-12 flex-shrink-0 text-[10px] uppercase font-semibold" :class="kindColor(s.kind)">{{ s.kind }}</span>
          <span class="text-sm text-gray-800 dark:text-gray-100 truncate font-mono" :style="{ paddingLeft: `${s.depth * 14}px` }">{{ s.name }}</span>
          <span class="ml-auto pl-3 text-xs text-gray-400 flex-shrink-0">{{ s.line }}</span>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, nextTick, onMounted, ref, watch} from 'vue'
import {useI18n} from 'vue-i18n'
import {ListTree} from 'lucide-vue-next'
import {fetchDocumentSymbols, lspSymbolKindLabel} from '../editor/lspExtension'

const {t} = useI18n()

interface Symbol { name: string; kind: string; line: number; depth: number }

const props = defineProps<{ code: string; language?: string; currentLine?: number; view?: any }>()
const emit = defineEmits<{ go: [line: number]; close: [] }>()

// LSP 精确符号（可用时优先于正则）
const lspSymbols = ref<Symbol[] | null>(null)

const query = ref('')
const activeIndex = ref(0)
const inputRef = ref<HTMLInputElement | null>(null)
const listRef = ref<HTMLElement | null>(null)
const itemRefs: HTMLElement[] = []
const setItemRef = (el: any, i: number) => {
  if (el) itemRefs[i] = el
}

// 预选：定位到光标所在（或其上方最近）的符号，打开即落在当前函数
const preselectAtCursor = () => {
  if (!props.currentLine || !symbols.value.length) {
    return
  }
  let idx = -1
  for (let i = 0; i < symbols.value.length; i++) {
    if (symbols.value[i].line <= props.currentLine) idx = i
    else break
  }
  if (idx >= 0) {
    activeIndex.value = idx
    nextTick(() => itemRefs[idx]?.scrollIntoView({block: 'center'}))
  }
}

onMounted(async () => {
  inputRef.value?.focus()
  preselectAtCursor()
  // 尝试用 LSP documentSymbol 精确符号替换正则结果
  if (props.view) {
    try {
      const syms = await fetchDocumentSymbols(props.view)
      if (syms && syms.length) {
        lspSymbols.value = syms.map(s => ({name: s.name, kind: lspSymbolKindLabel(s.kind), line: s.line, depth: s.depth}))
        activeIndex.value = 0
        preselectAtCursor()
      }
    }
    catch { /* 回退到正则 */ }
  }
})

// 控制关键字，避免把 if/for 等当成方法名
const STOP = new Set(['if', 'for', 'while', 'switch', 'catch', 'return', 'else', 'do', 'foreach', 'with', 'when', 'match'])

// 按语言归类
const family = computed(() => {
  const l = (props.language || '').toLowerCase()
  if (l.startsWith('python')) return 'python'
  if (l === 'go') return 'go'
  if (l === 'rust' || l === 'cangjie') return 'rust'
  if (l === 'java') return 'java'
  if (l === 'ruby') return 'ruby'
  if (l === 'php') return 'php'
  if (l === 'c' || l === 'cpp' || l === 'objective-c' || l === 'objective-cpp') return 'c'
  if (l.includes('javascript') || l.includes('typescript') || l === 'nodejs') return 'js'
  return 'generic'
})

// 各语言族的提取规则：[正则, kind, 捕获组下标]
const rulesFor = (fam: string): [RegExp, string, number][] => {
  switch (fam) {
    case 'python':
      return [
        [/^\s*class\s+([A-Za-z_]\w*)/, 'class', 1],
        [/^\s*(?:async\s+)?def\s+([A-Za-z_]\w*)/, 'def', 1],
      ]
    case 'js':
      return [
        [/^\s*(?:export\s+)?(?:default\s+)?(?:async\s+)?function\*?\s+([A-Za-z_$][\w$]*)/, 'func', 1],
        [/^\s*(?:export\s+)?(?:abstract\s+)?class\s+([A-Za-z_$][\w$]*)/, 'class', 1],
        [/^\s*(?:export\s+)?interface\s+([A-Za-z_$][\w$]*)/, 'iface', 1],
        [/^\s*(?:export\s+)?type\s+([A-Za-z_$][\w$]*)/, 'type', 1],
        [/^\s*(?:export\s+)?(?:const|let|var)\s+([A-Za-z_$][\w$]*)\s*=\s*(?:async\s*)?(?:\([^)]*\)|[A-Za-z_$][\w$]*)\s*=>/, 'func', 1],
        [/^\s+(?:public\s+|private\s+|protected\s+|static\s+|async\s+|get\s+|set\s+)*([A-Za-z_$][\w$]*)\s*\([^)]*\)\s*\{/, 'method', 1],
      ]
    case 'rust':
      return [
        [/^\s*(?:pub(?:\([^)]*\))?\s+)?fn\s+([A-Za-z_]\w*)/, 'fn', 1],
        [/^\s*(?:pub(?:\([^)]*\))?\s+)?struct\s+([A-Za-z_]\w*)/, 'struct', 1],
        [/^\s*(?:pub(?:\([^)]*\))?\s+)?enum\s+([A-Za-z_]\w*)/, 'enum', 1],
        [/^\s*(?:pub(?:\([^)]*\))?\s+)?trait\s+([A-Za-z_]\w*)/, 'trait', 1],
        [/^\s*impl(?:\s*<[^>]*>)?\s+(?:[A-Za-z_][\w:]*\s+for\s+)?([A-Za-z_]\w*)/, 'impl', 1],
      ]
    case 'go':
      return [
        [/^\s*func\s+(?:\([^)]*\)\s*)?([A-Za-z_]\w*)/, 'func', 1],
        [/^\s*type\s+([A-Za-z_]\w*)/, 'type', 1],
      ]
    case 'java':
      return [
        [/^\s*(?:public|private|protected|abstract|final|static|\s)*\s*(?:class|interface|enum)\s+([A-Za-z_]\w*)/, 'class', 1],
        [/^\s*(?:public|private|protected)\s+(?:static\s+|final\s+|abstract\s+|synchronized\s+)*[\w<>\[\],.\s]+?\s+([A-Za-z_]\w*)\s*\([^;{]*\)\s*\{?/, 'method', 1],
      ]
    case 'c':
      return [
        [/^\s*(?:struct|class)\s+([A-Za-z_]\w*)/, 'type', 1],
        [/^[A-Za-z_][\w<>:\*&\s]*?\s+([A-Za-z_]\w*)\s*\([^;]*\)\s*\{/, 'func', 1],
      ]
    case 'ruby':
      return [
        [/^\s*(?:class|module)\s+([A-Za-z_]\w*)/, 'class', 1],
        [/^\s*def\s+([A-Za-z_][\w?!.]*)/, 'def', 1],
      ]
    case 'php':
      return [
        [/^\s*(?:abstract\s+|final\s+)?(?:class|interface|trait)\s+([A-Za-z_]\w*)/, 'class', 1],
        [/^\s*(?:public|private|protected|static|\s)*function\s+([A-Za-z_]\w*)/, 'func', 1],
      ]
    default:
      return [
        [/^\s*(?:export\s+)?(?:async\s+)?function\s+([A-Za-z_$][\w$]*)/, 'func', 1],
        [/^\s*(?:def|fn|func)\s+([A-Za-z_]\w*)/, 'func', 1],
        [/^\s*class\s+([A-Za-z_]\w*)/, 'class', 1],
      ]
  }
}

// LSP 符号可用则优先，否则回退到正则提取
const symbols = computed<Symbol[]>(() => lspSymbols.value ?? regexSymbols.value)

const regexSymbols = computed<Symbol[]>(() => {
  const lines = (props.code || '').split('\n')
  const rules = rulesFor(family.value)
  const raw: { name: string; kind: string; line: number; indent: number }[] = []
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i]
    for (const [re, kind, g] of rules) {
      const m = re.exec(line)
      if (m && m[g] && !STOP.has(m[g])) {
        // 缩进宽度（tab 记为 4），用于推断层级
        const lead = line.slice(0, line.length - line.trimStart().length)
        const indent = lead.replace(/\t/g, '    ').length
        raw.push({name: m[g], kind, line: i + 1, indent})
        break
      }
    }
  }
  // 将出现过的缩进值排序后映射为层级，避免受具体缩进大小影响
  const levels = Array.from(new Set(raw.map(s => s.indent))).sort((a, b) => a - b)
  return raw.map(s => ({name: s.name, kind: s.kind, line: s.line, depth: levels.indexOf(s.indent)}))
})

const filtered = computed<Symbol[]>(() => {
  const q = query.value.trim().toLowerCase()
  if (!q) return symbols.value
  return symbols.value.filter(s => s.name.toLowerCase().includes(q))
})

watch(filtered, () => {
  activeIndex.value = 0
})

const kindColor = (kind: string) => {
  switch (kind) {
    case 'class':
    case 'struct':
    case 'enum':
    case 'type':
    case 'iface':
      return 'text-purple-500'
    case 'impl':
    case 'trait':
      return 'text-blue-500'
    default:
      return 'text-amber-600 dark:text-amber-400'
  }
}

const move = (delta: number) => {
  const len = filtered.value.length
  if (len === 0) return
  activeIndex.value = (activeIndex.value + delta + len) % len
  nextTick(() => itemRefs[activeIndex.value]?.scrollIntoView({block: 'nearest'}))
}

const choose = (s?: Symbol) => {
  if (s) {
    emit('go', s.line)
    emit('close')
  }
}
</script>
