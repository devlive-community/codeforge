import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useToast } from '../plugins/toast'

export function useLogCleanup(emit: any, loadLogFiles: () => Promise<void>)
{
    const toast = useToast()

    const keepDays = ref(30)

    const keepDaysOptions = [
        { label: '保留 1 天', value: 1 },
        { label: '保留 7 天', value: 7 },
        { label: '保留 14 天', value: 14 },
        { label: '保留 30 天', value: 30 },
        { label: '保留 90 天', value: 90 }
    ]

    const clearLogs = async () => {
        try {
            await invoke('clear_logs', { keepDays: parseInt(keepDays.value.toString()) })
            await loadLogFiles()
            toast.success(`已清理 ${ keepDays.value } 天前的日志`)
            emit('settings-changed', 'logCleanup', keepDays.value)
        }
        catch (error) {
            console.error('Failed to clear old logs:', error)
            const errorMessage = '清理日志失败, 错误信息: ' + error
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
