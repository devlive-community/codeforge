<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-16 px-6 pb-6" @click="emit('close')">
    <div class="w-full max-w-[520px] bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col"
         @click.stop>
      <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
          <Crosshair class="w-4 h-4 text-gray-400"/>
          <span>{{ t('git.bisectTitle') }}</span>
        </div>
        <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('close')">
          <X class="w-4 h-4"/>
        </button>
      </div>

      <div class="p-4 space-y-3">
        <!-- 未开始 -->
        <template v-if="!state.active">
          <p class="text-xs text-gray-500 dark:text-gray-400 leading-relaxed">{{ t('git.bisectIdleHint') }}</p>
          <Button size="sm" :loading="busy" @click="run('start')">{{ t('git.bisectStart') }}</Button>
        </template>

        <!-- 进行中 -->
        <template v-else>
          <div class="rounded border border-gray-200 dark:border-gray-700 px-3 py-2">
            <div class="text-[11px] text-gray-400">{{ t('git.bisectCurrent') }}</div>
            <div class="flex items-baseline gap-2 mt-0.5">
              <span class="font-mono text-xs text-amber-600 dark:text-amber-400">{{ state.head }}</span>
              <span class="text-sm text-gray-800 dark:text-gray-100 truncate">{{ state.subject }}</span>
            </div>
          </div>
          <div class="flex flex-wrap gap-2">
            <Button size="sm" type="success" :loading="busy" @click="run('good')">{{ t('git.bisectGood') }}</Button>
            <Button size="sm" type="danger" :loading="busy" @click="run('bad')">{{ t('git.bisectBad') }}</Button>
            <Button size="sm" type="secondary" :loading="busy" @click="run('skip')">{{ t('git.bisectSkip') }}</Button>
            <Button size="sm" type="secondary" :loading="busy" @click="run('reset')">{{ t('git.bisectReset') }}</Button>
          </div>
        </template>

        <pre v-if="message" class="text-[11px] font-mono whitespace-pre-wrap text-gray-600 dark:text-gray-300 bg-gray-50 dark:bg-gray-800/60 rounded p-2 max-h-40 overflow-auto">{{ message }}</pre>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {onMounted, reactive, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {Crosshair, X} from 'lucide-vue-next'
import Button from '../ui/Button.vue'
import {useToast} from '../plugins/toast'
import {useI18n} from 'vue-i18n'

interface BisectState { active: boolean; head: string; subject: string; message: string }

const props = defineProps<{ rootDir: string }>()
const emit = defineEmits<{ close: []; changed: [] }>()

const toast = useToast()
const {t} = useI18n()
const state = reactive<BisectState>({active: false, head: '', subject: '', message: ''})
const message = ref('')
const busy = ref(false)

const load = async () => {
  try {
    const s = await invoke<BisectState>('git_bisect_state', {root: props.rootDir})
    Object.assign(state, s)
  }
  catch (error) {
    toast.error(t('git.bisectFailed') + ': ' + error)
  }
}

const run = async (action: string) => {
  busy.value = true
  try {
    message.value = await invoke<string>('git_bisect', {root: props.rootDir, action, rev: ''})
    await load()
    emit('changed')
  }
  catch (error) {
    toast.error(t('git.bisectFailed') + ': ' + error)
    await load()
  }
  finally {
    busy.value = false
  }
}

onMounted(load)
</script>
