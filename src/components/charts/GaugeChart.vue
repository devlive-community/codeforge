<template>
  <div ref="el" class="w-full h-full"/>
</template>

<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref, watch} from 'vue'
import * as echarts from 'echarts/core'
import {GaugeChart as EGauge} from 'echarts/charts'
import {CanvasRenderer} from 'echarts/renderers'
import {useTheme} from '../../composables/useTheme'

echarts.use([EGauge, CanvasRenderer])

const props = defineProps<{
  value: number
  max: number
  name?: string
}>()

const {isDark} = useTheme()
const el = ref<HTMLElement>()
let chart: echarts.ECharts | null = null

const buildOption = (): echarts.EChartsCoreOption => {
  const text = isDark.value ? '#d1d5db' : '#374151'
  const track = isDark.value ? '#374151' : '#e5e7eb'
  return {
    series: [{
      type: 'gauge',
      min: 0,
      max: props.max || 100,
      progress: {show: true, width: 16, roundCap: true},
      axisLine: {roundCap: true, lineStyle: {width: 16, color: [[1, track]]}},
      axisTick: {distance: -22, lineStyle: {color: text}},
      splitLine: {distance: -26, length: 10, lineStyle: {color: text}},
      axisLabel: {distance: -8, color: text, fontSize: 10},
      pointer: {length: '62%', width: 5},
      anchor: {show: true, size: 14, itemStyle: {color: '#3b82f6'}},
      title: {offsetCenter: [0, '72%'], color: text, fontSize: 13},
      detail: {valueAnimation: true, offsetCenter: [0, '44%'], color: text, fontSize: 26, fontWeight: 'bolder', formatter: (v: number) => `${Math.round(v * 100) / 100}`},
      data: [{value: props.value, name: props.name || ''}]
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

watch(() => [props.value, props.max, props.name, isDark.value], render, {deep: true})

onBeforeUnmount(() => {
  ro?.disconnect()
  chart?.dispose()
  chart = null
})
</script>
