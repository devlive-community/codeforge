<template>
  <div ref="scroller" class="overflow-auto h-full" @scroll="onScroll">
    <table class="w-full border-collapse text-xs">
      <thead class="sticky top-0 z-10">
        <tr class="bg-gray-50 dark:bg-gray-800">
          <th v-if="showIndex" class="text-left font-semibold px-2 py-1.5 border-b border-gray-200 dark:border-gray-700 text-gray-400 w-12">#</th>
          <th v-for="(c, ci) in columns" :key="ci" class="text-left font-semibold px-3 py-1.5 border-b border-gray-200 dark:border-gray-700 text-gray-600 dark:text-gray-300 whitespace-nowrap">{{ c }}</th>
        </tr>
      </thead>
      <tbody>
        <tr v-if="topPad > 0" :style="{height: topPad + 'px'}"/>
        <tr v-for="(row, i) in visibleRows" :key="start + i" class="hover:bg-gray-50 dark:hover:bg-gray-800/50" :style="{height: rowHeight + 'px'}">
          <td v-if="showIndex" class="px-2 border-b border-gray-100 dark:border-gray-800 text-gray-400 whitespace-nowrap">{{ start + i + 1 }}</td>
          <td v-for="(_c, ci) in columns" :key="ci" class="px-3 border-b border-gray-100 dark:border-gray-800 font-mono whitespace-nowrap"
              :class="row[ci] === null || row[ci] === undefined ? 'text-gray-400 italic' : 'text-gray-700 dark:text-gray-300'">{{ fmt(row[ci]) }}</td>
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
import {computed, onBeforeUnmount, onMounted, ref} from 'vue'

const props = withDefaults(defineProps<{
  columns: string[]
  rows: any[][]
  rowHeight?: number
  showIndex?: boolean
  buffer?: number
}>(), {rowHeight: 28, showIndex: true, buffer: 8})

const scroller = ref<HTMLElement>()
const scrollTop = ref(0)
const viewportH = ref(0)

const onScroll = () => {
  scrollTop.value = scroller.value?.scrollTop || 0
}

const start = computed(() => Math.max(0, Math.floor(scrollTop.value / props.rowHeight) - props.buffer))
const end = computed(() => {
  const visible = Math.ceil((viewportH.value || 1) / props.rowHeight) + props.buffer * 2
  return Math.min(props.rows.length, start.value + visible)
})
const visibleRows = computed(() => props.rows.slice(start.value, end.value))
const topPad = computed(() => start.value * props.rowHeight)
const bottomPad = computed(() => (props.rows.length - end.value) * props.rowHeight)

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
onBeforeUnmount(() => ro?.disconnect())
</script>
