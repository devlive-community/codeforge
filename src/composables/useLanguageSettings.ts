import {computed, nextTick, ref, watch} from 'vue'
import {ContainerIcon, FileIcon, PickaxeIcon, Settings2} from 'lucide-vue-next'
import {usePluginConfig} from './usePluginConfig'
import {useCodeMirrorEditor} from './useCodeMirrorEditor'
import {useTheme} from './useTheme'
import {i18n} from '../i18n'

const t = (key: string) => i18n.global.t(key)

export function useLanguageSettings(emit: any)
{
    const {isDark} = useTheme()
    const activeTab = ref('general')
    const pluginEnabledStates = ref<Record<string, boolean>>({})

    const tabsData = computed(() => [
        {
            key: 'general',
            label: t('settings.language.tabGeneral'),
            icon: Settings2
        },
        {
            key: 'environment',
            label: t('settings.language.tabEnvironment'),
            icon: ContainerIcon
        },
        {
            key: 'template',
            label: t('settings.language.tabTemplate'),
            icon: FileIcon
        },
        {
            key: 'advanced',
            label: t('settings.language.tabAdvanced'),
            icon: PickaxeIcon
        }
    ])

    const consoleTypes = computed(() => [{label: t('settings.language.ctConsole'), value: 'console'}, {label: 'Web', value: 'web'}, {label: 'JSON', value: 'json'}, {label: 'Markdown', value: 'markdown'}, {label: 'XML', value: 'xml'}, {label: 'YAML', value: 'yaml'}, {label: t('settings.language.ctSqlTable'), value: 'sqltable'}, {label: t('settings.language.ctTable'), value: 'table'}, {label: t('settings.language.ctXlsx'), value: 'xlsx'}])

    const {
        activePlugin,
        tabsPluginData,
        pluginConfig,
        globalConfig,
        isSaving,
        handleTabChange,
        selectExecuteHome,
        updateGlobalConfig,
        initializePlugin,
        getSupportedLanguages
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

        // 添加主题扩展：深色模式下使用深色编辑器主题
        const themeExtension = getThemeExtension(isDark.value ? 'githubDark' : 'githubLight')
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
    }, {immediate: false})

    // 监听插件配置变化
    watch(() => pluginConfig.value?.template, (newTemplate) => {
        console.log('Template changed:', newTemplate)
    }, {immediate: false})

    // 跟随应用深色模式切换模板编辑器主题
    // 仅更新扩展（vue-codemirror 就地重配置），避免卸载重挂的白色闪烁
    watch(isDark, async () => {
        await updateExtensions()
    })

    const handlePluginToggle = async (language: string, enabled: boolean, _event: Event) => {
        if (!globalConfig.value) return

        let plugin = globalConfig.value.plugins?.find((p: any) => p.language === language)

        if (!plugin && globalConfig.value.custom_plugins) {
            plugin = globalConfig.value.custom_plugins.find((p: any) => p.language === language)
        }

        if (plugin) {
            plugin.enabled = enabled
            await updateGlobalConfig(plugin)
        }
    }

    const syncPluginStates = () => {
        if (globalConfig.value) {
            const states: Record<string, boolean> = {}
            tabsPluginData.value.forEach((tab) => {
                let plugin = globalConfig.value.plugins?.find((p: any) => p.language === tab.key)

                if (!plugin && globalConfig.value.custom_plugins) {
                    plugin = globalConfig.value.custom_plugins.find((p: any) => p.language === tab.key)
                }

                states[tab.key] = plugin?.enabled !== false
            })
            pluginEnabledStates.value = states
        }
    }

    watch(tabsPluginData, () => {
        syncPluginStates()
    }, { immediate: true })

    watch(pluginConfig, () => {
        if (activePlugin.value && pluginConfig.value) {
            pluginEnabledStates.value[activePlugin.value] = pluginConfig.value.enabled
        }
    }, { deep: true })

    const initialize = async () => {
        console.log('Component mounted')

        await initializePlugin()
        console.log('Plugin initialized:', {
            activePlugin: activePlugin.value,
            template: pluginConfig.value?.template
        })

        syncPluginStates()

        await initializeEditor()
        console.log('Editor initialized')

        await updateExtensions()
        console.log('Extensions updated:', currentExtensions.value)
    }

    const reloadLanguages = async () => {
        await getSupportedLanguages()
        await initializePlugin()
        syncPluginStates()
    }

    return {
        activeTab,
        tabsData,
        consoleTypes,
        activePlugin,
        tabsPluginData,
        pluginConfig,
        pluginEnabledStates,
        isSaving,
        handleTabChange,
        handlePluginToggle,
        selectExecuteHome,
        isEditorReady,
        currentExtensions,
        currentLanguage,
        templateContent,
        updateExtensions,
        initialize,
        reloadLanguages
    }
}