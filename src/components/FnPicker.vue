<template>
  <div class="relative inline-block">
    <button class="text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-2 py-1 min-w-[84px] cursor-pointer flex items-center gap-1 hover:border-blue-400"
            @click="toggle">
      <span :class="modelValue ? 'font-mono' : 'text-gray-400'" class="truncate">{{ modelValue || t('qb.noFn') }}</span>
      <ChevronDown class="w-3 h-3 ml-auto text-gray-400 flex-shrink-0"/>
    </button>
    <template v-if="open">
      <div class="fixed inset-0 z-10" @click="close"></div>
      <div class="absolute left-0 mt-1 z-20 w-56 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded shadow-lg p-2">
        <input ref="input" v-model="kw" :placeholder="t('qb.searchFn')" @keydown.enter.prevent="commitCustom"
               class="w-full text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-900 px-2 py-1 mb-1 focus:outline-none"/>
        <div class="max-h-56 overflow-auto">
          <button class="w-full text-left text-xs px-2 py-0.5 rounded hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer text-gray-500" @click="pick('')">{{ t('qb.noFn') }}</button>
          <button v-if="customName" class="w-full text-left text-xs px-2 py-0.5 rounded hover:bg-blue-50 dark:hover:bg-blue-900/20 cursor-pointer text-blue-500 font-mono" @click="pick(customName)">
            {{ t('qb.useCustom', { name: customName }) }}
          </button>
          <template v-for="cat in FUNC_CATS" :key="cat">
            <div v-if="funcsInCat(cat).length" class="text-[10px] uppercase tracking-wide text-gray-400 px-1 pt-1.5">{{ t('qb.cat.' + cat) }}</div>
            <button v-for="f in funcsInCat(cat)" :key="f.name"
                    class="w-full text-left text-xs px-2 py-0.5 rounded hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer font-mono"
                    @click="pick(f.name)">{{ f.name }}</button>
          </template>
          <div v-if="filtered.length === 0 && !customName" class="text-xs text-gray-400 px-2 py-1">{{ t('qb.noFnMatch') }}</div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import {computed, nextTick, ref} from 'vue'
import {ChevronDown} from 'lucide-vue-next'
import {useI18n} from 'vue-i18n'
import {FUNC_CATS, FUNC_DEFS} from '../utils/sqlFunctions'

defineProps<{ modelValue: string }>()
const emit = defineEmits<{ 'update:modelValue': [value: string] }>()
const {t} = useI18n()

const open = ref(false)
const kw = ref('')
const input = ref<HTMLInputElement>()

const filtered = computed(() => {
  const q = kw.value.trim().toLowerCase()
  return q ? FUNC_DEFS.filter(f => f.name.toLowerCase().includes(q)) : FUNC_DEFS
})
const funcsInCat = (cat: string) => filtered.value.filter(f => f.cat === cat)
// 输入的名字若与建议无精确匹配，则提供“使用自定义”入口（支持任意函数）
const customName = computed(() => {
  const raw = kw.value.trim()
  if (!raw) {
    return ''
  }
  const up = raw.toUpperCase()
  return FUNC_DEFS.some(f => f.name === up) ? '' : up
})

const toggle = async () => {
  open.value = !open.value
  if (open.value) {
    kw.value = ''
    await nextTick()
    input.value?.focus()
  }
}
const close = () => { open.value = false }
const pick = (name: string) => {
  emit('update:modelValue', name)
  close()
}
const commitCustom = () => {
  const raw = kw.value.trim()
  if (raw) {
    pick(raw.toUpperCase())
  }
}
</script>
