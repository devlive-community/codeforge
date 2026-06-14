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
      <!-- LSP 状态（点击开关问题面板）-->
      <button v-if="lspState.status !== 'off'"
              class="flex items-center space-x-2 px-1 rounded cursor-pointer hover:bg-white/20 transition-colors"
              :title="lspState.status === 'connecting' ? `语言服务索引中：${lspState.language}` : `语言服务已就绪：${lspState.language}（点击查看问题）`"
              @click="emit('toggleProblems')">
        <span class="flex items-center space-x-1">
          <RefreshCw v-if="lspState.status === 'connecting'" class="w-3 h-3 animate-spin"/>
          <span v-else class="w-1.5 h-1.5 rounded-full bg-emerald-300"/>
          <span>{{ lspState.status === 'connecting' ? 'LSP 索引中' : 'LSP' }}</span>
        </span>
        <span v-if="errorCount" class="flex items-center space-x-0.5"><XCircle class="w-3 h-3"/><span>{{ errorCount }}</span></span>
        <span v-if="warningCount" class="flex items-center space-x-0.5"><AlertTriangle class="w-3 h-3"/><span>{{ warningCount }}</span></span>
      </button>

      <div class="flex items-center space-x-2">
        <Hash class="w-3 h-3 font-normal"/>
        <span><strong>{{ codeLength }}</strong> 字符</span>
      </div>

      <!-- 终端 -->
      <button @click="emit('toggleTerminal')"
              class="p-1 rounded cursor-pointer hover:bg-white/20 transition-colors"
              :title="`终端（${terminalShortcut}）`">
        <TerminalIcon class="w-3.5 h-3.5"/>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { AlertTriangle, Clock, Hash, RefreshCw, Terminal as TerminalIcon, XCircle } from 'lucide-vue-next'
import { computed, toRefs } from 'vue'
import { useStatusBar } from '../composables/useStatusBar'
import { lspState } from '../editor/lspStatus'
import { diagnostics } from '../editor/lspDiagnostics'
import { useShortcuts } from '../composables/useShortcuts'

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
  toggleTerminal: []
  toggleProblems: []
}>()

const errorCount = computed(() => diagnostics.value.filter(d => d.severity === 'error').length)
const warningCount = computed(() => diagnostics.value.filter(d => d.severity === 'warning').length)

const { envInfo, isLoading } = toRefs(props)

// 终端快捷键提示（跟随用户自定义绑定）
const { getBinding, formatCombo } = useShortcuts()
const terminalShortcut = computed(() => formatCombo(getBinding('toggleTerminal')))

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
