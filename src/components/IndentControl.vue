<template>
  <div class="relative">
    <button class="hover:text-gray-700 dark:hover:text-gray-200 cursor-pointer"
            :title="t('statusbar.indentTitle')"
            @click.stop="open = !open">
      {{ label }}
    </button>

    <template v-if="open">
      <div class="fixed inset-0 z-40" @click="open = false" @contextmenu.prevent="open = false"></div>
      <div class="absolute top-full right-0 mt-2 z-50 bg-white dark:bg-gray-800 dark:text-gray-100 rounded-md shadow-lg border border-gray-200 dark:border-gray-700 py-1 text-xs min-w-[140px]">
        <div class="px-3 py-1 text-[10px] uppercase tracking-wide text-gray-400">{{ t('statusbar.indentType') }}</div>
        <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer flex items-center justify-between gap-3"
                @click="setTabs(false)">
          {{ t('statusbar.spaces') }}<Check v-if="!config.indent_with_tab" class="w-3.5 h-3.5 text-brand-500"/>
        </button>
        <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer flex items-center justify-between gap-3"
                @click="setTabs(true)">
          {{ t('statusbar.tabs') }}<Check v-if="config.indent_with_tab" class="w-3.5 h-3.5 text-brand-500"/>
        </button>
        <div class="border-t border-gray-100 dark:border-gray-700 my-1"></div>
        <div class="px-3 py-1 text-[10px] uppercase tracking-wide text-gray-400">{{ t('statusbar.tabWidth') }}</div>
        <button v-for="s in sizes" :key="s"
                class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer flex items-center justify-between gap-3"
                @click="setSize(s)">
          {{ s }}<Check v-if="(config.tab_size ?? 2) === s" class="w-3.5 h-3.5 text-brand-500"/>
        </button>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import {computed, ref} from 'vue'
import {useI18n} from 'vue-i18n'
import {Check} from 'lucide-vue-next'
import type {EditorConfig} from '../types/app'

const {t} = useI18n()
const props = defineProps<{ config: EditorConfig }>()

const open = ref(false)
const sizes = [2, 4, 8]

const label = computed(() =>
  `${props.config.indent_with_tab ? t('statusbar.tabs') : t('statusbar.spaces')}: ${props.config.tab_size ?? 2}`)

const setTabs = (v: boolean) => {
  props.config.indent_with_tab = v
  open.value = false
}
const setSize = (s: number) => {
  props.config.tab_size = s
  open.value = false
}
</script>
