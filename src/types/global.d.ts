import { ToastManager } from '../plugins/toast.ts'

declare module '@vue/runtime-core'
{
    interface ComponentCustomProperties
    {
        $toast: ToastManager
        $Toast: ToastManager
    }
}
