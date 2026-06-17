import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ExecutionResult } from '../types/app.ts'
import { i18n } from '../i18n'

const t = (key: string, params?: Record<string, unknown>) => i18n.global.t(key, params ?? {})

export function useCodeExecution(toast: any)
{
    const code = ref('')
    const output = ref('')
    const isRunning = ref(false)
    const isSuccess = ref(false)
    const lastExecutionTime = ref(0)

    // 当前运行任务的唯一标识，用于事件路由（支持多标签并发运行）
    const currentTaskId = ref<string | null>(null)
    // 最近一次执行记录 id，用于关联 AI 对话
    const currentExecutionId = ref<number | null>(null)

    // 实时输出相关
    const realTimeOutput = ref('')
    const realTimeStderr = ref('')

    interface RunOptions
    {
        language: string
        envInstalled: boolean
        envLanguage: string
        filePath?: string | null
        args?: string[]
        stdin?: string
        env?: Record<string, string>
        // 指定要运行的代码（如运行选中片段）；不传则运行编辑器全部内容
        codeOverride?: string
    }

    const runCode = async (options: RunOptions) => {
        const {language, envInstalled, envLanguage, filePath, args, stdin, env, codeOverride} = options
        if (!envInstalled) {
            toast.error(t('toast.envNotInstalled', {lang: envLanguage}))
            return
        }

        // 生成本次运行的 task_id，事件按它路由
        const taskId = crypto.randomUUID()
        currentTaskId.value = taskId

        isRunning.value = true

        // 清空所有输出
        output.value = ''
        realTimeOutput.value = ''
        realTimeStderr.value = ''
        isSuccess.value = false
        lastExecutionTime.value = 0

        try {
            const result: ExecutionResult = await invoke('execute_code', {
                request: {
                    code: codeOverride ?? code.value,
                    language,
                    task_id: taskId,
                    file_path: filePath || null,
                    args: args && args.length ? args : null,
                    stdin: stdin ? stdin : null,
                    env: env && Object.keys(env).length ? env : null
                }
            })

            lastExecutionTime.value = result.execution_time
            isSuccess.value = result.success
            if (result.id != null) {
                currentExecutionId.value = result.id
            }

            if (result.success) {
                toast.success(t('toast.runSuccess', {ms: result.execution_time}))
            }
            else {
                toast.error(t('toast.runFailCheckOutput'))
            }
        }
        catch (error) {
            output.value = t('toast.outRunFailed') + error
            toast.error(t('toast.runFailCheckLog'))
            isRunning.value = false
        }
    }

    const stopCode = async () => {
        if (!isRunning.value || !currentTaskId.value) {
            return
        }

        try {
            const result = await invoke<boolean>('stop_execution', {
                taskId: currentTaskId.value
            })

            if (result) {
                toast.info(t('toast.stopping'))
            }
            else {
                toast.warning(t('toast.noRunningTask'))
            }
        }
        catch (error) {
            console.error('Error stopping execution:', error)
            toast.error(t('toast.stopFailed'))
        }
    }

    const clearOutput = () => {
        output.value = ''
        realTimeOutput.value = ''
        realTimeStderr.value = ''
        toast.info(t('toast.outputCleared'))
    }

    // 处理实时输出
    const handleRealtimeOutput = (data: any) => {
        // 只处理当前任务的输出
        if (data.task_id !== currentTaskId.value) {
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

    // 处理执行完成
    const handleExecutionComplete = (data: any) => {
        if (data.task_id === currentTaskId.value) {
            isRunning.value = false
            isSuccess.value = data.success
            if (data.execution_time) {
                lastExecutionTime.value = data.execution_time
            }
        }
    }

    // 处理执行停止
    const handleExecutionStopped = (data: any) => {
        if (data.task_id === currentTaskId.value) {
            isRunning.value = false
            output.value += '\n\n🛑 ' + t('toast.outStopped')
            toast.warning(t('toast.runStopped'))
        }
    }

    // 处理执行超时
    const handleExecutionTimeout = (data: any) => {
        if (data.task_id === currentTaskId.value) {
            isRunning.value = false
            output.value += '\n\n⚠️ ' + t('toast.outTimeout')
            toast.error(t('toast.runTimeout'))
        }
    }

    // 处理执行错误
    const handleExecutionError = (data: any) => {
        if (data.task_id === currentTaskId.value) {
            isRunning.value = false
            output.value += `\n\n❌ ${t('toast.outError')}${ data.error }`
            toast.error(t('toast.runError'))
        }
    }

    return {
        code,
        output,
        isRunning,
        isSuccess,
        lastExecutionTime,
        currentTaskId,
        currentExecutionId,
        runCode,
        stopCode,
        clearOutput,
        handleRealtimeOutput,
        handleExecutionComplete,
        handleExecutionStopped,
        handleExecutionTimeout,
        handleExecutionError
    }
}
