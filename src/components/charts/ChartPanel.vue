<template>
  <div class="flex h-full min-h-0 overflow-hidden">
    <!-- 配置侧栏 -->
    <div class="w-56 flex-shrink-0 border-r border-gray-200 dark:border-gray-700 flex flex-col min-h-0 h-full bg-gray-50 dark:bg-gray-800/40">
      <!-- 图表类型（固定在顶部）-->
      <div class="flex-shrink-0 px-3 py-2 border-b border-gray-200 dark:border-gray-700">
        <div class="flex items-center justify-between mb-1">
          <span class="text-[11px] text-gray-400">{{ t('chart.type') }}</span>
          <div class="flex items-center gap-1">
            <button class="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[11px] text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" :title="t('chart.savePresetTitle')" @click="savingPreset = !savingPreset">
              <Star class="w-3 h-3"/>{{ t('chart.savePreset') }}
            </button>
            <button class="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[11px] text-violet-500 hover:bg-violet-50 dark:hover:bg-violet-900/30 cursor-pointer" :title="t('chart.aiConfigTitle')" @click="toggleAi">
              <Sparkles class="w-3 h-3"/>{{ t('chart.aiConfig') }}
            </button>
          </div>
        </div>
        <Select v-model="chartType" :options="chartTypes" searchable :button-classes="['!py-1', '!px-2.5', 'text-xs', '!rounded-md']"/>
        <!-- 保存预设输入 -->
        <div v-if="savingPreset" class="mt-1.5 flex items-center gap-1">
          <input v-model="presetName" type="text" :placeholder="t('chart.presetNamePlaceholder')" autofocus
                 class="flex-1 min-w-0 px-2 py-0.5 text-[11px] rounded border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 text-gray-700 dark:text-gray-200 focus:outline-none focus:border-blue-400"
                 @keydown.enter="confirmSavePreset" @keydown.esc="savingPreset = false"/>
          <button class="px-2 py-0.5 text-[11px] rounded bg-blue-500 text-white hover:bg-blue-600 disabled:opacity-50 cursor-pointer" :disabled="!presetName.trim()" @click="confirmSavePreset">{{ t('chart.save') }}</button>
          <button class="px-1.5 py-0.5 text-[11px] rounded text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="savingPreset = false">{{ t('chart.cancel') }}</button>
        </div>
        <!-- 预设列表 -->
        <div v-if="presets.length" class="mt-1.5 flex flex-wrap gap-1">
          <span v-for="p in presets" :key="p.name"
                class="group inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[10px] bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-300 cursor-pointer hover:bg-blue-100 dark:hover:bg-blue-900/40"
                :title="t('chart.applyPreset', { name: p.name })" @click="applyPreset(p.name)">
            {{ p.name }}
            <X class="w-2.5 h-2.5 opacity-0 group-hover:opacity-100 hover:text-red-500" @click.stop="deletePreset(p.name)"/>
          </span>
        </div>
        <div v-if="aiOpen" class="mt-2 p-2 rounded border border-violet-200 dark:border-violet-800 bg-violet-50/40 dark:bg-violet-900/10">
          <textarea v-model="aiPrompt" rows="2" :disabled="aiLoading"
                    :placeholder="t('chart.aiPlaceholder')"
                    class="w-full px-2 py-1 text-[11px] rounded border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 text-gray-700 dark:text-gray-200 resize-none focus:outline-none focus:border-violet-400"
                    @keydown.meta.enter="aiGenerate" @keydown.ctrl.enter="aiGenerate"/>
          <div v-if="aiError" class="mt-1 text-[10px] text-red-500 whitespace-pre-wrap">{{ aiError }}</div>
          <div class="mt-1.5 flex justify-end">
            <button class="inline-flex items-center gap-1 px-2.5 py-0.5 rounded bg-violet-500 text-white text-[11px] hover:bg-violet-600 disabled:opacity-50 cursor-pointer"
                    :disabled="aiLoading || !aiPrompt.trim()" @click="aiGenerate">
              <RefreshCw v-if="aiLoading" class="w-3 h-3 animate-spin"/>
              <Sparkles v-else class="w-3 h-3"/>
              {{ aiLoading ? t('chart.generating') : t('chart.generate') }}
            </button>
          </div>
        </div>
      </div>

      <!-- 可用字段（独立滚动）-->
      <div class="flex-1 min-h-0 overflow-y-auto px-3 py-2 border-b border-gray-200 dark:border-gray-700">
        <div class="text-[11px] text-gray-400 mb-1.5">{{ t('chart.fields') }}</div>
        <div class="flex flex-wrap gap-1.5">
          <div v-for="f in fields" :key="f.name" draggable="true"
               class="inline-flex items-center gap-1 px-2 py-1 rounded border text-xs cursor-pointer active:cursor-grabbing select-none bg-white dark:bg-gray-800 border-gray-200 dark:border-gray-700 hover:border-blue-400"
               :title="t('chart.fieldTitle')"
               @dragstart="onDragStart($event, f.name)"
               @click="openFieldMenu(f, $event)"
               @dblclick="dblAdd(f)">
            <component :is="f.numeric ? Hash : Type" class="w-3 h-3" :class="f.numeric ? 'text-emerald-500' : 'text-amber-500'"/>
            {{ f.name }}
          </div>
          <div v-if="fields.length === 0" class="text-xs text-gray-400">{{ t('chart.noColumns') }}</div>
        </div>
      </div>

      <!-- 维度/指标/散点/选项：固定在下方始终可见 -->
      <div class="flex-shrink-0 overflow-y-auto max-h-[60%] border-t border-gray-200 dark:border-gray-700">
      <!-- 维度 -->
      <div v-if="meta.layout === 'dims' && meta.dimsZone" class="px-3 py-2 border-b border-gray-200 dark:border-gray-700"
           @dragover.prevent="dragOver = 'dim'" @dragleave="dragOver = ''" @drop.prevent="onDrop('dim')">
        <div class="text-[11px] text-gray-400 mb-1.5">{{ meta.needDims === 0 ? t('chart.dimsOptional') : t('chart.dimsAxis') }}</div>
        <div class="min-h-[28px] rounded border border-dashed p-1 flex flex-wrap gap-1 transition-colors"
             :class="dragOver === 'dim' ? 'border-blue-400 bg-blue-50/50 dark:bg-blue-900/20' : 'border-gray-300 dark:border-gray-600'">
          <span v-for="(d, i) in dimensions" :key="d" class="inline-flex items-center gap-1 px-2 py-0.5 rounded text-xs"
                :class="i === 0 ? 'bg-blue-100 dark:bg-blue-900/40 text-blue-700 dark:text-blue-300' : 'bg-indigo-100 dark:bg-indigo-900/40 text-indigo-700 dark:text-indigo-300'">
            <span v-if="i === 0 && meta.needDims > 0" class="text-[9px] opacity-70">{{ t('chart.axis') }}</span>
            {{ d }}
            <X class="w-3 h-3 cursor-pointer hover:text-red-500" @click="removeDim(d)"/>
          </span>
          <span v-if="dimensions.length === 0" class="text-[11px] text-gray-400 px-1 py-0.5">{{ t('chart.dropDims') }}</span>
        </div>
      </div>

      <!-- 指标 -->
      <div v-if="meta.layout === 'dims'" class="px-3 py-2 border-b border-gray-200 dark:border-gray-700"
           @dragover.prevent="dragOver = 'metric'" @dragleave="dragOver = ''" @drop.prevent="onDrop('metric')">
        <div class="text-[11px] text-gray-400 mb-1.5 flex items-center justify-between">
          <span>{{ t('chart.metricAxis') }}</span>
          <Select v-if="meta.usesAgg" v-model="agg" :options="aggOptions" :button-classes="['!py-0.5', '!px-1.5', 'text-[11px]', '!rounded']" class="w-20"/>
        </div>
        <div class="min-h-[28px] rounded border border-dashed p-1 flex flex-wrap gap-1 transition-colors"
             :class="dragOver === 'metric' ? 'border-blue-400 bg-blue-50/50 dark:bg-blue-900/20' : 'border-gray-300 dark:border-gray-600'">
          <span v-for="m in metrics" :key="m" class="inline-flex items-center gap-1 px-2 py-0.5 rounded text-xs bg-emerald-100 dark:bg-emerald-900/40 text-emerald-700 dark:text-emerald-300">
            {{ m }}
            <X class="w-3 h-3 cursor-pointer hover:text-red-500" @click="removeMetric(m)"/>
          </span>
          <span v-if="metrics.length === 0" class="text-[11px] text-gray-400 px-1 py-0.5">{{ t('chart.dropMetric') }}</span>
        </div>
      </div>

      <!-- 散点图配置：X / Y / 分组 -->
      <template v-if="meta.layout === 'scatter'">
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

      <!-- 字段映射说明 -->
      <p v-if="meta.note" class="px-3 py-2 text-[10px] text-gray-400 leading-snug border-b border-gray-200 dark:border-gray-700">{{ t(`chart.note.${chartType}`) }}</p>

      <!-- 显示选项 -->
      <div v-if="hasOptions" class="px-3 py-2 space-y-2">
        <div v-if="meta.sortable" class="flex items-center justify-between">
          <span class="text-[11px] text-gray-400">{{ t('chart.sort') }}</span>
          <Select v-model="sortOrder" :options="sortOptions" :button-classes="['!py-0.5', '!px-1.5', 'text-[11px]', '!rounded']" class="w-24"/>
        </div>
        <div v-if="meta.sortable" class="flex items-center justify-between">
          <span class="text-[11px] text-gray-400">{{ t('chart.topN') }}</span>
          <input v-model.number="topN" type="number" min="0" :placeholder="t('chart.all')"
                 class="w-16 px-1.5 py-0.5 text-[11px] rounded border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-200"/>
        </div>
        <div class="flex flex-wrap gap-x-3 gap-y-1 text-[11px] text-gray-500 dark:text-gray-400 pt-0.5">
          <label v-if="supportsLabel" class="flex items-center gap-1.5 cursor-pointer"><input v-model="showLabel" type="checkbox" class="accent-blue-500"/>{{ t('chart.labelValue') }}</label>
          <label v-if="chartType === 'bar'" class="flex items-center gap-1.5 cursor-pointer"><input v-model="horizontal" type="checkbox" class="accent-blue-500"/>{{ t('chart.horizontal') }}</label>
          <label v-if="isLineLike" class="flex items-center gap-1.5 cursor-pointer"><input v-model="smooth" type="checkbox" class="accent-blue-500"/>{{ t('chart.smooth') }}</label>
          <label v-if="(chartType === 'bar' || isLineLike || chartType === 'polarBar') && shaped.series.length > 1" class="flex items-center gap-1.5 cursor-pointer"><input v-model="stacked" type="checkbox" class="accent-blue-500"/>{{ t('chart.stacked') }}</label>
          <label v-if="chartType === 'pie'" class="flex items-center gap-1.5 cursor-pointer"><input v-model="ring" type="checkbox" class="accent-blue-500"/>{{ t('chart.ring') }}</label>
          <label v-if="chartType === 'radar'" class="flex items-center gap-1.5 cursor-pointer"><input v-model="radarFill" type="checkbox" class="accent-blue-500"/>{{ t('chart.fill') }}</label>
          <label v-if="chartType === 'combo' && shaped.series.length > 1" class="flex items-center gap-1.5 cursor-pointer"><input v-model="dualAxis" type="checkbox" class="accent-blue-500"/>{{ t('chart.dualY') }}</label>
        </div>
      </div>
      </div>
    </div>

    <!-- 图表区 -->
    <div ref="chartHost" class="relative flex-1 min-w-0 min-h-0 overflow-hidden p-3">
      <div v-if="ready" class="absolute top-2 right-2 z-20">
        <button class="p-1.5 rounded-md text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 bg-white/70 dark:bg-gray-800/70 hover:bg-gray-100 dark:hover:bg-gray-700 backdrop-blur cursor-pointer"
                :title="t('chart.exportCopy')" @click="menuOpen = !menuOpen">
          <Download class="w-3.5 h-3.5"/>
        </button>
        <template v-if="menuOpen">
          <div class="fixed inset-0 z-10" @click="menuOpen = false"/>
          <div class="absolute right-0 top-full mt-1 z-20 w-32 py-1 rounded-md border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 shadow-lg text-xs">
            <button v-for="item in exportItems" :key="item.label" class="w-full flex items-center gap-2 px-3 py-1.5 text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="runExport(item.fn)">
              <component :is="item.icon" class="w-3.5 h-3.5"/>{{ item.label }}
            </button>
          </div>
        </template>
      </div>
      <div v-if="!ready" class="h-full flex flex-col items-center justify-center text-gray-400 gap-2">
        <BarChart3 class="w-8 h-8"/>
        <p class="text-xs">{{ t(`chart.empty.${chartType}`) }}</p>
      </div>
      <BarChart v-else-if="chartType === 'bar'" :categories="shaped.categories" :series="shaped.series"
                :horizontal="horizontal" :stacked="stacked" :show-label="showLabel"/>
      <LineChart v-else-if="isLineLike" :categories="shaped.categories" :series="shaped.series"
                 :area="chartType === 'area'" :smooth="smooth" :stacked="stacked" :show-label="showLabel"/>
      <ComboChart v-else-if="chartType === 'combo'" :categories="shaped.categories" :series="shaped.series"
                  :dual-axis="dualAxis" :show-label="showLabel"/>
      <PieChart v-else-if="chartType === 'pie'" :data="pieData" :ring="ring" :show-label="showLabel"/>
      <PieChart v-else-if="chartType === 'rose'" :data="pieData" rose :show-label="showLabel"/>
      <ScatterChart v-else-if="chartType === 'scatter'" :series="scatterSeries" :x-name="xField" :y-name="yField"/>
      <ScatterChart v-else-if="chartType === 'effectScatter'" :series="scatterSeries" :x-name="xField" :y-name="yField" effect/>
      <RadarChart v-else-if="chartType === 'radar'" :categories="shaped.categories" :series="shaped.series" :area="radarFill" :show-label="showLabel"/>
      <FunnelChart v-else-if="chartType === 'funnel'" :data="pieData" :show-label="showLabel"/>
      <HeatmapChart v-else-if="chartType === 'heatmap'" :x-cats="heatmap.xCats" :y-cats="heatmap.yCats" :cells="heatmap.cells" :min="heatmap.min" :max="heatmap.max" :show-label="showLabel"/>
      <GaugeChart v-else-if="chartType === 'gauge'" :value="gauge.value" :max="gauge.max" :name="gauge.name"/>
      <SankeyChart v-else-if="chartType === 'sankey'" :nodes="sankey.nodes" :links="sankey.links"/>
      <SunburstChart v-else-if="chartType === 'sunburst'" :data="treeData" :show-label="showLabel"/>
      <TreemapChart v-else-if="chartType === 'treemap'" :data="treeData" :show-label="showLabel"/>
      <TreeChart v-else-if="chartType === 'tree'" :data="treeData" :show-label="showLabel"/>
      <BoxplotChart v-else-if="chartType === 'boxplot'" :categories="boxplot.categories" :boxes="boxplot.boxes" :outliers="boxplot.outliers"/>
      <CandlestickChart v-else-if="chartType === 'candlestick'" :categories="candle.categories" :values="candle.values"/>
      <ParallelChart v-else-if="chartType === 'parallel'" :axes="parallel.axes" :series="parallel.series"/>
      <ThemeRiverChart v-else-if="chartType === 'themeriver'" :data="river.data" :categories="river.categories"/>
      <CalendarChart v-else-if="chartType === 'calendar'" :data="calendar.data" :range="calendar.range" :max="calendar.max"/>
      <GraphChart v-else-if="chartType === 'graph'" :nodes="graph.nodes" :links="graph.links" :show-label="showLabel"/>
      <PolarBarChart v-else-if="chartType === 'polarBar'" :categories="shaped.categories" :series="shaped.series" :stacked="stacked"/>
      <PictorialBarChart v-else-if="chartType === 'pictorialBar'" :categories="shaped.categories" :series="shaped.series"/>
      <WordCloudChart v-else-if="chartType === 'wordcloud'" :data="pieData"/>
      <LiquidFillChart v-else-if="chartType === 'liquidFill'" :value="liquid.value" :name="liquid.name"/>
      <MapChart v-else-if="chartType === 'mapChina'" map-type="china" :data="pieData" :max="mapMax"/>
      <MapChart v-else-if="chartType === 'mapWorld'" map-type="world" :data="pieData" :max="mapMax"/>
    </div>

    <!-- 单击字段：选择添加到维度/指标（或散点的 X/Y/分组） -->
    <Teleport to="body">
      <template v-if="fieldMenu">
        <div class="fixed inset-0 z-[60]" @click="fieldMenu = null"/>
        <div class="fixed z-[61] min-w-28 py-1 rounded-md border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 shadow-lg text-xs"
             :style="{left: fieldMenu.left + 'px', top: fieldMenu.top + 'px'}">
          <div class="px-3 py-1 text-[11px] text-gray-400 border-b border-gray-200 dark:border-gray-700 truncate">{{ fieldMenu.name }}</div>
          <template v-if="meta.layout === 'scatter'">
            <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer text-gray-600 dark:text-gray-300" @click="pickTarget('x')">{{ t('chart.setX') }}</button>
            <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer text-gray-600 dark:text-gray-300" @click="pickTarget('y')">{{ t('chart.setY') }}</button>
            <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer text-gray-600 dark:text-gray-300" @click="pickTarget('group')">{{ t('chart.setGroup') }}</button>
          </template>
          <template v-else>
            <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer text-blue-600 dark:text-blue-300" @click="pickTarget('dim')">{{ t('chart.addDim') }}</button>
            <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer text-emerald-600 dark:text-emerald-300" @click="pickTarget('metric')">{{ t('chart.addMetric') }}</button>
          </template>
        </div>
      </template>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import {computed, ref, watch} from 'vue'
