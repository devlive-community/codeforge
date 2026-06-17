<template>
  <div :class="containerClasses">
    <!-- Tab 头部导航 -->
    <div :class="headerClasses">
      <!-- 导航头部插槽（如筛选框，位于标签列表上方） -->
      <div v-if="$slots['nav-header']" class="w-full mb-2">
        <slot name="nav-header"/>
      </div>

      <div v-if="tabs.length > 0 || addable" :class="navClasses" ref="tabsNav">
        <!-- 滑动指示器 -->
        <div v-if="showIndicator && type === 'line'" :class="indicatorClasses" :style="indicatorStyle"/>

        <!-- Tab 按钮 -->
        <button v-for="(tab, index) in tabs"
                :key="tab.key || tab.name || index"
                ref="tabButtons"
                @click="selectTab(tab, index)"
                @keydown.enter="selectTab(tab, index)"
                @keydown.space.prevent="selectTab(tab, index)"
                @keydown.arrow-left.prevent="selectPrevTab"
                @keydown.arrow-right.prevent="selectNextTab"
                :class="getTabButtonClasses(tab, index)"
                :disabled="tab.disabled"
                :aria-selected="isActiveTab(tab)"
                :aria-controls="`tabpanel-${tab.key || tab.name || index}`"
                role="tab"
                :tabindex="isActiveTab(tab) ? 0 : -1">
          <slot name="tab-button" :tab="tab" :index="index" :isActive="isActiveTab(tab)">
            <component v-if="tab.icon && !tab.svgIcon" :is="tab.icon" :class="iconClasses"/>

            <div v-else-if="tab.svgIcon" v-html="tab.svgIcon" :class="iconClasses"/>

            <img v-else-if="tab.svgUrl" :src="tab.svgUrl" :class="iconClasses" alt="icon"/>

            <span v-if="tab.label || tab.name" class="tab-text">
              {{ tab.label || tab.name }}
            </span>

            <span v-if="tab.badge !== undefined && tab.badge !== null" :class="badgeClasses">
              {{ tab.badge }}
            </span>

            <button v-if="tab.closable && closable" @click.stop="closeTab(tab, index)" :class="closeButtonClasses" :aria-label="`${t('ui.close')} ${tab.label || tab.name}`">
              <X class="w-3 h-3"/>
            </button>
          </slot>
        </button>

        <!-- 添加按钮 -->
        <button v-if="addable" @click="$emit('add')" :class="addButtonClasses" :aria-label="t('ui.addTab')">
          <Plus class="w-4 h-4"/>
        </button>
      </div>

      <!-- 额外操作区域 -->
      <div v-if="$slots.extra" class="tabs-extra">
        <slot name="extra"/>
      </div>
    </div>

    <!-- Tab 内容区域 -->
    <div :class="contentClasses">
      <div v-for="(tab, index) in tabs"
           :key="`content-${tab.key || tab.name || index}`"
           v-show="isActiveTab(tab)"
           :class="paneClasses"
           :id="`tabpanel-${tab.key || tab.name || index}`"
           role="tabpanel"
           :aria-labelledby="`tab-${tab.key || tab.name || index}`">
        <!-- 使用插槽内容 -->
        <slot :name="tab.key || tab.name" :tab="tab" :index="index">
          <!-- 回退到 tab.content -->
          <component v-if="tab.content" :is="tab.content"/>
          <div v-else-if="tab.html" v-html="tab.html"></div>
          <div v-else>{{ tab.text || t('ui.empty') }}</div>
        </slot>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Component } from 'vue'
import { computed, nextTick, onMounted, ref, watch } from 'vue'
import { Plus, X } from 'lucide-vue-next'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

// Tab 接口定义
export interface Tab
{
  key?: string
  name?: string
  label?: string
  icon?: Component
  disabled?: boolean
  closable?: boolean
  badge?: string | number
  content?: Component
  html?: string
  text?: string
  svgIcon?: string
  svgUrl?: string

  [key: string]: any
}

// Props 定义
interface Props
{
  // 数据
  tabs: Tab[]
  modelValue?: string | number

  // 外观
  type?: 'line' | 'card' | 'button' | 'segment'
  size?: 'sm' | 'md' | 'lg'
  position?: 'top' | 'bottom' | 'left' | 'right'

  // 功能
  closable?: boolean
  addable?: boolean
  scrollable?: boolean
  centered?: boolean

  // 样式
  showIndicator?: boolean
  animated?: boolean

  // 自定义类名
  containerClass?: string | string[]
  headerClass?: string | string[]
  navClass?: string | string[]
  tabButtonClass?: string | string[]
  contentClass?: string | string[]
  paneClass?: string | string[]
  indicatorClass?: string | string[]
  iconClass?: string | string[]
  badgeClass?: string | string[]
  closeButtonClass?: string | string[]
  addButtonClass?: string | string[]
}

