<template>
  <div :class="wrapperClasses" class="inline-flex items-center">
    <!-- 标签 -->
    <label v-if="label && labelPosition === 'left'" :for="inputId" :class="labelClasses" class="select-none">
      {{ label }}
    </label>

    <!-- 数字输入器容器 -->
    <div :class="containerClasses" class="relative inline-flex items-center overflow-hidden">
      <!-- 减少按钮 -->
      <button v-if="controls && controlsPosition !== 'right'"
              :disabled="disabled || loading || isAtMin"
              :class="decreaseButtonClasses"
              class="flex items-center justify-center transition-all duration-200 focus:outline-none border-r border-gray-300 h-full"
              type="button"
              @click="decrease"
              @mousedown="startContinuousChange('decrease')"
              @mouseup="stopContinuousChange"
              @mouseleave="stopContinuousChange">
        <component :is="decreaseIcon || Minus" :class="iconClasses"/>
      </button>

      <!-- 输入框 -->
      <input :id="inputId"
             ref="inputRef"
             :value="displayValue"
             :disabled="disabled || loading"
             :readonly="readonly"
             :placeholder="placeholder"
             :class="inputClasses"
             :min="min"
             :max="max"
             :step="step"
             class="transition-all duration-200 focus:outline-none [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
             type="number"
             @input="handleInput"
             @blur="handleBlur"
             @focus="handleFocus"
             v-bind="$attrs"/>

      <!-- 加载状态 -->
      <div v-if="loading" class="absolute inset-0 flex items-center justify-center bg-white bg-opacity-75">
        <div :class="loaderClasses" class="animate-spin rounded-full border-2 border-current border-t-transparent"></div>
      </div>

      <!-- 右侧控制按钮 -->
      <div v-if="controls && controlsPosition === 'right'" class="absolute right-1 flex flex-col">
        <!-- 增加按钮 -->
        <button :disabled="disabled || loading || isAtMax"
                :class="rightControlButtonClasses"
                class="flex items-center justify-center transition-all duration-200 focus:outline-none"
                type="button"
                @click="increase"
                @mousedown="startContinuousChange('increase')"
                @mouseup="stopContinuousChange"
                @mouseleave="stopContinuousChange">
          <component :is="increaseIcon || Plus" :class="rightIconClasses"/>
        </button>

        <!-- 减少按钮 -->
        <button :disabled="disabled || loading || isAtMin"
                :class="rightControlButtonClasses"
                class="flex items-center justify-center transition-all duration-200 focus:outline-none"
                type="button"
                @click="decrease"
                @mousedown="startContinuousChange('decrease')"
                @mouseup="stopContinuousChange"
                @mouseleave="stopContinuousChange">
          <component :is="decreaseIcon || Minus" :class="rightIconClasses"/>
        </button>
      </div>

      <!-- 增加按钮（左右布局） -->
      <button v-if="controls && controlsPosition !== 'right'"
              :disabled="disabled || loading || isAtMax"
              :class="increaseButtonClasses"
              class="flex items-center justify-center transition-all duration-200 focus:outline-none border-l border-gray-300 h-full"
              type="button"
              @click="increase"
              @mousedown="startContinuousChange('increase')"
              @mouseup="stopContinuousChange"
              @mouseleave="stopContinuousChange">
        <component :is="increaseIcon || Plus" :class="iconClasses"/>
      </button>
    </div>

    <!-- 标签 -->
    <label v-if="label && labelPosition === 'right'" :for="inputId" :class="labelClasses" class="select-none">
      {{ label }}
    </label>

    <!-- 错误信息 -->
    <div v-if="error && showError" :class="errorClasses" class="absolute top-full left-0 mt-1">
      {{ error }}
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Component } from 'vue'
import { computed, nextTick, onUnmounted, ref } from 'vue'
import { Minus, Plus } from 'lucide-vue-next'

interface Props
{
  // v-model 绑定值
  modelValue?: number | null
  // 最小值
  min?: number
  // 最大值
  max?: number
  // 步长
  step?: number
  // 精度（小数位数）
  precision?: number
  // 组件尺寸
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl'
  // 禁用状态
  disabled?: boolean
  // 只读状态
  readonly?: boolean
  // 加载状态
  loading?: boolean
  // 占位符
  placeholder?: string
  // 标签
  label?: string
  // 标签位置
  labelPosition?: 'left' | 'right'
  // 是否显示控制按钮
  controls?: boolean
  // 控制按钮位置
  controlsPosition?: 'sides' | 'right'
  // 增加按钮图标
  increaseIcon?: Component | string
  // 减少按钮图标
  decreaseIcon?: Component | string
  // 是否连续改变（长按）
  continuousChange?: boolean
  // 连续改变间隔（毫秒）
  changeInterval?: number
  // 错误信息
  error?: string
  // 是否显示错误
  showError?: boolean
  // 组件 ID
  id?: string
  // 自定义类名
  customClass?: string | string[]
  // 输入框样式变体
  variant?: 'outline' | 'filled' | 'borderless'
  // 颜色主题
  color?: 'primary' | 'success' | 'warning' | 'danger' | 'info' | 'secondary'
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: null,
  step: 1,
  precision: 0,
  size: 'md',
  disabled: false,
  readonly: false,
  loading: false,
  labelPosition: 'left',
  controls: true,
  controlsPosition: 'sides',
  continuousChange: true,
  changeInterval: 150,
  showError: true,
  variant: 'outline',
  color: 'primary'
})

