<template>
  <div v-if="debug.status.value !== 'inactive'"
       class="fixed top-2 left-1/2 -translate-x-1/2 z-40 flex items-center gap-1 px-2 py-1 rounded-lg bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 shadow-lg">
    <Tooltip :text="t('debug.continue')">
      <button class="p-1.5 rounded hover:bg-gray-100 dark:hover:bg-gray-700 text-emerald-600 dark:text-emerald-400 disabled:opacity-30 cursor-pointer"
              :disabled="debug.status.value !== 'stopped'" @click="debug.continue()">
        <Play class="w-4 h-4"/>
      </button>
    </Tooltip>
    <Tooltip :text="t('debug.pause')">
      <button class="p-1.5 rounded hover:bg-gray-100 dark:hover:bg-gray-700 disabled:opacity-30 cursor-pointer"
              :disabled="debug.status.value !== 'running'" @click="debug.pause()">
        <Pause class="w-4 h-4"/>
      </button>
    </Tooltip>
    <div class="w-px h-4 bg-gray-200 dark:bg-gray-700 mx-0.5"></div>
    <Tooltip :text="t('debug.stepOver')">
      <button class="p-1.5 rounded hover:bg-gray-100 dark:hover:bg-gray-700 disabled:opacity-30 cursor-pointer"
              :disabled="debug.status.value !== 'stopped'" @click="debug.stepOver()">
        <StepForward class="w-4 h-4"/>
      </button>
    </Tooltip>
    <Tooltip :text="t('debug.stepIn')">
      <button class="p-1.5 rounded hover:bg-gray-100 dark:hover:bg-gray-700 disabled:opacity-30 cursor-pointer"
              :disabled="debug.status.value !== 'stopped'" @click="debug.stepIn()">
        <ArrowDownToLine class="w-4 h-4"/>
      </button>
    </Tooltip>
    <Tooltip :text="t('debug.stepOut')">
      <button class="p-1.5 rounded hover:bg-gray-100 dark:hover:bg-gray-700 disabled:opacity-30 cursor-pointer"
              :disabled="debug.status.value !== 'stopped'" @click="debug.stepOut()">
        <ArrowUpFromLine class="w-4 h-4"/>
      </button>
    </Tooltip>
    <div class="w-px h-4 bg-gray-200 dark:bg-gray-700 mx-0.5"></div>
    <Tooltip :text="t('debug.restart')">
      <button class="p-1.5 rounded hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="debug.restart()">
        <RotateCcw class="w-4 h-4"/>
      </button>
    </Tooltip>
    <Tooltip :text="t('debug.stop')">
      <button class="p-1.5 rounded hover:bg-gray-100 dark:hover:bg-gray-700 text-red-500 cursor-pointer" @click="debug.stopSession()">
        <Square class="w-4 h-4"/>
      </button>
    </Tooltip>
    <span class="ml-1 text-[11px] text-gray-400 select-none">{{ statusLabel }}</span>
  </div>
</template>

<script setup lang="ts">
import {computed} from 'vue'
import {ArrowDownToLine, ArrowUpFromLine, Pause, Play, RotateCcw, Square, StepForward} from 'lucide-vue-next'
import {useI18n} from 'vue-i18n'
import Tooltip from '../ui/Tooltip.vue'
import {useDebug} from '../composables/useDebug'

const {t} = useI18n()
const debug = useDebug()

const statusLabel = computed(() => {
  switch (debug.status.value) {
    case 'starting':
      return t('debug.statusStarting')
    case 'running':
      return t('debug.statusRunning')
    case 'stopped':
      return t('debug.statusStopped')
    default:
      return ''
  }
})
</script>
