<template>
  <div class="flex h-full min-h-0">
    <!-- 配置侧栏 -->
    <div class="w-52 flex-shrink-0 border-r border-gray-200 dark:border-gray-700 flex flex-col min-h-0 bg-gray-50 dark:bg-gray-800/40">
      <!-- 图表类型 -->
      <div class="px-3 py-2 border-b border-gray-200 dark:border-gray-700">
        <div class="text-[11px] text-gray-400 mb-1">图表类型</div>
        <Select v-model="chartType" :options="chartTypes" :button-classes="['!py-1', '!px-2.5', 'text-xs', '!rounded-md']"/>
      </div>

      <!-- 可用字段 -->
      <div class="px-3 py-2 border-b border-gray-200 dark:border-gray-700 min-h-0 overflow-auto">
        <div class="text-[11px] text-gray-400 mb-1.5">字段（拖拽到下方）</div>
        <div class="flex flex-wrap gap-1.5">
          <div v-for="f in fields" :key="f.name" draggable="true"
               class="inline-flex items-center gap-1 px-2 py-1 rounded border text-xs cursor-grab active:cursor-grabbing select-none bg-white dark:bg-gray-800 border-gray-200 dark:border-gray-700 hover:border-blue-400"
               @dragstart="onDragStart($event, f.name)"
               @dblclick="quickAdd(f)">
            <component :is="f.numeric ? Hash : Type" class="w-3 h-3" :class="f.numeric ? 'text-emerald-500' : 'text-amber-500'"/>
            {{ f.name }}
          </div>
          <div v-if="fields.length === 0" class="text-xs text-gray-400">无可用列</div>
        </div>
      </div>

      <!-- 维度 -->
      <div class="px-3 py-2 border-b border-gray-200 dark:border-gray-700"
           @dragover.prevent="dragOver = 'dim'" @dragleave="dragOver = ''" @drop.prevent="onDrop('dim')">
        <div class="text-[11px] text-gray-400 mb-1.5">维度（分类轴）</div>
        <div class="min-h-[28px] rounded border border-dashed p-1 flex flex-wrap gap-1 transition-colors"
             :class="dragOver === 'dim' ? 'border-blue-400 bg-blue-50/50 dark:bg-blue-900/20' : 'border-gray-300 dark:border-gray-600'">
          <span v-if="dimension" class="inline-flex items-center gap-1 px-2 py-0.5 rounded text-xs bg-blue-100 dark:bg-blue-900/40 text-blue-700 dark:text-blue-300">
            {{ dimension }}
            <X class="w-3 h-3 cursor-pointer hover:text-red-500" @click="dimension = ''"/>
          </span>
          <span v-else class="text-[11px] text-gray-400 px-1 py-0.5">拖入一个分类列</span>
        </div>
      </div>

      <!-- 指标 -->
      <div class="px-3 py-2 flex-1 min-h-0 overflow-auto"
           @dragover.prevent="dragOver = 'metric'" @dragleave="dragOver = ''" @drop.prevent="onDrop('metric')">
        <div class="text-[11px] text-gray-400 mb-1.5 flex items-center justify-between">
          <span>指标（数值轴）</span>
          <Select v-model="agg" :options="aggOptions" :button-classes="['!py-0.5', '!px-1.5', 'text-[11px]', '!rounded']" class="w-20"/>
        </div>
        <div class="min-h-[28px] rounded border border-dashed p-1 flex flex-wrap gap-1 transition-colors"
             :class="dragOver === 'metric' ? 'border-blue-400 bg-blue-50/50 dark:bg-blue-900/20' : 'border-gray-300 dark:border-gray-600'">
          <span v-for="m in metrics" :key="m" class="inline-flex items-center gap-1 px-2 py-0.5 rounded text-xs bg-emerald-100 dark:bg-emerald-900/40 text-emerald-700 dark:text-emerald-300">
            {{ m }}
            <X class="w-3 h-3 cursor-pointer hover:text-red-500" @click="removeMetric(m)"/>
          </span>
          <span v-if="metrics.length === 0" class="text-[11px] text-gray-400 px-1 py-0.5">拖入一个或多个数值列</span>
        </div>

        <!-- 柱状图选项 -->
        <div v-if="chartType === 'bar'" class="mt-2 space-y-1 text-[11px] text-gray-500 dark:text-gray-400">
          <label class="flex items-center gap-1.5 cursor-pointer"><input v-model="horizontal" type="checkbox" class="accent-blue-500"/>横向</label>
          <label v-if="metrics.length > 1" class="flex items-center gap-1.5 cursor-pointer"><input v-model="stacked" type="checkbox" class="accent-blue-500"/>堆叠</label>
        </div>
      </div>
    </div>

    <!-- 图表区 -->
    <div class="flex-1 min-w-0 min-h-0 p-3">
      <div v-if="!ready" class="h-full flex flex-col items-center justify-center text-gray-400 gap-2">
        <BarChart3 class="w-8 h-8"/>
        <p class="text-xs">拖入「维度」和「指标」生成图表</p>
      </div>
      <BarChart v-else-if="chartType === 'bar'" :categories="shaped.categories" :series="shaped.series" :horizontal="horizontal" :stacked="stacked"/>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, ref, watch} from 'vue'
import {BarChart3, Hash, Type, X} from 'lucide-vue-next'
import Select from '../../ui/Select.vue'
import BarChart from './BarChart.vue'
import {AGG_LABELS, type AggKind, aggregateByDimension, isNumericColumn} from './shape'

const props = defineProps<{
  columns: string[]
  rows: any[][]
}>()

const chartTypes = [{value: 'bar', label: '柱状图'}]
const chartType = ref('bar')

const aggOptions = (Object.keys(AGG_LABELS) as AggKind[]).map(k => ({value: k, label: AGG_LABELS[k]}))
const agg = ref<AggKind>('sum')

const dimension = ref('')
const metrics = ref<string[]>([])
const horizontal = ref(false)
const stacked = ref(false)
const dragOver = ref('')

const fields = computed(() => props.columns.map((name, i) => ({name, numeric: isNumericColumn(props.rows, i)})))

// 列变化（新查询）时，清空已配置的、已不存在的字段
watch(() => props.columns, (cols) => {
  if (dimension.value && !cols.includes(dimension.value)) {
    dimension.value = ''
  }
  metrics.value = metrics.value.filter(m => cols.includes(m))
})

let dragField = ''
const onDragStart = (e: DragEvent, name: string) => {
  dragField = name
  e.dataTransfer?.setData('text/plain', name)
}

const onDrop = (zone: 'dim' | 'metric') => {
  dragOver.value = ''
  const name = dragField
  if (!name || !props.columns.includes(name)) {
    return
  }
  if (zone === 'dim') {
    dimension.value = name
  }
  else if (!metrics.value.includes(name)) {
    metrics.value = [...metrics.value, name]
  }
  dragField = ''
}

// 双击快速添加：数值列进指标，否则作维度
const quickAdd = (f: { name: string; numeric: boolean }) => {
  if (f.numeric) {
    if (!metrics.value.includes(f.name)) {
      metrics.value = [...metrics.value, f.name]
    }
  }
  else {
    dimension.value = f.name
  }
}

const removeMetric = (m: string) => {
  metrics.value = metrics.value.filter(x => x !== m)
}

const ready = computed(() => !!dimension.value && metrics.value.length > 0)
const shaped = computed(() => ready.value
  ? aggregateByDimension({columns: props.columns, rows: props.rows}, dimension.value, metrics.value, agg.value)
  : {categories: [], series: []})
</script>