const emit = defineEmits<{
  'update:modelValue': [value: number | null]
  change: [value: number | null, event: Event]
  focus: [event: FocusEvent]
  blur: [event: FocusEvent]
  increase: [value: number | null]
  decrease: [value: number | null]
}>()

const inputRef = ref<HTMLInputElement>()
const continuousTimer = ref<number>()

// 生成唯一 ID
const inputId = computed(() => props.id || `number-${ Math.random().toString(36).substr(2, 9) }`)

// 判断是否到达边界
const isAtMin = computed(() => {
  if (props.min === undefined || props.modelValue === null) {
    return false
  }
  return props.modelValue <= props.min
})

const isAtMax = computed(() => {
  if (props.max === undefined || props.modelValue === null) {
    return false
  }
  return props.modelValue >= props.max
})

// 格式化显示值
const displayValue = computed(() => {
  if (props.modelValue === null || props.modelValue === undefined) {
    return ''
  }
  return Number(props.modelValue.toFixed(props.precision))
})

// 尺寸配置
const sizeConfig = computed(() => {
  const configs = {
    xs: {
      input: 'h-6 px-2 text-xs',
      button: 'h-6 w-6',
      rightButton: 'h-3 w-4',
      icon: 'w-3 h-3',
      rightIcon: 'w-2 h-2',
      loader: 'w-3 h-3',
      label: 'text-xs',
      error: 'text-xs'
    },
    sm: {
      input: 'h-8 px-3 text-sm',
      button: 'h-8 w-8',
      rightButton: 'h-4 w-5',
      icon: 'w-3 h-3',
      rightIcon: 'w-2.5 h-2.5',
      loader: 'w-3 h-3',
      label: 'text-sm',
      error: 'text-xs'
    },
    md: {
      input: 'h-10 px-4 text-sm',
      button: 'h-10 w-10',
      rightButton: 'h-5 w-6',
      icon: 'w-4 h-4',
      rightIcon: 'w-3 h-3',
      loader: 'w-4 h-4',
      label: 'text-sm',
      error: 'text-sm'
    },
    lg: {
      input: 'h-12 px-4 text-base',
      button: 'h-12 w-12',
      rightButton: 'h-6 w-7',
      icon: 'w-4 h-4',
      rightIcon: 'w-3 h-3',
      loader: 'w-4 h-4',
      label: 'text-base',
      error: 'text-sm'
    },
    xl: {
      input: 'h-14 px-6 text-lg',
      button: 'h-14 w-14',
      rightButton: 'h-7 w-8',
      icon: 'w-5 h-5',
      rightIcon: 'w-3.5 h-3.5',
      loader: 'w-5 h-5',
      label: 'text-lg',
      error: 'text-base'
    }
  }
  return configs[props.size]
})

// 颜色配置
const colorConfig = computed(() => {
  const configs = {
    primary: {
      ring: 'focus:ring-blue-500',
      border: 'border-blue-500',
      button: 'text-gray-600',
      buttonBorder: 'border-blue-500'
    },
    success: {
      ring: 'focus:ring-green-500',
      border: 'border-green-500',
      button: 'text-gray-600',
      buttonBorder: 'border-green-500'
    },
    warning: {
      ring: 'focus:ring-yellow-500',
      border: 'border-yellow-500',
      button: 'text-gray-600',
      buttonBorder: 'border-yellow-500'
    },
    danger: {
      ring: 'focus:ring-red-500',
      border: 'border-red-500',
      button: 'text-gray-600',
      buttonBorder: 'border-red-500'
    },
    info: {
      ring: 'focus:ring-cyan-500',
      border: 'border-cyan-500',
      button: 'text-gray-600',
      buttonBorder: 'border-cyan-500'
    },
    secondary: {
      ring: 'focus:ring-gray-500',
      border: 'border-gray-500',
      button: 'text-gray-600',
      buttonBorder: 'border-gray-500'
    }
  }
  return configs[props.color]
})

