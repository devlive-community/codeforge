<template>
  <div ref="el" class="w-full h-full"/>
</template>

<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref, watch} from 'vue'
import * as echarts from 'echarts/core'
import {HeatmapChart as EHeatmap} from 'echarts/charts'
import {GridComponent, TooltipComponent, VisualMapComponent} from 'echarts/components'
import {CanvasRenderer} from 'echarts/renderers'
import {useTheme} from '../../composables/useTheme'

echarts.use([EHeatmap, GridComponent, TooltipComponent, VisualMapComponent, CanvasRenderer])

const props = defineProps<{
  xCats: string[]
  yCats: string[]
  cells: [number, number, number][]
  min: number
  max: number
  showLabel?: boolean
}>()

const {isDark} = useTheme()
const el = ref<HTMLElement>()
let chart: echarts.ECharts | null = null

const buildOption = (): echarts.EChartsCoreOption => {
  const text = isDark.value ? '#d1d5db' : '#374151'
  const axisLine = isDark.value ? '#374151' : '#e5e7eb'
  return {
    tooltip: {
      position: 'top',
      formatter: (p: any) => `${props.xCats[p.value[0]]} / ${props.yCats[p.value[1]]}: ${p.value[2]}`
    },
    grid: {left: 8, right: 16, top: 12, bottom: 40, containLabel: true},
    xAxis: {type: 'category', data: props.xCats, splitArea: {show: true}, axisLabel: {color: text, rotate: props.xCats.length > 8 ? 30 : 0}, axisLine: {lineStyle: {color: axisLine}}},
    yAxis: {type: 'category', data: props.yCats, splitArea: {show: true}, axisLabel: {color: text}, axisLine: {lineStyle: {color: axisLine}}},
    visualMap: {
      min: props.min,
      max: props.max || 1,
      calculable: true,
      orient: 'horizontal',
      left: 'center',
      bottom: 0,
      textStyle: {color: text},
      inRange: {color: isDark.value ? ['#1e3a8a', '#3b82f6', '#fbbf24', '#ef4444'] : ['#dbeafe', '#60a5fa', '#fbbf24', '#ef4444']}
    },
    series: [{
      type: 'heatmap',
      data: props.cells,
      label: {show: props.showLabel, color: text, fontSize: 10},
      emphasis: {itemStyle: {shadowBlur: 8, shadowColor: 'rgba(0,0,0,0.3)'}}
    }]
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

watch(() => [props.xCats, props.yCats, props.cells, props.min, props.max, props.showLabel, isDark.value], render, {deep: true})

onBeforeUnmount(() => {
  ro?.disconnect()
  chart?.dispose()
  chart = null
})
</script>
