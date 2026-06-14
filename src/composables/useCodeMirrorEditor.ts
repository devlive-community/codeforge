import {nextTick, ref, shallowRef, watch} from 'vue'
import {debounce} from 'lodash-es'
import {useTheme} from './useTheme'
import {python} from '@codemirror/lang-python'
import {javascript} from '@codemirror/lang-javascript'
import {go} from '@codemirror/lang-go'
import {java} from '@codemirror/lang-java'
import {rust} from '@codemirror/lang-rust'
import {cpp} from '@codemirror/lang-cpp'
import {html} from '@codemirror/lang-html'
import {css} from '@codemirror/lang-css'
import {xml} from '@codemirror/lang-xml'
import {php} from '@codemirror/lang-php'
import {shell} from '@codemirror/legacy-modes/mode/shell'
import {swift} from '@codemirror/legacy-modes/mode/swift'
import {kotlin, objectiveC, objectiveCpp, scala} from '@codemirror/legacy-modes/mode/clike'
import {clojure} from '@codemirror/legacy-modes/mode/clojure'
import {ruby} from '@codemirror/legacy-modes/mode/ruby'
import {groovy} from '@codemirror/legacy-modes/mode/groovy'
import {r} from "@codemirror/legacy-modes/mode/r"
import {haskell} from "@codemirror/legacy-modes/mode/haskell"
import {lua} from "@codemirror/legacy-modes/mode/lua"
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
import {EditorView, keymap} from "@codemirror/view";
import {Prec} from "@codemirror/state";
import {useCodeMirrorFontFamily} from "./useCodeMirrorFontFamily.ts";
import {diffGutterExtension} from "../editor/diffGutter";
import {aiCompleteExtension} from "../editor/aiComplete";
import {cursorListener} from "../editor/cursorInfo";
import {json} from "@codemirror/lang-json";
import {markdown} from "@codemirror/lang-markdown";
import {yaml} from "@codemirror/lang-yaml";
import {sql} from "@codemirror/lang-sql";
import {useSnippets} from "./useSnippets";
import {createLspExtensions} from "../editor/lspExtension";

interface Props
{
    modelValue: string
    language?: string
    filePath?: string | null
    rootDir?: string | null
}

// 自定义提示框样式（悬浮文档 / 诊断 / 补全），跟随明暗主题
function buildTooltipTheme(dark: boolean) {
    const bg = dark ? '#1f2937' : '#ffffff'
    const border = dark ? '#374151' : '#e5e7eb'
    const text = dark ? '#e5e7eb' : '#1f2937'
    const codeBg = dark ? '#111827' : '#f3f4f6'
    const sel = dark ? '#2563eb' : '#dbeafe'
    const selText = dark ? '#ffffff' : '#1e3a8a'
    return EditorView.theme({
        '.cm-tooltip': {
            border: `1px solid ${border}`,
            borderRadius: '8px',
            backgroundColor: bg,
            color: text,
            boxShadow: dark ? '0 8px 28px rgba(0,0,0,0.5)' : '0 8px 28px rgba(0,0,0,0.14)',
            fontSize: '12px',
            overflow: 'hidden'
        },
        '.cm-tooltip.cm-tooltip-hover': {maxWidth: '480px'},
        '.cm-tooltip-hover .cm-tooltip-section': {
            padding: '8px 10px',
            borderTop: `1px solid ${border}`,
            lineHeight: '1.5'
        },
        '.cm-tooltip-hover .cm-tooltip-section:first-child': {borderTop: 'none'},
        '.cm-tooltip-hover pre, .cm-tooltip-hover code': {
            backgroundColor: codeBg,
            borderRadius: '4px',
            padding: '1px 4px',
            fontFamily: 'monospace',
            whiteSpace: 'pre-wrap'
        },
        '.cm-tooltip-hover pre': {padding: '8px 10px', margin: '4px 0', overflowX: 'auto'},
        // 诊断悬浮
        '.cm-tooltip.cm-tooltip-lint': {padding: '0'},
        '.cm-diagnostic': {padding: '6px 10px', borderLeft: 'none', marginLeft: '0'},
        '.cm-diagnostic-error': {borderLeft: '3px solid #ef4444'},
        '.cm-diagnostic-warning': {borderLeft: '3px solid #f59e0b'},
        '.cm-diagnostic-info': {borderLeft: '3px solid #3b82f6'},
        // 自动补全
        '.cm-tooltip-autocomplete > ul': {fontFamily: 'monospace', maxHeight: '16em'},
        '.cm-tooltip-autocomplete > ul > li': {padding: '3px 8px'},
        '.cm-tooltip-autocomplete > ul > li[aria-selected]': {
            backgroundColor: sel,
            color: selText
        },
        '.cm-completionIcon': {opacity: '0.7', paddingRight: '6px'},
        '.cm-completionDetail': {color: dark ? '#9ca3af' : '#6b7280', fontStyle: 'normal'}
    }, {dark})
}

