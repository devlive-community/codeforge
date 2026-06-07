<template>
  <div class="flex h-full min-h-0">
    <!-- 配置侧栏 -->
    <div class="w-56 flex-shrink-0 border-r border-gray-200 dark:border-gray-700 flex flex-col min-h-0 bg-gray-50 dark:bg-gray-800/40 overflow-auto">
      <!-- 图表类型 -->
      <div class="px-3 py-2 border-b border-gray-200 dark:border-gray-700">
        <div class="text-[11px] text-gray-400 mb-1">图表类型</div>
        <Select v-model="chartType" :options="chartTypes" :button-classes="['!py-1', '!px-2.5', 'text-xs', '!rounded-md']"/>
      </div>

      <!-- 可用字段 -->
      <div class="px-3 py-2 border-b border-gray-200 dark:border-gray-700">
        <div class="text-[11px] text-gray-400 mb-1.5">字段（拖拽 / 双击）</div>
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
      <div v-if="!isScatter" class="px-3 py-2 border-b border-gray-200 dark:border-gray-700"
           @dragover.prevent="dragOver = 'dim'" @dragleave="dragOver = ''" @drop.prevent="onDrop('dim')">
        <div class="text-[11px] text-gray-400 mb-1.5">维度（首个为分类轴，其余分组）</div>
        <div class="min-h-[28px] rounded border border-dashed p-1 flex flex-wrap gap-1 transition-colors"
             :class="dragOver === 'dim' ? 'border-blue-400 bg-blue-50/50 dark:bg-blue-900/20' : 'border-gray-300 dark:border-gray-600'">
          <span v-for="(d, i) in dimensions" :key="d" class="inline-flex items-center gap-1 px-2 py-0.5 rounded text-xs"
                :class="i === 0 ? 'bg-blue-100 dark:bg-blue-900/40 text-blue-700 dark:text-blue-300' : 'bg-indigo-100 dark:bg-indigo-900/40 text-indigo-700 dark:text-indigo-300'">
            <span v-if="i === 0" class="text-[9px] opacity-70">轴</span>
            {{ d }}
            <X class="w-3 h-3 cursor-pointer hover:text-red-500" @click="removeDim(d)"/>
          </span>
          <span v-if="dimensions.length === 0" class="text-[11px] text-gray-400 px-1 py-0.5">拖入分类列（可多个）</span>
        </div>
      </div>

      <!-- 指标 -->
      <div v-if="!isScatter" class="px-3 py-2 border-b border-gray-200 dark:border-gray-700"
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
      </div>

      <!-- 散点图配置：X / Y / 分组 -->
      <template v-if="isScatter">
        <div v-for="z in scatterZones" :key="z.key" class="px-3 py-2 border-b border-gray-200 dark:border-gray-700"
             @dragover.prevent="dragOver = z.key" @dragleave="dragOver = ''" @drop.prevent="onDropScatter(z.key)">
          <div class="text-[11px] text-gray-400 mb-1.5">{{ z.label }}</div>
          <div class="min-h-[28px] rounded border border-dashed p-1 flex flex-wrap gap-1 transition-colors"
               :class="dragOver === z.key ? 'border-blue-400 bg-blue-50/50 dark:bg-blue-900/20' : 'border-gray-300 dark:border-gray-600'">
            <span v-if="z.model.value" class="inline-flex items-center gap-1 px-2 py-0.5 rounded text-xs"
                  :class="z.key === 'group' ? 'bg-indigo-100 dark:bg-indigo-900/40 text-indigo-700 dark:text-indigo-300' : 'bg-emerald-100 dark:bg-emerald-900/40 text-emerald-700 dark:text-emerald-300'">
              {{ z.model.value }}
              <X class="w-3 h-3 cursor-pointer hover:text-red-500" @click="z.model.value = ''"/>
            </span>
            <span v-else class="text-[11px] text-gray-400 px-1 py-0.5">{{ z.hint }}</span>
          </div>
        </div>
      </template>

      <!-- 显示选项 -->
      <div v-if="!isScatter" class="px-3 py-2 space-y-2">
        <div v-if="!isHeatmap" class="flex items-center justify-between">
          <span class="text-[11px] text-gray-400">排序</span>
          <Select v-model="sortOrder" :options="sortOptions" :button-classes="['!py-0.5', '!px-1.5', 'text-[11px]', '!rounded']" class="w-24"/>
        </div>
        <div v-if="!isHeatmap" class="flex items-center justify-between">
          <span class="text-[11px] text-gray-400">显示前 N 项</span>
          <input v-model.number="topN" type="number" min="0" placeholder="全部"
                 class="w-16 px-1.5 py-0.5 text-[11px] rounded border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-200"/>
        </div>
        <div class="flex flex-wrap gap-x-3 gap-y-1 text-[11px] text-gray-500 dark:text-gray-400 pt-0.5">
          <label class="flex items-center gap-1.5 cursor-pointer"><input v-model="showLabel" type="checkbox" class="accent-blue-500"/>数值标签</label>
          <label v-if="chartType === 'bar'" class="flex items-center gap-1.5 cursor-pointer"><input v-model="horizontal" type="checkbox" class="accent-blue-500"/>横向</label>
          <label v-if="isLineLike" class="flex items-center gap-1.5 cursor-pointer"><input v-model="smooth" type="checkbox" class="accent-blue-500"/>平滑</label>
          <label v-if="(chartType === 'bar' || isLineLike) && shaped.series.length > 1" class="flex items-center gap-1.5 cursor-pointer"><input v-model="stacked" type="checkbox" class="accent-blue-500"/>堆叠</label>
          <label v-if="chartType === 'pie'" class="flex items-center gap-1.5 cursor-pointer"><input v-model="ring" type="checkbox" class="accent-blue-500"/>环形</label>
          <label v-if="chartType === 'radar'" class="flex items-center gap-1.5 cursor-pointer"><input v-model="radarFill" type="checkbox" class="accent-blue-500"/>填充</label>
        </div>
        <p v-if="chartType === 'pie' || chartType === 'funnel'" class="text-[10px] text-gray-400 leading-snug">取首个维度作分类、首个指标作数值</p>
        <p v-if="isHeatmap" class="text-[10px] text-gray-400 leading-snug">前两个维度作 X/Y 轴、首个指标作热力值</p>
      </div>
    </div>

    <!-- 图表区 -->
    <div class="flex-1 min-w-0 min-h-0 p-3">
      <div v-if="!ready" class="h-full flex flex-col items-center justify-center text-gray-400 gap-2">
        <BarChart3 class="w-8 h-8"/>
        <p class="text-xs">{{ emptyHint }}</p>
      </div>
      <BarChart v-else-if="chartType === 'bar'" :categories="shaped.categories" :series="shaped.series"
                :horizontal="horizontal" :stacked="stacked" :show-label="showLabel"/>
      <LineChart v-else-if="isLineLike" :categories="shaped.categories" :series="shaped.series"
                 :area="chartType === 'area'" :smooth="smooth" :stacked="stacked" :show-label="showLabel"/>
      <PieChart v-else-if="chartType === 'pie'" :data="pieData" :ring="ring" :show-label="showLabel"/>
      <ScatterChart v-else-if="isScatter" :series="scatterSeries" :x-name="xField" :y-name="yField"/>
      <RadarChart v-else-if="chartType === 'radar'" :categories="shaped.categories" :series="shaped.series" :area="radarFill" :show-label="showLabel"/>
      <FunnelChart v-else-if="chartType === 'funnel'" :data="pieData" :show-label="showLabel"/>
      <HeatmapChart v-else-if="isHeatmap" :x-cats="heatmap.xCats" :y-cats="heatmap.yCats" :cells="heatmap.cells" :min="heatmap.min" :max="heatmap.max" :show-label="showLabel"/>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, ref, watch} from 'vue'
