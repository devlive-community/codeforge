<template>
  <div ref="el" class="w-full h-full"/>
</template>

<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref, watch} from 'vue'
import * as echarts from 'echarts/core'
import {BarChart as EBar, LineChart as ELine} from 'echarts/charts'
import {GridComponent, LegendComponent, TooltipComponent} from 'echarts/components'
import {CanvasRenderer} from 'echarts/renderers'
import {useTheme} from '../../composables/useTheme'

echarts.use([EBar, ELine, GridComponent, LegendComponent, TooltipComponent, CanvasRenderer])

const props = defineProps<{
  categories: string[]
  // 首个 series 渲染为柱、其余为线
  series: { name: string; data: (number | null)[] }[]
  // 开启后线系列使用右侧第二 Y 轴
  dualAxis?: boolean
  showLabel?: boolean
}>()

const {isDark} = useTheme()
const el = ref<HTMLElement>()
let chart: echarts.ECharts | null = null

const buildOption = (): echarts.EChartsCoreOption => {
  const text = isDark.value ? '#d1d5db' : '#374151'
  const axisLine = isDark.value ? '#374151' : '#e5e7eb'
  const valAxis = (name?: string) => ({
    type: 'value' as const,
    name,
    nameTextStyle: {color: text},
    axisLabel: {color: text},
    splitLine: {lineStyle: {color: axisLine}}
  })
  // 首个为柱，其余为线；线系列在双轴模式下挂到第二 Y 轴
  const lineNames: string[] = []
  const series = props.series.map((s, i) => {
    const isLine = i > 0
    if (isLine) {
      lineNames.push(s.name)
    }
    return {
      name: s.name,
      type: isLine ? ('line' as const) : ('bar' as const),
      yAxisIndex: props.dualAxis && isLine ? 1 : 0,
      data: s.data,
      barMaxWidth: 48,
      smooth: isLine ? true : undefined,
      label: {show: props.showLabel, position: 'top', color: text, fontSize: 10},
      emphasis: {focus: 'series'}
    }
  })
  const yAxis = props.dualAxis
    ? [valAxis(props.series[0]?.name), valAxis(lineNames.join(' / '))]
    : [valAxis()]
  return {
    tooltip: {trigger: 'axis', axisPointer: {type: 'cross'}},
    legend: {show: props.series.length > 1, top: 0, textStyle: {color: text}},
    grid: {left: 8, right: 16, top: props.series.length > 1 ? 28 : 12, bottom: 8, containLabel: true},
    xAxis: {type: 'category', data: props.categories, axisLabel: {color: text}, axisLine: {lineStyle: {color: axisLine}}},
    yAxis,
    series
  }
}

const render = () => {
  if (!chart) {
    return
  }
  chart.setOption(buildOption(), true)
}

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

watch(() => [props.categories, props.series, props.dualAxis, props.showLabel, isDark.value], render, {deep: true})

onBeforeUnmount(() => {
  ro?.disconnect()
  chart?.dispose()
  chart = null
})
</script>
