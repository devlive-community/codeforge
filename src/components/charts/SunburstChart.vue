<template>
  <div ref="el" class="w-full h-full"/>
</template>

<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref, watch} from 'vue'
import * as echarts from 'echarts/core'
import {SunburstChart as ESunburst} from 'echarts/charts'
import {TooltipComponent} from 'echarts/components'
import {CanvasRenderer} from 'echarts/renderers'
import {useTheme} from '../../composables/useTheme'
import type {TreeNode} from './shape'

echarts.use([ESunburst, TooltipComponent, CanvasRenderer])

const props = defineProps<{
  data: TreeNode[]
  showLabel?: boolean
}>()

const {isDark} = useTheme()
const el = ref<HTMLElement>()
let chart: echarts.ECharts | null = null

const buildOption = (): echarts.EChartsCoreOption => {
  const text = isDark.value ? '#d1d5db' : '#374151'
  const border = isDark.value ? '#111827' : '#ffffff'
  return {
    tooltip: {trigger: 'item', formatter: (p: any) => `${p.name}: ${p.value}`},
    series: [{
      type: 'sunburst',
      data: props.data,
      radius: ['12%', '92%'],
      itemStyle: {borderColor: border, borderWidth: 1},
      label: {show: props.showLabel !== false, color: text, fontSize: 10, minAngle: 8},
      emphasis: {focus: 'ancestor'},
      levels: [{}, {r0: '12%', r: '45%'}, {r0: '45%', r: '70%'}, {r0: '70%', r: '92%'}]
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
