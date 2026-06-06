<template>
  <div ref="el" class="w-full h-full"/>
</template>

<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref, watch} from 'vue'
import * as echarts from 'echarts/core'
import {RadarChart as ERadar} from 'echarts/charts'
import {LegendComponent, TooltipComponent} from 'echarts/components'
import {CanvasRenderer} from 'echarts/renderers'
import {useTheme} from '../../composables/useTheme'

echarts.use([ERadar, LegendComponent, TooltipComponent, CanvasRenderer])

const props = defineProps<{
  categories: string[]
  series: { name: string; data: (number | null)[] }[]
  area?: boolean
  showLabel?: boolean
}>()

const {isDark} = useTheme()
const el = ref<HTMLElement>()
let chart: echarts.ECharts | null = null

const buildOption = (): echarts.EChartsCoreOption => {
  const text = isDark.value ? '#d1d5db' : '#374151'
  const split = isDark.value ? '#374151' : '#e5e7eb'
  const indicator = props.categories.map((name, i) => {
    const max = Math.max(0, ...props.series.map(s => Number(s.data[i] ?? 0)))
    return {name, max: max > 0 ? max * 1.1 : 1}
  })
  return {
    tooltip: {trigger: 'item'},
    legend: {show: props.series.length > 1, top: 0, textStyle: {color: text}},
    radar: {
      indicator,
      center: ['50%', '55%'],
      radius: '65%',
      axisName: {color: text, fontSize: 11},
      splitLine: {lineStyle: {color: split}},
      axisLine: {lineStyle: {color: split}},
      splitArea: {areaStyle: {color: isDark.value ? ['transparent', 'rgba(255,255,255,0.02)'] : ['transparent', 'rgba(0,0,0,0.02)']}}
    },
    series: [{
      type: 'radar',
      areaStyle: props.area ? {opacity: 0.15} : undefined,
      label: {show: props.showLabel, color: text, fontSize: 10},
      data: props.series.map(s => ({name: s.name, value: s.data.map(v => Number(v ?? 0))}))
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

watch(() => [props.categories, props.series, props.area, props.showLabel, isDark.value], render, {deep: true})

onBeforeUnmount(() => {
  ro?.disconnect()
  chart?.dispose()
  chart = null
})
</script>