import {useI18n} from 'vue-i18n'
import {debounce} from 'lodash-es'
import * as echarts from 'echarts/core'
import {SVGRenderer} from 'echarts/renderers'
import {invoke} from '@tauri-apps/api/core'
import {BarChart3, Copy, Download, FileDown, Image as ImageIcon, Hash, RefreshCw, Sparkles, Star, Type, X} from 'lucide-vue-next'
import {useTheme} from '../../composables/useTheme'
import {kvGetJSON, kvSetJSON} from '../../composables/useKvStore'
import {useAiConfig} from '../../composables/useAiConfig'
import {useToast} from '../../plugins/toast'
import {downloadCsv} from '../../utils/csv'
import Select from '../../ui/Select.vue'

// 仅用于导出矢量图：临时以 SVG 渲染器复刻当前图表
echarts.use([SVGRenderer])
import BarChart from './BarChart.vue'
import LineChart from './LineChart.vue'
import ComboChart from './ComboChart.vue'
import PieChart from './PieChart.vue'
import ScatterChart from './ScatterChart.vue'
import RadarChart from './RadarChart.vue'
import FunnelChart from './FunnelChart.vue'
import HeatmapChart from './HeatmapChart.vue'
import GaugeChart from './GaugeChart.vue'
import SankeyChart from './SankeyChart.vue'
import SunburstChart from './SunburstChart.vue'
import TreemapChart from './TreemapChart.vue'
import TreeChart from './TreeChart.vue'
import BoxplotChart from './BoxplotChart.vue'
import CandlestickChart from './CandlestickChart.vue'
import ParallelChart from './ParallelChart.vue'
import ThemeRiverChart from './ThemeRiverChart.vue'
import CalendarChart from './CalendarChart.vue'
import GraphChart from './GraphChart.vue'
import PolarBarChart from './PolarBarChart.vue'
import PictorialBarChart from './PictorialBarChart.vue'
import WordCloudChart from './WordCloudChart.vue'
import LiquidFillChart from './LiquidFillChart.vue'
import MapChart from './MapChart.vue'
import {AGG_LABELS, type AggKind, aggregateColumn, boxplotData, calendarData, candlestickData, graphData, heatmapData, hierarchy, isNumericColumn, parallelData, pivot, sankeyData, scatterData, sortAndLimit, themeRiverData} from './shape'

