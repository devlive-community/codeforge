import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ExecutionResult } from '../types/app.ts'

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
    }

    const runCode = async (options: RunOptions) => {
        const {language, envInstalled, envLanguage, filePath, args, stdin} = options
        if (!envInstalled) {
            toast.error(`${ envLanguage } 环境未安装`)
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
                    code: code.value,
                    language,
                    task_id: taskId,
                    file_path: filePath || null,
                    args: args && args.length ? args : null,
                    stdin: stdin ? stdin : null
                }
            })

            lastExecutionTime.value = result.execution_time
            isSuccess.value = result.success
            if (result.id != null) {
                currentExecutionId.value = result.id
            }

            if (result.success) {
                toast.success(`代码执行成功，用时 ${ result.execution_time } 毫秒`)
            }
            else {
                toast.error('代码执行失败，查看输出的错误信息')
            }
        }
        catch (error) {
            output.value = `代码执行失败: ${ error }`
            toast.error('代码执行失败，请检查日志')
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
                toast.info('正在停止代码执行...')
            }
            else {
                toast.warning('没有找到正在运行的任务')
            }
        }
        catch (error) {
            console.error('Error stopping execution:', error)
            toast.error('停止执行失败')
        }
    }

    const clearOutput = () => {
        output.value = ''
        realTimeOutput.value = ''
        realTimeStderr.value = ''
        toast.info('输出已清空')
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
            output.value += '\n\n🛑 代码执行已被用户停止'
            toast.warning('代码执行已停止')
        }
    }

    // 处理执行超时
    const handleExecutionTimeout = (data: any) => {
        if (data.task_id === currentTaskId.value) {
            isRunning.value = false
            output.value += '\n\n⚠️ 代码执行超时（30秒）'
            toast.error('代码执行超时')
        }
    }

    // 处理执行错误
    const handleExecutionError = (data: any) => {
        if (data.task_id === currentTaskId.value) {
            isRunning.value = false
            output.value += `\n\n❌ 执行错误: ${ data.error }`
            toast.error('代码执行出错')
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
