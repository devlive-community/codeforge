<template>
  <div class="relative" ref="selectContainer">
    <!-- Select 按钮 -->
    <button type="button"
            @click="toggleDropdown"
            @keydown.enter.prevent="toggleDropdown"
            @keydown.space.prevent="toggleDropdown"
            @keydown.escape="closeDropdown"
            @keydown.arrow-down.prevent="openDropdown"
            @keydown.arrow-up.prevent="openDropdown"
            :class="[
              'relative w-full cursor-pointer rounded-lg border bg-white py-1 pl-3 pr-10 text-left shadow-sm transition-all duration-200',
              disabled ? 'cursor-not-allowed bg-gray-50 text-gray-400' : 'hover:border-gray-400',
              ...buttonClasses
            ]"
            :disabled="disabled"
            :aria-expanded="isOpen"
            :aria-haspopup="true"
            role="combobox">
      <span class="block truncate">
        {{ selectedLabel || placeholder }}
      </span>
      <span class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2">
        <ArrowUpIcon class="h-5 w-5 text-gray-400 transition-transform duration-200"
                     :class="{ 'rotate-180': isOpen }"
                     aria-hidden="true">
        </ArrowUpIcon>
      </span>
    </button>

    <!-- Dropdown 列表 -->
    <Transition enter-active-class="transition duration-200 ease-out"
                enter-from-class="transform scale-95 opacity-0"
                enter-to-class="transform scale-100 opacity-100"
                leave-active-class="transition duration-150 ease-in"
                leave-from-class="transform scale-100 opacity-100"
                leave-to-class="transform scale-95 opacity-0"
                @before-enter="$emit('before-open')"
                @after-enter="$emit('after-open')"
                @before-leave="$emit('before-close')"
                @after-leave="$emit('after-close')">
      <div v-show="isOpen"
           class="absolute z-50 mt-1 max-h-60 w-full overflow-auto rounded-md bg-white text-base shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none"
           :class="dropdownClasses"
           role="listbox"
           :aria-labelledby="buttonId">
        <!-- 搜索框 (可选) -->
        <div v-if="searchable" class="sticky top-0 bg-white p-2 border-b border-gray-100">
          <input v-model="searchQuery"
                 type="text"
                 class="w-full px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
                 :placeholder="searchPlaceholder"
                 @click.stop
                 ref="searchInput"/>
        </div>

        <!-- 选项列表 -->
        <template v-if="filteredOptions.length > 0">
          <div v-for="(option, index) in filteredOptions"
               :key="getOptionValue(option)"
               @click="selectOption(option)"
               @keydown.enter.prevent="selectOption(option)"
               @keydown.space.prevent="selectOption(option)"
               :class="[
                  'relative cursor-pointer select-none py-1 my-1 pl-3 pr-9 transition-colors duration-150',
                  isSelected(option)
                    ? 'bg-blue-400 text-white'
                    : 'text-gray-900 hover:bg-blue-50',
                  highlightedIndex === index ? 'bg-blue-100' : ''
                ]"
               :aria-selected="isSelected(option)"
               role="option"
               tabindex="-1">
            <span :class="['block truncate', isSelected(option) ? 'font-medium' : 'font-normal']">
              {{ getOptionLabel(option) }}
            </span>

            <!-- 选中图标 -->
            <span v-if="isSelected(option)"
                  class="absolute inset-y-0 right-0 flex items-center pr-2">
              <CheckIcon class="h-5 w-5" aria-hidden="true"/>
            </span>
          </div>
        </template>

        <!-- 无选项提示 -->
        <div v-else class="px-3 py-2 text-gray-500 text-sm">
          {{ noOptionsText }}
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from 'vue'
import { ArrowUpIcon, CheckIcon } from 'lucide-vue-next'

// Props 定义
interface Option
{
  label: string
  value: any
  disabled?: boolean

  [key: string]: any
}

