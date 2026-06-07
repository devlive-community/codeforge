<template>
  <div ref="el" class="w-full h-full"/>
</template>

<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref, watch} from 'vue'
import * as echarts from 'echarts/core'
import {HeatmapChart as EHeatmap} from 'echarts/charts'
import {CalendarComponent, TooltipComponent, VisualMapComponent} from 'echarts/components'
import {CanvasRenderer} from 'echarts/renderers'
import {useTheme} from '../../composables/useTheme'

echarts.use([EHeatmap, CalendarComponent, VisualMapComponent, TooltipComponent, CanvasRenderer])

const props = defineProps<{
  data: [string, number][]
  range: [string, string] | string
  max: number
}>()

const {isDark} = useTheme()
const el = ref<HTMLElement>()
let chart: echarts.ECharts | null = null

const buildOption = (): echarts.EChartsCoreOption => {
  const text = isDark.value ? '#d1d5db' : '#374151'
  const border = isDark.value ? '#374151' : '#e5e7eb'
  return {
    tooltip: {formatter: (p: any) => `${p.value[0]}: ${p.value[1]}`},
    visualMap: {
      min: 0,
      max: props.max || 1,
      calculable: true,
      orient: 'horizontal',
      left: 'center',
      bottom: 0,
      textStyle: {color: text},
      inRange: {color: isDark.value ? ['#1e3a8a', '#3b82f6', '#fbbf24', '#ef4444'] : ['#dbeafe', '#60a5fa', '#fbbf24', '#ef4444']}
    },
    calendar: {
      top: 24,
      left: 36,
      right: 16,
      cellSize: ['auto', 16],
      range: props.range,
      itemStyle: {borderColor: border, color: 'transparent'},
      splitLine: {lineStyle: {color: border}},
      dayLabel: {color: text},
      monthLabel: {color: text},
      yearLabel: {color: text}
    },
    series: [{type: 'heatmap', coordinateSystem: 'calendar', data: props.data}]
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

watch(() => [props.data, props.range, props.max, isDark.value], render, {deep: true})

onBeforeUnmount(() => {
  ro?.disconnect()
  chart?.dispose()
  chart = null
})
</script>
