<template>
  <div class="fixed left-0 right-0 bottom-0 z-40 h-56 bg-white dark:bg-gray-900 border-t border-gray-200 dark:border-gray-700 shadow-2xl flex flex-col">
    <!-- 头部 -->
    <div class="flex items-center justify-between px-4 py-2 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
      <div class="flex items-center gap-3 text-sm font-medium text-gray-700 dark:text-gray-200">
        <span>{{ t('diag.title') }}</span>
        <span v-if="errorCount" class="flex items-center gap-1 text-xs text-red-500">
          <XCircle class="w-3.5 h-3.5"/>{{ errorCount }}
        </span>
        <span v-if="warningCount" class="flex items-center gap-1 text-xs text-amber-500">
          <AlertTriangle class="w-3.5 h-3.5"/>{{ warningCount }}
        </span>
      </div>
      <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" :title="t('diag.close')" @click="emit('close')">
        <X class="w-4 h-4"/>
      </button>
    </div>

    <!-- 列表 -->
    <div class="flex-1 overflow-y-auto text-sm">
      <div v-if="!diagnostics.length" class="px-4 py-6 text-center text-gray-400">
        {{ t('diag.empty') }}
      </div>
      <button v-for="(d, i) in diagnostics" :key="i"
              class="flex w-full items-start gap-2 px-4 py-1.5 text-left hover:bg-gray-100 dark:hover:bg-gray-800 cursor-pointer"
              @click="emit('go', d.line, d.col)">
        <component :is="iconOf(d.severity)" class="w-4 h-4 mt-0.5 flex-shrink-0" :class="colorOf(d.severity)"/>
        <span class="flex-1 text-gray-700 dark:text-gray-200 break-words">{{ d.message }}</span>
        <span class="text-xs text-gray-400 flex-shrink-0 mt-0.5">[{{ d.line }}:{{ d.col }}]</span>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import {computed} from 'vue'
import {useI18n} from 'vue-i18n'
import {AlertTriangle, Info, X, XCircle} from 'lucide-vue-next'
import {diagnostics} from '../editor/lspDiagnostics'

const {t} = useI18n()

const emit = defineEmits<{
  go: [line: number, col: number]
  close: []
}>()

const errorCount = computed(() => diagnostics.value.filter(d => d.severity === 'error').length)
const warningCount = computed(() => diagnostics.value.filter(d => d.severity === 'warning').length)

const iconOf = (sev: string) => (sev === 'error' ? XCircle : sev === 'warning' ? AlertTriangle : Info)
const colorOf = (sev: string) =>
  sev === 'error' ? 'text-red-500' : sev === 'warning' ? 'text-amber-500' : 'text-blue-500'
</script>
