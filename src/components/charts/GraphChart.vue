<template>
  <div ref="el" class="w-full h-full"/>
</template>

<script setup lang="ts">
import {computed, onBeforeUnmount, onMounted, ref, watch} from 'vue'
import * as echarts from 'echarts/core'
import {GraphChart as EGraph} from 'echarts/charts'
import {TooltipComponent} from 'echarts/components'
import {CanvasRenderer} from 'echarts/renderers'
import {useTheme} from '../../composables/useTheme'

echarts.use([EGraph, TooltipComponent, CanvasRenderer])

const props = defineProps<{
  nodes: { name: string; value: number }[]
  links: { source: string; target: string; value: number }[]
  showLabel?: boolean
}>()

const {isDark} = useTheme()
const el = ref<HTMLElement>()
let chart: echarts.ECharts | null = null

// 按节点权重映射符号大小
const sized = computed(() => {
  const vals = props.nodes.map(n => n.value)
  const max = Math.max(1, ...vals)
  return props.nodes.map(n => ({...n, symbolSize: 10 + (n.value / max) * 38}))
})

const buildOption = (): echarts.EChartsCoreOption => {
  const text = isDark.value ? '#d1d5db' : '#374151'
  return {
    tooltip: {trigger: 'item', formatter: (p: any) => p.dataType === 'edge'
      ? `${p.data.source} → ${p.data.target}: ${p.data.value}`
      : `${p.name}: ${p.value}`},
    series: [{
      type: 'graph',
      layout: 'force',
      roam: true,
      draggable: true,
      data: sized.value,
      links: props.links,
      force: {repulsion: 120, edgeLength: [40, 120], gravity: 0.08},
      label: {show: props.showLabel !== false, color: text, fontSize: 11, position: 'right'},
      lineStyle: {color: isDark.value ? '#4b5563' : '#cbd5e1', curveness: 0.1, opacity: 0.7},
      emphasis: {focus: 'adjacency', lineStyle: {width: 3}}
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

watch(() => [props.nodes, props.links, props.showLabel, isDark.value], render, {deep: true})

onBeforeUnmount(() => {
  ro?.disconnect()
  chart?.dispose()
  chart = null
})
</script>
