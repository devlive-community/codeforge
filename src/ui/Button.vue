<template>
  <button :type="htmlType"
          :disabled="disabled || loading"
          :class="buttonClasses"
          class="flex items-center space-x-2 justify-center rounded-md transition-all duration-200 cursor-pointer"
          @click="handleClick"
          v-bind="$attrs">
    <!-- 加载状态图标 -->
    <div v-if="loading" class="animate-spin mr-2">
      <Loader2 class="w-4 h-4"/>
    </div>

    <!-- 左侧图标 -->
    <component v-else-if="icon && iconPosition === 'left'"
               :is="icon"
               :class="iconClasses"/>

    <!-- 按钮文本 -->
    <span v-if="$slots.default || text" :class="textClasses">
      <slot>{{ text }}</slot>
    </span>

    <!-- 右侧图标 -->
    <component v-if="icon && iconPosition === 'right' && !loading"
               :is="icon"
               :class="iconClasses"/>
  </button>
</template>

<script setup lang="ts">
import type { Component } from 'vue'
import { computed } from 'vue'
import { Loader2 } from 'lucide-vue-next'

interface Props
{
  // 按钮类型
  type?: 'primary' | 'secondary' | 'success' | 'warning' | 'danger' | 'info' | 'ghost' | 'link'
  // 按钮尺寸
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl'
  // 按钮变体
  variant?: 'solid' | 'outline' | 'ghost' | 'soft'
  // HTML 按钮类型
  htmlType?: 'button' | 'submit' | 'reset'
  // 禁用状态
  disabled?: boolean
  // 加载状态
  loading?: boolean
  // 圆形按钮
  circle?: boolean
  // 圆角按钮
  round?: boolean
  // 块级按钮
  block?: boolean
  // 图标
  icon?: Component | string
  // 图标位置
  iconPosition?: 'left' | 'right'
  // 仅图标按钮
  iconOnly?: boolean
  // 按钮文本
  text?: string
  // 自定义类名
  customClass?: string | string[]
}

const props = withDefaults(defineProps<Props>(), {
  type: 'primary',
  size: 'md',
  variant: 'solid',
  htmlType: 'button',
  disabled: false,
  loading: false,
  circle: false,
  round: false,
  block: false,
  iconPosition: 'left',
  iconOnly: false
})

const emit = defineEmits<{
  click: [event: MouseEvent]
}>()

// 基础样式
const baseClasses = computed(() => [
  'inline-flex items-center justify-center font-medium transition-all duration-200',
  'focus:outline-none focus:ring-2 focus:ring-offset-2',
  'disabled:cursor-not-allowed disabled:opacity-50',
  // 圆形按钮
  props.circle ? 'rounded-full' : props.round ? 'rounded-full' : 'rounded-lg',
  // 块级按钮
  props.block ? 'w-full' : '',
  // 仅图标按钮
  props.iconOnly ? 'p-0' : ''
])

// 尺寸样式
const sizeClasses = computed(() => {
  const sizes = {
    xs: props.iconOnly ? 'w-6 h-6' : 'px-2 py-1 text-xs',
    sm: props.iconOnly ? 'w-8 h-8' : 'px-3 py-1.5 text-sm',
    md: props.iconOnly ? 'w-10 h-10' : 'px-4 py-2 text-sm',
    lg: props.iconOnly ? 'w-12 h-12' : 'px-6 py-3 text-base',
    xl: props.iconOnly ? 'w-14 h-14' : 'px-8 py-4 text-lg'
  }
  return sizes[props.size]
})

