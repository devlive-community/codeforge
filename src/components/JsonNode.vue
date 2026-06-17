<template>
  <div>
    <div class="flex items-start hover:bg-gray-100 dark:hover:bg-gray-800/60 rounded px-1"
         :style="{ paddingLeft: `${depth * 14 + 4}px` }">
      <!-- 折叠开关 -->
      <span v-if="isContainer"
            class="cursor-pointer select-none w-4 flex-shrink-0 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
            @click="open = !open">{{ open ? '▾' : '▸' }}</span>
      <span v-else class="w-4 flex-shrink-0"></span>

      <span class="break-all">
        <!-- 键名 -->
        <template v-if="keyName !== undefined">
          <span class="text-purple-600 dark:text-purple-400">"{{ keyName }}"</span><span class="text-gray-400">: </span>
        </template>

        <!-- 容器 -->
        <template v-if="isContainer">
          <span class="text-gray-500">{{ openBracket }}</span>
          <template v-if="!open">
            <span class="cursor-pointer text-gray-400" @click="open = true"> {{ entries.length }} {{ isArray ? t('view.items') : t('view.keys') }} </span>
            <span class="text-gray-500">{{ closeBracket }}</span>
          </template>
          <span v-else-if="entries.length === 0" class="text-gray-500">{{ closeBracket }}</span>
        </template>

        <!-- 原始值 -->
        <template v-else>
          <span :class="primClass">{{ display }}</span>
        </template>
      </span>
    </div>

    <template v-if="isContainer && open && entries.length">
      <JsonNode v-for="(e, i) in entries"
                :key="i"
                :key-name="isArray ? undefined : e.k"
                :value="e.v"
                :depth="depth + 1"/>
      <div class="text-gray-500" :style="{ paddingLeft: `${depth * 14 + 4 + 16}px` }">{{ closeBracket }}</div>
    </template>
  </div>
</template>

<script setup lang="ts">
import {computed, ref} from 'vue'
import {useI18n} from 'vue-i18n'

const {t} = useI18n()

const props = defineProps<{
  value: any
  keyName?: string | number
  depth: number
}>()

const isArray = computed(() => Array.isArray(props.value))
const isContainer = computed(() => props.value !== null && typeof props.value === 'object')

// 顶部两层默认展开
const open = ref(props.depth < 2)

const entries = computed<{ k?: string; v: any }[]>(() => {
  if (!isContainer.value) {
    return []
  }
  if (isArray.value) {
    return (props.value as any[]).map(v => ({v}))
  }
  return Object.keys(props.value).map(k => ({k, v: props.value[k]}))
})

const openBracket = computed(() => (isArray.value ? '[' : '{'))
const closeBracket = computed(() => (isArray.value ? ']' : '}'))

const display = computed(() => {
  const v = props.value
  if (v === null) return 'null'
  if (typeof v === 'string') return `"${v}"`
  return String(v)
})

const primClass = computed(() => {
  const v = props.value
  if (v === null) return 'text-gray-400'
  if (typeof v === 'string') return 'text-green-600 dark:text-green-400'
  if (typeof v === 'number') return 'text-blue-600 dark:text-blue-400'
  if (typeof v === 'boolean') return 'text-amber-600 dark:text-amber-400'
  return 'text-gray-700 dark:text-gray-300'
})
</script>
