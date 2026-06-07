<template>
  <div ref="el" class="w-full h-full"/>
</template>

<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref, watch} from 'vue'
import * as echarts from 'echarts/core'
import {ThemeRiverChart as ERiver} from 'echarts/charts'
import {LegendComponent, SingleAxisComponent, TooltipComponent} from 'echarts/components'
import {CanvasRenderer} from 'echarts/renderers'
import {useTheme} from '../../composables/useTheme'

echarts.use([ERiver, SingleAxisComponent, LegendComponent, TooltipComponent, CanvasRenderer])

const props = defineProps<{
  data: [string, number, string][]
  categories: string[]
}>()

const {isDark} = useTheme()
const el = ref<HTMLElement>()
let chart: echarts.ECharts | null = null

const buildOption = (): echarts.EChartsCoreOption => {
  const text = isDark.value ? '#d1d5db' : '#374151'
  return {
    tooltip: {trigger: 'axis', axisPointer: {type: 'line'}},
    legend: {data: props.categories, top: 0, type: 'scroll', textStyle: {color: text}},
    singleAxis: {
      type: 'time',
      top: 36,
      bottom: 24,
      axisLabel: {color: text},
      axisLine: {lineStyle: {color: isDark.value ? '#374151' : '#e5e7eb'}}
    },
    series: [{
      type: 'themeRiver',
      data: props.data,
      label: {show: false},
      emphasis: {focus: 'series'}
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

watch(() => [props.data, props.categories, isDark.value], render, {deep: true})

onBeforeUnmount(() => {
  ro?.disconnect()
  chart?.dispose()
  chart = null
})
</script>