const props = defineProps<{
  columns: string[]
  rows: any[][]
}>()

const {t} = useI18n()

// 图表元数据：新增图表只需在此加一条 + 对应渲染/塑形
// label/empty/note 的文案统一存于 i18n 的 chart.types/empty/note.<value>，此处仅保留结构信息
interface ChartMeta {
  layout: 'dims' | 'scatter' // 配置布局
  needDims: number // 最少维度（dims 布局）
  needMetrics: number // 最少指标
  dimsZone?: boolean // 是否显示维度区（默认 true）
  usesAgg?: boolean // 是否使用聚合方式（默认 true）
  sortable?: boolean // 是否显示 排序/TopN
  note?: boolean // 是否有字段映射说明（文案见 chart.note.<value>）
}

const CHART_META: Record<string, ChartMeta> = {
  bar: {layout: 'dims', needDims: 1, needMetrics: 1, dimsZone: true, usesAgg: true, sortable: true},
  line: {layout: 'dims', needDims: 1, needMetrics: 1, dimsZone: true, usesAgg: true, sortable: true},
  combo: {layout: 'dims', needDims: 1, needMetrics: 2, dimsZone: true, usesAgg: true, sortable: true, note: true},
  area: {layout: 'dims', needDims: 1, needMetrics: 1, dimsZone: true, usesAgg: true, sortable: true},
  pie: {layout: 'dims', needDims: 1, needMetrics: 1, dimsZone: true, usesAgg: true, sortable: true, note: true},
  scatter: {layout: 'scatter', needDims: 0, needMetrics: 0},
  radar: {layout: 'dims', needDims: 1, needMetrics: 1, dimsZone: true, usesAgg: true},
  funnel: {layout: 'dims', needDims: 1, needMetrics: 1, dimsZone: true, usesAgg: true, sortable: true, note: true},
  heatmap: {layout: 'dims', needDims: 2, needMetrics: 1, dimsZone: true, usesAgg: true, note: true},
  gauge: {layout: 'dims', needDims: 0, needMetrics: 1, dimsZone: false, usesAgg: true, note: true},
  sankey: {layout: 'dims', needDims: 2, needMetrics: 1, dimsZone: true, usesAgg: true, note: true},
  sunburst: {layout: 'dims', needDims: 1, needMetrics: 1, dimsZone: true, usesAgg: true, note: true},
  treemap: {layout: 'dims', needDims: 1, needMetrics: 1, dimsZone: true, usesAgg: true, note: true},
  tree: {layout: 'dims', needDims: 1, needMetrics: 1, dimsZone: true, usesAgg: true, note: true},
  boxplot: {layout: 'dims', needDims: 1, needMetrics: 1, dimsZone: true, usesAgg: false, note: true},
  candlestick: {layout: 'dims', needDims: 1, needMetrics: 4, dimsZone: true, usesAgg: false, note: true},
  parallel: {layout: 'dims', needDims: 0, needMetrics: 2, dimsZone: true, usesAgg: false, note: true},
  themeriver: {layout: 'dims', needDims: 2, needMetrics: 1, dimsZone: true, usesAgg: true, note: true},
  calendar: {layout: 'dims', needDims: 1, needMetrics: 1, dimsZone: true, usesAgg: true, note: true},
  rose: {layout: 'dims', needDims: 1, needMetrics: 1, dimsZone: true, usesAgg: true, sortable: true, note: true},
  effectScatter: {layout: 'scatter', needDims: 0, needMetrics: 0},
  graph: {layout: 'dims', needDims: 2, needMetrics: 1, dimsZone: true, usesAgg: true, note: true},
  polarBar: {layout: 'dims', needDims: 1, needMetrics: 1, dimsZone: true, usesAgg: true, sortable: true, note: true},
  pictorialBar: {layout: 'dims', needDims: 1, needMetrics: 1, dimsZone: true, usesAgg: true, sortable: true, note: true},
  wordcloud: {layout: 'dims', needDims: 1, needMetrics: 1, dimsZone: true, usesAgg: true, sortable: true, note: true},
  liquidFill: {layout: 'dims', needDims: 0, needMetrics: 1, dimsZone: false, usesAgg: true, note: true},
  mapChina: {layout: 'dims', needDims: 1, needMetrics: 1, dimsZone: true, usesAgg: true, note: true},
  mapWorld: {layout: 'dims', needDims: 1, needMetrics: 1, dimsZone: true, usesAgg: true, note: true}
}

