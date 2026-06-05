<template>
  <div ref="containerRef" class="overflow-hidden h-full w-full flex" :class="isVertical ? 'flex-col' : 'flex-row'">
    <!-- 主面板（编辑器） -->
    <div :style="primaryStyle" class="overflow-hidden">
      <slot name="primary"></slot>
    </div>

    <!-- 拖拽分隔条 -->
    <div
      class="relative bg-gray-200 dark:bg-gray-700 hover:bg-blue-500 transition-colors flex-shrink-0 group"
      :class="isVertical ? 'h-1 cursor-row-resize' : 'w-1 cursor-col-resize'"
      @mousedown="startResize"
      @touchstart="startResize">
      <div class="absolute" :class="isVertical ? 'inset-x-0 -top-1 -bottom-1' : 'inset-y-0 -left-1 -right-1'"></div>
      <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 opacity-0 group-hover:opacity-100 transition-opacity">
        <div class="bg-blue-500 rounded-full" :class="isVertical ? 'h-1 w-8' : 'w-1 h-8'"></div>
      </div>
    </div>

    <!-- 副面板（控制台） -->
    <div class="flex-1 overflow-hidden">
      <slot name="secondary"></slot>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed, onMounted, onUnmounted, ref, watch} from 'vue'
import {SplitDirection} from '../types/app.ts'

interface Props
{
  direction?: SplitDirection
  minPrimary?: number
  minSecondary?: number
}

const props = withDefaults(defineProps<Props>(), {
  direction: 'horizontal',
  minPrimary: 300,
  minSecondary: 300
})

const containerRef = ref<HTMLElement | null>(null)
const primarySize = ref(0)
const isResizing = ref(false)

const isVertical = computed(() => props.direction === 'vertical')

// 每个方向单独记忆尺寸
const storageKey = computed(() => `resizable-panels-${props.direction}`)

const primaryStyle = computed(() => isVertical.value
    ? {height: `${primarySize.value}px`}
    : {width: `${primarySize.value}px`})

// 容器在当前方向上的可用尺寸
const containerSize = () => {
  if (!containerRef.value) {
    return 0
  }
  return isVertical.value ? containerRef.value.clientHeight : containerRef.value.clientWidth
}

const clampSize = (size: number, total: number) => {
  const max = total - props.minSecondary
  return Math.max(props.minPrimary, Math.min(size, max))
}

const initSize = () => {
  const total = containerSize()
  if (total <= 0) {
    return
  }

  // 默认主面板占 60%
  let size = Math.floor(total * 0.6)

  const saved = localStorage.getItem(storageKey.value)
  if (saved) {
    const parsed = parseInt(saved, 10)
    if (!Number.isNaN(parsed)) {
      size = parsed
    }
  }

  primarySize.value = clampSize(size, total)
}

onMounted(() => {
  initSize()
  window.addEventListener('resize', handleWindowResize)
})

onUnmounted(() => {
  window.removeEventListener('resize', handleWindowResize)
})

// 方向切换时按新方向重新初始化尺寸
watch(() => props.direction, () => {
  initSize()
})

const handleWindowResize = () => {
  const total = containerSize()
  if (total > 0) {
    primarySize.value = clampSize(primarySize.value, total)
  }
}

const startResize = (e: MouseEvent | TouchEvent) => {
  e.preventDefault()
  isResizing.value = true

  document.addEventListener('mousemove', handleResize)
  document.addEventListener('mouseup', stopResize)
  document.addEventListener('touchmove', handleResize)
  document.addEventListener('touchend', stopResize)

  document.body.style.userSelect = 'none'
  document.body.style.cursor = isVertical.value ? 'row-resize' : 'col-resize'
}

const handleResize = (e: MouseEvent | TouchEvent) => {
  if (!isResizing.value || !containerRef.value) {
    return
  }

  const rect = containerRef.value.getBoundingClientRect()
  let newSize: number
  if (isVertical.value) {
    const clientY = 'touches' in e ? e.touches[0].clientY : e.clientY
    newSize = clientY - rect.top
  }
  else {
    const clientX = 'touches' in e ? e.touches[0].clientX : e.clientX
    newSize = clientX - rect.left
  }

  primarySize.value = clampSize(newSize, isVertical.value ? rect.height : rect.width)
}

const stopResize = () => {
  if (!isResizing.value) {
    return
  }
  isResizing.value = false

  document.removeEventListener('mousemove', handleResize)
  document.removeEventListener('mouseup', stopResize)
  document.removeEventListener('touchmove', handleResize)
  document.removeEventListener('touchend', stopResize)

  document.body.style.userSelect = ''
  document.body.style.cursor = ''

  localStorage.setItem(storageKey.value, primarySize.value.toString())
}
</script>
