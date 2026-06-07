<template>
  <div ref="scroller" class="overflow-auto" :class="maxHeight ? '' : 'h-full'" :style="maxHeight ? {maxHeight: maxHeight + 'px'} : undefined" @scroll="onScroll">
    <table class="border-collapse text-xs" :style="{width: totalWidth + 'px', tableLayout: 'fixed'}">
      <colgroup>
        <col v-if="showIndex" :style="{width: indexW + 'px'}"/>
        <col v-for="(_c, ci) in columns" :key="ci" :style="{width: widths[ci] + 'px'}"/>
      </colgroup>
      <thead class="sticky top-0 z-10">
        <tr class="bg-gray-50 dark:bg-gray-800">
          <th v-if="showIndex" class="text-left font-semibold px-2 py-1.5 border-b border-gray-200 dark:border-gray-700 text-gray-400">#</th>
          <th v-for="(c, ci) in columns" :key="ci"
              class="relative text-left font-semibold px-3 py-1.5 border-b border-gray-200 dark:border-gray-700 text-gray-600 dark:text-gray-300 overflow-hidden whitespace-nowrap text-ellipsis cursor-pointer select-none hover:bg-gray-100 dark:hover:bg-gray-700/50"
              :title="c" @click="toggleSort(ci)">
            {{ c }}<span v-if="sortCol === ci" class="text-blue-500">{{ sortDir === 1 ? ' ▲' : ' ▼' }}</span>
            <span class="absolute top-0 right-0 h-full w-1.5 cursor-col-resize hover:bg-blue-400/60" @click.stop @mousedown.stop.prevent="startResize(ci, $event)"/>
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-if="topPad > 0" :style="{height: topPad + 'px'}"/>
        <tr v-for="(row, i) in visibleRows" :key="start + i" class="hover:bg-gray-50 dark:hover:bg-gray-800/50" :style="{height: rowHeight + 'px'}">
          <td v-if="showIndex" class="px-2 border-b border-gray-100 dark:border-gray-800 text-gray-400 whitespace-nowrap">{{ start + i + 1 }}</td>
          <td v-for="(_c, ci) in columns" :key="ci" class="px-3 border-b border-gray-100 dark:border-gray-800 font-mono overflow-hidden whitespace-nowrap text-ellipsis"
              :class="row[ci] === null || row[ci] === undefined ? 'text-gray-400 italic' : 'text-gray-700 dark:text-gray-300'" :title="fmt(row[ci])">{{ fmt(row[ci]) }}</td>
        </tr>
        <tr v-if="bottomPad > 0" :style="{height: bottomPad + 'px'}"/>
        <tr v-if="rows.length === 0">
          <td :colspan="columns.length + (showIndex ? 1 : 0)" class="px-3 py-2 text-center text-gray-400">（0 行）</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import {computed, onBeforeUnmount, onMounted, ref, watch} from 'vue'

const props = withDefaults(defineProps<{
  columns: string[]
  rows: any[][]
  rowHeight?: number
  showIndex?: boolean
  buffer?: number
  maxHeight?: number
}>(), {rowHeight: 28, showIndex: true, buffer: 8})

const indexW = 48
const scroller = ref<HTMLElement>()
const scrollTop = ref(0)
const viewportH = ref(0)

const onScroll = () => {
  scrollTop.value = scroller.value?.scrollTop || 0
}

// ---- 列宽（内容启发式初始化，可拖拽调整） ----
const widths = ref<number[]>([])
const userResized = ref(false)
const estWidth = (name: string, ci: number): number => {
  let maxLen = (name || '').length
  const n = Math.min(props.rows.length, 50)
  for (let i = 0; i < n; i++) {
    const v = props.rows[i]?.[ci]
    const s = v === null || v === undefined ? 4 : String(v).length
    if (s > maxLen) {
      maxLen = s
    }
  }
  return Math.min(360, Math.max(80, Math.round(maxLen * 7.5 + 24)))
}
const recomputeWidths = () => {
  widths.value = props.columns.map((c, ci) => estWidth(c, ci))
}
watch(() => props.columns, () => {
  userResized.value = false
  recomputeWidths()
}, {immediate: true})
watch(() => props.rows.length, () => {
  if (!userResized.value) {
    recomputeWidths()
  }
})
const totalWidth = computed(() => (props.showIndex ? indexW : 0) + widths.value.reduce((a, b) => a + b, 0))

let resizing = -1
let startX = 0
let startW = 0
const onResizeMove = (e: MouseEvent) => {
  if (resizing < 0) {
    return
  }
  const w = Math.max(48, startW + (e.clientX - startX))
  const arr = [...widths.value]
  arr[resizing] = w
  widths.value = arr
}
const onResizeUp = () => {
  resizing = -1
  document.removeEventListener('mousemove', onResizeMove)
  document.removeEventListener('mouseup', onResizeUp)
}
const startResize = (ci: number, e: MouseEvent) => {
  resizing = ci
  startX = e.clientX
  startW = widths.value[ci]
  userResized.value = true
  document.addEventListener('mousemove', onResizeMove)
  document.addEventListener('mouseup', onResizeUp)
}

// ---- 排序（点击列头：升 → 降 → 取消） ----
const sortCol = ref(-1)
const sortDir = ref<1 | -1>(1)
const toggleSort = (ci: number) => {
  if (sortCol.value !== ci) {
    sortCol.value = ci
    sortDir.value = 1
  }
  else if (sortDir.value === 1) {
    sortDir.value = -1
  }
  else {
    sortCol.value = -1
  }
}
const compare = (a: any, b: any): number => {
  const an = a === null || a === undefined || a === ''
  const bn = b === null || b === undefined || b === ''
  if (an && bn) {
    return 0
  }
  if (an) {
    return 1
  }
  if (bn) {
    return -1
  }
  const na = Number(a)
  const nb = Number(b)
  if (!isNaN(na) && !isNaN(nb)) {
    return na - nb
  }
  return String(a).localeCompare(String(b))
}
const sortedRows = computed(() => {
  if (sortCol.value < 0) {
    return props.rows
  }
  const ci = sortCol.value
  const dir = sortDir.value
  return [...props.rows].sort((a, b) => compare(a[ci], b[ci]) * dir)
})
// 列变化时重置排序
watch(() => props.columns, () => {
  sortCol.value = -1
})

// ---- 虚拟滚动窗口 ----
const start = computed(() => Math.max(0, Math.floor(scrollTop.value / props.rowHeight) - props.buffer))
const end = computed(() => {
  const visible = Math.ceil((viewportH.value || 1) / props.rowHeight) + props.buffer * 2
  return Math.min(sortedRows.value.length, start.value + visible)
})
const visibleRows = computed(() => sortedRows.value.slice(start.value, end.value))
const topPad = computed(() => start.value * props.rowHeight)
const bottomPad = computed(() => (sortedRows.value.length - end.value) * props.rowHeight)

const fmt = (v: any) => {
  if (v === null || v === undefined) {
    return 'NULL'
  }
  if (typeof v === 'object') {
    return JSON.stringify(v)
  }
  return String(v)
}

let ro: ResizeObserver | null = null
onMounted(() => {
  if (!scroller.value) {
    return
  }
  viewportH.value = scroller.value.clientHeight
  ro = new ResizeObserver(() => {
    viewportH.value = scroller.value?.clientHeight || 0
  })
  ro.observe(scroller.value)
})
onBeforeUnmount(() => {
  ro?.disconnect()
  document.removeEventListener('mousemove', onResizeMove)
  document.removeEventListener('mouseup', onResizeUp)
})
</script>
