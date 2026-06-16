<template>
  <div ref="el" class="w-full h-full"/>
</template>

<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref, watch} from 'vue'
import {useI18n} from 'vue-i18n'
import * as echarts from 'echarts/core'
import {BoxplotChart as EBox, ScatterChart as EScatter} from 'echarts/charts'
import {GridComponent, TooltipComponent} from 'echarts/components'
import {CanvasRenderer} from 'echarts/renderers'
import {useTheme} from '../../composables/useTheme'

echarts.use([EBox, EScatter, GridComponent, TooltipComponent, CanvasRenderer])

const props = defineProps<{
  categories: string[]
  boxes: number[][]
  outliers: [number, number][]
}>()

const {isDark} = useTheme()
const {t} = useI18n()
const el = ref<HTMLElement>()
let chart: echarts.ECharts | null = null

const buildOption = (): echarts.EChartsCoreOption => {
  const text = isDark.value ? '#d1d5db' : '#374151'
  const axisLine = isDark.value ? '#374151' : '#e5e7eb'
  return {
    tooltip: {trigger: 'item'},
    grid: {left: 8, right: 16, top: 12, bottom: 8, containLabel: true},
    xAxis: {type: 'category', data: props.categories, axisLabel: {color: text, rotate: props.categories.length > 8 ? 30 : 0}, axisLine: {lineStyle: {color: axisLine}}},
    yAxis: {type: 'value', axisLabel: {color: text}, splitLine: {lineStyle: {color: axisLine}}},
    series: [
      {name: t('chart.boxName'), type: 'boxplot', data: props.boxes, itemStyle: {borderColor: '#3b82f6'}},
      {name: t('chart.outlierName'), type: 'scatter', data: props.outliers, symbolSize: 6, itemStyle: {color: '#ef4444'}}
    ]
  }
}

const render = () => chart?.setOption(buildOption(), true)

let ro: ResizeObserver | null = null
onMounted(() => {
  if (!el.value) {
    return
  }
  chart = echarts.init(el.value, undefined, {renderer: 'canvas'})
  render()
  ro = new ResizeObserver(() => chart?.resize())
  ro.observe(el.value)
})

watch(() => [props.categories, props.boxes, props.outliers, isDark.value], render, {deep: true})

onBeforeUnmount(() => {
  ro?.disconnect()
  chart?.dispose()
  chart = null
})
</script>
