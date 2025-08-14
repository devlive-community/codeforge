import type { Ref } from 'vue'
import { listen, UnlistenFn } from '@tauri-apps/api/event'
import { CodeOutputEvent } from '../types/app.ts'

interface EventManagerOptions
{
    showAbout: Ref<boolean>
    showSettings: Ref<boolean>
    showUpdate: Ref<boolean>
    output: Ref<string>
    isRunning: Ref<boolean>
    isSuccess: Ref<boolean>
    lastExecutionTime: Ref<number>
    currentLanguage: Ref<string>
    toast: any
    handleRealtimeOutput: (currentLanguage: string, data: any) => void
    handleExecutionComplete: (currentLanguage: string, data: any) => void
    handleExecutionStopped: (currentLanguage: string, data: any) => void
    handleExecutionTimeout: (currentLanguage: string, data: any) => void
    handleExecutionError: (currentLanguage: string, data: any) => void
}

export function useEventManager(options: EventManagerOptions)
{
    const {
        showAbout,
        showSettings,
        showUpdate,
        currentLanguage,
        handleRealtimeOutput,
        handleExecutionComplete,
        handleExecutionStopped,
        handleExecutionTimeout,
        handleExecutionError
    } = options

    // 事件监听器引用
    let unlistenAboutFn: UnlistenFn | null = null
    let unlistenSettingsFn: UnlistenFn | null = null
    let unlistenUpdateFn: UnlistenFn | null = null
    let unlistenOutputFn: UnlistenFn | null = null
    let unlistenExecutionStartFn: UnlistenFn | null = null
    let unlistenExecutionCompleteFn: UnlistenFn | null = null
    let unlistenExecutionStoppedFn: UnlistenFn | null = null
    let unlistenExecutionTimeoutFn: UnlistenFn | null = null
    let unlistenExecutionErrorFn: UnlistenFn | null = null

    // 处理实时输出
    const handleRealtimeOutputWrapper = (event: any) => {
        const data: CodeOutputEvent = event.payload
        console.log('实时输出:', data)
        handleRealtimeOutput(currentLanguage.value, data)
    }

    // 处理执行状态事件
    const handleExecutionStart = (event: any) => {
        const data = event.payload
        if (data.language === currentLanguage.value) {
            console.log('代码开始执行')
        }
    }

    const handleExecutionCompleteWrapper = (event: any) => {
        const data = event.payload
        console.log('代码执行完成')
        handleExecutionComplete(currentLanguage.value, data)
    }

    const handleExecutionStoppedWrapper = (event: any) => {
        const data = event.payload
        console.log('代码执行已停止')
        handleExecutionStopped(currentLanguage.value, data)
    }

    const handleExecutionTimeoutWrapper = (event: any) => {
        const data = event.payload
        handleExecutionTimeout(currentLanguage.value, data)
    }

    const handleExecutionErrorWrapper = (event: any) => {
        const data = event.payload
        handleExecutionError(currentLanguage.value, data)
    }

    const initializeEventListeners = async () => {
        // 监听来自 Rust 的各种事件
        unlistenAboutFn = await listen('show-about', () => {
            showAbout.value = true
        })

        unlistenSettingsFn = await listen('show-settings', () => {
            showSettings.value = true
        })

        unlistenUpdateFn = await listen('show-update', () => {
            showUpdate.value = true
        })

        // 监听实时输出事件
        unlistenOutputFn = await listen('code-output', handleRealtimeOutputWrapper)

        // 监听执行状态事件
        unlistenExecutionStartFn = await listen('code-execution-start', handleExecutionStart)
        unlistenExecutionCompleteFn = await listen('code-execution-complete', handleExecutionCompleteWrapper)
        unlistenExecutionStoppedFn = await listen('code-execution-stopped', handleExecutionStoppedWrapper)
        unlistenExecutionTimeoutFn = await listen('code-execution-timeout', handleExecutionTimeoutWrapper)
        unlistenExecutionErrorFn = await listen('code-execution-error', handleExecutionErrorWrapper)
    }

    const cleanupEventListeners = () => {
        // 清理所有事件监听器
        const listeners = [
            unlistenAboutFn,
            unlistenSettingsFn,
            unlistenUpdateFn,
            unlistenOutputFn,
            unlistenExecutionStartFn,
            unlistenExecutionCompleteFn,
            unlistenExecutionStoppedFn,
            unlistenExecutionTimeoutFn,
            unlistenExecutionErrorFn
        ]

        listeners.forEach(listener => {
            if (listener) {
                listener()
            }
        })
    }

    return {
        initializeEventListeners,
        cleanupEventListeners
    }
}
