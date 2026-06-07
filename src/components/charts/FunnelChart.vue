<template>
  <div ref="el" class="w-full h-full"/>
</template>

<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref, watch} from 'vue'
import * as echarts from 'echarts/core'
import {FunnelChart as EFunnel} from 'echarts/charts'
import {LegendComponent, TooltipComponent} from 'echarts/components'
import {CanvasRenderer} from 'echarts/renderers'
import {useTheme} from '../../composables/useTheme'

echarts.use([EFunnel, LegendComponent, TooltipComponent, CanvasRenderer])

const props = defineProps<{
  data: { name: string; value: number }[]
  showLabel?: boolean
}>()

const {isDark} = useTheme()
const el = ref<HTMLElement>()
let chart: echarts.ECharts | null = null

const buildOption = (): echarts.EChartsCoreOption => {
  const text = isDark.value ? '#d1d5db' : '#374151'
  const border = isDark.value ? '#111827' : '#ffffff'
  return {
    tooltip: {trigger: 'item', formatter: '{b}: {c} ({d}%)'},
    legend: {type: 'scroll', orient: 'vertical', right: 0, top: 'middle', textStyle: {color: text}},
    series: [{
      type: 'funnel',
      left: '8%',
      right: '24%',
      top: 16,
      bottom: 12,
      minSize: '0%',
      maxSize: '100%',
      sort: 'descending',
      gap: 2,
      itemStyle: {borderColor: border, borderWidth: 1},
      label: {show: props.showLabel !== false, color: '#fff', formatter: '{b}: {c}'},
      data: props.data
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

watch(() => [props.data, props.showLabel, isDark.value], render, {deep: true})

onBeforeUnmount(() => {
  ro?.disconnect()
  chart?.dispose()
  chart = null
})
</script>
