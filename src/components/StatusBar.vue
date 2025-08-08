<template>
  <div class="bg-blue-600 text-white px-4 py-2 text-sm flex items-center justify-between">
    <div class="flex items-center space-x-6">
      <div class="flex items-center space-x-2">
        <component :is="envInfo.installed ? CheckCircle : XCircle"
                   :class="envInfo.installed ? 'text-green-300' : 'text-red-300'"
                   class="w-4 h-4"/>
        <span>{{ envInfo.installed ? `${ envInfo.language }: ${ envInfo.version }` : `${ envInfo.language } 环境未安装` }}</span>
      </div>

      <div v-if="executionTime > 0" class="flex items-center space-x-2">
        <Clock class="w-4 h-4"/>
        <span>最新: <strong>{{ executionTime }}</strong> 毫秒</span>
      </div>
    </div>

    <div class="flex items-center space-x-4">
      <div class="flex items-center space-x-2">
        <Hash class="w-3 h-3 font-normal"/>
        <span><strong>{{ codeLength }}</strong> 字符</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { CheckCircle, Clock, Hash, XCircle } from 'lucide-vue-next'

defineProps<{
  envInfo: {
    installed: boolean
    version: string
    path: string,
    language: string
  }
  executionTime: number
  codeLength: number
}>()
</script>