interface Props
{
  modelValue?: any
  options: Option[] | string[] | number[]
  placeholder?: string
  disabled?: boolean
  searchable?: boolean
  searchPlaceholder?: string
  noOptionsText?: string
  valueKey?: string
  labelKey?: string
  buttonClasses?: string[]
  dropdownClasses?: string[]
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: '请选择...',
  disabled: false,
  searchable: false,
  searchPlaceholder: '搜索选项...',
  noOptionsText: '无可用选项',
  valueKey: 'value',
  labelKey: 'label',
  buttonClasses: () => [],
  dropdownClasses: () => []
})

// Emits 定义
const emit = defineEmits<{
  'update:modelValue': [value: any]
  'change': [value: any, option: Option | string | number]
  'before-open': []
  'after-open': []
  'before-close': []
  'after-close': []
  'search': [query: string]
}>()

// 响应式数据
const isOpen = ref(false)
const searchQuery = ref('')
const highlightedIndex = ref(-1)
const selectContainer = ref<HTMLElement>()
const searchInput = ref<HTMLInputElement>()
const buttonId = `select-button-${ Math.random().toString(36).substr(2, 9) }`

// 计算属性
const normalizedOptions = computed(() => {
  return props.options.map(option => {
    if (typeof option === 'string' || typeof option === 'number') {
      return { label: String(option), value: option }
    }
    return {
      ...option,
      label: option[props.labelKey] || option.label,
      value: option[props.valueKey] || option.value,
      disabled: option.disabled || false
    }
  })
})

const filteredOptions = computed(() => {
  if (!props.searchable || !searchQuery.value) {
    return normalizedOptions.value
  }

  const query = searchQuery.value.toLowerCase()
  return normalizedOptions.value.filter(option =>
      option.label.toLowerCase().includes(query)
  )
})

const selectedOption = computed(() => {
  return normalizedOptions.value.find(option =>
      option.value === props.modelValue
  )
})

const selectedLabel = computed(() => {
  return selectedOption.value?.label || ''
})

// 方法
const getOptionValue = (option: Option) => option.value
const getOptionLabel = (option: Option) => option.label

const isSelected = (option: Option) => {
  return option.value === props.modelValue
}

const toggleDropdown = () => {
  if (props.disabled) {
    return
  }

  if (isOpen.value) {
    closeDropdown()
  }
  else {
    openDropdown()
  }
}

const openDropdown = () => {
  if (props.disabled) {
    return
  }

  isOpen.value = true
  highlightedIndex.value = -1

  if (props.searchable) {
    nextTick(() => {
      searchInput.value?.focus()
    })
  }
}

const closeDropdown = () => {
  isOpen.value = false
  searchQuery.value = ''
  highlightedIndex.value = -1
}

const selectOption = (option: Option) => {
  if (option.disabled) {
    return
  }

  emit('update:modelValue', option.value)
  emit('change', option.value, option)
  closeDropdown()
}

// 键盘导航
const handleKeydown = (event: KeyboardEvent) => {
  if (!isOpen.value) {
    return
  }

  switch (event.key) {
    case 'ArrowDown':
      event.preventDefault()
      highlightedIndex.value = Math.min(
          highlightedIndex.value + 1,
          filteredOptions.value.length - 1
      )
      break
    case 'ArrowUp':
      event.preventDefault()
      highlightedIndex.value = Math.max(highlightedIndex.value - 1, 0)
      break
    case 'Enter':
      event.preventDefault()
      if (highlightedIndex.value >= 0) {
        selectOption(filteredOptions.value[highlightedIndex.value])
      }
      break
    case 'Escape':
      event.preventDefault()
      closeDropdown()
      break
  }
}

// 点击外部关闭
const handleClickOutside = (event: Event) => {
  if (selectContainer.value && !selectContainer.value.contains(event.target as Node)) {
    closeDropdown()
  }
}

// 监听搜索查询
watch(searchQuery, (newQuery) => {
  emit('search', newQuery)
  highlightedIndex.value = -1
})

// 生命周期
onMounted(() => {
  document.addEventListener('click', handleClickOutside)
  document.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
  document.removeEventListener('keydown', handleKeydown)
})
</script>
