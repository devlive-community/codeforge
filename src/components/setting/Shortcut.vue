<template>
  <div>
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white flex items-center">
        <Keyboard class="w-5 h-5 mr-2"/>
        快捷键
      </h3>
      <Button type="secondary" size="sm" @click="resetAll">全部重置</Button>
    </div>

    <div class="space-y-1">
      <div v-for="action in actions"
           :key="action.id"
           class="flex items-center justify-between px-3 py-2 rounded hover:bg-gray-50 dark:hover:bg-gray-800">
        <span class="text-sm text-gray-700 dark:text-gray-300">{{ action.label }}</span>

        <div class="flex items-center space-x-2">
          <span v-if="recordingId === action.id"
                class="text-xs text-blue-600 px-2 py-1 rounded border border-blue-300 border-dashed">
            按下快捷键…
          </span>
          <kbd v-else
               class="text-xs px-2 py-1 rounded bg-gray-100 dark:bg-gray-700 border border-gray-200 dark:border-gray-600 font-mono min-w-[2rem] text-center">
            {{ formatCombo(getBinding(action.id)) }}
          </kbd>

          <button class="text-xs text-blue-500 hover:text-blue-600 cursor-pointer"
                  @click="startRecording(action.id)">
            {{ recordingId === action.id ? '取消' : '修改' }}
          </button>
          <button class="text-xs text-gray-400 hover:text-gray-600 cursor-pointer"
                  @click="resetBinding(action.id)">
            重置
          </button>
        </div>
      </div>
    </div>

    <p class="mt-4 text-xs text-gray-400">提示：修改后关闭设置即生效。</p>
  </div>
</template>

<script setup lang="ts">
import {onMounted, onUnmounted, ref} from 'vue'
import {Keyboard} from 'lucide-vue-next'
import Button from '../../ui/Button.vue'
import {comboFromEvent, formatCombo, useShortcuts} from '../../composables/useShortcuts'

const {actions, getBinding, setBinding, resetBinding, resetAll} = useShortcuts()

const recordingId = ref<string | null>(null)

const onRecordKeydown = (e: KeyboardEvent) => {
  if (!recordingId.value) {
    return
  }
  e.preventDefault()
  e.stopPropagation()

  if (e.key === 'Escape') {
    recordingId.value = null
    return
  }

  const combo = comboFromEvent(e)
  if (combo) {
    setBinding(recordingId.value, combo)
    recordingId.value = null
  }
}

const startRecording = (id: string) => {
  recordingId.value = recordingId.value === id ? null : id
}

onMounted(() => {
  // 捕获阶段监听，优先于其它处理
  window.addEventListener('keydown', onRecordKeydown, true)
})

onUnmounted(() => {
  window.removeEventListener('keydown', onRecordKeydown, true)
})
</script>