const props = withDefaults(defineProps<Props>(), {
  type: 'line',
  size: 'md',
  position: 'top',
  closable: false,
  addable: false,
  scrollable: false,
  centered: false,
  showIndicator: true,
  animated: true
})

// Emits 定义
const emit = defineEmits<{
  'update:modelValue': [value: string | number]
  'change': [activeKey: string | number, tab: Tab]
  'close': [tab: Tab, index: number]
  'add': []
}>()

// 响应式数据
const tabsNav = ref<HTMLElement>()
const tabButtons = ref<HTMLElement[]>([])
const activeTabIndex = ref(0)
const indicatorStyle = ref({})

// 计算属性
const containerClasses = computed(() => {
  const baseClasses = ['w-full']

  // 位置相关
  if (props.position === 'bottom') {
    baseClasses.push('flex', 'flex-col-reverse')
  }
  else if (props.position === 'left' || props.position === 'right') {
    baseClasses.push('flex')
  }

  // 自定义类名
  if (props.containerClass) {
    const customClasses = Array.isArray(props.containerClass) ? props.containerClass : [props.containerClass]
    baseClasses.push(...customClasses.filter(Boolean))
  }

  return baseClasses
})

const headerClasses = computed(() => {
  const baseClasses = ['flex', 'items-center']

  // 位置相关边框
  if (props.position === 'top') {
    baseClasses.push('justify-between', 'border-gray-200', 'dark:border-gray-700')
  }
  else if (props.position === 'bottom') {
    baseClasses.push('justify-between', 'border-t', 'border-gray-200', 'dark:border-gray-700')
  }
  else if (props.position === 'left') {
    // 纵向排列时从顶部依次堆叠，避免 justify-between 把导航头部与列表拉开
    baseClasses.push('justify-start', 'flex-col', 'border-gray-200', 'dark:border-gray-700', 'mr-4', 'border-b-0')
  }
  else if (props.position === 'right') {
    baseClasses.push('justify-start', 'flex-col', 'border-l', 'border-gray-200', 'dark:border-gray-700', 'ml-4', 'order-2', 'border-b-0')
  }

  // 自定义类名
  if (props.headerClass) {
    const customClasses = Array.isArray(props.headerClass) ? props.headerClass : [props.headerClass]
    baseClasses.push(...customClasses.filter(Boolean))
  }

  return baseClasses
})

const navClasses = computed(() => {
  const baseClasses = ['relative', 'flex', 'items-center']

  // 位置相关
  if (props.position === 'left' || props.position === 'right') {
    baseClasses.push('flex-col')
  }

  // 滚动
  if (props.scrollable) {
    baseClasses.push('overflow-x-auto')
    // 隐藏滚动条
    baseClasses.push('[&::-webkit-scrollbar]:hidden', '[-ms-overflow-style:none]', '[scrollbar-width:none]')
  }

  // 居中
  if (props.centered) {
    baseClasses.push('justify-center')
  }

  // 类型相关背景
  if (props.type === 'card' || props.type === 'segment') {
    baseClasses.push('bg-gray-50', 'dark:bg-gray-800', 'rounded-lg', 'p-1')
  }

  // 自定义类名
  if (props.navClass) {
    const customClasses = Array.isArray(props.navClass) ? props.navClass : [props.navClass]
    baseClasses.push(...customClasses.filter(Boolean))
  }

  return baseClasses
})

