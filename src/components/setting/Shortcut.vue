<template>
  <div>
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white flex items-center">
        <Keyboard class="w-5 h-5 mr-2"/>
        {{ t('settings.nav.shortcut') }}
      </h3>
      <Button type="secondary" size="sm" @click="resetAll">{{ t('settings.shortcut.resetAll') }}</Button>
    </div>

    <!-- 快捷键预设：套用 / 保存当前 / 删除 -->
    <div class="flex items-center flex-wrap gap-2 mb-4 p-3 rounded-lg border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/50">
      <span class="text-xs font-medium text-gray-500 dark:text-gray-400">{{ t('settings.shortcut.presets') }}</span>
      <select v-if="presets.length" class="text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-2 py-1 focus:outline-none cursor-pointer max-w-[160px]"
              :value="''" @change="onApplyPreset">
        <option value="" disabled>{{ t('settings.shortcut.applyPreset') }}</option>
        <option v-for="p in presets" :key="p.name" :value="p.name">{{ p.name }}</option>
      </select>
      <div v-for="p in presets" :key="'chip' + p.name" class="inline-flex items-center gap-1 text-xs px-2 py-0.5 rounded bg-white dark:bg-gray-700 border border-gray-200 dark:border-gray-600">
        <span class="truncate max-w-[100px]">{{ p.name }}</span>
        <button class="text-gray-400 hover:text-red-500 cursor-pointer" @click="deletePreset(p.name)"><X class="w-3 h-3"/></button>
      </div>
      <div class="flex items-center gap-1 ml-auto">
        <input v-model="newPresetName" :placeholder="t('settings.shortcut.presetNamePlaceholder')" @keydown.enter="doSavePreset"
               class="text-xs rounded border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-2 py-1 focus:outline-none w-32"/>
        <Button size="sm" :disabled="!newPresetName.trim()" @click="doSavePreset">{{ t('settings.shortcut.savePreset') }}</Button>
      </div>
    </div>

    <div class="space-y-1">
      <div v-for="action in actions"
           :key="action.id"
           class="flex items-center justify-between px-3 py-2 rounded hover:bg-gray-50 dark:hover:bg-gray-800">
        <span class="text-sm text-gray-700 dark:text-gray-300">{{ action.label }}</span>

        <div class="flex items-center space-x-2">
          <span v-if="recordingId === action.id"
                class="text-xs text-blue-600 px-2 py-1 rounded border border-blue-300 border-dashed">
            {{ t('settings.shortcut.press') }}
          </span>
          <kbd v-else
               class="text-xs px-2 py-1 rounded bg-gray-100 dark:bg-gray-700 border border-gray-200 dark:border-gray-600 font-mono min-w-[2rem] text-center">
            {{ formatCombo(getBinding(action.id)) }}
          </kbd>

          <button class="text-xs text-blue-500 hover:text-blue-600 cursor-pointer"
                  @click="startRecording(action.id)">
            {{ recordingId === action.id ? t('settings.shortcut.cancel') : t('settings.shortcut.edit') }}
          </button>
          <button class="text-xs text-gray-400 hover:text-gray-600 cursor-pointer"
                  @click="resetBinding(action.id)">
            {{ t('settings.shortcut.reset') }}
          </button>
        </div>
      </div>
    </div>

    <p class="mt-4 text-xs text-gray-400">{{ t('settings.shortcut.hint') }}</p>
  </div>
</template>

<script setup lang="ts">
import {onMounted, onUnmounted, ref} from 'vue'
import {useI18n} from 'vue-i18n'
import {Keyboard, X} from 'lucide-vue-next'
import Button from '../../ui/Button.vue'
import {useToast} from '../../plugins/toast'
import {comboFromEvent, formatCombo, useShortcuts} from '../../composables/useShortcuts'

const {t} = useI18n()
const toast = useToast()
const {actions, getBinding, setBinding, resetBinding, resetAll, presets, savePreset, applyPreset, deletePreset} = useShortcuts()

const recordingId = ref<string | null>(null)

// 快捷键预设
const newPresetName = ref('')
const doSavePreset = () => {
  const name = newPresetName.value.trim()
  if (!name) {
    return
  }
  savePreset(name)
  newPresetName.value = ''
  toast.success(t('settings.shortcut.presetSaved', {name}))
}
const onApplyPreset = (e: Event) => {
  const name = (e.target as HTMLSelectElement).value
  ;(e.target as HTMLSelectElement).value = ''
  if (name) {
    applyPreset(name)
    toast.success(t('settings.shortcut.presetApplied', {name}))
  }
}

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