const chartTypes = computed(() => Object.keys(CHART_META).map(value => ({value, label: t(`chart.types.${value}`)})))
const chartType = ref('bar')
const meta = computed(() => CHART_META[chartType.value])
const isLineLike = computed(() => chartType.value === 'line' || chartType.value === 'area')

const supportsLabel = computed(() => ['bar', 'line', 'area', 'combo', 'pie', 'rose', 'funnel', 'radar', 'heatmap', 'sunburst', 'treemap', 'tree'].includes(chartType.value))
const hasOptions = computed(() => meta.value.sortable || supportsLabel.value
  || ['bar', 'line', 'area', 'pie', 'radar'].includes(chartType.value))

const aggOptions = computed(() => (Object.keys(AGG_LABELS) as AggKind[]).map(k => ({value: k, label: t(`chart.agg.${k}`)})))
const agg = ref<AggKind>('sum')
// 聚合方式显示名（用于 仪表盘/水球图 标题）
const aggLabel = (k: AggKind) => t(`chart.agg.${k}`)

const sortOptions = computed(() => [
  {value: 'none', label: t('chart.sortNone')},
  {value: 'desc', label: t('chart.sortDesc')},
  {value: 'asc', label: t('chart.sortAsc')}
])
const sortOrder = ref<'none' | 'asc' | 'desc'>('none')
const topN = ref<number>(0)

