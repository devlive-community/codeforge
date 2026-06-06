<template>
  <div ref="el" class="w-full h-full"/>
</template>

<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref, watch} from 'vue'
import * as echarts from 'echarts/core'
import {ScatterChart as EScatter} from 'echarts/charts'
import {GridComponent, LegendComponent, TooltipComponent} from 'echarts/components'
import {CanvasRenderer} from 'echarts/renderers'
import {useTheme} from '../../composables/useTheme'

echarts.use([EScatter, GridComponent, LegendComponent, TooltipComponent, CanvasRenderer])

const props = defineProps<{
  series: { name: string; points: [number, number][] }[]
  xName?: string
  yName?: string
}>()

const {isDark} = useTheme()
const el = ref<HTMLElement>()
let chart: echarts.ECharts | null = null

const buildOption = (): echarts.EChartsCoreOption => {
  const text = isDark.value ? '#d1d5db' : '#374151'
  const axisLine = isDark.value ? '#374151' : '#e5e7eb'
  const multi = props.series.length > 1
  return {
    tooltip: {
      trigger: 'item',
      formatter: (p: any) => `${props.xName || 'x'}: ${p.value[0]}<br/>${props.yName || 'y'}: ${p.value[1]}`
    },
    legend: {show: multi, top: 0, textStyle: {color: text}},
    grid: {left: 8, right: 16, top: multi ? 28 : 12, bottom: 8, containLabel: true},
    xAxis: {type: 'value', name: props.xName, nameTextStyle: {color: text}, axisLabel: {color: text}, axisLine: {lineStyle: {color: axisLine}}, splitLine: {lineStyle: {color: axisLine}}},
    yAxis: {type: 'value', name: props.yName, nameTextStyle: {color: text}, axisLabel: {color: text}, axisLine: {lineStyle: {color: axisLine}}, splitLine: {lineStyle: {color: axisLine}}},
    series: props.series.map(s => ({
      name: s.name,
      type: 'scatter',
      symbolSize: 8,
      data: s.points,
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

watch(() => [props.series, props.xName, props.yName, isDark.value], render, {deep: true})

onBeforeUnmount(() => {
  ro?.disconnect()
  chart?.dispose()
  chart = null
})
</script>