import {BarChart3, Hash, Type, X} from 'lucide-vue-next'
import Select from '../../ui/Select.vue'
import BarChart from './BarChart.vue'
import LineChart from './LineChart.vue'
import PieChart from './PieChart.vue'
import ScatterChart from './ScatterChart.vue'
import RadarChart from './RadarChart.vue'
import FunnelChart from './FunnelChart.vue'
import HeatmapChart from './HeatmapChart.vue'
import {AGG_LABELS, type AggKind, heatmapData, isNumericColumn, pivot, scatterData, sortAndLimit} from './shape'

const props = defineProps<{
  columns: string[]
  rows: any[][]
}>()

const chartTypes = [
  {value: 'bar', label: '柱状图'},
  {value: 'line', label: '折线图'},
  {value: 'area', label: '面积图'},
  {value: 'pie', label: '饼图'},
  {value: 'scatter', label: '散点图'},
  {value: 'radar', label: '雷达图'},
  {value: 'funnel', label: '漏斗图'},
  {value: 'heatmap', label: '热力图'}
]
const chartType = ref('bar')
const isLineLike = computed(() => chartType.value === 'line' || chartType.value === 'area')
const isScatter = computed(() => chartType.value === 'scatter')
const isHeatmap = computed(() => chartType.value === 'heatmap')
const emptyHint = computed(() => {
  if (isScatter.value) {
    return '拖入「X 指标」和「Y 指标」生成图表'
  }
  if (isHeatmap.value) {
    return '拖入两个维度和一个指标生成图表'
  }
  return '拖入「维度」和「指标」生成图表'
})