const dimensions = ref<string[]>([])
const metrics = ref<string[]>([])
// 散点图字段
const xField = ref('')
const yField = ref('')
const groupField = ref('')
const scatterZones = computed(() => [
  {key: 'x' as const, label: t('chart.scatterX'), hint: t('chart.dropNumeric'), model: xField},
  {key: 'y' as const, label: t('chart.scatterY'), hint: t('chart.dropNumeric'), model: yField},
  {key: 'group' as const, label: t('chart.scatterGroup'), hint: t('chart.dropCategory'), model: groupField}
])
const horizontal = ref(false)
const stacked = ref(false)
const smooth = ref(false)
const ring = ref(false)
const radarFill = ref(true)
const showLabel = ref(false)
const dualAxis = ref(false)
const dragOver = ref('')

// 配置持久化：恢复上次选择，字段按当前列过滤
const CFG_KEY = 'chart.lastConfig'
const saved = kvGetJSON<Record<string, any>>(CFG_KEY, {})
if (saved.chartType && CHART_META[saved.chartType]) {
  chartType.value = saved.chartType
}
if (saved.agg) {
  agg.value = saved.agg
}
if (saved.sortOrder) {
  sortOrder.value = saved.sortOrder
}
if (typeof saved.topN === 'number') {
  topN.value = saved.topN
}
if (Array.isArray(saved.dimensions)) {
  dimensions.value = saved.dimensions.filter((d: string) => props.columns.includes(d))
}
if (Array.isArray(saved.metrics)) {
  metrics.value = saved.metrics.filter((m: string) => props.columns.includes(m))
}
if (saved.xField && props.columns.includes(saved.xField)) {
  xField.value = saved.xField
}
if (saved.yField && props.columns.includes(saved.yField)) {
  yField.value = saved.yField
}
if (saved.groupField && props.columns.includes(saved.groupField)) {
  groupField.value = saved.groupField
}
for (const k of ['horizontal', 'stacked', 'smooth', 'ring', 'showLabel', 'dualAxis'] as const) {
  if (typeof saved[k] === 'boolean') {
    ({horizontal, stacked, smooth, ring, showLabel, dualAxis}[k]).value = saved[k]
  }
}
if (typeof saved.radarFill === 'boolean') {
  radarFill.value = saved.radarFill
}

