import { nextTick, ref } from 'vue'
import { BracesIcon, CodeIcon, ShieldIcon } from 'lucide-vue-next'

export function useSettings(emit: any)
{
    // 状态管理
    const isVisible = ref(false)
    const activeTab = ref('general')

    // 标签页配置
    const tabsData = [
        { key: 'general', label: '通用', icon: ShieldIcon },
        { key: 'editor', label: '编辑器', icon: CodeIcon },
        { key: 'language', label: '语言', icon: BracesIcon }
    ]

    const handleEditorSettingsChanged = (config: any) => {
        console.log('设置模态框接收到编辑器配置变更:', config)
        emit('settings-changed', config)
    }

    const handleLanguageSettingsChanged = (config: any) => {
        console.log('设置模态框接收到语言配置变更:', config)
        emit('settings-changed', config)
    }

    const handleEditorError = (message: string) => {
        console.error('编辑器设置错误:', message)
    }

    // 关闭设置
    const closeSettings = () => {
        isVisible.value = false
        setTimeout(() => {
            emit('close')
        }, 300)
    }

    // 初始化模态框
    const initialize = async () => {
        // 延迟显示动画
        await nextTick()
        setTimeout(() => {
            isVisible.value = true
        }, 50)
    }

    return {
        isVisible,
        activeTab,
        tabsData,
        handleEditorSettingsChanged,
        handleLanguageSettingsChanged,
        handleEditorError,
        closeSettings,
        initialize
    }
}