const getTabButtonClasses = (tab: Tab, _index: number) => {
  const baseClasses = [
    'relative', 'flex', 'items-center', 'gap-2', 'font-medium',
    'transition-all', 'duration-200', 'cursor-pointer', 'whitespace-nowrap'
  ]

  // 位置相关
  if (props.position === 'left' || props.position === 'right') {
    baseClasses.push('my-0.5', 'w-full')
  }
  else {
    baseClasses.push('mx-0.5')
  }

  // 尺寸
  if (props.size === 'sm') {
    baseClasses.push('px-3', 'py-1.5', 'text-xs')
  }
  else if (props.size === 'lg') {
    baseClasses.push('px-6', 'py-3', 'text-base')
  }
  else {
    baseClasses.push('px-4', 'py-2', 'text-sm')
  }

  // 禁用状态
  if (tab.disabled) {
    baseClasses.push('opacity-50', 'cursor-not-allowed', 'pointer-events-none')
  }

  const isActive = isActiveTab(tab)

  // 类型特定样式
  switch (props.type) {
    case 'line':
      baseClasses.push('border-b-2', 'border-transparent')
      if (isActive) {
        baseClasses.push('text-blue-600', 'dark:text-blue-400', 'border-blue-600', 'dark:border-blue-400')
      }
      else {
        baseClasses.push('text-gray-600', 'hover:text-blue-500', 'dark:text-gray-400', 'dark:hover:text-blue-300', 'hover:border-blue-300', 'dark:hover:border-blue-500')
      }
      break

    case 'card':
    case 'segment':
      baseClasses.push('rounded-md')
      if (isActive) {
        baseClasses.push('bg-white', 'dark:bg-gray-700', 'shadow-sm', 'text-blue-600', 'dark:text-blue-400')
      }
      else {
        baseClasses.push('text-gray-600', 'hover:text-blue-500', 'dark:text-gray-400', 'dark:hover:text-blue-300', 'hover:bg-gray-100', 'dark:hover:bg-gray-600')
      }
      break

    case 'button':
      baseClasses.push('border', 'border-gray-300', 'dark:border-gray-600', '-ml-px', 'first:ml-0', 'first:rounded-l-lg', 'last:rounded-r-lg')
      if (isActive) {
        baseClasses.push('bg-blue-600', 'text-white', 'border-blue-600', 'z-10')
      }
      else {
        baseClasses.push('bg-white',
            'dark:bg-gray-800',
            'text-gray-600',
            'hover:text-blue-500',
            'dark:text-gray-400',
            'dark:hover:text-blue-300',
            'hover:bg-blue-50',
            'dark:hover:bg-blue-900/20')
      }
      break

    default:
      if (isActive) {
        baseClasses.push('text-blue-600', 'dark:text-blue-400')
      }
      else {
        baseClasses.push('text-gray-600', 'hover:text-blue-500', 'dark:text-gray-400', 'dark:hover:text-blue-300')
      }
  }

  // 自定义类名
  if (props.tabButtonClass) {
    const customClasses = Array.isArray(props.tabButtonClass) ? props.tabButtonClass : [props.tabButtonClass]
    baseClasses.push(...customClasses.filter(Boolean))
  }

  return baseClasses
}

const contentClasses = computed(() => {
  const baseClasses = ['w-full']

  // 位置相关边距
  if (props.position === 'top') {
    baseClasses.push('mt-4')
  }
  else if (props.position === 'bottom') {
    baseClasses.push('mb-4')
  }
  else if (props.position === 'left' || props.position === 'right') {
    baseClasses.push('flex-1', 'mt-0')
  }

  // 自定义类名
  if (props.contentClass) {
    const customClasses = Array.isArray(props.contentClass) ? props.contentClass : [props.contentClass]
    baseClasses.push(...customClasses.filter(Boolean))
  }

  return baseClasses
})

const paneClasses = computed(() => {
  const baseClasses = ['w-full']

  // 动画
  if (props.animated) {
    baseClasses.push('transition-opacity', 'duration-200')
  }

  // 自定义类名
  if (props.paneClass) {
    const customClasses = Array.isArray(props.paneClass) ? props.paneClass : [props.paneClass]
    baseClasses.push(...customClasses.filter(Boolean))
  }

  return baseClasses
})

const indicatorClasses = computed(() => {
  const baseClasses = [
    'absolute', 'transition-all', 'duration-300', 'ease-out', 'rounded-full'
  ]

  // 位置相关
  if (props.position === 'top' || props.position === 'bottom') {
    baseClasses.push('bottom-0', 'h-0.5', 'bg-blue-600', 'dark:bg-blue-400')
  }
  else {
    baseClasses.push('left-0', 'w-0.5', 'bg-blue-600', 'dark:bg-blue-400')
  }

  // 自定义类名
  if (props.indicatorClass) {
    const customClasses = Array.isArray(props.indicatorClass) ? props.indicatorClass : [props.indicatorClass]
    baseClasses.push(...customClasses.filter(Boolean))
  }

  return baseClasses
})

const iconClasses = computed(() => {
  const baseClasses = ['w-4', 'h-4', 'flex-shrink-0']

  // 自定义类名
  if (props.iconClass) {
    const customClasses = Array.isArray(props.iconClass) ? props.iconClass : [props.iconClass]
    baseClasses.push(...customClasses.filter(Boolean))
  }

  return baseClasses
})

const badgeClasses = computed(() => {
  const baseClasses = [
    'inline-flex', 'items-center', 'justify-center', 'min-w-[1.25rem]', 'h-5',
    'px-1.5', 'text-xs', 'font-medium', 'rounded-full', 'bg-red-500', 'text-white', 'ml-1'
  ]

  // 自定义类名
  if (props.badgeClass) {
    const customClasses = Array.isArray(props.badgeClass) ? props.badgeClass : [props.badgeClass]
    baseClasses.push(...customClasses.filter(Boolean))
  }

  return baseClasses
})

