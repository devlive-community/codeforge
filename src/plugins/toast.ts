import { App, reactive } from 'vue'

export interface ToastOptions
{
    message: string
    type?: 'success' | 'error' | 'warning' | 'info'
    duration?: number
    showProgress?: boolean
}

interface ToastItem
    extends ToastOptions
{
    id: number
    show: boolean
}

export class ToastManager
{
    private toasts = reactive<ToastItem[]>([])
    private toastId = 0

    show(options: ToastOptions | string)
    {
        const config: ToastOptions = typeof options === 'string'
            ? { message: options }
            : options

        // 清除现有的 toast
        if (this.toasts.length > 0) {
            this.toasts.splice(0, this.toasts.length)
        }

        const toast: ToastItem = {
            id: ++this.toastId,
            show: true,
            type: 'success',
            duration: 3000,
            showProgress: true,
            ...config
        }

        this.toasts.push(toast)

        // 自动移除
        if (toast.duration as number > 0) {
            setTimeout(() => {
                this.remove(toast.id)
            }, toast.duration)
        }

        return toast.id
    }

    success(message: string, options?: Partial<ToastOptions>)
    {
        return this.show({ message, type: 'success', ...options })
    }

    error(message: string, options?: Partial<ToastOptions>)
    {
        return this.show({ message, type: 'error', ...options })
    }

    warning(message: string, options?: Partial<ToastOptions>)
    {
        return this.show({ message, type: 'warning', ...options })
    }

    info(message: string, options?: Partial<ToastOptions>)
    {
        return this.show({ message, type: 'info', ...options })
    }

    remove(id: number)
    {
        const index = this.toasts.findIndex(toast => toast.id === id)
        if (index > -1) {
            this.toasts[index].show = false
            // 等待动画完成后移除
            setTimeout(() => {
                const currentIndex = this.toasts.findIndex(toast => toast.id === id)
                if (currentIndex > -1) {
                    this.toasts.splice(currentIndex, 1)
                }
            }, 300)
        }
    }

    clear()
    {
        this.toasts.forEach(toast => {
            toast.show = false
        })
        setTimeout(() => {
            this.toasts.length = 0
        }, 300)
    }

    getToasts()
    {
        return this.toasts
    }
}

const toastManager = new ToastManager()

export const ToastPlugin = {
    install(app: App)
    {
        // 全局属性
        app.config.globalProperties.$toast = toastManager
        app.config.globalProperties.$Toast = toastManager

        // 提供/注入
        app.provide('toast', toastManager)
        app.provide('$toast', toastManager)
    }
}

export const useToast = () => toastManager
export const $toast = toastManager

// 默认导出
export default ToastPlugin
