<template>
  <div ref="containerRef" class="flex overflow-hidden h-full">
    <!-- 左侧面板 -->
    <div :style="{ width: `${leftWidth}px` }" class="overflow-hidden">
      <slot name="left"></slot>
    </div>

    <!-- 拖拽分隔条 -->
    <div
      class="relative w-1 bg-gray-200 hover:bg-blue-500 cursor-col-resize transition-colors flex-shrink-0 group"
      @mousedown="startResize"
      @touchstart="startResize">
      <div class="absolute inset-y-0 -left-1 -right-1"></div>
      <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 opacity-0 group-hover:opacity-100 transition-opacity">
        <div class="w-1 h-8 bg-blue-500 rounded-full"></div>
      </div>
    </div>

    <!-- 右侧面板 -->
    <div class="flex-1 overflow-hidden">
      <slot name="right"></slot>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'

interface Props {
  minLeftWidth?: number
  minRightWidth?: number
  defaultLeftWidth?: number
}

const props = withDefaults(defineProps<Props>(), {
  minLeftWidth: 300,
  minRightWidth: 300,
  defaultLeftWidth: 0
})

const containerRef = ref<HTMLElement | null>(null)
const leftWidth = ref(0)
const isResizing = ref(false)

// 初始化左侧宽度
onMounted(() => {
  if (containerRef.value) {
    const containerWidth = containerRef.value.clientWidth
    if (props.defaultLeftWidth > 0) {
      leftWidth.value = props.defaultLeftWidth
    } else {
      // 默认左侧占 60%
      leftWidth.value = Math.floor(containerWidth * 0.6)
    }

    // 从 localStorage 读取保存的宽度
    const savedWidth = localStorage.getItem('resizable-panels-left-width')
    if (savedWidth) {
      const width = parseInt(savedWidth, 10)
      if (width >= props.minLeftWidth && width <= containerWidth - props.minRightWidth) {
        leftWidth.value = width
      }
    }
  }

  window.addEventListener('resize', handleWindowResize)
})

onUnmounted(() => {
  window.removeEventListener('resize', handleWindowResize)
})

// 窗口大小改变时调整面板宽度
const handleWindowResize = () => {
  if (containerRef.value) {
    const containerWidth = containerRef.value.clientWidth
    const maxLeftWidth = containerWidth - props.minRightWidth

    if (leftWidth.value > maxLeftWidth) {
      leftWidth.value = maxLeftWidth
    } else if (leftWidth.value < props.minLeftWidth) {
      leftWidth.value = props.minLeftWidth
    }
  }
}

// 开始调整大小
const startResize = (e: MouseEvent | TouchEvent) => {
  e.preventDefault()
  isResizing.value = true

  // 添加全局事件监听
  document.addEventListener('mousemove', handleResize)
  document.addEventListener('mouseup', stopResize)
  document.addEventListener('touchmove', handleResize)
  document.addEventListener('touchend', stopResize)

  // 添加选择禁用样式
  document.body.style.userSelect = 'none'
  document.body.style.cursor = 'col-resize'
}

// 调整大小
const handleResize = (e: MouseEvent | TouchEvent) => {
  if (!isResizing.value || !containerRef.value) return

  const containerRect = containerRef.value.getBoundingClientRect()
  const clientX = 'touches' in e ? e.touches[0].clientX : e.clientX

  let newLeftWidth = clientX - containerRect.left

  // 限制最小/最大宽度
  const maxLeftWidth = containerRect.width - props.minRightWidth
  newLeftWidth = Math.max(props.minLeftWidth, Math.min(newLeftWidth, maxLeftWidth))

  leftWidth.value = newLeftWidth
}

// 停止调整大小
const stopResize = () => {
  if (isResizing.value) {
    isResizing.value = false

    // 移除全局事件监听
    document.removeEventListener('mousemove', handleResize)
    document.removeEventListener('mouseup', stopResize)
    document.removeEventListener('touchmove', handleResize)
    document.removeEventListener('touchend', stopResize)

    // 移除选择禁用样式
    document.body.style.userSelect = ''
    document.body.style.cursor = ''

    // 保存宽度到 localStorage
    localStorage.setItem('resizable-panels-left-width', leftWidth.value.toString())
  }
}
</script>