// 类型和变体样式
const typeClasses = computed(() => {
  const { type, variant } = props

  const styles = {
    primary: {
      solid: 'bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500 border border-transparent',
      outline: 'border border-blue-600 text-blue-600 hover:bg-blue-50 focus:ring-blue-500',
      ghost: 'text-blue-600 hover:bg-blue-50 focus:ring-blue-500 border border-transparent',
      soft: 'bg-blue-50 text-blue-600 hover:bg-blue-100 focus:ring-blue-500 border border-transparent'
    },
    secondary: {
      solid: 'bg-gray-600 text-white hover:bg-gray-700 focus:ring-gray-500 border border-transparent',
      outline: 'border border-gray-300 text-gray-700 hover:bg-gray-50 focus:ring-gray-500',
      ghost: 'text-gray-700 hover:bg-gray-50 focus:ring-gray-500 border border-transparent',
      soft: 'bg-gray-50 text-gray-700 hover:bg-gray-100 focus:ring-gray-500 border border-transparent'
    },
    success: {
      solid: 'bg-green-600 text-white hover:bg-green-700 focus:ring-green-500 border border-transparent',
      outline: 'border border-green-600 text-green-600 hover:bg-green-50 focus:ring-green-500',
      ghost: 'text-green-600 hover:bg-green-50 focus:ring-green-500 border border-transparent',
      soft: 'bg-green-50 text-green-600 hover:bg-green-100 focus:ring-green-500 border border-transparent'
    },
    warning: {
      solid: 'bg-yellow-500 text-white hover:bg-yellow-600 focus:ring-yellow-500 border border-transparent',
      outline: 'border border-yellow-500 text-yellow-600 hover:bg-yellow-50 focus:ring-yellow-500',
      ghost: 'text-yellow-600 hover:bg-yellow-50 focus:ring-yellow-500 border border-transparent',
      soft: 'bg-yellow-50 text-yellow-600 hover:bg-yellow-100 focus:ring-yellow-500 border border-transparent'
    },
    danger: {
      solid: 'bg-red-600 text-white hover:bg-red-700 focus:ring-red-500 border border-transparent',
      outline: 'border border-red-600 text-red-600 hover:bg-red-50 focus:ring-red-500',
      ghost: 'text-red-600 hover:bg-red-50 focus:ring-red-500 border border-transparent',
      soft: 'bg-red-50 text-red-600 hover:bg-red-100 focus:ring-red-500 border border-transparent'
    },
    info: {
      solid: 'bg-cyan-600 text-white hover:bg-cyan-700 focus:ring-cyan-500 border border-transparent',
      outline: 'border border-cyan-600 text-cyan-600 hover:bg-cyan-50 focus:ring-cyan-500',
      ghost: 'text-cyan-600 hover:bg-cyan-50 focus:ring-cyan-500 border border-transparent',
      soft: 'bg-cyan-50 text-cyan-600 hover:bg-cyan-100 focus:ring-cyan-500 border border-transparent'
    },
    link: {
      solid: 'text-blue-600 hover:text-blue-700 underline focus:ring-blue-500 border border-transparent bg-transparent p-0',
      outline: 'text-blue-600 hover:text-blue-700 underline focus:ring-blue-500 border border-transparent bg-transparent p-0',
      ghost: 'text-blue-600 hover:text-blue-700 underline focus:ring-blue-500 border border-transparent bg-transparent p-0',
      soft: 'text-blue-600 hover:text-blue-700 underline focus:ring-blue-500 border border-transparent bg-transparent p-0'
    }
  }

  return styles[type as keyof typeof styles]?.[variant] || styles.primary.solid
})

// 图标样式
const iconClasses = computed(() => {
  const hasText = props.text || !!(props as any).$slots?.default
  const sizeMap = {
    xs: 'w-3 h-3',
    sm: 'w-4 h-4',
    md: 'w-4 h-4',
    lg: 'w-5 h-5',
    xl: 'w-6 h-6'
  }

  const marginMap = {
    left: hasText && !props.iconOnly ? 'mr-2' : '',
    right: hasText && !props.iconOnly ? 'ml-2' : ''
  }

  return [
    sizeMap[props.size],
    marginMap[props.iconPosition]
  ].filter(Boolean)
})

// 文本样式
const textClasses = computed(() => {
  if (props.iconOnly) {
    return 'sr-only'
  }
  return ''
})

// 最终样式组合
const buttonClasses = computed(() => [
  ...baseClasses.value,
  sizeClasses.value,
  typeClasses.value,
  ...(Array.isArray(props.customClass) ? props.customClass : [props.customClass]).filter(Boolean)
])

// 点击处理
const handleClick = (event: MouseEvent) => {
  if (!props.disabled && !props.loading) {
    emit('click', event)
  }
}
</script>
