<template>
  <div ref="el" class="w-full h-full"/>
</template>

<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref, watch} from 'vue'
import * as echarts from 'echarts/core'
import {ParallelChart as EParallel} from 'echarts/charts'
import {LegendComponent, ParallelComponent, TooltipComponent} from 'echarts/components'
import {CanvasRenderer} from 'echarts/renderers'
import {useTheme} from '../../composables/useTheme'

echarts.use([EParallel, ParallelComponent, LegendComponent, TooltipComponent, CanvasRenderer])

const props = defineProps<{
  axes: string[]
  series: { name: string; data: number[][] }[]
}>()

const {isDark} = useTheme()
const el = ref<HTMLElement>()
let chart: echarts.ECharts | null = null

const buildOption = (): echarts.EChartsCoreOption => {
  const text = isDark.value ? '#d1d5db' : '#374151'
  const axisLine = isDark.value ? '#4b5563' : '#cbd5e1'
  const multi = props.series.length > 1
  return {
    tooltip: {},
    legend: {show: multi, top: 0, textStyle: {color: text}},
    parallelAxis: props.axes.map((name, i) => ({dim: i, name})),
    parallel: {
      left: 16,
      right: 24,
      top: multi ? 32 : 16,
      bottom: 16,
      axisExpandable: true,
      parallelAxisDefault: {nameTextStyle: {color: text}, axisLabel: {color: text}, axisLine: {lineStyle: {color: axisLine}}}
    },
    series: props.series.map(s => ({
      name: s.name,
      type: 'parallel',
      data: s.data,
      lineStyle: {width: 1, opacity: 0.5},
      emphasis: {lineStyle: {width: 2, opacity: 1}}
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

watch(() => [props.axes, props.series, isDark.value], render, {deep: true})

onBeforeUnmount(() => {
  ro?.disconnect()
  chart?.dispose()
  chart = null
})
</script>
