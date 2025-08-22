<template>
  <div :class="wrapperClasses" class="inline-flex items-center">
    <!-- Switch 主体 -->
    <button :id="id"
            :disabled="disabled || loading"
            :class="switchClasses"
            :aria-checked="modelValue"
            :aria-describedby="describedBy"
            role="switch"
            type="button"
            class="relative inline-flex shrink-0 cursor-pointer transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
            @click="handleToggle"
            v-bind="$attrs">

      <!-- 加载状态 -->
      <div v-if="loading" :class="loaderClasses" class="absolute inset-0 flex items-center justify-center">
        <div class="animate-spin rounded-full border-2 border-current border-t-transparent opacity-60"></div>
      </div>

      <!-- 滑块 -->
      <span :class="thumbClasses"
            class="pointer-events-none inline-block transform rounded-full bg-white shadow-lg ring-0 transition duration-200 ease-in-out">

        <!-- 选中状态图标 -->
        <component v-if="checkedIcon && modelValue && !loading"
                   :is="checkedIcon"
                   :class="iconClasses"
                   class="absolute inset-0 flex items-center justify-center text-current"/>

        <!-- 未选中状态图标 -->
        <component v-else-if="uncheckedIcon && !modelValue && !loading"
                   :is="uncheckedIcon"
                   :class="iconClasses"
                   class="absolute inset-0 flex items-center justify-center text-current"/>
      </span>
    </button>

    <!-- 标签文本 -->
    <label v-if="label || $slots.default"
           :for="id"
           :class="labelClasses"
           class="cursor-pointer select-none">
      <slot>{{ label }}</slot>
    </label>
  </div>
</template>

<script setup lang="ts">
import type { Component } from 'vue'
import { computed } from 'vue'

interface Props
{
  // v-model 绑定值
  modelValue?: boolean
  // 开关尺寸
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl'
  // 开关颜色主题
  color?: 'primary' | 'success' | 'warning' | 'danger' | 'info' | 'secondary'
  // 禁用状态
  disabled?: boolean
  // 加载状态
  loading?: boolean
  // 标签文本
  label?: string
  // 标签位置
  labelPosition?: 'left' | 'right'
  // 选中状态图标
  checkedIcon?: Component | string
  // 未选中状态图标
  uncheckedIcon?: Component | string
  // 自定义选中颜色
  checkedColor?: string
  // 自定义未选中颜色
  uncheckedColor?: string
  // 组件 ID
  id?: string
  // aria-describedby
  describedBy?: string
  // 自定义类名
  customClass?: string | string[]
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: false,
  size: 'md',
  color: 'primary',
  disabled: false,
  loading: false,
  labelPosition: 'right'
})

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  change: [value: boolean, event: Event]
}>()

// 包装器样式
const wrapperClasses = computed(() => [
  'inline-flex items-center gap-3',
  props.labelPosition === 'left' ? 'flex-row-reverse' : 'flex-row',
  ...(Array.isArray(props.customClass) ? props.customClass : [props.customClass]).filter(Boolean)
])

// 尺寸配置
const sizeConfig = computed(() => {
  const configs = {
    xs: {
      switch: 'h-4 w-7',
      thumb: 'h-3 w-3',
      translate: 'translate-x-3',
      loader: 'w-2 h-2 border-[1px]',
      icon: 'w-2 h-2',
      label: 'text-xs'
    },
    sm: {
      switch: 'h-5 w-9',
      thumb: 'h-4 w-4',
      translate: 'translate-x-4',
      loader: 'w-2.5 h-2.5 border-[1px]',
      icon: 'w-2.5 h-2.5',
      label: 'text-sm'
    },
    md: {
      switch: 'h-6 w-11',
      thumb: 'h-5 w-5',
      translate: 'translate-x-5',
      loader: 'w-3 h-3 border-[1.5px]',
      icon: 'w-3 h-3',
      label: 'text-sm'
    },
    lg: {
      switch: 'h-7 w-12',
      thumb: 'h-6 w-6',
      translate: 'translate-x-5',
      loader: 'w-3.5 h-3.5 border-[1.5px]',
      icon: 'w-3.5 h-3.5',
      label: 'text-base'
    },
    xl: {
      switch: 'h-8 w-14',
      thumb: 'h-7 w-7',
      translate: 'translate-x-6',
      loader: 'w-4 h-4 border-2',
      icon: 'w-4 h-4',
      label: 'text-base'
    }
  }
  return configs[props.size]
})

// 颜色配置
const colorConfig = computed(() => {
  if (props.checkedColor && props.uncheckedColor) {
    return {
      checked: props.checkedColor,
      unchecked: props.uncheckedColor,
      focus: 'ring-blue-500'
    }
  }

  const configs = {
    primary: {
      checked: 'bg-blue-600',
      unchecked: 'bg-gray-200',
      focus: 'focus:ring-blue-500'
    },
    success: {
      checked: 'bg-green-600',
      unchecked: 'bg-gray-200',
      focus: 'focus:ring-green-500'
    },
    warning: {
      checked: 'bg-yellow-500',
      unchecked: 'bg-gray-200',
      focus: 'focus:ring-yellow-500'
    },
    danger: {
      checked: 'bg-red-600',
      unchecked: 'bg-gray-200',
      focus: 'focus:ring-red-500'
    },
    info: {
      checked: 'bg-cyan-600',
      unchecked: 'bg-gray-200',
      focus: 'focus:ring-cyan-500'
    },
    secondary: {
      checked: 'bg-gray-600',
      unchecked: 'bg-gray-200',
      focus: 'focus:ring-gray-500'
    }
  }
  return configs[props.color]
})

// Switch 主体样式
const switchClasses = computed(() => [
  sizeConfig.value.switch,
  props.modelValue ? colorConfig.value.checked : colorConfig.value.unchecked,
  colorConfig.value.focus,
  'rounded-full border-2 border-transparent'
])

// 滑块样式
const thumbClasses = computed(() => [
  sizeConfig.value.thumb,
  props.modelValue ? sizeConfig.value.translate : 'translate-x-0'
])

// 加载器样式
const loaderClasses = computed(() => [
  sizeConfig.value.loader
])

// 图标样式
const iconClasses = computed(() => [
  sizeConfig.value.icon,
  props.modelValue
      ? (props.color === 'warning' ? 'text-yellow-600' : 'text-white')
      : 'text-gray-400'
])

// 标签样式
const labelClasses = computed(() => [
  sizeConfig.value.label,
  props.disabled ? 'text-gray-400' : 'text-gray-700',
  'font-medium'
])

// 切换处理
const handleToggle = (event: Event) => {
  if (props.disabled || props.loading) {
    return
  }

  const newValue = !props.modelValue
  emit('update:modelValue', newValue)
  emit('change', newValue, event)
}
</script>
