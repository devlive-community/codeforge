<template>
  <div class="relative w-full h-full">
    <div ref="el" class="w-full h-full"/>
    <button v-if="stack.length"
            class="absolute top-2 left-2 z-10 inline-flex items-center gap-1 px-2 py-1 rounded-md text-xs text-gray-600 dark:text-gray-300 bg-white/80 dark:bg-gray-800/80 border border-gray-200 dark:border-gray-700 hover:bg-gray-100 dark:hover:bg-gray-700 backdrop-blur cursor-pointer"
            @click="back">
      <ChevronLeft class="w-3.5 h-3.5"/>{{ t('chart.mapBack') }}
    </button>
  </div>
</template>

<script setup lang="ts">
import {onBeforeUnmount, onMounted, ref, watch} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import * as echarts from 'echarts/core'
import {MapChart as EMap} from 'echarts/charts'
import {TooltipComponent, VisualMapComponent} from 'echarts/components'
import {CanvasRenderer} from 'echarts/renderers'
import {ChevronLeft} from 'lucide-vue-next'
import {useI18n} from 'vue-i18n'
import {useTheme} from '../../composables/useTheme'
import {useToast} from '../../plugins/toast'

echarts.use([EMap, VisualMapComponent, TooltipComponent, CanvasRenderer])

const props = defineProps<{
  mapType: 'china' | 'world'
  data: { name: string; value: number }[]
  max: number
}>()

const {isDark} = useTheme()
const {t} = useI18n()
const toast = useToast()
const el = ref<HTMLElement>()
let chart: echarts.ECharts | null = null

interface GeoFeature { properties: { name: string; adcode?: number; childrenNum?: number } }
interface GeoJson { features: GeoFeature[] }

// 已注册地图的会话级缓存：key（china/world/省市 adcode）-> geojson
const mapCache = new Map<string, GeoJson>()
const currentKey = ref<string>(props.mapType)
let currentGeo: GeoJson | null = null
// 下钻面包屑（仅中国地图体系）：保存可返回的上级 key
const stack = ref<string[]>([])

// 载入并注册某级地图：china/world 走本地离线 json，省/市级按 adcode 联网取 DataV GeoAtlas
const loadGeo = async (key: string): Promise<GeoJson | null> => {
  const cached = mapCache.get(key)
  if (cached) {
    return cached
  }
  try {
    let geo: GeoJson
    if (key === 'china') {
      geo = (await import('./geo/china.json')).default as any
    }
    else if (key === 'world') {
      geo = (await import('./geo/world.json')).default as any
    }
    else {
      // 省/市级：走后端命令，落盘缓存到 ~/.codeforge/cache/geo，首次联网后离线复用
      const raw = await invoke<string>('fetch_area_geojson', {adcode: key})
      geo = JSON.parse(raw)
    }
    echarts.registerMap(key, geo as any)
    mapCache.set(key, geo)
    return geo
  }
  catch (e) {
    console.error('加载地图失败:', e)
    return null
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
      map: currentKey.value,
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
  const geo = await loadGeo(currentKey.value)
  if (!geo) {
    toast.error(t('chart.mapLoadFailed'))
    return
  }
  currentGeo = geo
  chart.setOption(buildOption(), true)
}

// 点击区域下钻：仅中国地图体系；点中带下级（childrenNum>0）的区域才进入
const onMapClick = async (params: any) => {
  if (props.mapType !== 'china' || !currentGeo) {
    return
  }
  const feat = currentGeo.features.find(f => f.properties.name === params.name)
  const adcode = feat?.properties.adcode
  if (!adcode || (feat?.properties.childrenNum ?? 0) <= 0) {
    return
  }
  stack.value = [...stack.value, currentKey.value]
  currentKey.value = String(adcode)
  await render()
}

const back = async () => {
  const prev = stack.value[stack.value.length - 1]
  if (prev === undefined) {
    return
  }
  stack.value = stack.value.slice(0, -1)
  currentKey.value = prev
  await render()
}

let ro: ResizeObserver | null = null
onMounted(() => {
  if (!el.value) {
    return
  }
  chart = echarts.init(el.value, undefined, {renderer: 'canvas'})
  chart.on('click', onMapClick)
  render()
  ro = new ResizeObserver(() => chart?.resize())
  ro.observe(el.value)
})

// 切换 china↔world 时重置下钻层级
watch(() => props.mapType, () => {
  stack.value = []
  currentKey.value = props.mapType
  render()
})
watch(() => [props.data, props.max, isDark.value], render, {deep: true})

onBeforeUnmount(() => {
  ro?.disconnect()
  chart?.dispose()
  chart = null
})
</script>
