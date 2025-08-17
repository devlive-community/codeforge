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

    // 处理编辑器设置变更
    const handleEditorSettingsChanged = (config: any) => {
        console.log('设置模态框接收到编辑器配置变更:', config)
        // 向上传递事件到主组件
        emit('settings-changed', config)
    }

    // 处理编辑器错误
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
        // 状态
        isVisible,
        activeTab,
        tabsData,

        // 方法
        handleEditorSettingsChanged,
        handleEditorError,
        closeSettings,
        initialize
    }
}
