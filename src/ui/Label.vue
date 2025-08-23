<template>
  <div :class="wrapperClasses" class="flex">
    <!-- 标签 -->
    <label v-if="label" :for="forId" :class="labelClasses" class="block font-medium">
      {{ label }}
      <span v-if="required" class="text-red-500 ml-1">*</span>
    </label>

    <!-- 描述文字 -->
    <p v-if="description" :class="descriptionClasses" class="mt-1">
      {{ description }}
    </p>

    <!-- 表单控件容器 -->
    <div :class="controlClasses">
      <slot :id="forId"/>
    </div>

    <!-- 错误信息 -->
    <div v-if="error && showError" :class="errorClasses" class="mt-1">
      <span class="text-red-500 text-sm">{{ error }}</span>
    </div>

    <!-- 帮助文本 -->
    <div v-if="helpText" :class="helpClasses" class="mt-1">
      <span class="text-gray-500 text-sm">{{ helpText }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props
{
  // 标签文本
  label?: string
  // 描述文本
  description?: string
  // 是否必填
  required?: boolean
  // 错误信息
  error?: string
  // 是否显示错误
  showError?: boolean
  // 帮助文本
  helpText?: string
  // 布局方向
  direction?: 'vertical' | 'horizontal'
  // 标签位置（仅横向布局有效）
  labelPosition?: 'left' | 'right' | 'top'
  // 标签宽度（仅横向布局有效）
  labelWidth?: string | number
  // 标签对齐方式
  labelAlign?: 'left' | 'center' | 'right'
  // 尺寸
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl'
  // 间距
  gap?: 'xs' | 'sm' | 'md' | 'lg' | 'xl'
  // 自定义 ID
  id?: string
  // 自定义类名
  customClass?: string | string[]
  // 暗色主题
  dark?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  direction: 'vertical',
  labelPosition: 'top',
  labelAlign: 'left',
  size: 'md',
  gap: 'sm',
  showError: true,
  dark: false
})

// 生成唯一 ID
const forId = computed(() => props.id || `label-${ Math.random().toString(36).substr(2, 9) }`)

// 尺寸配置
const sizeConfig = computed(() => {
  const configs = {
    xs: {
      label: 'text-xs',
      description: 'text-xs',
      error: 'text-xs',
      help: 'text-xs'
    },
    sm: {
      label: 'text-sm',
      description: 'text-xs',
      error: 'text-xs',
      help: 'text-xs'
    },
    md: {
      label: 'text-sm',
      description: 'text-sm',
      error: 'text-sm',
      help: 'text-sm'
    },
    lg: {
      label: 'text-base',
      description: 'text-sm',
      error: 'text-sm',
      help: 'text-sm'
    },
    xl: {
      label: 'text-lg',
      description: 'text-base',
      error: 'text-sm',
      help: 'text-sm'
    }
  }
  return configs[props.size]
})

// 间距配置
const gapConfig = computed(() => {
  const configs = {
    xs: 'gap-1',
    sm: 'gap-2',
    md: 'gap-3',
    lg: 'gap-4',
    xl: 'gap-6'
  }
  return configs[props.gap]
})

// 包装器样式
const wrapperClasses = computed(() => {
  const classes = []

  // 基础样式
  if (props.direction === 'vertical') {
    classes.push('flex-col')
  }
  else {
    classes.push('flex-row items-start')
    if (props.labelPosition === 'right') {
      classes.push('flex-row-reverse')
    }
  }

  // 间距
  classes.push(gapConfig.value)

  // 自定义类名
  if (Array.isArray(props.customClass)) {
    classes.push(...props.customClass)
  }
  else if (props.customClass) {
    classes.push(props.customClass)
  }

  return classes
})

// 标签样式
const labelClasses = computed(() => {
  const classes = [sizeConfig.value.label]

  // 颜色主题
  if (props.dark) {
    classes.push('text-gray-300')
  }
  else {
    classes.push('text-gray-700')
  }

  // 对齐方式
  if (props.labelAlign === 'center') {
    classes.push('text-center')
  }
  else if (props.labelAlign === 'right') {
    classes.push('text-right')
  }

  // 横向布局时的宽度和间距
  if (props.direction === 'horizontal') {
    if (props.labelWidth) {
      classes.push('flex-shrink-0')
    }
    else {
      classes.push('min-w-0 flex-shrink-0')
    }
  }

  return classes
})

// 描述样式
const descriptionClasses = computed(() => [
  sizeConfig.value.description,
  props.dark ? 'text-gray-400' : 'text-gray-500'
])

// 控件容器样式
const controlClasses = computed(() => {
  const classes = []

  if (props.direction === 'horizontal') {
    classes.push('flex-1 min-w-0')
  }
  else {
    classes.push('w-full')
  }

  return classes
})

// 错误信息样式
const errorClasses = computed(() => [
  sizeConfig.value.error
])

// 帮助文本样式
const helpClasses = computed(() => [
  sizeConfig.value.help
])
</script>
