<template>
  <div class="relative w-full h-full overflow-auto bg-[#0b1020]">
    <div class="relative" :style="{ width: `${graph.w}px`, height: `${graph.h}px` }">
      <svg class="absolute inset-0 pointer-events-none" :width="graph.w" :height="graph.h">
        <path v-for="(e, i) in graph.edges" :key="i" :d="e.d" fill="none" stroke="#3b82f6" stroke-width="1.5" stroke-opacity="0.6"/>
      </svg>

      <div v-for="n in graph.nodes" :key="n.id"
           class="absolute rounded-xl border shadow-lg"
           :class="n.kind === 'trigger' ? 'border-amber-500/50 bg-[#2a230f]/95' : 'border-blue-500/40 bg-[#111a2e]/95'"
           :style="{ left: `${n.x}px`, top: `${n.y}px`, width: `${CARD_W}px` }">
        <div class="px-3 py-2 font-bold text-sm border-b border-white/10 truncate flex items-center gap-1.5"
             :class="n.kind === 'trigger' ? 'text-amber-200' : 'text-gray-100'">
          <component :is="n.kind === 'trigger' ? Zap : Workflow" class="w-3.5 h-3.5"/>
          {{ n.title }}
        </div>
        <div v-for="(r, ri) in n.rows" :key="ri"
             class="flex items-center gap-2 px-3 border-b border-white/5 last:border-0"
             :style="{ height: `${ROW_H}px` }">
          <span v-if="r.tag" class="text-[10px] px-1 rounded bg-white/10 text-gray-300 flex-shrink-0">{{ r.tag }}</span>
          <span class="flex-1 text-xs text-gray-300 font-mono truncate">{{ r.text }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed} from 'vue'
import {Workflow, Zap} from 'lucide-vue-next'

const props = defineProps<{ workflow: any }>()

const CARD_W = 280
const HEADER_H = 40
const ROW_H = 30
const X_GAP = 340
const V_GAP = 28
const PAD = 32

const asArray = (v: any): any[] => (Array.isArray(v) ? v : v == null ? [] : [v])

// 触发事件列表
const triggers = computed<string[]>(() => {
  const on = props.workflow?.on
  if (!on) return []
  if (typeof on === 'string') return [on]
  if (Array.isArray(on)) return on.map(String)
  if (typeof on === 'object') return Object.keys(on)
  return []
})

// 某个 step 的展示文本
const stepText = (s: any): string => {
  if (s == null) return ''
  if (typeof s === 'string') return s
  if (s.name) return s.name
  if (s.uses) return `uses: ${s.uses}`
  if (s.run) return `run: ${String(s.run).split('\n')[0]}`
  return '(step)'
}

interface WRow { tag?: string; text: string }
interface WNode { id: string; title: string; kind: 'trigger' | 'job'; rows: WRow[]; x: number; y: number; height: number; deps: string[]; level: number }

const graph = computed(() => {
  const jobs = (props.workflow?.jobs && typeof props.workflow.jobs === 'object') ? props.workflow.jobs : {}
  const jobIds = Object.keys(jobs)

  const needsOf = (id: string): string[] =>
      asArray(jobs[id]?.needs).map(String).filter((n: string) => jobIds.includes(n))

  // 拓扑层级（最长路径）
  const levelCache = new Map<string, number>()
  const levelOf = (id: string, seen = new Set<string>()): number => {
    if (levelCache.has(id)) return levelCache.get(id)!
    if (seen.has(id)) return 0
    seen.add(id)
    const deps = needsOf(id)
    const lv = deps.length ? Math.max(...deps.map(d => levelOf(d, seen))) + 1 : 0
    levelCache.set(id, lv)
    return lv
  }

  const nodes: WNode[] = []

  // 触发节点（最左列）
  if (triggers.value.length) {
    nodes.push({
      id: '__trigger__', title: '触发 (on)', kind: 'trigger',
      rows: triggers.value.map(t => ({text: t})),
      x: 0, y: 0, height: HEADER_H + triggers.value.length * ROW_H, deps: [], level: -1
    })
  }

  // job 节点
  for (const id of jobIds) {
    const job = jobs[id] || {}
    const rows: WRow[] = []
    if (job['runs-on']) {
      rows.push({tag: 'runs-on', text: String(job['runs-on'])})
    }
    for (const s of asArray(job.steps)) {
      rows.push({tag: 'step', text: stepText(s)})
    }
    if (rows.length === 0) {
      rows.push({text: '(无步骤)'})
    }
    nodes.push({
      id, title: job.name || id, kind: 'job', rows,
      x: 0, y: 0, height: HEADER_H + rows.length * ROW_H,
      deps: needsOf(id), level: levelOf(id)
    })
  }

  // 按层级分列；触发节点列 = 0，job 列 = level + (有触发?1:0)
  const hasTrigger = triggers.value.length > 0
  const colOf = (n: WNode) => (n.kind === 'trigger' ? 0 : n.level + (hasTrigger ? 1 : 0))

  // 每列垂直堆叠
  const colCursor = new Map<number, number>()
  // 按列分组后排布
  const byCol = new Map<number, WNode[]>()
  for (const n of nodes) {
    const c = colOf(n)
    if (!byCol.has(c)) byCol.set(c, [])
    byCol.get(c)!.push(n)
  }
  for (const [col, list] of Array.from(byCol.entries()).sort((a, b) => a[0] - b[0])) {
    let y = PAD
    for (const n of list) {
      n.x = PAD + col * X_GAP
      n.y = y
      y += n.height + V_GAP
    }
    colCursor.set(col, y)
  }

  // 连线：触发 → level0 job；job → 依赖它的 job
  const idMap = new Map(nodes.map(n => [n.id, n]))
  const edges: { d: string }[] = []
  const link = (from: WNode, to: WNode) => {
    const sx = from.x + CARD_W
    const sy = from.y + from.height / 2
    const tx = to.x
    const ty = to.y + to.height / 2
    edges.push({d: `M ${sx} ${sy} C ${sx + 80} ${sy}, ${tx - 80} ${ty}, ${tx} ${ty}`})
  }
  const trigger = idMap.get('__trigger__')
  for (const n of nodes) {
    if (n.kind !== 'job') continue
    if (n.deps.length === 0) {
      if (trigger) link(trigger, n)
    }
    else {
      for (const d of n.deps) {
        const dn = idMap.get(d)
        if (dn) link(dn, n)
      }
    }
  }

  const w = Math.max(...nodes.map(n => n.x + CARD_W), 0) + PAD
  const h = Math.max(PAD, ...nodes.map(n => n.y + n.height)) + PAD
  return {nodes, edges, w, h}
})
</script>
