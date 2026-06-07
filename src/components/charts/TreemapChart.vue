<template>
  <div ref="el" class="w-full h-full"/>
</template>

<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref, watch} from 'vue'
import * as echarts from 'echarts/core'
import {TreemapChart as ETreemap} from 'echarts/charts'
import {TooltipComponent} from 'echarts/components'
import {CanvasRenderer} from 'echarts/renderers'
import {useTheme} from '../../composables/useTheme'
import type {TreeNode} from './shape'

echarts.use([ETreemap, TooltipComponent, CanvasRenderer])

const props = defineProps<{
  data: TreeNode[]
  showLabel?: boolean
}>()

const {isDark} = useTheme()
const el = ref<HTMLElement>()
let chart: echarts.ECharts | null = null

const buildOption = (): echarts.EChartsCoreOption => {
  const border = isDark.value ? '#111827' : '#ffffff'
  return {
    tooltip: {trigger: 'item', formatter: (p: any) => `${p.name}: ${p.value}`},
    series: [{
      type: 'treemap',
      data: props.data,
      roam: false,
      nodeClick: 'zoomToNode',
      label: {show: props.showLabel !== false, color: '#fff', fontSize: 11},
      upperLabel: {show: true, height: 20, color: '#fff'},
      itemStyle: {borderColor: border, borderWidth: 1, gapWidth: 1},
      levels: [
        {itemStyle: {gapWidth: 2, borderWidth: 0}},
        {itemStyle: {gapWidth: 1}, colorSaturation: [0.35, 0.6]},
        {itemStyle: {gapWidth: 1}, colorSaturation: [0.3, 0.5]}
      ]
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
