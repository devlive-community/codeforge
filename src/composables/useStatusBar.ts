import {type Ref} from 'vue'
import {CheckCircle, Loader2, XCircle} from 'lucide-vue-next'
import {EnvInfo} from '../types/app.ts'
import {i18n} from '../i18n'

export function useStatusBar(envInfo: Ref<EnvInfo>, isLoading: Ref<boolean>)
{
    const t = i18n.global.t
    // 计算状态颜色
    const getStatusColor = () => {
        if (isLoading.value) {
            return 'bg-blue-500'
        }
        return envInfo.value.installed ? 'bg-green-500' : 'bg-red-500'
    }

    // 计算状态图标
    const getStatusIcon = () => {
        if (isLoading.value) {
            return Loader2
        }
        return envInfo.value.installed ? CheckCircle : XCircle
    }

    // 计算图标样式
    const getIconClass = () => {
        if (isLoading.value) {
            return 'text-blue-200'
        }
        return envInfo.value.installed ? 'text-green-300' : 'text-white'
    }

    // 计算状态文本
    const getStatusText = () => {
        if (isLoading.value) {
            return `${envInfo.value.language}: ${t('status.checking')}`
        }

        if (envInfo.value.installed) {
            return `${envInfo.value.language}: ${envInfo.value.version || '--'}`
        }
        else {
            return `${envInfo.value.language}: ${t('status.notInstalled')}`
        }
    }

    return {
        getStatusColor,
        getStatusIcon,
        getIconClass,
        getStatusText
    }
}
