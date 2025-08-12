import type { Ref } from 'vue'
import { ref } from 'vue'
import { listen, UnlistenFn } from '@tauri-apps/api/event'
import { CodeOutputEvent } from '../types/app.ts'

interface EventManagerOptions
{
    showAbout: Ref<boolean>
    showSettings: Ref<boolean>
    output: Ref<string>
    isRunning: Ref<boolean>
    isSuccess: Ref<boolean>
    lastExecutionTime: Ref<number>
    currentLanguage: Ref<string>
    toast: any
}

export function useEventManager(options: EventManagerOptions)
{
    const {
        showAbout,
        showSettings,
        output,
        isRunning,
        isSuccess,
        lastExecutionTime,
        currentLanguage,
        toast
    } = options

    // 事件监听器引用
    let unlistenAboutFn: UnlistenFn | null = null
    let unlistenSettingsFn: UnlistenFn | null = null
    let unlistenOutputFn: UnlistenFn | null = null
    let unlistenExecutionStartFn: UnlistenFn | null = null
    let unlistenExecutionCompleteFn: UnlistenFn | null = null
    let unlistenExecutionStoppedFn: UnlistenFn | null = null
    let unlistenExecutionTimeoutFn: UnlistenFn | null = null
    let unlistenExecutionErrorFn: UnlistenFn | null = null

    // 实时输出相关
    const realTimeOutput = ref('')
    const realTimeStderr = ref('')

    // 处理实时输出
    const handleRealtimeOutput = (event: any) => {
        const data: CodeOutputEvent = event.payload
        console.log('实时输出:', data)

        // 只处理当前语言的输出
        if (data.language !== currentLanguage.value) {
            return
        }

        if (data.type === 'stdout') {
            realTimeOutput.value += data.content + '\n'
        }
        else if (data.type === 'stderr') {
            realTimeStderr.value += data.content + '\n'
        }

        // 合并输出显示
        let combinedOutput = ''
        if (realTimeOutput.value) {
            combinedOutput += realTimeOutput.value
        }
        if (realTimeStderr.value) {
            if (combinedOutput) {
                combinedOutput += '\n'
            }
            combinedOutput += realTimeStderr.value
        }

        output.value = combinedOutput
    }

    // 处理执行状态事件
    const handleExecutionStart = (event: any) => {
        const data = event.payload
        if (data.language === currentLanguage.value) {
            console.log('代码开始执行')
        }
    }

    const handleExecutionComplete = (event: any) => {
        const data = event.payload
        if (data.language === currentLanguage.value) {
            isRunning.value = false
            isSuccess.value = data.success
            if (data.execution_time) {
                lastExecutionTime.value = data.execution_time
            }
            console.log('代码执行完成')
        }
    }

    const handleExecutionStopped = (event: any) => {
        const data = event.payload
        if (data.language === currentLanguage.value) {
            isRunning.value = false
            output.value += '\n\n🛑 代码执行已被用户停止'
            toast.warning('代码执行已停止')
            console.log('代码执行已停止')
        }
    }

    const handleExecutionTimeout = (event: any) => {
        const data = event.payload
        if (data.language === currentLanguage.value) {
            isRunning.value = false
            output.value += '\n\n⚠️ 代码执行超时（30秒）'
            toast.error('代码执行超时')
        }
    }

    const handleExecutionError = (event: any) => {
        const data = event.payload
        if (data.language === currentLanguage.value) {
            isRunning.value = false
            output.value += `\n\n❌ 执行错误: ${ data.error }`
            toast.error('代码执行出错')
        }
    }

    const initializeEventListeners = async () => {
        // 监听来自 Rust 的各种事件
        unlistenAboutFn = await listen('show-about', () => {
            showAbout.value = true
        })

        unlistenSettingsFn = await listen('show-settings', () => {
            showSettings.value = true
        })

        // 监听实时输出事件
        unlistenOutputFn = await listen('code-output', handleRealtimeOutput)

        // 监听执行状态事件
        unlistenExecutionStartFn = await listen('code-execution-start', handleExecutionStart)
        unlistenExecutionCompleteFn = await listen('code-execution-complete', handleExecutionComplete)
        unlistenExecutionStoppedFn = await listen('code-execution-stopped', handleExecutionStopped)
        unlistenExecutionTimeoutFn = await listen('code-execution-timeout', handleExecutionTimeout)
        unlistenExecutionErrorFn = await listen('code-execution-error', handleExecutionError)
    }

    const cleanupEventListeners = () => {
        // 清理所有事件监听器
        const listeners = [
            unlistenAboutFn,
            unlistenSettingsFn,
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
