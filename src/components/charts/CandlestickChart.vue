<template>
  <div ref="el" class="w-full h-full"/>
</template>

<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref, watch} from 'vue'
import * as echarts from 'echarts/core'
import {CandlestickChart as ECandle} from 'echarts/charts'
import {DataZoomComponent, GridComponent, TooltipComponent} from 'echarts/components'
import {CanvasRenderer} from 'echarts/renderers'
import {useTheme} from '../../composables/useTheme'

echarts.use([ECandle, GridComponent, TooltipComponent, DataZoomComponent, CanvasRenderer])

const props = defineProps<{
  categories: string[]
  values: number[][]
}>()

const {isDark} = useTheme()
const el = ref<HTMLElement>()
let chart: echarts.ECharts | null = null

const buildOption = (): echarts.EChartsCoreOption => {
  const text = isDark.value ? '#d1d5db' : '#374151'
  const axisLine = isDark.value ? '#374151' : '#e5e7eb'
  return {
    tooltip: {trigger: 'axis', axisPointer: {type: 'cross'}},
    grid: {left: 8, right: 16, top: 12, bottom: 48, containLabel: true},
    xAxis: {type: 'category', data: props.categories, axisLabel: {color: text}, axisLine: {lineStyle: {color: axisLine}}},
    yAxis: {type: 'value', scale: true, axisLabel: {color: text}, splitLine: {lineStyle: {color: axisLine}}},
    dataZoom: [{type: 'inside'}, {type: 'slider', height: 18, bottom: 12}],
    series: [{
      type: 'candlestick',
      data: props.values,
      itemStyle: {color: '#ef4444', color0: '#10b981', borderColor: '#ef4444', borderColor0: '#10b981'}
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

watch(() => [props.categories, props.values, isDark.value], render, {deep: true})

onBeforeUnmount(() => {
  ro?.disconnect()
  chart?.dispose()
  chart = null
})
</script>
