<template>
  <div class="relative w-full h-full overflow-auto bg-[#0b1020]">
    <div class="relative" :style="{ width: `${graph.w}px`, height: `${graph.h}px` }">
      <!-- 连线 -->
      <svg class="absolute inset-0 pointer-events-none" :width="graph.w" :height="graph.h">
        <path v-for="(e, i) in graph.edges" :key="i" :d="e.d" fill="none" stroke="#3b82f6" stroke-width="1.5" stroke-opacity="0.6"/>
      </svg>

      <!-- 卡片节点 -->
      <div v-for="n in graph.nodes" :key="n.id"
           class="absolute rounded-xl border border-blue-500/40 bg-[#111a2e]/95 shadow-lg"
           :style="{ left: `${n.x}px`, top: `${n.y}px`, width: `${CARD_W}px` }">
        <div class="px-3 py-2 font-bold text-sm text-gray-100 border-b border-white/10 truncate">{{ n.title }}</div>
        <div v-for="(r, ri) in n.rows" :key="ri"
             class="flex items-center gap-2 px-3 border-b border-white/5 last:border-0"
             :style="{ height: `${ROW_H}px` }">
          <span class="text-gray-400 text-xs flex-shrink-0 max-w-[40%] truncate">{{ r.key }}</span>
          <span class="flex-1 px-2 py-0.5 rounded bg-black/30 border border-white/10 font-mono text-xs truncate"
                :class="r.childId ? 'text-blue-300' : 'text-gray-200'">{{ r.display }}</span>
          <button v-if="r.childId"
                  class="flex-shrink-0 w-5 h-5 flex items-center justify-center rounded border border-blue-500/40 text-blue-300 hover:bg-blue-500/20 cursor-pointer"
                  :title="expanded.has(r.childId) ? t('view.collapse') : t('view.expand')"
                  @click="toggle(r.childId)">
            <ChevronDown class="w-3.5 h-3.5 transition-transform" :class="{ '-rotate-90': !expanded.has(r.childId) }"/>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, reactive, watch} from 'vue'
import {ChevronDown} from 'lucide-vue-next'
import {useI18n} from 'vue-i18n'

const {t} = useI18n()

const props = defineProps<{ value: any; rootTitle?: string }>()

const CARD_W = 260
const HEADER_H = 40
const ROW_H = 34
const X_GAP = 320
const V_GAP = 24
const PAD = 32

const isObj = (v: any) => v !== null && typeof v === 'object'
const entriesOf = (v: any): [string, any][] =>
    Array.isArray(v) ? v.map((x, i) => [String(i), x]) : Object.entries(v)

const summary = (v: any) => (Array.isArray(v) ? `[ ${v.length} ]` : '{ … }')
const scalar = (v: any) => {
  if (v === null) return 'null'
  const s = typeof v === 'string' ? `"${v}"` : String(v)
  return s.length > 40 ? s.slice(0, 40) + '…' : s
}

// 展开状态（按 childId 路径）；默认展开前两层
const expanded = reactive(new Set<string>())
const initExpand = (v: any, path: string, depth: number) => {
  if (!isObj(v) || depth >= 2) return
  for (const [k, val] of entriesOf(v)) {
    if (isObj(val)) {
      const childId = path ? `${path}/${k}` : k
      expanded.add(childId)
      initExpand(val, childId, depth + 1)
    }
  }
}
watch(() => props.value, (v) => {
  expanded.clear()
  initExpand(v, '', 0)
}, {immediate: true})

const toggle = (id: string) => {
  if (expanded.has(id)) {
    expanded.delete(id)
  }
  else {
    expanded.add(id)
  }
}

interface GRow { key: string; display: string; childId?: string; value?: any }
interface GNode { id: string; title: string; rows: GRow[]; x: number; y: number; height: number; children: { node: GNode; rowIndex: number }[] }

const graph = computed(() => {
  const nodes: GNode[] = []

  const build = (value: any, path: string, title: string): GNode => {
    const id = path || 'root'
    const ents = isObj(value) ? entriesOf(value) : [['(value)', value] as [string, any]]
    const rows: GRow[] = ents.map(([k, v]) => {
      const expandable = isObj(v)
      const childId = expandable ? (path ? `${path}/${k}` : k) : undefined
      return {key: k, value: v, childId, display: expandable ? summary(v) : scalar(v)}
    })
    const node: GNode = {
      id, title, rows,
      x: 0, y: 0,
      height: HEADER_H + rows.length * ROW_H,
      children: []
    }
    nodes.push(node)
    rows.forEach((r, ri) => {
      if (r.childId && expanded.has(r.childId)) {
        const child = build(r.value, r.childId, r.key)
        node.children.push({node: child, rowIndex: ri})
      }
    })
    return node
  }

  const root = build(props.value, '', props.rootTitle || 'ROOT')

  // 布局：x 按深度，y 用游标堆叠叶子、父节点居中于子节点
  let cursor = PAD
  const layout = (node: GNode, depth: number) => {
    node.x = PAD + depth * X_GAP
    if (node.children.length === 0) {
      node.y = cursor
      cursor += node.height + V_GAP
      return
    }
    for (const c of node.children) {
      layout(c.node, depth + 1)
    }
    const first = node.children[0].node
    const last = node.children[node.children.length - 1].node
    node.y = (first.y + last.y + last.height) / 2 - node.height / 2
    if (node.y < PAD) node.y = PAD
  }
  layout(root, 0)

  // 连线
  const edges: { d: string }[] = []
  for (const node of nodes) {
    for (const {node: child, rowIndex} of node.children) {
      const sx = node.x + CARD_W
      const sy = node.y + HEADER_H + rowIndex * ROW_H + ROW_H / 2
      const tx = child.x
      const ty = child.y + child.height / 2
      edges.push({d: `M ${sx} ${sy} C ${sx + 70} ${sy}, ${tx - 70} ${ty}, ${tx} ${ty}`})
    }
  }

  const w = Math.max(...nodes.map(n => n.x + CARD_W), 0) + PAD
  const h = Math.max(cursor, ...nodes.map(n => n.y + n.height)) + PAD
  return {nodes, edges, w, h}
})
</script>
