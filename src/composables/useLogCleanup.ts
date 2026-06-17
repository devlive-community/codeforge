import { computed, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useToast } from '../plugins/toast'
import { i18n } from '../i18n'

const t = (key: string, params?: Record<string, unknown>) => i18n.global.t(key, params ?? {})

export function useLogCleanup(emit: any, loadLogFiles: () => Promise<void>)
{
    const toast = useToast()

    const keepDays = ref(30)

    const keepDaysOptions = computed(() => [
        { label: t('toast.keepDaysOption', { n: 1 }), value: 1 },
        { label: t('toast.keepDaysOption', { n: 7 }), value: 7 },
        { label: t('toast.keepDaysOption', { n: 14 }), value: 14 },
        { label: t('toast.keepDaysOption', { n: 30 }), value: 30 },
        { label: t('toast.keepDaysOption', { n: 90 }), value: 90 }
    ])

    const clearLogs = async () => {
        try {
            await invoke('clear_logs', { keepDays: parseInt(keepDays.value.toString()) })
            await loadLogFiles()
            toast.success(t('toast.logsCleanedDays', { n: keepDays.value }))
            emit('settings-changed', 'logCleanup', keepDays.value)
        }
        catch (error) {
            console.error('Failed to clear old logs:', error)
            const errorMessage = t('toast.logsCleanFailed') + error
            toast.error(errorMessage)
            emit('error', errorMessage)
        }
    }

    return {
        keepDays,
        keepDaysOptions,
        clearLogs
    }
}
