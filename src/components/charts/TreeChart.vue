<template>
  <div ref="el" class="w-full h-full"/>
</template>

<script setup lang="ts">
import {computed, onBeforeUnmount, onMounted, ref, watch} from 'vue'
import {useI18n} from 'vue-i18n'
import * as echarts from 'echarts/core'
import {TreeChart as ETree} from 'echarts/charts'
import {TooltipComponent} from 'echarts/components'
import {CanvasRenderer} from 'echarts/renderers'
import {useTheme} from '../../composables/useTheme'
import type {TreeNode} from './shape'

echarts.use([ETree, TooltipComponent, CanvasRenderer])

const props = defineProps<{
  data: TreeNode[]
  rootName?: string
  showLabel?: boolean
}>()

const {isDark} = useTheme()
const {t} = useI18n()
const el = ref<HTMLElement>()
let chart: echarts.ECharts | null = null

// echarts tree 需要单一根节点，这里包一层
const rootData = computed<TreeNode>(() => ({name: props.rootName || t('chart.rootAll'), children: props.data}))

const buildOption = (): echarts.EChartsCoreOption => {
  const text = isDark.value ? '#d1d5db' : '#374151'
  return {
    tooltip: {trigger: 'item', triggerOn: 'mousemove', formatter: (p: any) => p.value != null ? `${p.name}: ${p.value}` : `${p.name}`},
    series: [{
      type: 'tree',
      data: [rootData.value],
      top: '4%',
      left: '8%',
      bottom: '4%',
      right: '18%',
      symbolSize: 8,
      orient: 'LR',
      expandAndCollapse: true,
      initialTreeDepth: 3,
      label: {show: props.showLabel !== false, position: 'left', verticalAlign: 'middle', align: 'right', color: text, fontSize: 11},
      leaves: {label: {position: 'right', verticalAlign: 'middle', align: 'left'}},
      lineStyle: {color: isDark.value ? '#4b5563' : '#cbd5e1'},
      emphasis: {focus: 'descendant'},
      animationDuration: 400
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

watch(() => [props.data, props.rootName, props.showLabel, isDark.value], render, {deep: true})

onBeforeUnmount(() => {
  ro?.disconnect()
  chart?.dispose()
  chart = null
})
</script>