const closeButtonClasses = computed(() => {
  const baseClasses = [
    'flex', 'items-center', 'justify-center', 'w-5', 'h-5', 'rounded-full',
    'transition-colors', 'hover:bg-gray-200', 'dark:hover:bg-gray-600', 'ml-1', 'flex-shrink-0'
  ]

  // 自定义类名
  if (props.closeButtonClass) {
    const customClasses = Array.isArray(props.closeButtonClass) ? props.closeButtonClass : [props.closeButtonClass]
    baseClasses.push(...customClasses.filter(Boolean))
  }

  return baseClasses
})

const addButtonClasses = computed(() => {
  const baseClasses = [
    'flex', 'items-center', 'justify-center', 'w-8', 'h-8', 'rounded-md',
    'transition-colors', 'text-gray-400', 'hover:text-gray-600',
    'hover:bg-gray-100', 'dark:hover:bg-gray-700', 'ml-2', 'flex-shrink-0'
  ]

  // 自定义类名
  if (props.addButtonClass) {
    const customClasses = Array.isArray(props.addButtonClass) ? props.addButtonClass : [props.addButtonClass]
    baseClasses.push(...customClasses.filter(Boolean))
  }

  return baseClasses
})

// 当前活跃的 tab
const activeKey = computed(() => {
  if (props.modelValue !== undefined) {
    return props.modelValue
  }
  const activeTab = props.tabs[activeTabIndex.value]
  return activeTab?.key || activeTab?.name || activeTabIndex.value
})

// 方法
const getTabKey = (tab: Tab, index: number) => {
  return tab.key || tab.name || index
}

const isActiveTab = (tab: Tab) => {
  const tabKey = tab.key || tab.name
  return tabKey ? tabKey === activeKey.value : props.tabs.indexOf(tab) === activeTabIndex.value
}

const selectTab = (tab: Tab, index: number) => {
  if (tab.disabled) {
    return
  }

  const tabKey = getTabKey(tab, index)
  activeTabIndex.value = index

  emit('update:modelValue', tabKey)
  emit('change', tabKey, tab)

  updateIndicator()
}

const closeTab = (tab: Tab, index: number) => {
  emit('close', tab, index)
}

const selectPrevTab = () => {
  const enabledTabs = props.tabs.filter(tab => !tab.disabled)
  if (enabledTabs.length === 0) {
    return
  }

  const currentIndex = enabledTabs.findIndex(tab => isActiveTab(tab))
  const prevIndex = currentIndex > 0 ? currentIndex - 1 : enabledTabs.length - 1
  const prevTab = enabledTabs[prevIndex]
  const realIndex = props.tabs.indexOf(prevTab)

  selectTab(prevTab, realIndex)
}

const selectNextTab = () => {
  const enabledTabs = props.tabs.filter(tab => !tab.disabled)
  if (enabledTabs.length === 0) {
    return
  }

  const currentIndex = enabledTabs.findIndex(tab => isActiveTab(tab))
  const nextIndex = currentIndex < enabledTabs.length - 1 ? currentIndex + 1 : 0
  const nextTab = enabledTabs[nextIndex]
  const realIndex = props.tabs.indexOf(nextTab)

  selectTab(nextTab, realIndex)
}

// 更新指示器位置
const updateIndicator = async () => {
  if (!props.showIndicator || !tabButtons.value?.length) {
    return
  }

  await nextTick()

  const activeButton = tabButtons.value[activeTabIndex.value]
  if (!activeButton) {
    return
  }

  const { offsetLeft, offsetWidth, offsetTop, offsetHeight } = activeButton

  if (props.position === 'top' || props.position === 'bottom') {
    indicatorStyle.value = {
      transform: `translateX(${ offsetLeft }px)`,
      width: `${ offsetWidth }px`
    }
  }
  else {
    indicatorStyle.value = {
      transform: `translateY(${ offsetTop }px)`,
      height: `${ offsetHeight }px`
    }
  }
}

// 监听变化
watch(() => props.modelValue, (newVal) => {
  if (newVal !== undefined) {
    const index = props.tabs.findIndex(tab =>
        (tab.key || tab.name) === newVal
    )
    if (index !== -1) {
      activeTabIndex.value = index
      updateIndicator()
    }
  }
})

watch(() => props.tabs, () => {
  nextTick(updateIndicator)
}, { deep: true })

// 生命周期
onMounted(() => {
  // 初始化活跃标签
  if (props.modelValue !== undefined) {
    const index = props.tabs.findIndex(tab =>
        (tab.key || tab.name) === props.modelValue
    )
    if (index !== -1) {
      activeTabIndex.value = index
    }
  }

  updateIndicator()
})
</script>
