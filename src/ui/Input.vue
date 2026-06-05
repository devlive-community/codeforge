<template>
  <div :class="wrapperClasses" class="inline-flex items-center">
    <!-- 标签 -->
    <label
        v-if="label && labelPosition === 'left'"
        :for="inputId"
        :class="labelClasses"
        class="select-none">
      {{ label }}
    </label>

    <!-- 输入框容器 -->
    <div :class="containerClasses" class="relative inline-flex items-center">
      <!-- 前置图标或插槽 -->
      <div v-if="prefixIcon || $slots.prefix" class="absolute left-3 flex items-center pointer-events-none z-10">
        <slot name="prefix">
          <component v-if="prefixIcon" :is="prefixIcon" :class="iconClasses"/>
        </slot>
      </div>

      <!-- 输入框 -->
      <input
          :id="inputId"
          ref="inputRef"
          :value="modelValue"
          :type="inputType"
          :disabled="disabled || loading"
          :readonly="readonly"
          :placeholder="placeholder"
          :maxlength="maxLength"
          :minlength="minLength"
          :pattern="pattern"
          :autocomplete="autocomplete"
          :class="inputClasses"
          class="transition-all duration-200 focus:outline-none w-full"
          @input="handleInput"
          @change="handleChange"
          @blur="handleBlur"
          @focus="handleFocus"
          @keydown="handleKeydown"
          @keyup="handleKeyup"
          @keypress="handleKeypress"
          v-bind="$attrs"/>

      <!-- 清除按钮 -->
      <button
          v-if="clearable && modelValue && !disabled && !readonly"
          type="button"
          :class="clearButtonClasses"
          class="absolute right-3 flex items-center justify-center transition-all duration-200 hover:bg-gray-100 rounded-full"
          @click="handleClear">
        <component :is="clearIcon || X" :class="iconClasses"/>
      </button>

      <!-- 后置图标或插槽（当没有清除按钮时） -->
      <div v-else-if="suffixIcon || $slots.suffix" class="absolute right-3 flex items-center pointer-events-none z-10">
        <slot name="suffix">
          <component v-if="suffixIcon" :is="suffixIcon" :class="iconClasses"/>
        </slot>
      </div>

      <!-- 密码显示切换按钮 -->
      <button
          v-if="type === 'password' && showPasswordToggle"
          type="button"
          :class="passwordToggleClasses"
          class="absolute right-3 flex items-center justify-center transition-all duration-200 hover:bg-gray-100 rounded-full"
          @click="togglePasswordVisibility">
        <component :is="passwordVisible ? EyeOff : Eye" :class="iconClasses"/>
      </button>

      <!-- 加载状态 -->
      <div v-if="loading" class="absolute right-3 flex items-center">
        <div :class="loaderClasses" class="animate-spin rounded-full border-2 border-current border-t-transparent"></div>
      </div>

      <!-- 字符计数 -->
      <div v-if="showCount && maxLength" :class="countClasses" class="absolute right-3 bottom-1 text-xs">
        {{ characterCount }}/{{ maxLength }}
      </div>
    </div>

    <!-- 标签 -->
    <label
        v-if="label && labelPosition === 'right'"
        :for="inputId"
        :class="labelClasses"
        class="select-none">
      {{ label }}
    </label>
  </div>
</template>

<script setup lang="ts">
import type { Component } from 'vue'
// 获取插槽
import { computed, ref, useSlots } from 'vue'
import { Eye, EyeOff, X } from 'lucide-vue-next'

interface Props
{
  // v-model 绑定值
  modelValue?: string | number | null
  // 输入框类型
  type?: 'text' | 'password' | 'email' | 'number' | 'tel' | 'url' | 'search'
  // 组件尺寸
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl'
  // 样式变体
  variant?: 'outline' | 'filled' | 'borderless' | 'underline'
  // 颜色主题
  color?: 'primary' | 'success' | 'warning' | 'danger' | 'info' | 'secondary'
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
  // 最大长度
  maxLength?: number
  // 最小长度
  minLength?: number
  // 正则模式
  pattern?: string
  // 自动完成
  autocomplete?: string
  // 是否可清除
  clearable?: boolean
  // 是否显示字符计数
  showCount?: boolean
  // 密码显示切换
  showPasswordToggle?: boolean
  // 前置图标
  prefixIcon?: Component | string
  // 后置图标
  suffixIcon?: Component | string
  // 清除图标
  clearIcon?: Component | string
  // 组件 ID
  id?: string
  // 自定义类名
  customClass?: string | string[]
  // 圆角
  rounded?: boolean | 'full'
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: '',
  type: 'text',
  size: 'md',
  variant: 'outline',
  color: 'primary',
  disabled: false,
  readonly: false,
  loading: false,
  labelPosition: 'left',
  clearable: false,
  showCount: false,
  showPasswordToggle: true,
  rounded: false
})

