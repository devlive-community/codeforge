import {ref, type Ref, onMounted, onUnmounted} from 'vue'
import {invoke, convertFileSrc} from '@tauri-apps/api/core'
import {listen} from '@tauri-apps/api/event'
import {EnvInfo, Language, LanguageInfo} from '../types/app.ts'

export function useLanguageManager(
    code: Ref<string>,
    clearOutput: () => void,
    toast: any
)
{
    const currentLanguage = ref('')
    const supportedLanguages = ref<Language[]>([])
    const globalConfig = ref(null as any)

    // 🔥 按语言缓存编辑器代码，切换语言时保留各语言已编写的代码
    const codeCache = ref<Record<string, string>>({})

    // 🔥 添加加载状态
    const isLoadingEnvInfo = ref(false)

    const envInfo = ref<EnvInfo>({
        installed: false,
        version: '检查中...',
        path: '检查中...',
        language: ''
    })

    const getLanguageDisplayName = (languageValue: string) => {
        const language = supportedLanguages.value.find(lang => lang.value === languageValue)
        return language ? language.name : languageValue
    }

    // 获取当前语言的插件配置
    const getCurrentPluginConfig = () => {
        if (!globalConfig.value) {
            console.warn('globalConfig 还未加载，请先调用 initialize()')
            return null
        }

        if (!currentLanguage.value) {
            console.warn('currentLanguage 为空，请先设置语言')
            return null
        }

        if (!globalConfig.value.plugins) {
            console.warn('插件配置为空')
            return null
        }

        return globalConfig.value.plugins.find((p: any) => p.language === currentLanguage.value && p.enabled) || null
    }

    const getCurrentConsoleType = () => {
        const pluginConfig = getCurrentPluginConfig()
        return pluginConfig?.console_type || 'console'
    }

    const refreshEnvInfo = async () => {
        // 确保有当前语言才进行检查
        if (!currentLanguage.value) {
            return
        }

        // 🔥 设置加载状态
        isLoadingEnvInfo.value = true
        envInfo.value = {
            installed: false,
            version: '检查中...',
            path: '检查中...',
            language: getLanguageDisplayName(currentLanguage.value)
        }

        try {
            const info: LanguageInfo = await invoke('get_info', {
                language: currentLanguage.value
            })

            envInfo.value = {
                installed: info.installed,
                version: info.version,
                path: info.path,
                language: info.language
            }
        }
        catch (error) {
            console.error('Error checking Env installation:', error)
            envInfo.value = {
                installed: false,
                version: 'Error',
                path: 'Error',
                language: getLanguageDisplayName(currentLanguage.value)
            }
        }
        finally {
            isLoadingEnvInfo.value = false
        }
    }

    const getSupportedLanguages = async () => {
        try {
            const languages = await invoke<Language[]>('get_supported_languages')
            const allLanguages = languages.map((language) => {
                let customPlugin = globalConfig.value?.custom_plugins?.find((p: any) => p.language === language.value)

                return {
                    name: language.name,
                    value: language.value,
                    svgUrl: customPlugin?.icon_path
                        ? convertFileSrc(customPlugin.icon_path)
                        : `/icons/${language.value.replace(/\d+$/, '')}.svg`
                }
            })

            if (globalConfig.value) {
                const filtered = allLanguages.filter((language) => {
                    let plugin = globalConfig.value.plugins?.find((p: any) => p.language === language.value)

                    if (!plugin && globalConfig.value.custom_plugins) {
                        plugin = globalConfig.value.custom_plugins.find((p: any) => p.language === language.value)
                    }

                    const enabled = !plugin || plugin.enabled !== false
                    console.log(`语言 ${language.name} (${language.value}): enabled=${enabled}`)
                    return enabled
                })
                console.log('过滤后的语言列表:', filtered.map(l => l.name))
                supportedLanguages.value = filtered
            } else {
                console.log('未找到配置，显示所有语言')
                supportedLanguages.value = allLanguages
            }
        }
        catch (error) {
            console.error('Error getting supported languages:', error)
            supportedLanguages.value = []
        }
    }

    const getConfigure = async () => {
        try {
            globalConfig.value = await invoke<any>('get_app_config')
        }
        catch (error) {
            toast.error('获取配置失败 - 错误信息: ' + error)
        }
    }

    const filterPluginTemplate = (plugin: any) => {
        if (globalConfig.value) {
            let foundPlugin = globalConfig.value.plugins?.find((p: any) => p.language === plugin)

            if (!foundPlugin && globalConfig.value.custom_plugins) {
                foundPlugin = globalConfig.value.custom_plugins.find((p: any) => p.language === plugin)
            }

            return foundPlugin?.template || ''
        }
        return ''
    }

    const handleLanguageChange = async (newLanguage: string) => {
        if (newLanguage === currentLanguage.value) {
            return
        }

        // 保存当前语言的代码，避免切换语言后已编写的代码丢失
        if (currentLanguage.value) {
            codeCache.value[currentLanguage.value] = code.value
        }

        currentLanguage.value = newLanguage

        // 优先恢复该语言之前编写的代码，没有则使用代码模板
        if (Object.prototype.hasOwnProperty.call(codeCache.value, newLanguage)) {
            code.value = codeCache.value[newLanguage]
        }
        else {
            code.value = filterPluginTemplate(newLanguage)
        }

        // 清空输出
        clearOutput()

        refreshEnvInfo()

        toast.info(`已切换到 ${getLanguageDisplayName(newLanguage)}`)
    }

    // 仅切换语言（刷新环境信息），不改动编辑器内容——用于打开文件时按扩展名切语言
    const applyLanguage = (newLanguage: string) => {
        if (newLanguage === currentLanguage.value) {
            return
        }
        currentLanguage.value = newLanguage
        refreshEnvInfo()
    }

    const refreshLanguageList = async () => {
        console.log('=== 开始刷新语言列表 ===')
        console.log('刷新前的语言列表:', supportedLanguages.value.map(l => l.name))

        await getConfigure()
        console.log('配置已重新加载')

        await getSupportedLanguages()
        console.log('语言列表已重新获取:', supportedLanguages.value.map(l => l.name))

        const currentStillAvailable = supportedLanguages.value.some(lang => lang.value === currentLanguage.value)
        console.log(`当前语言 ${currentLanguage.value} 是否仍然可用:`, currentStillAvailable)

        if (!currentStillAvailable && supportedLanguages.value.length > 0) {
            currentLanguage.value = supportedLanguages.value[0].value
            // 恢复该语言已缓存的代码，没有则使用代码模板
            if (Object.prototype.hasOwnProperty.call(codeCache.value, currentLanguage.value)) {
                code.value = codeCache.value[currentLanguage.value]
            }
            else {
                code.value = filterPluginTemplate(currentLanguage.value)
                codeCache.value[currentLanguage.value] = code.value
            }
            console.log('当前语言已禁用，切换到:', currentLanguage.value)
        }

        await refreshEnvInfo()
        console.log('=== 刷新语言列表完成 ===')
    }

    const initialize = async () => {
        await getConfigure()

        await getSupportedLanguages()

        if (supportedLanguages.value.length > 0) {
            currentLanguage.value = supportedLanguages.value[0].value
            console.log('当前语言:', currentLanguage.value)

            const template = filterPluginTemplate(currentLanguage.value)
            console.log('使用的模板:', template)
            code.value = template
            codeCache.value[currentLanguage.value] = template

            refreshEnvInfo()
        }
        else {
            code.value = 'No supported languages found'
        }
    }

    // 监听配置更新事件
    let unlistenConfigUpdate: (() => void) | null = null

    onMounted(async () => {
        unlistenConfigUpdate = await listen('config-updated', async () => {
            console.log('收到配置更新事件，重新加载语言列表和环境信息')
            await refreshLanguageList()
        })
    })

    onUnmounted(() => {
        if (unlistenConfigUpdate) {
            unlistenConfigUpdate()
        }
    })

    return {
        currentLanguage,
        supportedLanguages,
        globalConfig,
        envInfo,
        isLoadingEnvInfo,
        getLanguageDisplayName,
        handleLanguageChange,
        applyLanguage,
        refreshLanguageList,
        refreshEnvInfo,
        initialize,
        getCurrentPluginConfig,
        getCurrentConsoleType
    }
}
