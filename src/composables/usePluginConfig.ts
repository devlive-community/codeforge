import { ref, watch, onMounted, onUnmounted } from 'vue'
import { invoke, convertFileSrc } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { debounce } from 'lodash-es'
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import { useToast } from '../plugins/toast'
import type PluginConfig from '../types/plugin'
import { i18n } from '../i18n'

const t = (key: string, params?: Record<string, unknown>) => i18n.global.t(key, params ?? {})

interface Language
{
    name: string
    value: string
}

interface TabData
{
    key: string
    label: string
    svgUrl?: string
}

export function usePluginConfig(emit?: any)
{
    const toast = useToast()

    // 状态管理
    const activePlugin = ref('')
    const activeTab = ref('general')
    const tabsPluginData = ref<TabData[]>([])
    const globalConfig = ref<any>(null)
    const isInitialLoad = ref(true)
    const isSaving = ref(false)

    let unlistenConfigUpdate: (() => void) | null = null

    const pluginConfig = ref<PluginConfig>({
        enabled: true,
        execute_home: '',
        extension: '',
        language: '',
        before_compile: '',
        after_compile: '',
        run_command: '',
        template: '',
        timeout: 30
    })

    // 获取支持的语言列表
    const getSupportedLanguages = async () => {
        try {
            const languages = await invoke<Language[]>('get_supported_languages')

            await getGlobalConfig()

            tabsPluginData.value = languages.map((language) => {
                let customPlugin = globalConfig.value?.custom_plugins?.find((p: any) => p.language === language.value)

                return {
                    key: language.value,
                    label: language.name,
                    svgUrl: customPlugin?.icon_path
                        ? convertFileSrc(customPlugin.icon_path)
                        : `/icons/${ language.value.replace(/\d+$/, '') }.svg`
                }
            })

            // 设置默认选中的插件
            if (tabsPluginData.value.length > 0 && !activePlugin.value) {
                activePlugin.value = tabsPluginData.value[0].key
            }

            console.log('支持的语言列表:', tabsPluginData.value)
        }
        catch (error) {
            console.error('获取支持的语言失败:', error)
            toast.error(t('toast.getLanguagesFailed') + error)
            tabsPluginData.value = []
        }
    }

    // 获取全局配置
    const getGlobalConfig = async () => {
        try {
            globalConfig.value = await invoke<any>('get_app_config')
            console.log('全局配置加载成功:', globalConfig.value)

            // 配置加载后处理当前插件配置
            handleTabChange()
        }
        catch (error) {
            console.error('获取配置失败:', error)
            toast.error(t('toast.getConfigFailed') + error)
        }
    }

    // 处理标签页切换
    const handleTabChange = () => {
        if (globalConfig.value && activePlugin.value) {
            isInitialLoad.value = true

            let foundPlugin = globalConfig.value.plugins?.find(
                (plugin: any) => plugin.language === activePlugin.value
            )

            if (!foundPlugin && globalConfig.value.custom_plugins) {
                foundPlugin = globalConfig.value.custom_plugins.find(
                    (plugin: any) => plugin.language === activePlugin.value
                )
            }

            if (foundPlugin) {
                pluginConfig.value = { ...foundPlugin }
                console.log('切换到插件配置:', activePlugin.value, foundPlugin)
            }
            else {
                console.warn('未找到插件配置:', activePlugin.value)
                pluginConfig.value = {
                    enabled: true,
                    execute_home: '',
                    extension: '',
                    language: activePlugin.value,
                    before_compile: '',
                    after_compile: '',
                    run_command: '',
                    template: '',
                    timeout: 30
                }
            }

            setTimeout(() => {
                isInitialLoad.value = false
            }, 100)
        }
    }

    // 选择执行环境目录
    const selectExecuteHome = async () => {
        try {
            const selected = await openDialog({
                directory: true,
                multiple: false,
                title: '选择语言环境目录'
            })

            if (selected) {
                pluginConfig.value.execute_home = selected as string
                console.log('选择的执行环境目录:', selected)
            }
        }
        catch (error) {
            console.error('选择目录失败:', error)
            if (emit) {
                emit('error', '选择目录失败')
            }
        }
    }

    // 更新全局配置
    const updateGlobalConfig = async (updatedPlugin: PluginConfig) => {
        if (!globalConfig.value) {
            console.error('全局配置未加载')
            return
        }

        try {
            isSaving.value = true

            let pluginIndex = globalConfig.value.plugins?.findIndex(
                (plugin: any) => plugin.language === updatedPlugin.language
            ) ?? -1

            let isCustomPlugin = false
            if (pluginIndex === -1 && globalConfig.value.custom_plugins) {
                pluginIndex = globalConfig.value.custom_plugins.findIndex(
                    (plugin: any) => plugin.language === updatedPlugin.language
                )
                isCustomPlugin = pluginIndex !== -1
            }

            if (isCustomPlugin && globalConfig.value.custom_plugins) {
                globalConfig.value.custom_plugins[pluginIndex] = { ...updatedPlugin }
            } else if (pluginIndex !== -1 && globalConfig.value.plugins) {
                globalConfig.value.plugins[pluginIndex] = { ...updatedPlugin }
            } else if (globalConfig.value.plugins) {
                globalConfig.value.plugins.push({ ...updatedPlugin })
            } else {
                globalConfig.value.plugins = [{ ...updatedPlugin }]
            }

            await invoke('update_app_config', { config: globalConfig.value })

            const pluginLabel = tabsPluginData.value.find(
                (tab: any) => tab.key === updatedPlugin.language
            )?.label || updatedPlugin.language

            toast.success(t('toast.pluginConfigSaved', {name: pluginLabel}))

            if (emit) {
                emit('settings-changed', updatedPlugin)
            }

            console.log('插件配置保存成功:', updatedPlugin)
        }
        catch (error) {
            console.error('保存配置失败:', error)
            toast.error(t('toast.saveConfigFailed') + error)

            if (emit) {
                emit('error', '保存配置失败')
            }
        }
        finally {
            isSaving.value = false
        }
    }

    // 防抖更新
    const debouncedUpdate = debounce((config: PluginConfig) => {
        updateGlobalConfig(config)
    }, 1000)

    // 手动保存配置
    const saveConfig = async () => {
        if (pluginConfig.value.language) {
            await updateGlobalConfig(pluginConfig.value)
        }
    }

    const resetPluginConfig = (language: string) => {
        pluginConfig.value = {
            enabled: true,
            execute_home: '',
            extension: '',
            language: language,
            before_compile: '',
            after_compile: '',
            run_command: '',
            template: '',
            timeout: 30
        }
    }

    // 获取插件配置
    const getPluginConfig = (language: string) => {
        if (globalConfig.value) {
            let plugin = globalConfig.value.plugins?.find(
                (plugin: any) => plugin.language === language
            )

            if (!plugin && globalConfig.value.custom_plugins) {
                plugin = globalConfig.value.custom_plugins.find(
                    (plugin: any) => plugin.language === language
                )
            }

            return plugin || null
        }
        return null
    }

    // 验证插件配置
    const validatePluginConfig = (config: PluginConfig): boolean => {
        if (!config.language) {
            toast.error(t('toast.langRequired'))
            return false
        }

        if (!config.run_command) {
            toast.error(t('toast.runCmdRequired'))
            return false
        }

        if (config.timeout && (config.timeout < 1 || config.timeout > 300)) {
            toast.error(t('toast.timeoutRange'))
            return false
        }

        return true
    }

    // 初始化插件管理器
    const initializePlugin = async () => {
        await getSupportedLanguages()
        await getGlobalConfig()
    }

    // 监听插件配置变化
    watch(pluginConfig, (newConfig, oldConfig) => {
        if (!isInitialLoad.value && oldConfig && newConfig.language) {
            console.log('插件配置变化:', oldConfig, '->', newConfig)
            debouncedUpdate(newConfig)
        }
    }, {
        deep: true,
        flush: 'post'
    })

    // 监听活动插件变化
    watch(activePlugin, (newPlugin, oldPlugin) => {
        if (newPlugin && newPlugin !== oldPlugin) {
            console.log('切换插件:', oldPlugin, '->', newPlugin)
            handleTabChange()
        }
    })

    // 监听配置更新事件
    onMounted(async () => {
        unlistenConfigUpdate = await listen('config-updated', async () => {
            console.log('收到配置更新事件，重新加载配置和语言列表')
            await getSupportedLanguages()
            await getGlobalConfig()
        })
    })

    // 清理事件监听
    onUnmounted(() => {
        if (unlistenConfigUpdate) {
            unlistenConfigUpdate()
        }
    })

    return {
        // 状态
        activePlugin,
        activeTab,
        tabsPluginData,
        globalConfig,
        pluginConfig,
        isSaving,

        // 方法
        getSupportedLanguages,
        getGlobalConfig,
        handleTabChange,
        selectExecuteHome,
        updateGlobalConfig,
        saveConfig,
        resetPluginConfig,
        getPluginConfig,
        validatePluginConfig,
        initializePlugin
    }
}