const emit = defineEmits<{
  'update:modelValue': [value: string | number | null]
  change: [value: string | number | null, event: Event]
  input: [value: string | number | null, event: Event]
  focus: [event: FocusEvent]
  blur: [event: FocusEvent]
  keydown: [event: KeyboardEvent]
  keyup: [event: KeyboardEvent]
  keypress: [event: KeyboardEvent]
  clear: []
}>()

const inputRef = ref<HTMLInputElement>()
const passwordVisible = ref(false)

// 生成唯一 ID
const inputId = computed(() => props.id || `input-${ Math.random().toString(36).substr(2, 9) }`)

// 输入框类型（处理密码显示切换）
const inputType = computed(() => {
  if (props.type === 'password') {
    return passwordVisible.value ? 'text' : 'password'
  }
  return props.type
})

// 字符计数
const characterCount = computed(() => {
  const value = props.modelValue
  if (value === null || value === undefined) {
    return 0
  }
  return String(value).length
})

// 尺寸配置
const sizeConfig = computed(() => {
  const configs = {
    xs: {
      input: 'h-6 px-2 text-xs',
      withIcon: 'pl-8 pr-8',
      withPrefix: 'pl-8',
      withSuffix: 'pr-8',
      icon: 'w-3 h-3',
      loader: 'w-3 h-3',
      button: 'w-5 h-5',
      label: 'text-xs',
      count: 'text-xs'
    },
    sm: {
      input: 'h-8 px-3 text-sm',
      withIcon: 'pl-9 pr-9',
      withPrefix: 'pl-9',
      withSuffix: 'pr-9',
      icon: 'w-3 h-3',
      loader: 'w-3 h-3',
      button: 'w-6 h-6',
      label: 'text-sm',
      count: 'text-xs'
    },
    md: {
      input: 'h-10 px-4 text-sm',
      withIcon: 'pl-10 pr-10',
      withPrefix: 'pl-10',
      withSuffix: 'pr-10',
      icon: 'w-4 h-4',
      loader: 'w-4 h-4',
      button: 'w-6 h-6',
      label: 'text-sm',
      count: 'text-xs'
    },
    lg: {
      input: 'h-12 px-4 text-base',
      withIcon: 'pl-11 pr-11',
      withPrefix: 'pl-11',
      withSuffix: 'pr-11',
      icon: 'w-4 h-4',
      loader: 'w-4 h-4',
      button: 'w-7 h-7',
      label: 'text-base',
      count: 'text-sm'
    },
    xl: {
      input: 'h-14 px-6 text-lg',
      withIcon: 'pl-12 pr-12',
      withPrefix: 'pl-12',
      withSuffix: 'pr-12',
      icon: 'w-5 h-5',
      loader: 'w-5 h-5',
      button: 'w-8 h-8',
      label: 'text-lg',
      count: 'text-sm'
    }
  }
  return configs[props.size]
})