const aggOptions = (Object.keys(AGG_LABELS) as AggKind[]).map(k => ({value: k, label: AGG_LABELS[k]}))
const agg = ref<AggKind>('sum')

const sortOptions = [
  {value: 'none', label: '原始顺序'},
  {value: 'desc', label: '降序'},
  {value: 'asc', label: '升序'}
]
const sortOrder = ref<'none' | 'asc' | 'desc'>('none')
const topN = ref<number>(0)

const dimensions = ref<string[]>([])
const metrics = ref<string[]>([])
// 散点图字段
const xField = ref('')
const yField = ref('')
const groupField = ref('')
const scatterZones = [
  {key: 'x' as const, label: 'X 指标（数值）', hint: '拖入数值列', model: xField},
  {key: 'y' as const, label: 'Y 指标（数值）', hint: '拖入数值列', model: yField},
  {key: 'group' as const, label: '分组（可选）', hint: '拖入分类列', model: groupField}
]
const horizontal = ref(false)
const stacked = ref(false)
const smooth = ref(false)
const ring = ref(false)
const radarFill = ref(true)
const showLabel = ref(false)
const dragOver = ref('')

const fields = computed(() => props.columns.map((name, i) => ({name, numeric: isNumericColumn(props.rows, i)})))

// 列变化（新查询）时，剔除已不存在的字段
watch(() => props.columns, (cols) => {
  dimensions.value = dimensions.value.filter(d => cols.includes(d))
  metrics.value = metrics.value.filter(m => cols.includes(m))
  for (const z of scatterZones) {
    if (z.model.value && !cols.includes(z.model.value)) {
      z.model.value = ''
    }
  }
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
    if (!dimensions.value.includes(name)) {
      dimensions.value = [...dimensions.value, name]
    }
  }
  else if (!metrics.value.includes(name)) {
    metrics.value = [...metrics.value, name]
  }
  dragField = ''
}

const onDropScatter = (zone: 'x' | 'y' | 'group') => {
  dragOver.value = ''
  const name = dragField
  if (!name || !props.columns.includes(name)) {
    return
  }
  const z = scatterZones.find(s => s.key === zone)!
  z.model.value = name
  dragField = ''
}

// 双击快速添加
const quickAdd = (f: { name: string; numeric: boolean }) => {
  if (isScatter.value) {
    if (f.numeric) {
      if (!xField.value) {
        xField.value = f.name
      }
      else if (!yField.value) {
        yField.value = f.name
      }
    }
    else if (!groupField.value) {
      groupField.value = f.name
    }
    return
  }
  if (f.numeric) {
    if (!metrics.value.includes(f.name)) {
      metrics.value = [...metrics.value, f.name]
    }
  }
  else if (!dimensions.value.includes(f.name)) {
    dimensions.value = [...dimensions.value, f.name]
  }
}

const removeDim = (d: string) => {
  dimensions.value = dimensions.value.filter(x => x !== d)
}
const removeMetric = (m: string) => {
  metrics.value = metrics.value.filter(x => x !== m)
}

const ready = computed(() => {
  if (isScatter.value) {
    return !!xField.value && !!yField.value
  }
  if (isHeatmap.value) {
    return dimensions.value.length >= 2 && metrics.value.length > 0
  }
  return dimensions.value.length > 0 && metrics.value.length > 0
})

const scatterSeries = computed(() => (isScatter.value && ready.value)
  ? scatterData({columns: props.columns, rows: props.rows}, xField.value, yField.value, groupField.value || undefined)
  : [])

const heatmap = computed(() => (isHeatmap.value && ready.value)
  ? heatmapData({columns: props.columns, rows: props.rows}, dimensions.value[0], dimensions.value[1], metrics.value[0], agg.value)
  : {xCats: [], yCats: [], cells: [] as [number, number, number][], min: 0, max: 0})

const shaped = computed(() => {
  if (!ready.value) {
    return {categories: [] as string[], series: [] as { name: string; data: (number | null)[] }[]}
  }
  const base = pivot({columns: props.columns, rows: props.rows}, dimensions.value, metrics.value, agg.value)
  return sortAndLimit(base, sortOrder.value, topN.value || 0)
})

// 饼图：取首个维度作扇区、首个指标(系列)作数值
const pieData = computed(() => {
  const s = shaped.value.series[0]
  if (!s) {
    return [] as { name: string; value: number }[]
  }
  return shaped.value.categories.map((name, i) => ({name, value: Number(s.data[i] ?? 0)}))
})
</script>
