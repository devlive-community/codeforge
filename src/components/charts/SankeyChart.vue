<template>
  <div ref="el" class="w-full h-full"/>
</template>

<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref, watch} from 'vue'
import * as echarts from 'echarts/core'
import {SankeyChart as ESankey} from 'echarts/charts'
import {TooltipComponent} from 'echarts/components'
import {CanvasRenderer} from 'echarts/renderers'
import {useTheme} from '../../composables/useTheme'
import {stripLevel} from './shape'

echarts.use([ESankey, TooltipComponent, CanvasRenderer])

const props = defineProps<{
  nodes: { name: string }[]
  links: { source: string; target: string; value: number }[]
}>()

const {isDark} = useTheme()
const el = ref<HTMLElement>()
let chart: echarts.ECharts | null = null

const buildOption = (): echarts.EChartsCoreOption => {
  const text = isDark.value ? '#d1d5db' : '#374151'
  return {
    tooltip: {trigger: 'item', formatter: (p: any) => p.dataType === 'edge'
      ? `${stripLevel(p.data.source)} → ${stripLevel(p.data.target)}: ${p.data.value}`
      : `${stripLevel(p.name)}`},
    series: [{
      type: 'sankey',
      left: 8,
      right: '14%',
      top: 12,
      bottom: 12,
      data: props.nodes,
      links: props.links,
      emphasis: {focus: 'adjacency'},
      label: {color: text, fontSize: 11, formatter: (p: any) => stripLevel(p.name)},
      lineStyle: {color: 'gradient', opacity: 0.45}
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

watch(() => [props.nodes, props.links, isDark.value], render, {deep: true})

onBeforeUnmount(() => {
  ro?.disconnect()
  chart?.dispose()
  chart = null
})
</script>