const buildConfig = () => ({
  chartType: chartType.value,
  agg: agg.value,
  sortOrder: sortOrder.value,
  topN: topN.value,
  dimensions: dimensions.value,
  metrics: metrics.value,
  xField: xField.value,
  yField: yField.value,
  groupField: groupField.value,
  horizontal: horizontal.value,
  stacked: stacked.value,
  smooth: smooth.value,
  ring: ring.value,
  radarFill: radarFill.value,
  showLabel: showLabel.value,
  dualAxis: dualAxis.value
})
// 套用配置（字段按当前列过滤）
const applyConfig = (cfg: Record<string, any>) => {
  if (cfg.chartType && CHART_META[cfg.chartType]) {
    chartType.value = cfg.chartType
  }
  if (cfg.agg) {
    agg.value = cfg.agg
  }
  if (cfg.sortOrder) {
    sortOrder.value = cfg.sortOrder
  }
  if (typeof cfg.topN === 'number') {
    topN.value = cfg.topN
  }
  dimensions.value = Array.isArray(cfg.dimensions) ? cfg.dimensions.filter((d: string) => props.columns.includes(d)) : []
  metrics.value = Array.isArray(cfg.metrics) ? cfg.metrics.filter((m: string) => props.columns.includes(m)) : []
  xField.value = cfg.xField && props.columns.includes(cfg.xField) ? cfg.xField : ''
  yField.value = cfg.yField && props.columns.includes(cfg.yField) ? cfg.yField : ''
  groupField.value = cfg.groupField && props.columns.includes(cfg.groupField) ? cfg.groupField : ''
  for (const k of ['horizontal', 'stacked', 'smooth', 'ring', 'radarFill', 'showLabel', 'dualAxis'] as const) {
    if (typeof cfg[k] === 'boolean') {
      ({horizontal, stacked, smooth, ring, radarFill, showLabel, dualAxis}[k]).value = cfg[k]
    }
  }
}

const persist = debounce(() => kvSetJSON(CFG_KEY, buildConfig()), 300)
watch([chartType, agg, sortOrder, topN, dimensions, metrics, xField, yField, groupField,
  horizontal, stacked, smooth, ring, radarFill, showLabel, dualAxis], persist, {deep: true})

// ---- 命名预设 ----
interface Preset { name: string; config: Record<string, any> }
const PRESETS_KEY = 'chart.presets'
const presets = ref<Preset[]>(kvGetJSON<Preset[]>(PRESETS_KEY, []))
const savingPreset = ref(false)
const presetName = ref('')
const confirmSavePreset = () => {
  const name = presetName.value.trim()
  if (!name) {
    return
  }
  const list = presets.value.filter(p => p.name !== name)
  list.push({name, config: buildConfig()})
  presets.value = list
  kvSetJSON(PRESETS_KEY, list)
  savingPreset.value = false
  presetName.value = ''
}
const applyPreset = (name: string) => {
  const p = presets.value.find(x => x.name === name)
  if (p) {
    applyConfig(p.config)
  }
}
const deletePreset = (name: string) => {
  presets.value = presets.value.filter(p => p.name !== name)
  kvSetJSON(PRESETS_KEY, presets.value)
}

const {isDark} = useTheme()
const toast = useToast()
const chartHost = ref<HTMLElement>()

// 取当前激活的 echarts 实例
const activeChart = (): echarts.ECharts | null => {
  const dom = chartHost.value?.querySelector('div[_echarts_instance_]') as HTMLElement | null
  return (dom ? echarts.getInstanceByDom(dom) : null) ?? null
}
const chartPng = (): string | null => {
  const inst = activeChart()
  return inst ? inst.getDataURL({type: 'png', pixelRatio: 2, backgroundColor: isDark.value ? '#111827' : '#ffffff'}) : null
}
// 导出当前图表为 PNG
const exportPng = () => {
  const url = chartPng()
  if (!url) {
    return
  }
  const a = document.createElement('a')
  a.href = url
  a.download = `chart-${chartType.value}-${Date.now()}.png`
  a.click()
}

// 复制图片到剪贴板
const copyImage = async () => {
  const url = chartPng()
  if (!url) {
    return
  }
  try {
    const blob = await (await fetch(url)).blob()
    await navigator.clipboard.write([new ClipboardItem({'image/png': blob})])
    toast.success(t('chart.imageCopied'))
  }
  catch {
    toast.error(t('chart.copyFailed'))
  }
}

