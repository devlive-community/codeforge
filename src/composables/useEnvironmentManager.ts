import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { DownloadProgress, EnvironmentInfo } from '../types/app'
import { onMounted, onUnmounted, ref } from 'vue'

export function useEnvironmentManager(language: string) {
    const environmentInfo = ref<EnvironmentInfo | null>(null)
    const downloadProgress = ref<DownloadProgress | null>(null)
    const isLoading = ref(false)
    const error = ref<string | null>(null)
    const isDownloading = ref(false)

    let unlistenProgress: (() => void) | null = null

    // 获取环境信息
    const fetchEnvironmentInfo = async () => {
        isLoading.value = true
        error.value = null

        try {
            const info = await invoke<EnvironmentInfo>('get_environment_info', {
                language
            })
            environmentInfo.value = info
        }
        catch (e) {
            error.value = e as string
            console.error('获取环境信息失败:', e)
        }
        finally {
            isLoading.value = false
        }
    }

    // 下载并安装版本
    const downloadAndInstall = async (version: string) => {
        isDownloading.value = true
        error.value = null
        downloadProgress.value = null

        try {
            const installPath = await invoke<string>('download_and_install_version', {
                language,
                version
            })

            console.log('安装成功:', installPath)

            // 刷新环境信息
            await fetchEnvironmentInfo()

            return installPath
        }
        catch (e) {
            error.value = e as string
            console.error('下载安装失败:', e)
            throw e
        }
        finally {
            isDownloading.value = false
            downloadProgress.value = null
        }
    }

    // 切换版本
    const switchVersion = async (version: string) => {
        isLoading.value = true
        error.value = null

        try {
            await invoke('switch_environment_version', {
                language,
                version
            })

            // 刷新环境信息
            await fetchEnvironmentInfo()
        }
        catch (e) {
            error.value = e as string
            console.error('切换版本失败:', e)
            throw e
        }
        finally {
            isLoading.value = false
        }
    }

    // 卸载版本
    const uninstallVersion = async (version: string) => {
        isLoading.value = true
        error.value = null

        try {
            await invoke('uninstall_environment_version', {
                language,
                version
            })

            // 刷新环境信息
            await fetchEnvironmentInfo()
        }
        catch (e) {
            error.value = e as string
            console.error('卸载版本失败:', e)
            throw e
        }
        finally {
            isLoading.value = false
        }
    }

    // 监听下载进度
    const setupProgressListener = async () => {
        unlistenProgress = await listen<DownloadProgress>('env-download-progress', (event) => {
            const progress = event.payload
            if (progress.language === language) {
                downloadProgress.value = progress
            }
        })
    }

    // 初始化
    onMounted(async () => {
        await setupProgressListener()
        await fetchEnvironmentInfo()
    })

    // 清理
    onUnmounted(() => {
        if (unlistenProgress) {
            unlistenProgress()
        }
    })

    return {
        environmentInfo,
        downloadProgress,
        isLoading,
        isDownloading,
        error,
        fetchEnvironmentInfo,
        downloadAndInstall,
        switchVersion,
        uninstallVersion
    }
}