// 颜色配置
const colorConfig = computed(() => {
  const configs = {
    primary: {
      border: 'border-gray-300 focus:border-blue-500 focus:ring-blue-500',
      filled: 'bg-blue-50 border-blue-200 focus:border-blue-500 focus:ring-blue-500',
      underline: 'border-b-gray-300 focus:border-b-blue-500',
      error: 'border-red-500 focus:border-red-500 focus:ring-red-500'
    },
    success: {
      border: 'border-gray-300 focus:border-green-500 focus:ring-green-500',
      filled: 'bg-green-50 border-green-200 focus:border-green-500 focus:ring-green-500',
      underline: 'border-b-gray-300 focus:border-b-green-500',
      error: 'border-green-500 focus:border-green-500 focus:ring-green-500'
    },
    warning: {
      border: 'border-gray-300 focus:border-yellow-500 focus:ring-yellow-500',
      filled: 'bg-yellow-50 border-yellow-200 focus:border-yellow-500 focus:ring-yellow-500',
      underline: 'border-b-gray-300 focus:border-b-yellow-500',
      error: 'border-yellow-500 focus:border-yellow-500 focus:ring-yellow-500'
    },
    danger: {
      border: 'border-gray-300 focus:border-red-500 focus:ring-red-500',
      filled: 'bg-red-50 border-red-200 focus:border-red-500 focus:ring-red-500',
      underline: 'border-b-gray-300 focus:border-b-red-500',
      error: 'border-red-500 focus:border-red-500 focus:ring-red-500'
    },
    info: {
      border: 'border-gray-300 focus:border-cyan-500 focus:ring-cyan-500',
      filled: 'bg-cyan-50 border-cyan-200 focus:border-cyan-500 focus:ring-cyan-500',
      underline: 'border-b-gray-300 focus:border-b-cyan-500',
      error: 'border-cyan-500 focus:border-cyan-500 focus:ring-cyan-500'
    },
    secondary: {
      border: 'border-gray-300 focus:border-gray-500 focus:ring-gray-500',
      filled: 'bg-gray-50 border-gray-200 focus:border-gray-500 focus:ring-gray-500',
      underline: 'border-b-gray-300 focus:border-b-gray-500',
      error: 'border-gray-500 focus:border-gray-500 focus:ring-gray-500'
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
  'relative inline-flex items-center w-full',
  props.disabled ? 'cursor-not-allowed' : ''
])

// 输入框样式
const inputClasses = computed(() => {
  const classes = [sizeConfig.value.input]

  // 基础样式
  classes.push('bg-white dark:bg-gray-800 dark:text-gray-100 dark:border-gray-600 dark:placeholder:text-gray-500 border')

  // 变体样式
  switch (props.variant) {
    case 'filled':
      classes.push('bg-gray-50', colorConfig.value.filled)
      break
    case 'borderless':
      classes.push('border-0 bg-transparent')
      break
    case 'underline':
      classes.push('border-0 border-b-2 rounded-none bg-transparent', colorConfig.value.underline)
      break
    default: // outline
      classes.push(colorConfig.value.border)
  }

  // 圆角
  if (props.variant !== 'underline') {
    if (props.rounded === 'full') {
      classes.push('rounded-full')
    }
    else if (props.rounded) {
      classes.push('rounded-lg')
    }
    else {
      classes.push('rounded-md')
    }
  }

  // 前后图标间距调整
  const hasPrefix = props.prefixIcon || !!slots.prefix
  const hasSuffix = props.suffixIcon || !!slots.suffix || props.clearable || (props.type === 'password' && props.showPasswordToggle) || props.loading

  if (hasPrefix && hasSuffix) {
    classes.push(sizeConfig.value.withIcon)
  }
  else if (hasPrefix) {
    classes.push(sizeConfig.value.withPrefix)
  }
  else if (hasSuffix) {
    classes.push(sizeConfig.value.withSuffix)
  }

  // 禁用状态
  if (props.disabled) {
    classes.push('cursor-not-allowed opacity-50 bg-gray-50')
  }

  // 只读状态
  if (props.readonly) {
    classes.push('cursor-default bg-gray-50')
  }

  return classes
})

// 图标样式
const iconClasses = computed(() => [
  sizeConfig.value.icon,
  'text-gray-400'
])

// 清除按钮样式
const clearButtonClasses = computed(() => [
  sizeConfig.value.button,
  'text-gray-400 hover:text-gray-600 focus:outline-none'
])

// 密码切换按钮样式
const passwordToggleClasses = computed(() => [
  sizeConfig.value.button,
  'text-gray-400 hover:text-gray-600 focus:outline-none'
])

// 标签样式
const labelClasses = computed(() => [
  sizeConfig.value.label,
  'font-medium',
  props.disabled ? 'text-gray-400' : 'text-gray-700'
])

// 加载器样式
const loaderClasses = computed(() => [
  sizeConfig.value.loader,
  'border-gray-300 border-t-blue-600'
])

// 字符计数样式
const countClasses = computed(() => [
  sizeConfig.value.count,
  characterCount.value > (props.maxLength || 0) ? 'text-red-500' : 'text-gray-400'
])

const slots = useSlots()

// 处理输入
const handleInput = (event: Event) => {
  const target = event.target as HTMLInputElement
  const value = target.value

  emit('update:modelValue', value)
  emit('input', value, event)
}

// 处理改变
const handleChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  const value = target.value

  emit('change', value, event)
}

// 处理聚焦
const handleFocus = (event: FocusEvent) => {
  emit('focus', event)
}

// 处理失焦
const handleBlur = (event: FocusEvent) => {
  emit('blur', event)
}

// 处理键盘事件
const handleKeydown = (event: KeyboardEvent) => {
  emit('keydown', event)
}

const handleKeyup = (event: KeyboardEvent) => {
  emit('keyup', event)
}

const handleKeypress = (event: KeyboardEvent) => {
  emit('keypress', event)
}

// 清除输入
const handleClear = () => {
  emit('update:modelValue', '')
  emit('clear')
  inputRef.value?.focus()
}

// 切换密码可见性
const togglePasswordVisibility = () => {
  passwordVisible.value = !passwordVisible.value
}

// 聚焦方法
const focus = () => {
  inputRef.value?.focus()
}

// 失焦方法
const blur = () => {
  inputRef.value?.blur()
}

// 选择文本
const select = () => {
  inputRef.value?.select()
}

// 暴露方法
defineExpose({
  focus,
  blur,
  select,
  inputRef
})
</script>
