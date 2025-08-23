import { computed, nextTick, ref, watch } from 'vue'
import { ContainerIcon, FileIcon, PickaxeIcon, Settings2 } from 'lucide-vue-next'
import { usePluginConfig } from './usePluginConfig'
import { useCodeMirrorEditor } from './useCodeMirrorEditor'

export function useLanguageSettings(emit: any)
{
    // 标签页管理
    const activeTab = ref('general')

    const tabsData = [
        {
            key: 'general',
            label: '通用配置',
            icon: Settings2
        },
        {
            key: 'environment',
            label: '环境配置',
            icon: ContainerIcon
        },
        {
            key: 'template',
            label: '模板配置',
            icon: FileIcon
        },
        {
            key: 'advanced',
            label: '高级配置',
            icon: PickaxeIcon
        }
    ]

    // 插件配置管理
    const {
        activePlugin,
        tabsPluginData,
        pluginConfig,
        handleTabChange,
        selectExecuteHome,
        initializePlugin
    } = usePluginConfig(emit)

    // 编辑器状态
    const isEditorReady = ref(false)
    const currentExtensions = ref<any[]>([])

    // 创建 computed 来响应式地获取当前语言
    const currentLanguage = computed(() => {
        return activePlugin.value || ''
    })

    // 创建 computed 来响应式地获取模板内容
    const templateContent = computed({
        get: () => pluginConfig.value?.template || '',
        set: (value: string) => {
            if (pluginConfig.value) {
                pluginConfig.value.template = value
            }
        }
    })

    const {
        initializeEditor,
        getLanguageExtension,
        getThemeExtension
    } = useCodeMirrorEditor(
        {
            modelValue: templateContent.value,
            language: currentLanguage.value
        }
    )

    // 更新扩展的函数
    const updateExtensions = async () => {
        const newExtensions = []

        // 添加主题扩展
        const themeExtension = getThemeExtension()
        newExtensions.push(themeExtension)

        // 添加语言扩展
        if (currentLanguage.value) {
            const langExtension = getLanguageExtension(currentLanguage.value)
            if (langExtension) {
                newExtensions.push(langExtension)
            }
        }

        currentExtensions.value = newExtensions

        if (!isEditorReady.value) {
            await nextTick()
            isEditorReady.value = true
        }
    }

    // 监听语言变化
    watch(currentLanguage, async (newLanguage) => {
        console.log('Language changed to:', newLanguage)
        if (newLanguage) {
            await updateExtensions()
        }
    }, { immediate: false })

    // 监听插件配置变化
    watch(() => pluginConfig.value?.template, (newTemplate) => {
        console.log('Template changed:', newTemplate)
    }, { immediate: false })

    // 初始化所有功能
    const initialize = async () => {
        console.log('Component mounted')

        // 先初始化插件配置
        await initializePlugin()
        console.log('Plugin initialized:', {
            activePlugin: activePlugin.value,
            template: pluginConfig.value?.template
        })

        // 再初始化编辑器
        await initializeEditor()
        console.log('Editor initialized')

        // 更新扩展
        await updateExtensions()
        console.log('Extensions updated:', currentExtensions.value)
    }

    return {
        // 标签页状态
        activeTab,
        tabsData,

        // 插件配置
        activePlugin,
        tabsPluginData,
        pluginConfig,
        handleTabChange,
        selectExecuteHome,

        // 编辑器状态
        isEditorReady,
        currentExtensions,
        currentLanguage,
        templateContent,

        // 方法
        updateExtensions,
        initialize
    }
}