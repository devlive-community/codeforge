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

    // 实时输出相关
    const realTimeOutput = ref('')
    const realTimeStderr = ref('')

    const runCode = async (currentLanguage: string, envInstalled: boolean, envLanguage: string) => {
        if (!envInstalled) {
            toast.error(`${ envLanguage } 环境未安装`)
            return
        }

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
                    language: currentLanguage
                }
            })

            lastExecutionTime.value = result.execution_time
            isSuccess.value = result.success

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

    const stopCode = async (currentLanguage: string) => {
        if (!isRunning.value) {
            return
        }

        try {
            const result = await invoke<boolean>('stop_execution', {
                language: currentLanguage
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
    const handleRealtimeOutput = (currentLanguage: string, data: any) => {
        // 只处理当前语言的输出
        if (data.language !== currentLanguage) {
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
    const handleExecutionComplete = (currentLanguage: string, data: any) => {
        if (data.language === currentLanguage) {
            isRunning.value = false
            isSuccess.value = data.success
            if (data.execution_time) {
                lastExecutionTime.value = data.execution_time
            }
        }
    }

    // 处理执行停止
    const handleExecutionStopped = (currentLanguage: string, data: any) => {
        if (data.language === currentLanguage) {
            isRunning.value = false
            output.value += '\n\n🛑 代码执行已被用户停止'
            toast.warning('代码执行已停止')
        }
    }

    // 处理执行超时
    const handleExecutionTimeout = (currentLanguage: string, data: any) => {
        if (data.language === currentLanguage) {
            isRunning.value = false
            output.value += '\n\n⚠️ 代码执行超时（30秒）'
            toast.error('代码执行超时')
        }
    }

    // 处理执行错误
    const handleExecutionError = (currentLanguage: string, data: any) => {
        if (data.language === currentLanguage) {
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
