<template>
  <div ref="scroller" class="overflow-auto" :class="maxHeight ? '' : 'h-full'" :style="maxHeight ? {maxHeight: maxHeight + 'px'} : undefined" @scroll="onScroll">
    <table class="w-full border-collapse text-xs" style="table-layout: auto">
      <thead class="sticky top-0 z-10">
        <tr class="bg-gray-50 dark:bg-gray-800">
          <th v-if="showIndex" class="text-left font-semibold px-2 py-1.5 border-b border-gray-200 dark:border-gray-700 text-gray-400 w-12">#</th>
          <th v-for="(c, ci) in columns" :key="ci" :style="colStyle(ci)"
              class="relative text-left font-semibold px-3 py-1.5 border-b border-gray-200 dark:border-gray-700 text-gray-600 dark:text-gray-300 overflow-hidden whitespace-nowrap text-ellipsis cursor-pointer select-none hover:bg-gray-100 dark:hover:bg-gray-700/50"
              :title="c" @click="toggleSort(ci)">
            {{ c }}<span v-if="sortCol === ci" class="text-blue-500">{{ sortDir === 1 ? ' ▲' : ' ▼' }}</span>
            <span class="absolute top-0 right-0 h-full w-1.5 cursor-col-resize hover:bg-blue-400/60" @click.stop @mousedown.stop.prevent="startResize(ci, $event)"/>
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-if="topPad > 0" :style="{height: topPad + 'px'}"><td :colspan="colCount"/></tr>
        <tr v-for="(row, i) in visibleRows" :key="start + i" class="hover:bg-gray-50 dark:hover:bg-gray-800/50" :style="{height: rowHeight + 'px'}">
          <td v-if="showIndex" class="px-2 border-b border-gray-100 dark:border-gray-800 text-gray-400 whitespace-nowrap">{{ start + i + 1 }}</td>
          <td v-for="(_c, ci) in columns" :key="ci" :style="colStyle(ci)"
              class="px-3 border-b border-gray-100 dark:border-gray-800 font-mono overflow-hidden whitespace-nowrap text-ellipsis"
              :class="[
                row[ci] === null || row[ci] === undefined ? 'text-gray-400 italic' : 'text-gray-700 dark:text-gray-300',
                editable ? 'cursor-text' : ''
              ]"
              :title="editable ? t('view.editHint') : fmt(row[ci])"
              @dblclick="editable && beginEdit(row, ci)">
            <input v-if="editing && editing.row === row && editing.ci === ci"
                   ref="editInput"
                   v-model="editValue"
                   class="w-full bg-white dark:bg-gray-900 text-gray-800 dark:text-gray-100 border border-blue-400 rounded px-1 py-0 font-mono outline-none"
                   @keydown.enter.prevent="commitEdit"
                   @keydown.esc.prevent="cancelEdit"
                   @blur="commitEdit"/>
            <template v-else>{{ fmt(row[ci]) }}</template>
          </td>
        </tr>
        <tr v-if="bottomPad > 0" :style="{height: bottomPad + 'px'}"><td :colspan="colCount"/></tr>
        <tr v-if="rows.length === 0">
          <td :colspan="colCount" class="px-3 py-2 text-center text-gray-400">{{ t('view.zeroRows') }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import {computed, nextTick, onBeforeUnmount, onMounted, ref, watch} from 'vue'
import {useI18n} from 'vue-i18n'

const {t} = useI18n()

// ---- 行内编辑（editable 时双击单元格）----
const editing = ref<{ row: any[]; ci: number } | null>(null)
const editValue = ref('')
const editInput = ref<HTMLInputElement[] | HTMLInputElement>()
const beginEdit = async (row: any[], ci: number) => {
  const v = row[ci]
  editing.value = {row, ci}
  editValue.value = v === null || v === undefined ? '' : String(v)
  await nextTick()
  const el = Array.isArray(editInput.value) ? editInput.value[0] : editInput.value
  el?.focus()
  el?.select()
}
const cancelEdit = () => { editing.value = null }
// 依据原值类型对输入做轻量强转：原为数字且新值为数值 → 数字；原为 null 且清空 → 仍 null
const coerce = (raw: string, old: any): any => {
  if (raw === '' && (old === null || old === undefined)) {
    return null
  }
  if (typeof old === 'number' && raw.trim() !== '' && !isNaN(Number(raw))) {
    return Number(raw)
  }
  return raw
}
const commitEdit = () => {
  const cur = editing.value
  if (!cur) {
    return
  }
  editing.value = null
  const oldValue = cur.row[cur.ci]
  const newValue = coerce(editValue.value, oldValue)
  // 值未变化则不触发回写
  if (newValue === oldValue) {
    return
  }
  emit('editCell', {row: [...cur.row], ci: cur.ci, oldValue, newValue})
}

const props = withDefaults(defineProps<{
  columns: string[]
  rows: any[][]
  rowHeight?: number
  showIndex?: boolean
  buffer?: number
  maxHeight?: number
  editable?: boolean
}>(), {rowHeight: 28, showIndex: true, buffer: 8, editable: false})

const emit = defineEmits<{ editCell: [payload: { row: any[]; ci: number; oldValue: any; newValue: any }] }>()

const scroller = ref<HTMLElement>()
const scrollTop = ref(0)
const viewportH = ref(0)

const onScroll = () => {
  scrollTop.value = scroller.value?.scrollTop || 0
}

const colCount = computed(() => props.columns.length + (props.showIndex ? 1 : 0))

// ---- 列宽：默认自动（按内容），仅被拖拽过的列施加显式宽度 ----
const widths = ref<(number | undefined)[]>([])
watch(() => props.columns, () => {
  widths.value = props.columns.map(() => undefined)
}, {immediate: true})
const colStyle = (ci: number) => {
  const w = widths.value[ci]
  return w != null ? {width: w + 'px', maxWidth: w + 'px'} : undefined
}

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
  // 从当前渲染宽度起拖（自动列也能接管）
  const th = (e.target as HTMLElement).parentElement as HTMLElement | null
  startW = widths.value[ci] ?? (th ? th.getBoundingClientRect().width : 120)
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
