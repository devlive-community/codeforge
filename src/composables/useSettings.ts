import { computed, nextTick, ref } from 'vue'
import { BracesIcon, CodeIcon, Cpu, Database, FileText, Globe, Keyboard, Server, ShieldIcon, Sparkles } from 'lucide-vue-next'
import { i18n } from '../i18n'

export function useSettings(emit: any)
{
    const t = i18n.global.t

    // 状态管理
    const isVisible = ref(false)
    const activeTab = ref('general')

    // 标签页配置（label 随界面语言响应式更新）
    const tabsData = computed(() => [
        { key: 'general', label: t('settings.nav.general'), icon: ShieldIcon },
        { key: 'editor', label: t('settings.nav.editor'), icon: CodeIcon },
        { key: 'shortcut', label: t('settings.nav.shortcut'), icon: Keyboard },
        { key: 'ai', label: t('settings.nav.ai'), icon: Sparkles },
        { key: 'database', label: t('settings.nav.database'), icon: Server },
        { key: 'language', label: t('settings.nav.language'), icon: BracesIcon },
        { key: 'lsp', label: t('settings.nav.lsp'), icon: Cpu },
        { key: 'network', label: t('settings.nav.network'), icon: Globe },
        { key: 'cache', label: t('settings.nav.cache'), icon: Database },
        { key: 'logs', label: t('settings.nav.logs'), icon: FileText }
    ])

    const handleEditorSettingsChanged = (config: any) => {
        console.log('设置模态框接收到编辑器配置变更:', config)
        emit('settings-changed', config)
    }

    const handleLanguageSettingsChanged = (config: any) => {
        console.log('设置模态框接收到语言配置变更:', config)
        emit('settings-changed', config)
    }

    const handleNetworkSettingsChanged = (config: any) => {
        console.log('设置模态框接收到网络配置变更:', config)
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
        handleNetworkSettingsChanged,
        handleEditorError,
        closeSettings,
        initialize
    }
}
