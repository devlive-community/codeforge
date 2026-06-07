<template>
  <div ref="el" class="w-full h-full"/>
</template>

<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref, watch} from 'vue'
import * as echarts from 'echarts/core'
import {BarChart as EBar} from 'echarts/charts'
import {LegendComponent, PolarComponent, TooltipComponent} from 'echarts/components'
import {CanvasRenderer} from 'echarts/renderers'
import {useTheme} from '../../composables/useTheme'

echarts.use([EBar, PolarComponent, LegendComponent, TooltipComponent, CanvasRenderer])

const props = defineProps<{
  categories: string[]
  series: { name: string; data: (number | null)[] }[]
  stacked?: boolean
}>()

const {isDark} = useTheme()
const el = ref<HTMLElement>()
let chart: echarts.ECharts | null = null

const buildOption = (): echarts.EChartsCoreOption => {
  const text = isDark.value ? '#d1d5db' : '#374151'
  const axisLine = isDark.value ? '#374151' : '#e5e7eb'
  const multi = props.series.length > 1
  return {
    tooltip: {trigger: 'item'},
    legend: {show: multi, top: 0, textStyle: {color: text}},
    polar: {radius: [16, '74%']},
    angleAxis: {type: 'category', data: props.categories, axisLabel: {color: text}, axisLine: {lineStyle: {color: axisLine}}},
    radiusAxis: {axisLabel: {color: text}, splitLine: {lineStyle: {color: axisLine}}},
    series: props.series.map(s => ({
      name: s.name,
      type: 'bar',
      coordinateSystem: 'polar',
      stack: props.stacked ? 'total' : undefined,
      data: s.data,
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

watch(() => [props.categories, props.series, props.stacked, isDark.value], render, {deep: true})

onBeforeUnmount(() => {
  ro?.disconnect()
  chart?.dispose()
  chart = null
})
</script>
