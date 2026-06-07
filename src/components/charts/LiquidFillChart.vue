<template>
  <div ref="el" class="w-full h-full"/>
</template>

<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref, watch} from 'vue'
// 水球图需完整 echarts + 扩展注册 'liquidFill' 系列
import * as echarts from 'echarts'
import 'echarts-liquidfill'
import {useTheme} from '../../composables/useTheme'

const props = defineProps<{
  value: number // 0~1 的比例
  name?: string
}>()

const {isDark} = useTheme()
const el = ref<HTMLElement>()
let chart: echarts.ECharts | null = null

const buildOption = (): echarts.EChartsCoreOption => {
  const text = isDark.value ? '#d1d5db' : '#374151'
  const r = Math.max(0, Math.min(1, props.value))
  return {
    series: [{
      type: 'liquidFill',
      radius: '72%',
      center: ['50%', '50%'],
      data: [r, r * 0.9, r * 0.8],
      color: ['#3b82f6', '#60a5fa', '#93c5fd'],
      backgroundStyle: {color: isDark.value ? '#1f2937' : '#f1f5f9'},
      outline: {borderDistance: 4, itemStyle: {borderColor: '#3b82f6', borderWidth: 2}},
      label: {
        formatter: () => `${(r * 100).toFixed(1)}%\n${props.name || ''}`,
        color: text,
        fontSize: 24,
        fontWeight: 'bold'
      }
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

watch(() => [props.value, props.name, isDark.value], render, {deep: true})

onBeforeUnmount(() => {
  ro?.disconnect()
  chart?.dispose()
  chart = null
})
</script>
