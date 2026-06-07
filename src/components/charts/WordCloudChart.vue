<template>
  <div ref="el" class="w-full h-full"/>
</template>

<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref, watch} from 'vue'
// 词云需完整 echarts + 扩展注册 'wordcloud' 系列
import * as echarts from 'echarts'
import 'echarts-wordcloud'
import {useTheme} from '../../composables/useTheme'

const props = defineProps<{
  data: { name: string; value: number }[]
}>()

const {isDark} = useTheme()
const el = ref<HTMLElement>()
let chart: echarts.ECharts | null = null

const buildOption = (): echarts.EChartsCoreOption => {
  const colors = ['#3b82f6', '#10b981', '#f59e0b', '#ef4444', '#8b5cf6', '#06b6d4', '#ec4899']
  return {
    tooltip: {show: true},
    series: [{
      type: 'wordcloud',
      shape: 'circle',
      left: 'center',
      top: 'center',
      width: '92%',
      height: '92%',
      sizeRange: [12, 60],
      rotationRange: [-45, 45],
      gridSize: 8,
      drawOutOfBound: false,
      textStyle: {color: () => colors[Math.floor(Math.random() * colors.length)]},
      emphasis: {textStyle: {fontWeight: 'bold', color: isDark.value ? '#fff' : '#111'}},
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

watch(() => [props.data, isDark.value], render, {deep: true})

onBeforeUnmount(() => {
  ro?.disconnect()
  chart?.dispose()
  chart = null
})
</script>
