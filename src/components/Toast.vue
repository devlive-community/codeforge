<template>
  <Teleport to="body">
    <Transition name="toast"
                enter-active-class="transition-all duration-300 ease-out"
                enter-from-class="transform translate-x-full opacity-0"
                enter-to-class="transform translate-x-0 opacity-100"
                leave-active-class="transition-all duration-200 ease-in"
                leave-from-class="transform translate-x-0 opacity-100"
                leave-to-class="transform translate-x-full opacity-0">
      <div v-show="show"
           class="fixed bottom-10 right-4 z-50 max-w-sm w-full shadow-lg rounded-lg pointer-events-auto overflow-hidden"
           role="alert"
           aria-live="assertive"
           aria-atomic="true"
           :class="toastClasses">
        <div class="p-4">
          <div class="flex items-start">
            <!-- 图标 -->
            <div class="flex-shrink-0">
              <component class="w-5 h-5" :is="iconComponent" :class="iconClasses"/>
            </div>

            <!-- 消息内容 -->
            <div class="ml-3 w-0 flex-1 pt-0.5">
              <p :class="textClasses" class="text-sm font-medium">
                {{ message }}
              </p>
            </div>

            <!-- 关闭按钮 -->
            <div class="ml-4 flex-shrink-0 flex">
              <button class="inline-flex rounded-md focus:outline-none focus:ring-2 focus:ring-offset-2 transition-colors duration-200 cursor-pointer"
                      :class="closeButtonClasses"
                      @click="handleClose">
                <span class="sr-only">关闭</span>
                <X class="w-4 h-4"/>
              </button>
            </div>
          </div>
        </div>

        <!-- 进度条 -->
        <div v-if="showProgress" class="h-1 bg-black bg-opacity-20">
          <div class="h-full transition-all duration-100 ease-linear" :class="progressClasses" :style="{ width: progressWidth + '%' }"/>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { AlertCircle, CheckCircle, Info, X, XCircle } from 'lucide-vue-next'

interface Props
{
  show: boolean
  message: string
  type?: 'success' | 'error' | 'warning' | 'info'
  duration?: number
  showProgress?: boolean
}

interface Emits
{
  (e: 'close'): void
}

const props = withDefaults(defineProps<Props>(), {
  type: 'success',
  duration: 3000,
  showProgress: true
})

const emit = defineEmits<Emits>()

const progressWidth = ref(100)
let progressTimer: NodeJS.Timeout | null = null
let autoCloseTimer: NodeJS.Timeout | null = null

// 图标组件映射
const iconMap = {
  success: CheckCircle,
  error: XCircle,
  warning: AlertCircle,
  info: Info
}

// 计算属性
const iconComponent = computed(() => iconMap[props.type])

const toastClasses = computed(() => {
  const baseClasses = 'bg-white border-l-4'
  const typeClasses = {
    success: 'border-green-500',
    error: 'border-red-500',
    warning: 'border-yellow-500',
    info: 'border-blue-500'
  }
  return `${ baseClasses } ${ typeClasses[props.type] }`
})

const iconClasses = computed(() => {
  const typeClasses = {
    success: 'text-green-500',
    error: 'text-red-500',
    warning: 'text-yellow-500',
    info: 'text-blue-500'
  }
  return typeClasses[props.type]
})

const textClasses = computed(() => {
  return 'text-gray-900'
})

const closeButtonClasses = computed(() => {
  const typeClasses = {
    success: 'text-green-400 hover:text-green-600 focus:ring-green-500',
    error: 'text-red-400 hover:text-red-600 focus:ring-red-500',
    warning: 'text-yellow-400 hover:text-yellow-600 focus:ring-yellow-500',
    info: 'text-blue-400 hover:text-blue-600 focus:ring-blue-500'
  }
  return typeClasses[props.type]
})

const progressClasses = computed(() => {
  const typeClasses = {
    success: 'bg-green-500',
    error: 'bg-red-500',
    warning: 'bg-yellow-500',
    info: 'bg-blue-500'
  }
  return typeClasses[props.type]
})

// 方法
const handleClose = () => {
  clearTimers()
  emit('close')
}

const startProgress = () => {
  if (!props.showProgress || props.duration <= 0) {
    return
  }

  progressWidth.value = 100
  const interval = 50
  const step = (interval / props.duration) * 100

  progressTimer = setInterval(() => {
    progressWidth.value -= step
    if (progressWidth.value <= 0) {
      progressWidth.value = 0
      clearInterval(progressTimer!)
    }
  }, interval)
}

const startAutoClose = () => {
  if (props.duration <= 0) {
    return
  }

  autoCloseTimer = setTimeout(() => {
    handleClose()
  }, props.duration)
}

const clearTimers = () => {
  if (progressTimer) {
    clearInterval(progressTimer)
    progressTimer = null
  }
  if (autoCloseTimer) {
    clearTimeout(autoCloseTimer)
    autoCloseTimer = null
  }
}

// 生命周期
onMounted(() => {
  startProgress()
  startAutoClose()
})

onUnmounted(() => {
  clearTimers()
})
</script>
