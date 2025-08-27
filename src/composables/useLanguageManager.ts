import {ref, type Ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
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
            supportedLanguages.value = languages.map((language) => ({
                name: language.name,
                value: language.value,
                svgUrl: `/icons/${language.value.replace(/\d+$/, '')}.svg`
            }))
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
        if (globalConfig.value && globalConfig.value.plugins) {
            return globalConfig.value.plugins.find((p: any) => p.language === plugin)?.template || ''
        }
        return ''
    }

    const handleLanguageChange = async (newLanguage: string) => {
        currentLanguage.value = newLanguage

        // 更新代码模板
        code.value = filterPluginTemplate(newLanguage)

        // 清空输出
        clearOutput()

        refreshEnvInfo()

        toast.info(`已切换到 ${getLanguageDisplayName(newLanguage)}`)
    }

    const initialize = async () => {
        // 获取支持的语言列表
        await getSupportedLanguages()

        // 获取配置
        await getConfigure()

        // 设置默认语言和初始代码模板
        if (supportedLanguages.value.length > 0) {
            currentLanguage.value = supportedLanguages.value[0].value
            console.log('当前语言:', currentLanguage.value)

            const template = filterPluginTemplate(currentLanguage.value)
            console.log('使用的模板:', template)
            code.value = template

            refreshEnvInfo()
        }
        else {
            code.value = 'No supported languages found'
        }
    }

    return {
        currentLanguage,
        supportedLanguages,
        globalConfig,
        envInfo,
        isLoadingEnvInfo,
        getLanguageDisplayName,
        handleLanguageChange,
        initialize,
        getCurrentPluginConfig,
        getCurrentConsoleType
    }
}
