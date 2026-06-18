<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-16 px-6 pb-6" @click="emit('close')">
    <div class="w-full max-w-[860px] max-h-full bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col"
         @click.stop>
      <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
          <Network class="w-4 h-4 text-gray-400"/>
          <span>{{ t('git.graphTitle') }}</span>
        </div>
        <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('close')">
          <X class="w-4 h-4"/>
        </button>
      </div>

      <div class="flex-1 overflow-auto">
        <div v-if="!rows.length" class="px-4 py-10 text-center text-sm text-gray-400">{{ t('git.graphEmpty') }}</div>
        <div v-for="row in rows" :key="row.c.hash"
             class="flex items-stretch hover:bg-gray-50 dark:hover:bg-gray-800/60 border-b border-gray-100 dark:border-gray-800/60">
          <svg :width="graphWidth" :height="ROW_H" class="flex-shrink-0" :viewBox="`0 0 ${graphWidth} ${ROW_H}`">
            <path v-for="(seg, si) in row.segs" :key="si" :d="seg.d" :stroke="seg.color" stroke-width="1.6" fill="none"/>
            <circle :cx="cx(row.col)" :cy="ROW_H / 2" r="4" :fill="laneColor(row.col)"
                    stroke="var(--cf-dot-stroke, #fff)" stroke-width="1.5"/>
          </svg>
          <div class="flex-1 min-w-0 flex items-center gap-2 pr-4 py-1.5 text-xs">
            <span v-for="(rf, ri) in row.refList" :key="ri"
                  class="px-1.5 py-0.5 rounded text-[10px] font-medium flex-shrink-0"
                  :class="rf.kind === 'tag' ? 'bg-amber-100 text-amber-700 dark:bg-amber-900/40 dark:text-amber-300' : 'bg-blue-100 text-blue-700 dark:bg-blue-900/40 dark:text-blue-300'">
              {{ rf.label }}
            </span>
            <span class="truncate text-gray-800 dark:text-gray-100">{{ row.c.subject }}</span>
            <span class="ml-auto flex-shrink-0 font-mono text-[11px] text-gray-400">{{ row.c.short }}</span>
            <span class="flex-shrink-0 text-[11px] text-gray-400 hidden sm:inline">{{ row.c.author }}</span>
            <span class="flex-shrink-0 text-[11px] text-gray-400">{{ row.c.date }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, onMounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {Network, X} from 'lucide-vue-next'
import {useToast} from '../plugins/toast'
import {useI18n} from 'vue-i18n'

const props = defineProps<{ rootDir: string }>()
const emit = defineEmits<{ close: [] }>()

const toast = useToast()
const {t} = useI18n()

interface GraphCommit {
  hash: string; short: string; parents: string[]; refs: string
  author: string; date: string; subject: string
}
interface Seg { d: string; color: string }
interface Ref { label: string; kind: 'tag' | 'branch' }
interface Row { c: GraphCommit; col: number; segs: Seg[]; refList: Ref[] }

const ROW_H = 32
const LANE_W = 14
const PALETTE = ['#3b82f6', '#10b981', '#f59e0b', '#ef4444', '#8b5cf6', '#ec4899', '#14b8a6', '#f97316']

const commits = ref<GraphCommit[]>([])
const laneColor = (i: number) => PALETTE[i % PALETTE.length]
const cx = (i: number) => i * LANE_W + LANE_W / 2

const parseRefs = (refs: string): Ref[] => {
  if (!refs.trim()) {
    return []
  }
  return refs.split(',').map(r => r.trim()).filter(Boolean).map(r => {
    if (r.startsWith('tag: ')) {
      return {label: r.slice(5), kind: 'tag' as const}
    }
    const label = r.replace(/^HEAD -> /, '')
    return {label, kind: 'branch' as const}
  })
}

// 基于经典「泳道」算法的提交图布局（最新在上，topo 序）
const rows = computed<Row[]>(() => {
  const lanes: (string | null)[] = []
  const out: Row[] = []

  for (const c of commits.value) {
    // 找到正在等待该提交的泳道（其子节点已绘制）
    const waiting: number[] = []
    lanes.forEach((h, i) => { if (h === c.hash) waiting.push(i) })

    let col: number
    if (waiting.length) {
      col = waiting[0]
    }
    else {
      col = lanes.indexOf(null)
      if (col === -1) {
        col = lanes.length
        lanes.push(null)
      }
    }

    const before = lanes.slice()
    // 多个子节点汇入：除主列外清空
    for (const k of waiting.slice(1)) {
      lanes[k] = null
    }

    // 安放父提交
    const parentCols: number[] = []
    if (c.parents.length === 0) {
      lanes[col] = null
    }
    else {
      lanes[col] = c.parents[0]
      parentCols.push(col)
      for (const p of c.parents.slice(1)) {
        let idx = lanes.indexOf(p)
        if (idx === -1) {
          idx = lanes.indexOf(null)
          if (idx === -1) {
            idx = lanes.length
            lanes.push(null)
          }
        }
        lanes[idx] = p
        parentCols.push(idx)
      }
    }
    while (lanes.length && lanes[lanes.length - 1] === null) {
      lanes.pop()
    }

    // 绘制线段
    const mid = ROW_H / 2
    const segs: Seg[] = []
    // 上半段：入边从顶部到圆点行
    before.forEach((h, i) => {
      if (h === null) {
        return
      }
      const target = h === c.hash ? col : i
      segs.push({d: `M ${cx(i)} 0 L ${cx(target)} ${mid}`, color: laneColor(i)})
    })
    // 下半段：出边从圆点行到底部
    // 1) 穿过本行、与本提交无关的泳道继续直下
    before.forEach((h, i) => {
      if (h !== null && h !== c.hash) {
        segs.push({d: `M ${cx(i)} ${mid} L ${cx(i)} ${ROW_H}`, color: laneColor(i)})
      }
    })
    // 2) 本提交到各父提交泳道
    for (const idx of parentCols) {
      segs.push({d: `M ${cx(col)} ${mid} L ${cx(idx)} ${ROW_H}`, color: laneColor(idx)})
    }

    out.push({c, col, segs, refList: parseRefs(c.refs)})
  }
  return out
})

// 用已计算 rows 的段坐标推断最大列，决定泳道区宽度
const graphWidth = computed(() => {
  let max = 1
  for (const r of rows.value) {
    if (r.col > max) {
      max = r.col
    }
    for (const s of r.segs) {
      const nums = s.d.match(/[\d.]+/g)?.map(Number) || []
      for (let i = 0; i < nums.length; i += 2) {
        const lane = Math.round((nums[i] - LANE_W / 2) / LANE_W)
        if (lane > max) {
          max = lane
        }
      }
    }
  }
  return (max + 1) * LANE_W + LANE_W / 2
})

const load = async () => {
  try {
    commits.value = await invoke<GraphCommit[]>('git_graph', {root: props.rootDir, limit: 300})
  }
  catch (error) {
    toast.error(t('git.graphFailed') + ': ' + error)
  }
}

onMounted(load)
</script>
