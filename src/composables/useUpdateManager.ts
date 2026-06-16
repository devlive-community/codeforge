import {ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {listen} from '@tauri-apps/api/event'
import {CheckCircle, Download, RefreshCw, Wifi} from 'lucide-vue-next'
import {kvSet} from './useKvStore'
import {i18n} from '../i18n'

const t = (key: string) => i18n.global.t(key)

interface UpdateInfo
{
    version: string
    date: string
    notes: string
    size: number
    url: string
}

interface AppInfo
{
    version: string
    build_time: string
    platform: string
    arch: string
}

// 更新状态枚举
const UpdateStatus = {
    CHECKING: 'checking',
    NO_UPDATE: 'no_update',
    UPDATE_AVAILABLE: 'update_available',
    DOWNLOADING: 'downloading',
    INSTALLING: 'installing',
    COMPLETED: 'completed',
    ERROR: 'error'
} as const

export function useUpdateManager()
{
    // 状态管理
    const isVisible = ref(false)
    const currentVersion = ref('1.0.0')
    const isChecking = ref(false)
    const hasUpdate = ref(false)
    const isUpdating = ref(false)
    const downloadProgress = ref(0)
    const downloadSpeed = ref(0)
    const errorMessage = ref('')
    const lastCheckTime = ref<string | null>(null)
    const progressText = ref(t('update.prepareDownload'))
    const updateInfo = ref<UpdateInfo | null>(null)
    const currentStatus = ref<keyof typeof UpdateStatus>('NO_UPDATE')

    // 计算属性改为函数
    const getStatusIcon = () => {
        switch (currentStatus.value) {
            case 'CHECKING':
                return RefreshCw
            case 'UPDATE_AVAILABLE':
                return Download
            case 'DOWNLOADING':
            case 'INSTALLING':
                return Download
            case 'COMPLETED':
                return CheckCircle
            case 'ERROR':
                return CheckCircle
            default:
                return Wifi
        }
    }

    const getIconClass = () => {
        switch (currentStatus.value) {
            case 'CHECKING':
                return 'text-blue-500 animate-spin'
            case 'UPDATE_AVAILABLE':
                return 'text-green-500'
            case 'DOWNLOADING':
            case 'INSTALLING':
                return 'text-blue-500 animate-pulse'
            case 'COMPLETED':
                return 'text-green-500'
            case 'ERROR':
                return 'text-red-500'
            default:
                return 'text-gray-400'
        }
    }

    const getIconBackgroundClass = () => {
        switch (currentStatus.value) {
            case 'CHECKING':
                return 'bg-blue-100 dark:bg-blue-900/50'
            case 'UPDATE_AVAILABLE':
                return 'bg-green-100 dark:bg-green-900/50'
            case 'DOWNLOADING':
            case 'INSTALLING':
                return 'bg-blue-100 dark:bg-blue-900/50'
            case 'COMPLETED':
                return 'bg-green-100 dark:bg-green-900/50'
            case 'ERROR':
                return 'bg-red-100 dark:bg-red-900/50'
            default:
                return 'bg-gray-100 dark:bg-gray-700'
        }
    }

    const getStatusTitle = () => {
        switch (currentStatus.value) {
            case 'CHECKING':
                return t('update.statusCheckingTitle')
            case 'UPDATE_AVAILABLE':
                return t('update.statusAvailableTitle')
            case 'DOWNLOADING':
                return t('update.statusDownloadingTitle')
            case 'INSTALLING':
                return t('update.statusInstallingTitle')
            case 'COMPLETED':
                return t('update.statusCompletedTitle')
            case 'ERROR':
                return t('update.statusErrorTitle')
            default:
                return t('update.statusDefaultTitle')
        }
    }

    const getStatusDescription = () => {
        switch (currentStatus.value) {
            case 'CHECKING':
                return t('update.statusCheckingDesc')
            case 'UPDATE_AVAILABLE':
                return t('update.statusAvailableDesc')
            case 'DOWNLOADING':
                return t('update.statusDownloadingDesc')
            case 'INSTALLING':
                return t('update.statusInstallingDesc')
            case 'COMPLETED':
                return t('update.statusCompletedDesc')
            case 'ERROR':
                return t('update.statusErrorDesc')
            default:
                return t('update.statusDefaultDesc')
        }
    }

    // 方法
    const loadAppInfo = async () => {
        try {
            const appInfo: AppInfo = await invoke('get_app_info')
            currentVersion.value = appInfo.version
        }
        catch (error) {
            console.error('Failed to get app info:', error)
        }
    }

    const checkForUpdates = async () => {
        isChecking.value = true
        currentStatus.value = 'CHECKING'
        errorMessage.value = ''

        try {
            const result = await invoke<UpdateInfo | null>('check_for_updates')
            lastCheckTime.value = new Date().toISOString()

            if (result) {
                updateInfo.value = result
                hasUpdate.value = true
                currentStatus.value = 'UPDATE_AVAILABLE'
                return result
            }
            else {
                hasUpdate.value = false
                currentStatus.value = 'NO_UPDATE'
                return null
            }
        }
        catch (error) {
            console.error('Failed to check for updates:', error)
            errorMessage.value = String(error)
            currentStatus.value = 'ERROR'
            return null
        }
        finally {
            isChecking.value = false
        }
    }

    const startUpdate = async () => {
        if (!updateInfo.value) {
            return
        }

        isUpdating.value = true
        currentStatus.value = 'DOWNLOADING'
        downloadProgress.value = 0
        errorMessage.value = ''

        try {
            await invoke('start_update', {updateInfo: updateInfo.value})
        }
        catch (error) {
            console.error('Failed to start update:', error)
            errorMessage.value = String(error)
            currentStatus.value = 'ERROR'
            isUpdating.value = false
        }
    }

    const skipUpdate = () => {
        if (!updateInfo.value) {
            return
        }

        // 存储跳过的版本信息
        kvSet('skipped_version', updateInfo.value.version)
    }

    // 事件监听
    const setupUpdateListeners = async () => {
        // 监听下载进度
        await listen('update-progress', (event: any) => {
            const {progress, speed, status} = event.payload
            downloadProgress.value = progress
            downloadSpeed.value = speed

            if (status === 'downloading') {
                progressText.value = t('update.downloadingFile')
                currentStatus.value = 'DOWNLOADING'
            }
            else if (status === 'installing') {
                progressText.value = t('update.installingUpdate')
                currentStatus.value = 'INSTALLING'
            }
        })

        // 监听更新完成
        await listen('update-complete', () => {
            downloadProgress.value = 100
            currentStatus.value = 'COMPLETED'
            isUpdating.value = false
            progressText.value = t('update.completed')
        })

        // 监听更新错误
        await listen('update-error', (event: any) => {
            errorMessage.value = event.payload.error
            currentStatus.value = 'ERROR'
            isUpdating.value = false
        })
    }

    // 工具函数
    const formatDate = (dateString: string) => {
        return new Date(dateString).toLocaleString('zh-CN')
    }

    const formatSize = (bytes: number) => {
        const units = ['B', 'KB', 'MB', 'GB']
        let size = bytes
        let unitIndex = 0

        while (size >= 1024 && unitIndex < units.length - 1) {
            size /= 1024
            unitIndex++
        }

        return `${size.toFixed(1)} ${units[unitIndex]}`
    }

    const formatSpeed = (bytesPerSecond: number) => {
        return `${formatSize(bytesPerSecond)}/s`
    }

    return {
        // 状态
        isVisible,
        currentVersion,
        isChecking,
        hasUpdate,
        isUpdating,
        downloadProgress,
        downloadSpeed,
        errorMessage,
        lastCheckTime,
        progressText,
        updateInfo,

        // 方法
        checkForUpdates,
        startUpdate,
        skipUpdate,
        loadAppInfo,
        setupUpdateListeners,

        // 计算属性函数
        getStatusIcon,
        getIconClass,
        getIconBackgroundClass,
        getStatusTitle,
        getStatusDescription,

        // 工具函数
        formatDate,
        formatSize,
        formatSpeed
    }
}