// 包装器样式
const wrapperClasses = computed(() => [
  'relative inline-flex items-center gap-3',
  ...(Array.isArray(props.customClass) ? props.customClass : [props.customClass]).filter(Boolean)
])

// 容器样式
const containerClasses = computed(() => [
  'inline-flex items-center',
  props.variant === 'outline' ? 'border rounded-md' : '',
  props.variant === 'filled' ? 'bg-gray-50 border border-gray-200 rounded-md' : '',
  props.variant === 'borderless' ? 'border-b' : '',
  props.disabled ? 'opacity-50 cursor-not-allowed' : '',
  props.error ? 'border-red-500' : 'border-gray-300 dark:border-gray-600',
  'dark:text-gray-100',
  colorConfig.value.ring
])

// 输入框样式
const inputClasses = computed(() => [
  sizeConfig.value.input,
  'bg-transparent border-none outline-none flex-1 min-w-0',
  props.controls && props.controlsPosition === 'right' ? 'pr-8' : '',
  props.disabled ? 'cursor-not-allowed' : '',
  'text-center'
])

// 按钮样式（左右布局）
const decreaseButtonClasses = computed(() => [
  'w-10', // 固定宽度
  colorConfig.value.button,
  props.disabled || isAtMin.value ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'
])

const increaseButtonClasses = computed(() => [
  'w-10', // 固定宽度
  colorConfig.value.button,
  props.disabled || isAtMax.value ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'
])

// 右侧控制按钮样式
const rightControlButtonClasses = computed(() => [
  sizeConfig.value.rightButton,
  'rounded text-gray-600',
  props.disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'
])

// 图标样式
const iconClasses = computed(() => [
  sizeConfig.value.icon
])

const rightIconClasses = computed(() => [
  sizeConfig.value.rightIcon
])

// 标签样式
const labelClasses = computed(() => [
  sizeConfig.value.label,
  'font-medium',
  props.disabled ? 'text-gray-400' : 'text-gray-700'
])

// 错误信息样式
const errorClasses = computed(() => [
  sizeConfig.value.error,
  'text-red-500'
])

// 加载器样式
const loaderClasses = computed(() => [
  sizeConfig.value.loader,
  'border-gray-300 border-t-blue-600'
])

// 格式化数值
const formatValue = (value: number): number => {
  if (props.precision === 0) {
    return Math.round(value)
  }
  return Number(value.toFixed(props.precision))
}

// 验证数值范围
const validateValue = (value: number): number => {
  if (props.min !== undefined && value < props.min) {
    return props.min
  }
  if (props.max !== undefined && value > props.max) {
    return props.max
  }
  return value
}

// 增加
const increase = () => {
  if (props.disabled || props.loading || isAtMax.value) {
    return
  }

  const currentValue = props.modelValue || 0
  const newValue = formatValue(validateValue(currentValue + props.step))

  emit('update:modelValue', newValue)
  emit('increase', newValue)
  emit('change', newValue, new Event('change'))
}

// 减少
const decrease = () => {
  if (props.disabled || props.loading || isAtMin.value) {
    return
  }

  const currentValue = props.modelValue || 0
  const newValue = formatValue(validateValue(currentValue - props.step))

  emit('update:modelValue', newValue)
  emit('decrease', newValue)
  emit('change', newValue, new Event('change'))
}

// 开始连续改变
const startContinuousChange = (type: 'increase' | 'decrease') => {
  if (!props.continuousChange) {
    return
  }

  const action = type === 'increase' ? increase : decrease

  continuousTimer.value = window.setInterval(() => {
    action()
  }, props.changeInterval)
}

// 停止连续改变
const stopContinuousChange = () => {
  if (continuousTimer.value) {
    clearInterval(continuousTimer.value)
    continuousTimer.value = undefined
  }
}

// 处理输入
const handleInput = (event: Event) => {
  const target = event.target as HTMLInputElement
  const value = target.value

  if (value === '' || value === '-') {
    emit('update:modelValue', null)
    return
  }

  const numValue = Number(value)
  if (!isNaN(numValue)) {
    const formattedValue = formatValue(validateValue(numValue))
    emit('update:modelValue', formattedValue)
    emit('change', formattedValue, event)
  }
}

// 处理失焦
const handleBlur = (event: FocusEvent) => {
  stopContinuousChange()
  emit('blur', event)

  // 格式化显示值
  if (props.modelValue !== null) {
    nextTick(() => {
      if (inputRef.value) {
        inputRef.value.value = displayValue.value.toString()
      }
    })
  }
}

// 处理聚焦
const handleFocus = (event: FocusEvent) => {
  emit('focus', event)
}

// 清理定时器
const cleanup = () => {
  stopContinuousChange()
}

onUnmounted(cleanup)
</script>
