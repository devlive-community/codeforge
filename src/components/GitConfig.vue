<template>
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-16 px-6 pb-6" @click="emit('close')">
    <div class="w-full max-w-[460px] bg-white dark:bg-gray-900 dark:text-gray-100 rounded-lg shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col"
         @click.stop>
      <div class="flex items-center justify-between px-4 py-2.5 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
        <div class="flex items-center gap-2 text-sm font-medium text-gray-700 dark:text-gray-200">
          <UserCog class="w-4 h-4 text-gray-400"/>
          <span>{{ t('git.configTitle') }}</span>
        </div>
        <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 cursor-pointer" @click="emit('close')">
          <X class="w-4 h-4"/>
        </button>
      </div>

      <div class="p-4 space-y-4">
        <!-- 提交身份 -->
        <div class="space-y-2">
          <div class="text-xs font-semibold text-gray-600 dark:text-gray-300">{{ t('git.identitySection') }}</div>
          <p class="text-[11px] text-gray-400">{{ t('git.identityHint') }}</p>
          <label class="block">
            <span class="text-xs text-gray-500 dark:text-gray-400">{{ t('git.identityName') }}</span>
            <input v-model="name"
                   class="mt-1 w-full text-sm border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-2 py-1 focus:outline-none focus:border-blue-500"
                   placeholder="Your Name"/>
          </label>
          <label class="block">
            <span class="text-xs text-gray-500 dark:text-gray-400">{{ t('git.identityEmail') }}</span>
            <input v-model="email"
                   class="mt-1 w-full text-sm border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-2 py-1 focus:outline-none focus:border-blue-500"
                   placeholder="you@example.com"/>
          </label>
        </div>

        <!-- 提交签名 -->
        <div class="space-y-2 pt-3 border-t border-gray-100 dark:border-gray-800">
          <div class="text-xs font-semibold text-gray-600 dark:text-gray-300">{{ t('git.signingSection') }}</div>
          <p class="text-[11px] text-gray-400">{{ t('git.signingHint') }}</p>
          <label class="flex items-center gap-2 cursor-pointer">
            <input v-model="gpgsign" type="checkbox" class="cursor-pointer"/>
            <span class="text-sm text-gray-700 dark:text-gray-200">{{ t('git.signingEnable') }}</span>
          </label>
          <label class="block">
            <span class="text-xs text-gray-500 dark:text-gray-400">{{ t('git.signingKey') }}</span>
            <input v-model="signingKey"
                   class="mt-1 w-full text-sm font-mono border border-gray-300 dark:border-gray-600 dark:bg-gray-800 rounded px-2 py-1 focus:outline-none focus:border-blue-500"
                   placeholder="3AA5C34371567BD2"/>
          </label>
        </div>

        <div class="flex justify-end gap-2 pt-1">
          <Button size="sm" type="secondary" @click="emit('close')">{{ t('git.cancel') }}</Button>
          <Button size="sm" :loading="busy" @click="save">{{ t('git.identitySave') }}</Button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {UserCog, X} from 'lucide-vue-next'
import Button from '../ui/Button.vue'
import {useToast} from '../plugins/toast'
import {useI18n} from 'vue-i18n'

const props = defineProps<{ rootDir: string }>()
const emit = defineEmits<{ close: [] }>()

const toast = useToast()
const {t} = useI18n()
const name = ref('')
const email = ref('')
const gpgsign = ref(false)
const signingKey = ref('')
const busy = ref(false)

const load = async () => {
  try {
    const [n, e] = await invoke<string[]>('git_get_identity', {root: props.rootDir})
    name.value = n || ''
    email.value = e || ''
    const [sign, key] = await invoke<string[]>('git_get_signing', {root: props.rootDir})
    gpgsign.value = sign === 'true'
    signingKey.value = key || ''
  }
  catch (error) {
    toast.error(t('git.identityFailed') + ': ' + error)
  }
}

const save = async () => {
  busy.value = true
  try {
    await invoke('git_set_identity', {root: props.rootDir, name: name.value.trim(), email: email.value.trim()})
    await invoke('git_set_signing', {root: props.rootDir, enabled: gpgsign.value, key: signingKey.value.trim()})
    toast.success(t('git.identitySaved'))
    emit('close')
  }
  catch (error) {
    toast.error(t('git.identityFailed') + ': ' + error)
  }
  finally {
    busy.value = false
  }
}

onMounted(load)
</script>
