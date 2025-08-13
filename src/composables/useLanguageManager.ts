import { ref, type Ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { EnvInfo, Language, LanguageInfo } from '../types/app.ts'

export function useLanguageManager(
    code: Ref<string>,
    clearOutput: () => void,
    toast: any
)
{
    const currentLanguage = ref('python2')
    const supportedLanguages = ref<Language[]>([])
    const globalConfig = ref(null as any)

    const envInfo = ref<EnvInfo>({
        installed: false,
        version: '检查中...',
        path: '检查中...',
        language: 'python'
    })

    const getLanguageDisplayName = (languageValue: string) => {
        const language = supportedLanguages.value.find(lang => lang.value === languageValue)
        return language ? language.name : languageValue
    }

    const refreshEnvInfo = async () => {
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
                language: currentLanguage.value
            }
        }
    }

    const getSupportedLanguages = async () => {
        try {
            const languages = await invoke<Language[]>('get_supported_languages')
            supportedLanguages.value = languages.map((language) => ({
                name: language.name,
                value: language.value,
                svgUrl: `/icons/${ language.value.replace(/\d+$/, '') }.svg`
            }))

            // 设置默认语言
            if (supportedLanguages.value.length > 0 && !currentLanguage.value) {
                currentLanguage.value = supportedLanguages.value[0].value
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
        if (globalConfig.value && globalConfig.value.plugins) {
            return globalConfig.value.plugins.find((p: any) => p.language === plugin).template
        }
        return null
    }

    const handleLanguageChange = async (newLanguage: string) => {
        currentLanguage.value = newLanguage

        // 更新代码模板
        code.value = filterPluginTemplate(newLanguage)

        // 清空输出
        clearOutput()

        // 刷新环境信息
        await refreshEnvInfo()

        toast.info(`已切换到 ${ getLanguageDisplayName(newLanguage) }`)
    }

    const initialize = async () => {
        await getSupportedLanguages()
        await refreshEnvInfo()
        await getConfigure()

        // 设置初始代码模板
        if (supportedLanguages.value.length > 0) {
            currentLanguage.value = supportedLanguages.value[0].value
            console.log('当前语言:', currentLanguage.value)
            const template = filterPluginTemplate(currentLanguage.value)
            console.log('使用的模板:', template)
            code.value = template
        }
        else {
            code.value = 'No supported languages found'
        }
    }

    return {
        currentLanguage,
        supportedLanguages,
        envInfo,
        getLanguageDisplayName,
        handleLanguageChange,
        initialize
    }
}
