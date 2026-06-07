<template>
  <div ref="el" class="w-full h-full"/>
</template>

<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref, watch} from 'vue'
import * as echarts from 'echarts/core'
import {MapChart as EMap} from 'echarts/charts'
import {TooltipComponent, VisualMapComponent} from 'echarts/components'
import {CanvasRenderer} from 'echarts/renderers'
import {useTheme} from '../../composables/useTheme'

echarts.use([EMap, VisualMapComponent, TooltipComponent, CanvasRenderer])

const props = defineProps<{
  mapType: 'china' | 'world'
  data: { name: string; value: number }[]
  max: number
}>()

const {isDark} = useTheme()
const el = ref<HTMLElement>()
let chart: echarts.ECharts | null = null

const registered = new Set<string>()
const ensureMap = async (type: 'china' | 'world'): Promise<boolean> => {
  if (registered.has(type)) {
    return true
  }
  try {
    const geo = type === 'china'
      ? (await import('./geo/china.json')).default
      : (await import('./geo/world.json')).default
    echarts.registerMap(type, geo as any)
    registered.add(type)
    return true
  }
  catch {
    return false
  }
}

const buildOption = (): echarts.EChartsCoreOption => {
  const text = isDark.value ? '#d1d5db' : '#374151'
  const area = isDark.value ? '#1f2937' : '#f1f5f9'
  const border = isDark.value ? '#374151' : '#cbd5e1'
  return {
    tooltip: {trigger: 'item', formatter: (p: any) => `${p.name}: ${p.value ?? '-'}`},
    visualMap: {
      min: 0,
      max: props.max || 1,
      calculable: true,
      left: 12,
      bottom: 12,
      textStyle: {color: text},
      inRange: {color: isDark.value ? ['#1e3a8a', '#3b82f6', '#fbbf24', '#ef4444'] : ['#dbeafe', '#60a5fa', '#fbbf24', '#ef4444']}
    },
    series: [{
      type: 'map',
      map: props.mapType,
      roam: true,
      emphasis: {label: {show: true}, itemStyle: {areaColor: '#fde68a'}},
      itemStyle: {areaColor: area, borderColor: border},
      label: {color: text},
      data: props.data
    }]
  }
}

const render = async () => {
  if (!chart) {
    return
  }
  const ok = await ensureMap(props.mapType)
  if (ok) {
    chart.setOption(buildOption(), true)
  }
}

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

watch(() => [props.mapType, props.data, props.max, isDark.value], render, {deep: true})

onBeforeUnmount(() => {
  ro?.disconnect()
  chart?.dispose()
  chart = null
})
</script>