// 导出当前图表为 SVG（临时用 SVG 渲染器复刻 option）
const exportSvg = () => {
  const inst = activeChart()
  if (!inst) {
    return
  }
  const tmp = document.createElement('div')
  tmp.style.cssText = `position:absolute;left:-99999px;top:0;width:${inst.getWidth()}px;height:${inst.getHeight()}px`
  document.body.appendChild(tmp)
  const svgInst = echarts.init(tmp, undefined, {renderer: 'svg'})
  svgInst.setOption(inst.getOption())
  svgInst.setOption({backgroundColor: isDark.value ? '#111827' : '#ffffff'})
  const svg = svgInst.renderToSVGString()
  svgInst.dispose()
  tmp.remove()
  const blob = new Blob([svg], {type: 'image/svg+xml;charset=utf-8'})
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `chart-${chartType.value}-${Date.now()}.svg`
  a.click()
  URL.revokeObjectURL(url)
}

// 导出图表底层数据为 CSV
const exportCsv = () => downloadCsv(props.columns, props.rows, `data-${Date.now()}.csv`)

const menuOpen = ref(false)
const exportItems = computed(() => [
  {label: t('chart.exportPng'), icon: Download, fn: exportPng},
  {label: t('chart.exportSvg'), icon: ImageIcon, fn: exportSvg},
  {label: t('chart.copyImage'), icon: Copy, fn: copyImage},
  {label: t('chart.exportCsv'), icon: FileDown, fn: exportCsv}
])
const runExport = (fn: () => void) => {
  fn()
  menuOpen.value = false
}

const fields = computed(() => props.columns.map((name, i) => ({name, numeric: isNumericColumn(props.rows, i)})))

// ---- AI 配图：自然语言 → 图表配置 ----
const {active: aiActive} = useAiConfig()
const aiOpen = ref(false)
const aiPrompt = ref('')
const aiLoading = ref(false)
const aiError = ref('')
const toggleAi = () => {
  aiOpen.value = !aiOpen.value
  aiError.value = ''
}
const aiGenerate = async () => {
  if (aiLoading.value || !aiPrompt.value.trim()) {
    return
  }
  if (!aiActive.value.apiKey?.trim()) {
    aiError.value = t('chart.noApiKey')
    return
  }
  aiLoading.value = true
  aiError.value = ''
  try {
    const cols = fields.value.map(f => `${f.name}:${f.numeric ? 'number' : 'text'}`).join(', ')
    const types = Object.keys(CHART_META).join(', ')
    const system = '你是数据可视化助手。根据可用列与用户需求，输出图表配置 JSON。\n'
      + `可用列(name:type): ${cols}\n`
      + `支持的图表类型(value): ${types}\n`
      + '严格输出 JSON：{"chartType":"...","dimensions":["列名"],"metrics":["列名"],"agg":"sum|count|avg|max|min"}。'
      + '只输出 JSON，不要解释或 Markdown。维度/指标必须使用上面的列名。'
    const text = await invoke<string>('ai_chat', {
      provider: aiActive.value.provider,
      baseUrl: aiActive.value.baseUrl,
      apiKey: aiActive.value.apiKey,
      model: aiActive.value.model,
      system,
      messages: [{role: 'user', content: aiPrompt.value.trim()}]
    })
    const cleaned = text.trim().replace(/^```(?:json)?\s*/i, '').replace(/```$/i, '').trim()
    const cfg = JSON.parse(cleaned)
    if (cfg.chartType && CHART_META[cfg.chartType]) {
      chartType.value = cfg.chartType
    }
    const dims = Array.isArray(cfg.dimensions) ? cfg.dimensions.filter((d: string) => props.columns.includes(d)) : []
    const mets = Array.isArray(cfg.metrics) ? cfg.metrics.filter((m: string) => props.columns.includes(m)) : []
    if (cfg.agg && (cfg.agg in AGG_LABELS)) {
      agg.value = cfg.agg
    }
    if (meta.value.layout === 'scatter') {
      xField.value = mets[0] || dims[0] || ''
      yField.value = mets[1] || ''
      groupField.value = dims[0] || ''
    }
    else {
      dimensions.value = dims
      metrics.value = mets
    }
    aiOpen.value = false
    aiPrompt.value = ''
  }
  catch (e: any) {
    aiError.value = t('chart.aiParseFail') + String(e?.message || e)
  }
  finally {
    aiLoading.value = false
  }
}

