import { nextTick, ref, watch } from 'vue'
import { python } from '@codemirror/lang-python'
import { javascript } from '@codemirror/lang-javascript'
import { go } from '@codemirror/lang-go'
import { java } from '@codemirror/lang-java'
import {
    abcdef,
    abyss,
    androidstudio,
    andromeda,
    atomone,
    aura,
    basicDark,
    basicLight,
    bbedit,
    bespin,
    consoleDark,
    consoleLight,
    copilot,
    darcula,
    dracula,
    duotoneDark,
    duotoneLight,
    eclipse,
    githubDark,
    githubLight,
    gruvboxDark,
    gruvboxLight,
    kimbie,
    material,
    materialDark,
    materialLight,
    monokai,
    monokaiDimmed,
    noctisLilac,
    nord,
    okaidia,
    quietlight,
    red,
    solarizedDark,
    solarizedLight,
    sublime,
    tokyoNight,
    tokyoNightDay,
    tokyoNightStorm,
    tomorrowNightBlue,
    vscodeDark,
    vscodeLight,
    whiteDark,
    whiteLight,
    xcodeDark,
    xcodeLight
} from '@uiw/codemirror-themes-all'
import { invoke } from '@tauri-apps/api/core'
import { useToast } from '../plugins/toast'

interface EditorConfig
{
    theme?: string
    indent_with_tab?: boolean
    tab_size?: number
}

interface Props
{
    modelValue: string
    language?: string
}

interface Emit
{
    (event: 'update:modelValue', value: string): void
}

export function useCodeMirrorEditor(props: Props, _emit: Emit)
{
    const toast = useToast()

    // 状态管理
    const isReady = ref(false)
    const extensions = ref<any[]>([])
    const editorConfig = ref<EditorConfig>({})

    // 主题映射
    const themeMap: Record<string, any> = {
        abcdef,
        abyss,
        androidstudio,
        andromeda,
        atomone,
        aura,
        basicLight,
        basicDark,
        bbedit,
        bespin,
        consoleLight,
        consoleDark,
        copilot,
        darcula,
        dracula,
        duotoneLight,
        duotoneDark,
        eclipse,
        githubLight,
        githubDark,
        gruvboxLight,
        gruvboxDark,
        kimbie,
        material,
        materialLight,
        materialDark,
        monokai,
        monokaiDimmed,
        noctisLilac,
        nord,
        okaidia,
        quietlight,
        red,
        solarizedLight,
        solarizedDark,
        sublime,
        tokyoNight,
        tokyoNightStorm,
        tokyoNightDay,
        tomorrowNightBlue,
        vscodeLight,
        vscodeDark,
        whiteLight,
        whiteDark,
        xcodeLight,
        xcodeDark
    }

    // 获取主题扩展
    const getThemeExtension = (themeName?: string) => {
        if (!themeName) {
            return githubLight // 默认主题
        }

        const theme = themeMap[themeName]
        if (theme) {
            return theme
        }

        console.warn(`主题 "${ themeName }" 未找到，使用默认主题`)
        return githubLight
    }

    // 获取语言扩展
    const getLanguageExtension = (language: string): any | null => {
        switch (language) {
            case 'python2':
            case 'python3':
                return python()
            case 'nodejs':
                return javascript()
            case 'go':
                return go()
            case 'java':
                return java()
            default:
                return null
        }
    }

    // 更新扩展的函数
    const updateExtensions = async () => {
        const result = []

        // 添加主题扩展
        const themeExtension = getThemeExtension(editorConfig.value?.theme)
        result.push(themeExtension)

        // 添加语言扩展
        if (props.language) {
            const langExtension = getLanguageExtension(props.language)
            if (langExtension) {
                result.push(langExtension)
            }
        }

        extensions.value = result

        // 如果组件还没准备好，等待下一个 tick 后设置为准备好
        if (!isReady.value) {
            await nextTick()
            isReady.value = true
        }
    }

    // 加载编辑器配置
    const loadEditorConfig = async () => {
        try {
            const globalConfig = await invoke<any>('get_app_config')

            if (globalConfig && globalConfig.editor) {
                editorConfig.value = globalConfig.editor
                console.log('加载编辑器配置:', editorConfig.value)

                // 配置加载后重新更新扩展
                await updateExtensions()
            }
            else {
                // 如果没有配置，使用默认配置
                editorConfig.value = {
                    theme: 'githubLight',
                    indent_with_tab: true,
                    tab_size: 2
                }
                await updateExtensions()
            }
        }
        catch (error) {
            console.error('获取配置失败:', error)
            toast.error('获取配置失败 - 错误信息: ' + error)

            // 失败时使用默认配置
            editorConfig.value = {
                theme: 'githubLight',
                indent_with_tab: true,
                tab_size: 2
            }
            await updateExtensions()
        }
    }

    // 重新渲染编辑器
    const reRenderEditor = async () => {
        isReady.value = false
        await nextTick()
        await updateExtensions()
    }

    // 设置主题
    const setTheme = async (themeName: string) => {
        if (themeMap[themeName]) {
            editorConfig.value.theme = themeName
            await reRenderEditor()
        }
        else {
            console.warn(`主题 "${ themeName }" 不存在`)
        }
    }

    // 获取可用主题列表
    const getAvailableThemes = () => {
        return Object.keys(themeMap)
    }

    // 获取当前主题
    const getCurrentTheme = () => {
        return editorConfig.value?.theme || 'githubLight'
    }

    // 初始化编辑器
    const initializeEditor = async () => {
        await loadEditorConfig()
    }

    // 监听语言变化
    watch(() => props.language, async () => {
        console.log('语言变化:', props.language)
        await reRenderEditor()
    }, { immediate: false })

    // 监听编辑器配置变化
    watch(() => editorConfig.value?.theme, async (newTheme, oldTheme) => {
        if (newTheme && newTheme !== oldTheme) {
            console.log('主题变化:', oldTheme, '->', newTheme)
            await reRenderEditor()
        }
    }, { immediate: false })

    // 监听缩进配置变化
    watch(() => [editorConfig.value?.indent_with_tab, editorConfig.value?.tab_size], async () => {
        // 缩进配置变化时重新渲染
        await reRenderEditor()
    }, { immediate: false })

    return {
        // 状态
        isReady,
        extensions,
        editorConfig,

        // 方法
        initializeEditor,
        loadEditorConfig,
        updateExtensions,
        reRenderEditor,
        setTheme,
        getAvailableThemes,
        getCurrentTheme,
        getThemeExtension,
        getLanguageExtension
    }
}
