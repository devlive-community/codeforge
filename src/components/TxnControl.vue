<template>
  <div class="inline-flex items-center gap-1">
    <template v-if="!txn.active.value">
      <Tooltip :text="t('txn.beginTitle')">
        <button class="inline-flex items-center gap-1 px-2 py-0.5 rounded text-xs text-amber-600 dark:text-amber-400 hover:bg-amber-50 dark:hover:bg-amber-900/30 cursor-pointer"
                :disabled="busy" @click="begin">
          <PlayCircle class="w-3.5 h-3.5"/>
          <span>{{ t('txn.begin') }}</span>
        </button>
      </Tooltip>
    </template>
    <template v-else>
      <span class="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-xs bg-amber-100 text-amber-700 dark:bg-amber-900/40 dark:text-amber-300">
        <span class="w-1.5 h-1.5 rounded-full bg-amber-500 animate-pulse"/>{{ t('txn.inTxn') }}
      </span>
      <button class="px-2 py-0.5 rounded text-xs text-emerald-600 dark:text-emerald-400 hover:bg-emerald-50 dark:hover:bg-emerald-900/30 cursor-pointer" :disabled="busy" @click="finish(true)">{{ t('txn.commit') }}</button>
      <button class="px-2 py-0.5 rounded text-xs text-red-500 hover:bg-red-50 dark:hover:bg-red-900/30 cursor-pointer" :disabled="busy" @click="finish(false)">{{ t('txn.rollback') }}</button>
    </template>
  </div>
</template>

<script setup lang="ts">
import {ref} from 'vue'
import {PlayCircle} from 'lucide-vue-next'
import {useI18n} from 'vue-i18n'
import Tooltip from '../ui/Tooltip.vue'
import {useSqlTxn} from '../composables/useSqlTxn'
import {useDbConnections} from '../composables/useDbConnections'
import {useToast} from '../plugins/toast'

const emit = defineEmits<{ notice: [text: string] }>()
const {t} = useI18n()
const txn = useSqlTxn()
const {resolveActiveSource} = useDbConnections()
const toast = useToast()
const busy = ref(false)

const begin = async () => {
  busy.value = true
  try {
    await txn.begin(resolveActiveSource())
    toast.success(t('txn.began'))
    emit('notice', t('txn.began'))
  }
  catch (e) {
    toast.error(t('txn.failed') + ': ' + e)
  }
  finally {
    busy.value = false
  }
}

const finish = async (commit: boolean) => {
  busy.value = true
  try {
    await txn.finish(commit)
    const msg = commit ? t('txn.committed') : t('txn.rolledBack')
    toast.success(msg)
    emit('notice', msg)
  }
  catch (e) {
    toast.error(t('txn.failed') + ': ' + e)
  }
  finally {
    busy.value = false
  }
}
</script>
