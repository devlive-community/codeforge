import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import { openPath } from '@tauri-apps/plugin-opener'
import { useToast } from '../plugins/toast'

export function useLogDirectory(emit: any)
{
    const toast = useToast()

    const currentLogDir = ref('')
    const newLogDir = ref('')
    const logFiles = ref<string[]>([])

    const loadLogDirectory = async () => {
        try {
            const logDir = await invoke<string>('get_log_directory')
            currentLogDir.value = logDir
            newLogDir.value = logDir
        }
        catch (error) {
            console.error('Failed to get current log directory:', error)
            toast.error('获取日志目录失败 - 错误信息: ' + error)
            emit('error', '获取日志目录失败')
        }
    }

    const loadLogFiles = async () => {
        try {
            logFiles.value = await invoke<string[]>('get_log_files')
        }
        catch (error) {
            console.error('Failed to get log files:', error)
            emit('error', '获取日志文件列表失败')
        }
    }

    const selectLogDirectory = async () => {
        try {
            const selected = await openDialog({
                directory: true,
                multiple: false,
                title: '选择日志目录'
            })

            if (selected) {
                newLogDir.value = selected as string
            }
        }
        catch (error) {
            console.error('Failed to select directory:', error)
            emit('error', '选择目录失败')
        }
    }

    const applyLogDirChange = async () => {
        try {
            await invoke('set_log_directory', { path: newLogDir.value })
            currentLogDir.value = newLogDir.value
            await loadLogFiles()
            toast.success('日志目录已更新')
            emit('settings-changed', 'logDirectory', newLogDir.value)
        }
        catch (error) {
            console.error('Failed to set log directory:', error)
            const errorMessage = '日志目录更新失败, 错误信息: ' + error
            toast.error(errorMessage)
            emit('error', errorMessage)
        }
    }

    const openLogDirectory = async () => {
        try {
            await openPath(currentLogDir.value)
        }
        catch (error) {
            console.error('Failed to open log directory:', error)
            emit('error', '打开日志目录失败')
        }
    }

    const resetLogDirectory = async () => {
        try {
            await invoke('reset_log_directory')
            await loadLogDirectory()
            await loadLogFiles()
            toast.success('日志目录已重置为默认')
            emit('settings-changed', 'logDirectory', 'reset')
        }
        catch (error) {
            console.error('Failed to reset log directory:', error)
            const errorMessage = '日志目录重置失败, 错误信息: ' + error
            toast.error(errorMessage)
            emit('error', errorMessage)
        }
    }

    const openLogFile = async (filename: string) => {
        try {
            const logPath = `${ currentLogDir.value }/${ filename }`
            await openPath(logPath)
        }
        catch (error) {
            console.error('Failed to open log file:', error)
            toast.error('打开日志文件失败 - 错误信息: ' + error)
            emit('error', '打开日志文件失败')
        }
    }

    return {
        currentLogDir,
        newLogDir,
        logFiles,
        loadLogDirectory,
        loadLogFiles,
        selectLogDirectory,
        applyLogDirChange,
        openLogDirectory,
        resetLogDirectory,
        openLogFile
    }
}