// 列变化（新查询）时，剔除已不存在的字段
watch(() => props.columns, (cols) => {
  dimensions.value = dimensions.value.filter(d => cols.includes(d))
  metrics.value = metrics.value.filter(m => cols.includes(m))
  for (const z of scatterZones.value) {
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
  const z = scatterZones.value.find(s => s.key === zone)!
  z.model.value = name
  dragField = ''
}

// 双击快速添加：数值列→指标，文本列→维度（散点：依次 X/Y/分组）
const quickAdd = (f: { name: string; numeric: boolean }) => {
  if (meta.value.layout === 'scatter') {
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

// 单击字段：在其右侧弹出菜单（延迟以便与双击区分）；双击则快速添加
const fieldMenu = ref<{ name: string; left: number; top: number } | null>(null)
let clickTimer: ReturnType<typeof setTimeout> | null = null
const openFieldMenu = (f: { name: string }, e: MouseEvent) => {
  const r = (e.currentTarget as HTMLElement).getBoundingClientRect()
  const left = Math.min(r.right + 6, window.innerWidth - 140)
  if (clickTimer) {
    clearTimeout(clickTimer)
  }
  clickTimer = setTimeout(() => {
    fieldMenu.value = {name: f.name, left, top: r.top}
    clickTimer = null
  }, 220)
}
const dblAdd = (f: { name: string; numeric: boolean }) => {
  if (clickTimer) {
    clearTimeout(clickTimer)
    clickTimer = null
  }
  quickAdd(f)
}
const pickTarget = (target: 'dim' | 'metric' | 'x' | 'y' | 'group') => {
  const name = fieldMenu.value?.name
  if (!name) {
    return
  }
  if (target === 'dim') {
    if (!dimensions.value.includes(name)) {
      dimensions.value = [...dimensions.value, name]
    }
  }
  else if (target === 'metric') {
    if (!metrics.value.includes(name)) {
      metrics.value = [...metrics.value, name]
    }
  }
  else if (target === 'x') {
    xField.value = name
  }
  else if (target === 'y') {
    yField.value = name
  }
  else if (target === 'group') {
    groupField.value = name
  }
  fieldMenu.value = null
}

const removeDim = (d: string) => {
  dimensions.value = dimensions.value.filter(x => x !== d)
}
const removeMetric = (m: string) => {
  metrics.value = metrics.value.filter(x => x !== m)
}

const ready = computed(() => {
  const m = meta.value
  if (m.layout === 'scatter') {
    return !!xField.value && !!yField.value
  }
  return dimensions.value.length >= m.needDims && metrics.value.length >= m.needMetrics
})

const table = computed(() => ({columns: props.columns, rows: props.rows}))

const scatterSeries = computed(() => (meta.value.layout === 'scatter' && ready.value)
  ? scatterData(table.value, xField.value, yField.value, groupField.value || undefined)
  : [])

const heatmap = computed(() => (chartType.value === 'heatmap' && ready.value)
  ? heatmapData(table.value, dimensions.value[0], dimensions.value[1], metrics.value[0], agg.value)
  : {xCats: [], yCats: [], cells: [] as [number, number, number][], min: 0, max: 0})

const sankey = computed(() => (chartType.value === 'sankey' && ready.value)
  ? sankeyData(table.value, dimensions.value, metrics.value[0], agg.value)
  : {nodes: [] as { name: string }[], links: [] as { source: string; target: string; value: number }[]})

// 层级数据：旭日图 / 矩形树图 / 树图共用
const treeData = computed(() => (['sunburst', 'treemap', 'tree'].includes(chartType.value) && ready.value)
  ? hierarchy(table.value, dimensions.value, metrics.value[0], agg.value)
  : [])

const boxplot = computed(() => (chartType.value === 'boxplot' && ready.value)
  ? boxplotData(table.value, dimensions.value[0], metrics.value[0])
  : {categories: [] as string[], boxes: [] as number[][], outliers: [] as [number, number][]})

const candle = computed(() => (chartType.value === 'candlestick' && ready.value)
  ? candlestickData(table.value, dimensions.value[0], metrics.value[0], metrics.value[1], metrics.value[2], metrics.value[3])
  : {categories: [] as string[], values: [] as number[][]})

const parallel = computed(() => (chartType.value === 'parallel' && ready.value)
  ? parallelData(table.value, metrics.value, dimensions.value[0])
  : {axes: [] as string[], series: [] as { name: string; data: number[][] }[]})

const river = computed(() => (chartType.value === 'themeriver' && ready.value)
  ? themeRiverData(table.value, dimensions.value[0], dimensions.value[1], metrics.value[0], agg.value)
  : {data: [] as [string, number, string][], categories: [] as string[]})

const calendar = computed(() => (chartType.value === 'calendar' && ready.value)
  ? calendarData(table.value, dimensions.value[0], metrics.value[0], agg.value)
  : {data: [] as [string, number][], range: new Date().getFullYear().toString() as [string, string] | string, max: 0})

const graph = computed(() => (chartType.value === 'graph' && ready.value)
  ? graphData(table.value, dimensions.value[0], dimensions.value[1], metrics.value[0], agg.value)
  : {nodes: [] as { name: string; value: number }[], links: [] as { source: string; target: string; value: number }[]})

// 水球图：聚合值 ÷ 该列最大值 → 0~1 比例
const liquid = computed(() => {
  if (!(chartType.value === 'liquidFill' && ready.value)) {
    return {value: 0, name: ''}
  }
  const m = metrics.value[0]
  const value = aggregateColumn(table.value, m, agg.value)
  const colMax = aggregateColumn(table.value, m, 'max')
  return {value: colMax > 0 ? value / colMax : 0, name: `${aggLabel(agg.value)}(${m})`}
})

// 仪表盘：首个指标聚合为单值，量程取略大于该值的“整”数
const niceMax = (v: number): number => {
  if (v <= 0) {
    return 100
  }
  const mag = Math.pow(10, Math.floor(Math.log10(v)))
  return Math.ceil((v * 1.1) / mag) * mag
}
const gauge = computed(() => {
  if (!(chartType.value === 'gauge' && ready.value)) {
    return {value: 0, max: 100, name: ''}
  }
  const m = metrics.value[0]
  const value = aggregateColumn(table.value, m, agg.value)
  return {value, max: niceMax(value), name: `${aggLabel(agg.value)}(${m})`}
})

const shaped = computed(() => {
  if (!ready.value || meta.value.layout !== 'dims') {
    return {categories: [] as string[], series: [] as { name: string; data: (number | null)[] }[]}
  }
  const base = pivot(table.value, dimensions.value, metrics.value, agg.value)
  return sortAndLimit(base, sortOrder.value, topN.value || 0)
})

// 饼图/漏斗图/词云/地图等：取首个维度作分类、首个指标(系列)作数值
const pieData = computed(() => {
  const s = shaped.value.series[0]
  if (!s) {
    return [] as { name: string; value: number }[]
  }
  return shaped.value.categories.map((name, i) => ({name, value: Number(s.data[i] ?? 0)}))
})
const mapMax = computed(() => pieData.value.reduce((m, d) => Math.max(m, d.value), 0))
</script>
