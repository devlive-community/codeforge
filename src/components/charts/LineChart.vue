<template>
  <div ref="el" class="w-full h-full"/>
</template>

<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref, watch} from 'vue'
import * as echarts from 'echarts/core'
import {LineChart as ELine} from 'echarts/charts'
import {GridComponent, LegendComponent, TooltipComponent} from 'echarts/components'
import {CanvasRenderer} from 'echarts/renderers'
import {useTheme} from '../../composables/useTheme'

echarts.use([ELine, GridComponent, LegendComponent, TooltipComponent, CanvasRenderer])

const props = defineProps<{
  categories: string[]
  series: { name: string; data: (number | null)[] }[]
  area?: boolean
  smooth?: boolean
  stacked?: boolean
  showLabel?: boolean
}>()

const {isDark} = useTheme()
const el = ref<HTMLElement>()
let chart: echarts.ECharts | null = null

const buildOption = (): echarts.EChartsCoreOption => {
  const text = isDark.value ? '#d1d5db' : '#374151'
  const axisLine = isDark.value ? '#374151' : '#e5e7eb'
  return {
    tooltip: {trigger: 'axis'},
    legend: {show: props.series.length > 1, top: 0, textStyle: {color: text}},
    grid: {left: 8, right: 16, top: props.series.length > 1 ? 28 : 12, bottom: 8, containLabel: true},
    xAxis: {type: 'category', boundaryGap: false, data: props.categories, axisLabel: {color: text}, axisLine: {lineStyle: {color: axisLine}}},
    yAxis: {type: 'value', axisLabel: {color: text}, splitLine: {lineStyle: {color: axisLine}}},
    series: props.series.map(s => ({
      name: s.name,
      type: 'line',
      smooth: props.smooth,
      stack: props.stacked ? 'total' : undefined,
      areaStyle: props.area ? (props.stacked ? {} : {opacity: 0.18}) : undefined,
      data: s.data,
      connectNulls: true,
      showSymbol: props.categories.length <= 40,
      label: {show: props.showLabel, position: 'top', color: text, fontSize: 10},
      emphasis: {focus: 'series'}
    }))
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

watch(() => [props.categories, props.series, props.area, props.smooth, props.stacked, props.showLabel, isDark.value], render, {deep: true})

onBeforeUnmount(() => {
  ro?.disconnect()
  chart?.dispose()
  chart = null
})
</script>
