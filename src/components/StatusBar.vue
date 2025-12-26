<template>
  <div class="text-white px-3.5 py-1 text-sm flex items-center justify-between"
       :class="[getStatusColor()]">
    <div class="flex items-center space-x-6">
      <div class="flex items-center space-x-2">
        <component :is="getStatusIcon()"
                   :class="[getIconClass(), { 'animate-spin': isLoading }]"
                   class="w-4 h-4"/>
        <span>{{ getStatusText() }}</span>
        <button @click="handleCheckEnvironment"
                :disabled="isLoading"
                class="ml-2 p-1 rounded cursor-pointer hover:bg-white/20 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                title="重新检查环境">
          <RefreshCw :class="{ 'animate-spin': isLoading }" class="w-3.5 h-3.5"/>
        </button>
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
import { Clock, Hash, RefreshCw } from 'lucide-vue-next'
import { toRefs } from 'vue'
import { useStatusBar } from '../composables/useStatusBar'

const props = defineProps<{
  envInfo: {
    installed: boolean
    version: string
    path: string,
    language: string
  }
  isLoading: boolean
  executionTime: number
  codeLength: number
}>()

const emit = defineEmits<{
  checkEnvironment: []
}>()

const { envInfo, isLoading } = toRefs(props)

const {
  getStatusColor,
  getStatusIcon,
  getIconClass,
  getStatusText
} = useStatusBar(envInfo, isLoading)

const handleCheckEnvironment = () => {
  emit('checkEnvironment')
}
</script>
