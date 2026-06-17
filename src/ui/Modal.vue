<template>
  <Teleport to="body">
    <Transition name="modal"
                enter-active-class="transition-all duration-300 ease-out"
                enter-from-class="opacity-0"
                enter-to-class="opacity-100"
                leave-active-class="transition-all duration-200 ease-in"
                leave-from-class="opacity-100"
                leave-to-class="opacity-0"
                @before-enter="$emit('before-open')"
                @after-enter="$emit('after-open')"
                @before-leave="$emit('before-close')"
                @after-leave="$emit('after-close')">
      <div v-show="show"
           class="fixed inset-0 z-50 flex items-center justify-center transition-all duration-300 ease-out"
           :class="[
              backdropClasses,
              { 'opacity-0': !isVisible, 'opacity-100': isVisible }
            ]"
           @click.self="handleBackdropClick">
        <div class="transform transition-all duration-300 ease-out"
             :class="[
                modalClasses,
                {
                  'scale-95 opacity-0 translate-y-4': !isVisible,
                  'scale-100 opacity-100 translate-y-0': isVisible
                }
              ]"
             role="dialog"
             aria-modal="true"
             :aria-labelledby="titleId">
          <!-- 头部 -->
          <div v-if="!hideHeader" class="flex items-center justify-between mb-6">
            <h2 v-if="title"
                :id="titleId"
                class="text-xl font-bold text-gray-900 dark:text-white">
              {{ title }}
            </h2>
            <slot name="title"/>

            <!-- 关闭按钮 -->
            <button v-if="closable"
                    class="text-gray-400 hover:cursor-pointer dark:hover:text-gray-200 transition-all duration-200 hover:scale-110 rounded-full p-1 hover:bg-gray-300 dark:hover:bg-gray-700"
                    @click="handleClose"
                    :aria-label="closeButtonLabel || t('ui.close')">
              <component :is="closeIcon" class="w-5 h-5"/>
            </button>
          </div>

          <!-- 内容区域 -->
          <div :class="contentClasses">
            <slot/>
          </div>

          <!-- 底部 -->
          <div v-if="$slots.footer" :class="footerClasses">
            <slot name="footer"/>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import type { Component } from 'vue'
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
import { X } from 'lucide-vue-next'

interface Props
{
  // 显示控制
  show: boolean

  // 标题配置
  title?: string
  hideHeader?: boolean

  // 关闭配置
  closable?: boolean
  closeOnBackdrop?: boolean
  closeOnEsc?: boolean
  closeIcon?: Component
  closeButtonLabel?: string

  // 尺寸配置
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl' | '2xl' | '3xl' | '4xl' | '5xl' | '6xl' | 'full'
  width?: string
  height?: string
  maxWidth?: string
  maxHeight?: string

  // 样式配置
  backdrop?: 'blur' | 'dark' | 'light' | 'none'
  rounded?: boolean
  shadow?: boolean
  border?: boolean

  // 内容配置
  padding?: 'none' | 'sm' | 'md' | 'lg' | 'xl'

  // 自定义类名
  backdropClass?: string | string[]
  modalClass?: string | string[]
  contentClass?: string | string[]
  footerClass?: string | string[]
}

const props = withDefaults(defineProps<Props>(), {
  closable: true,
  closeOnBackdrop: true,
  closeOnEsc: true,
  closeIcon: () => X,
  size: '2xl',
  backdrop: 'blur',
  rounded: true,
  shadow: true,
  border: true,
  padding: 'lg'
})

const emit = defineEmits<{
  'update:show': [value: boolean]
  'close': []
  'before-open': []
  'after-open': []
  'before-close': []
  'after-close': []
}>()

const isVisible = ref(false)
const titleId = `modal-title-${ Math.random().toString(36).substr(2, 9) }`

// 计算样式类
const backdropClasses = computed(() => {
  const base = 'backdrop-blur-sm'
  const variants = {
    blur: 'bg-white/20 dark:bg-gray-900/20',
    dark: 'bg-black/50',
    light: 'bg-white/50',
    none: ''
  }

  return [
    base,
    variants[props.backdrop],
    ...(Array.isArray(props.backdropClass) ? props.backdropClass : [props.backdropClass]).filter(Boolean)
  ]
})

const modalClasses = computed(() => {
  const base = [
    'bg-white/95 dark:bg-gray-800/95',
    props.backdrop === 'blur' ? 'backdrop-blur-md' : '',
    props.rounded ? 'rounded-xl' : '',
    props.shadow ? 'shadow-2xl' : '',
    props.border ? 'border border-white/20 dark:border-gray-700/30' : ''
  ]

  // 尺寸处理
  const sizeClasses = {
    xs: 'w-80 max-w-xs',
    sm: 'w-96 max-w-sm',
    md: 'w-[32rem] max-w-md',
    lg: 'w-[40rem] max-w-lg',
    xl: 'w-[48rem] max-w-xl',
    '2xl': 'w-[56rem] max-w-2xl',
    '3xl': 'w-[64rem] max-w-3xl',
    '4xl': 'w-[72rem] max-w-4xl',
    '5xl': 'w-[80rem] max-w-5xl',
    '6xl': 'w-[88rem] max-w-6xl',
    'full': 'w-full h-full max-w-none max-h-none'
  }

  // 内边距
  const paddingClasses = {
    none: '',
    sm: 'p-4',
    md: 'p-5',
    lg: 'p-6',
    xl: 'p-8'
  }

  const customSize = props.width || props.height || props.maxWidth || props.maxHeight
  const sizeClass = customSize ? '' : sizeClasses[props.size]

  return [
    ...base,
    sizeClass,
    paddingClasses[props.padding],
    'max-h-[90vh] overflow-y-auto',
    ...(Array.isArray(props.modalClass) ? props.modalClass : [props.modalClass]).filter(Boolean)
  ].filter(Boolean)
})

const contentClasses = computed(() => {
  return [
    ...(Array.isArray(props.contentClass) ? props.contentClass : [props.contentClass]).filter(Boolean)
  ]
})

const footerClasses = computed(() => {
  const base = 'border-t border-gray-200/50 dark:border-gray-600/50 pt-4 mt-6'
  return [
    base,
    ...(Array.isArray(props.footerClass) ? props.footerClass : [props.footerClass]).filter(Boolean)
  ]
})

// 事件处理
const handleClose = () => {
  isVisible.value = false
  setTimeout(() => {
    emit('update:show', false)
    emit('close')
  }, 200)
}

const handleBackdropClick = () => {
  if (props.closeOnBackdrop) {
    handleClose()
  }
}

const handleEscKey = (event: KeyboardEvent) => {
  if (props.closeOnEsc && event.key === 'Escape' && props.show) {
    handleClose()
  }
}

// 生命周期
onMounted(async () => {
  // 监听 ESC 键
  document.addEventListener('keydown', handleEscKey)

  // 延迟显示动画
  if (props.show) {
    await nextTick()
    setTimeout(() => {
      isVisible.value = true
    }, 50)
  }
})

// 监听 show 属性变化
const unwatchShow = watch(() => props.show, async (newVal) => {
  if (newVal) {
    await nextTick()
    setTimeout(() => {
      isVisible.value = true
    }, 50)
  }
  else {
    isVisible.value = false
  }
})

// 清理
onUnmounted(() => {
  document.removeEventListener('keydown', handleEscKey)
  unwatchShow()
})
</script>
