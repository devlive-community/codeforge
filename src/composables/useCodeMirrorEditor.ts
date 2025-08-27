import {nextTick, ref, watch} from 'vue'
import {python} from '@codemirror/lang-python'
import {javascript} from '@codemirror/lang-javascript'
import {go} from '@codemirror/lang-go'
import {java} from '@codemirror/lang-java'
import {rust} from '@codemirror/lang-rust'
import {cpp} from '@codemirror/lang-cpp'
import {shell} from '@codemirror/legacy-modes/mode/shell'
import {swift} from '@codemirror/legacy-modes/mode/swift'
import {kotlin, scala} from '@codemirror/legacy-modes/mode/clike'
import {clojure} from '@codemirror/legacy-modes/mode/clojure'
import {ruby} from '@codemirror/legacy-modes/mode/ruby'
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
import {invoke} from '@tauri-apps/api/core'
import {useToast} from '../plugins/toast'
import {StreamLanguage} from '@codemirror/language'
import {EditorConfig} from '../types/app.ts'
import {useCodeMirrorFunctionHelp} from './useCodeMirrorFunctionHelp'
import {useCodeMirrorSpaceOmission} from './useCodeMirrorSpaceOmission.ts'
import {EditorView} from "@codemirror/view";
import {useCodeMirrorFontFamily} from "./useCodeMirrorFontFamily.ts";

interface Props
{
    modelValue: string
    language?: string
}

export function useCodeMirrorEditor(props: Props)
{
    const toast = useToast()
    const {showFunctionHelpHover, functionHelpTheme} = useCodeMirrorFunctionHelp()

    // 状态管理
    const isReady = ref(false)
    const extensions = ref<any[]>([])
    const editorConfig = ref<EditorConfig>({})
    const defaultConfig = {
        theme: 'githubLight',
        indent_with_tab: true,
        tab_size: 2,
        font_size: 14,
        font_family: 'monospace',
        show_line_numbers: false,
        show_function_help: false
    }

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

        console.warn(`主题 "${themeName}" 未找到，使用默认主题`)
        return githubLight
    }

    // 获取语言扩展
    const getLanguageExtension = (language: string): any | null => {
        switch (language) {
            case 'python2':
            case 'python3':
                return python()
            case 'nodejs':
            case 'javascript-jquery':
            case 'javascript-browser':
            case 'javascript-nodejs':
                return javascript()
            case 'go':
                return go()
            case 'java':
                return java()
            case 'rust':
                return rust()
            case 'c':
            case 'cpp':
                return cpp()
            case 'shell':
            case 'applescript':
                return StreamLanguage.define(shell)
            case 'swift':
                return StreamLanguage.define(swift)
            case 'scala':
                return StreamLanguage.define(scala)
            case 'kotlin':
                return StreamLanguage.define(kotlin)
            case 'clojure':
                return StreamLanguage.define(clojure)
            case 'ruby':
                return StreamLanguage.define(ruby)
            case 'typescript':
            case 'typescript-browser':
            case 'typescript-nodejs':
                return javascript({typescript: true})
            default:
                return null
        }
    }

    // 隐藏行号的主题扩展
    const hideLineNumbersTheme = EditorView.theme({
        '.cm-lineNumbers': {
            display: 'none !important'
        },
        '.cm-gutters': {
            display: 'none !important'
        }
    })

    // 更新扩展的函数
    const updateExtensions = async (showLineNumbers?: boolean, showFunctionHelp?: boolean) => {
        const result = []

        // 添加主题扩展
        const themeExtension = getThemeExtension(editorConfig.value?.theme)
        result.push(themeExtension)

        // 添加函数帮助主题
        result.push(functionHelpTheme)

        // 设置字体
        const {fontFamilyTheme} = useCodeMirrorFontFamily(
            editorConfig.value?.font_family
        )
        result.push(fontFamilyTheme)

        // 添加语言扩展
        if (props.language) {
            const langExtension = getLanguageExtension(props.language)
            if (langExtension) {
                result.push(langExtension)
            }
        }

        // 处理行号显示逻辑
        const shouldShowLineNumbers = showLineNumbers ?? editorConfig.value?.show_line_numbers ?? false
        // 如果配置为不显示行号，则添加隐藏行号的扩展
        if (!shouldShowLineNumbers) {
            result.push(hideLineNumbersTheme)
        }

        const shouldShowFunctionHelp = showFunctionHelp ?? editorConfig.value?.show_function_help ?? false
        if (shouldShowFunctionHelp) {
            result.push(showFunctionHelpHover)
        }

        const shouldShowSpaceOmission = editorConfig.value?.space_dot_omission ?? false
        if (shouldShowSpaceOmission) {
            const {spaceOmissionPlugin, spaceOmissionTheme} = useCodeMirrorSpaceOmission(editorConfig.value?.font_family)
            result.push(spaceOmissionPlugin)
            result.push(spaceOmissionTheme)
        }

        extensions.value = result

        // 如果组件还没准备好，等待下一个 tick 后设置为准备好
        if (!isReady.value) {
            await nextTick()
            isReady.value = true
        }
    }

    // 其余代码完全保持不变...
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
                editorConfig.value = defaultConfig
                await updateExtensions()
            }
        }
        catch (error) {
            console.error('获取配置失败:', error)
            toast.error('获取配置失败 - 错误信息: ' + error)
            editorConfig.value = defaultConfig
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
            console.warn(`主题 "${themeName}" 不存在`)
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
    }, {immediate: false})

    // 监听编辑器配置变化
    watch(() => editorConfig.value?.theme, async (newTheme, oldTheme) => {
        if (newTheme && newTheme !== oldTheme) {
            console.log('主题变化:', oldTheme, '->', newTheme)
            await reRenderEditor()
        }
    }, {immediate: false})

    // 监听缩进配置变化
    watch(() => [editorConfig.value?.indent_with_tab, editorConfig.value?.tab_size], async () => {
        // 缩进配置变化时重新渲染
        await reRenderEditor()
    }, {immediate: false})

    // 监听行号显示配置变化
    watch(() => editorConfig.value?.show_line_numbers, async () => {
        console.log('行号显示配置变化:', editorConfig.value?.show_line_numbers)
        await reRenderEditor()
    }, {immediate: false})

    // 监听函数帮助配置变化
    watch(() => editorConfig.value?.show_function_help, async () => {
        console.log('函数帮助配置变化:', editorConfig.value?.show_function_help)
        await reRenderEditor()
    })

    watch(() => editorConfig.value?.space_dot_omission, async () => {
        console.log('是否显示空格省略:', editorConfig.value?.space_dot_omission)
        await reRenderEditor()
    })

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