export function useCodeMirrorEditor(props: Props)
{
    const toast = useToast()
    const {isDark} = useTheme()
    const {showFunctionHelpHover, functionHelpTheme} = useCodeMirrorFunctionHelp()

    // 状态管理
    const isReady = ref(false)
    // shallowRef：扩展里含 CodeMirror/LSP 客户端等大量可变内部状态的对象，
    // 绝不能被 Vue 深层响应式代理（会拖垮性能并触发"诊断→重建→重连"无限循环导致页面卡死）。
    // 仅在显式整体赋值 extensions.value = result 时才触发重建。
    const extensions = shallowRef<any[]>([])
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

    // 字体大小范围（与设置面板保持一致）
    const MIN_FONT_SIZE = 1
    const MAX_FONT_SIZE = 30
    const DEFAULT_FONT_SIZE = 14

    // 防抖持久化字体大小，避免连续快捷键导致频繁写盘
    const persistFontSize = debounce(async (size: number) => {
        try {
            const globalConfig = await invoke<any>('get_app_config')
            if (globalConfig) {
                globalConfig.editor = {...(globalConfig.editor || {}), font_size: size}
                await invoke('update_app_config', {config: globalConfig})
            }
        }
        catch (error) {
            console.error('保存字体大小失败:', error)
        }
    }, 500)

    // 设置字体大小（内联样式响应式更新，无需重新渲染编辑器）
    const setFontSize = (size: number) => {
        const clamped = Math.max(MIN_FONT_SIZE, Math.min(MAX_FONT_SIZE, Math.round(size)))
        if (editorConfig.value.font_size === clamped) {
            return
        }
        editorConfig.value.font_size = clamped
        persistFontSize(clamped)
    }

    const increaseFontSize = () => setFontSize((editorConfig.value?.font_size || DEFAULT_FONT_SIZE) + 1)
    const decreaseFontSize = () => setFontSize((editorConfig.value?.font_size || DEFAULT_FONT_SIZE) - 1)
    const resetFontSize = () => setFontSize(DEFAULT_FONT_SIZE)

    // 字体缩放快捷键：Cmd/Ctrl +/-/0
    const fontSizeKeymap = keymap.of([
        {key: 'Mod-=', preventDefault: true, run: () => (increaseFontSize(), true)},
        {key: 'Mod-+', preventDefault: true, run: () => (increaseFontSize(), true)},
        {key: 'Shift-Mod-=', preventDefault: true, run: () => (increaseFontSize(), true)},
        {key: 'Mod--', preventDefault: true, run: () => (decreaseFontSize(), true)},
        {key: 'Mod-0', preventDefault: true, run: () => (resetFontSize(), true)}
    ])

    // 代码片段：在光标前的单词等于某片段前缀时，按 Tab 展开（$0 / ${0} 为光标落点）
    const {findByPrefix} = useSnippets()
    const expandSnippet = (view: EditorView): boolean => {
        const {state} = view
        const sel = state.selection.main
        if (!sel.empty) {
            return false
        }
        const lineFrom = state.doc.lineAt(sel.head).from
        const before = state.sliceDoc(lineFrom, sel.head)
        const m = /([A-Za-z_]\w*)$/.exec(before)
        if (!m) {
            return false
        }
        const snip = findByPrefix(m[1], props.language || '')
        if (!snip) {
            return false
        }
        const from = sel.head - m[1].length
        let body = snip.body
        let cursorOffset = -1
        const ph = /\$\{0\}|\$0/.exec(body)
        if (ph) {
            cursorOffset = ph.index
            body = body.slice(0, ph.index) + body.slice(ph.index + ph[0].length)
        }
        view.dispatch({
            changes: {from, to: sel.head, insert: body},
            selection: {anchor: cursorOffset >= 0 ? from + cursorOffset : from + body.length},
            scrollIntoView: true
        })
        return true
    }
    // 高优先级拦截 Tab；未匹配片段则返回 false，回落到默认缩进
    // 比默认高、但低于 AI 幽灵补全的 Tab（接受补全优先）
    const snippetKeymap = Prec.high(keymap.of([{key: 'Tab', run: expandSnippet}]))

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
            case 'cangjie':
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
            case 'groovy':
                return StreamLanguage.define(groovy)
            case 'html':
                return html()
            case 'css':
                return css()
            case 'svg':
                return xml()
            case 'php':
                return php()
            case 'r':
                return StreamLanguage.define(r)
            case 'haskell':
                return StreamLanguage.define(haskell)
            case 'lua':
                return StreamLanguage.define(lua)
            case 'objective-c':
                return StreamLanguage.define(objectiveC)
            case 'objective-cpp':
                return StreamLanguage.define(objectiveCpp)
            case 'json':
                return json()
            case 'yaml':
                return yaml()
            case 'markdown':
                return markdown()
            case 'xml':
                return xml()
            case 'sql':
                return sql()
            case 'text':
                return null
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

        // 添加主题扩展：暗色模式下使用深色编辑器主题
        const themeName = isDark.value ? 'githubDark' : (editorConfig.value?.theme || 'githubLight')
        result.push(getThemeExtension(themeName))

        // 添加函数帮助主题
        result.push(functionHelpTheme)

        // 字体缩放快捷键（搜索/替换、折叠、括号匹配等由 vue-codemirror 的 basicSetup 提供）
        result.push(fontSizeKeymap)

        // 代码片段 Tab 展开
        result.push(snippetKeymap)

        // AI 幽灵补全（Tab 接受 / Esc 取消），由外部 dispatch 设置
        result.push(aiCompleteExtension)

        // 光标位置/选中长度（供状态栏显示）
        result.push(cursorListener)

        // Git 行内差异标记（标记数据由外部 dispatch 填充，无 git 时为空）
        result.push(diffGutterExtension)

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

        // 自定义悬浮/诊断/补全提示框样式（主题适配，仅样式不改定位）
        result.push(buildTooltipTheme(isDark.value))

        // LSP 语义能力（补全/悬浮/诊断/跳转/重命名）；草稿用 untitled 文档，无服务器时为 null
        if (props.language) {
            try {
                const lsp = await createLspExtensions(props.language, props.filePath, props.rootDir, {
                    tabSize: editorConfig.value?.tab_size ?? 4,
                    insertSpaces: !editorConfig.value?.indent_with_tab
                })
                if (lsp) {
                    result.push(lsp)
                }
            }
            catch (e) {
                console.warn('LSP 初始化失败:', e)
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

    // 文件切换：重建扩展以切换 LSP 文档（就地重配置，避免闪烁）
    watch(() => props.filePath, async () => {
        await updateExtensions()
    }, {immediate: false})

    // 监听编辑器配置变化
    watch(() => editorConfig.value?.theme, async (newTheme, oldTheme) => {
        if (newTheme && newTheme !== oldTheme) {
            console.log('主题变化:', oldTheme, '->', newTheme)
            await reRenderEditor()
        }
    }, {immediate: false})

    // 跟随应用深色模式切换编辑器主题
    // 仅更新扩展（vue-codemirror 会就地重配置），避免卸载重挂导致的白色闪烁
    watch(isDark, async () => {
        await updateExtensions()
    })

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
        getLanguageExtension,
        increaseFontSize,
        decreaseFontSize,
        resetFontSize,
        setFontSize
    }
}
