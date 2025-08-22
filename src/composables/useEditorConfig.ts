import { ref, watch } from 'vue'
import { debounce } from 'lodash-es'
import { invoke } from '@tauri-apps/api/core'
import { useToast } from '../plugins/toast'

interface EditorConfig
{
    tab_size?: number
    theme?: string
    indent_with_tab?: boolean
    font_size?: number
}

interface ThemeOption
{
    label: string
    value: string
}

export function useEditorConfig(emit?: any)
{
    const toast = useToast()

    // 状态管理
    const globalConfig = ref<any>(null)
    const editorConfig = ref<EditorConfig | null>(null)

    // 主题选项
    const themeOptions = ref<ThemeOption[]>([
        { label: 'abcdef', value: 'abcdef' },
        { label: 'abyss', value: 'abyss' },
        { label: 'androidstudio', value: 'androidstudio' },
        { label: 'andromeda', value: 'andromeda' },
        { label: 'atomone', value: 'atomone' },
        { label: 'aura', value: 'aura' },
        { label: 'basicDark', value: 'basicDark' },
        { label: 'basicLight', value: 'basicLight' },
        { label: 'bbedit', value: 'bbedit' },
        { label: 'bespin', value: 'bespin' },
        { label: 'consoleDark', value: 'consoleDark' },
        { label: 'consoleLight', value: 'consoleLight' },
        { label: 'copilot', value: 'copilot' },
        { label: 'darcula', value: 'darcula' },
        { label: 'dracula', value: 'dracula' },
        { label: 'duotoneDark', value: 'duotoneDark' },
        { label: 'duotoneLight', value: 'duotoneLight' },
        { label: 'eclipse', value: 'eclipse' },
        { label: 'githubDark', value: 'githubDark' },
        { label: 'githubLight', value: 'githubLight' },
        { label: 'gruvboxDark', value: 'gruvboxDark' },
        { label: 'gruvboxLight', value: 'gruvboxLight' },
        { label: 'kimbie', value: 'kimbie' },
        { label: 'material', value: 'material' },
        { label: 'materialDark', value: 'materialDark' },
        { label: 'materialLight', value: 'materialLight' },
        { label: 'monokai', value: 'monokai' },
        { label: 'monokaiDimmed', value: 'monokaiDimmed' },
        { label: 'noctisLilac', value: 'noctisLilac' },
        { label: 'nord', value: 'nord' },
        { label: 'okaidia', value: 'okaidia' },
        { label: 'quietlight', value: 'quietlight' },
        { label: 'red', value: 'red' },
        { label: 'solarizedDark', value: 'solarizedDark' },
        { label: 'solarizedLight', value: 'solarizedLight' },
        { label: 'sublime', value: 'sublime' },
        { label: 'tokyoNight', value: 'tokyoNight' },
        { label: 'tokyoNightDay', value: 'tokyoNightDay' },
        { label: 'tokyoNightStorm', value: 'tokyoNightStorm' },
        { label: 'tomorrowNightBlue', value: 'tomorrowNightBlue' },
        { label: 'vscodeDark', value: 'vscodeDark' },
        { label: 'vscodeLight', value: 'vscodeLight' },
        { label: 'whiteDark', value: 'whiteDark' },
        { label: 'whiteLight', value: 'whiteLight' },
        { label: 'xcodeDark', value: 'xcodeDark' },
        { label: 'xcodeLight', value: 'xcodeLight' }
    ])

    // 加载配置
    const loadConfig = async () => {
        try {
            globalConfig.value = await invoke<any>('get_app_config')
            editorConfig.value = globalConfig.value.editor
            console.log('加载编辑器配置成功:', editorConfig.value)
        }
        catch (error) {
            console.error('加载编辑器配置失败:', error)
            toast.error('获取编辑器配置失败 - 错误信息: ' + error)
            throw error
        }
    }

    // 更新配置
    const updateConfig = async (updatedEditor: EditorConfig) => {
        console.log('更新编辑器配置:', updatedEditor)
        try {
            if (!globalConfig.value) {
                throw new Error('全局配置未加载')
            }

            globalConfig.value.editor = { ...updatedEditor }
            await invoke('update_app_config', { config: globalConfig.value })

            toast.success('编辑器配置已保存')

            // 发送事件（如果提供了 emit）
            if (emit) {
                emit('settings-changed', updatedEditor)
            }

            console.log('编辑器配置保存成功')
        }
        catch (error) {
            console.error('保存编辑器配置失败:', error)
            toast.error('保存编辑器配置失败 - 错误信息: ' + error)

            if (emit) {
                emit('error', '保存编辑器配置失败')
            }

            throw error
        }
    }

    // 防抖更新
    const debouncedUpdate = debounce((config: EditorConfig) => {
        updateConfig(config)
    }, 1000)

    // 监听配置变化
    watch(editorConfig, (newConfig, oldConfig) => {
        // 只有在非初始加载时才触发更新
        if (oldConfig && newConfig) {
            console.log('编辑器配置变化:', oldConfig, '->', newConfig)
            debouncedUpdate(newConfig)
        }
    }, {
        deep: true,
        flush: 'post'
    })

    // 手动保存配置（不防抖）
    const saveConfig = async () => {
        if (editorConfig.value) {
            await updateConfig(editorConfig.value)
        }
    }

    // 重置为默认配置
    const resetToDefault = () => {
        editorConfig.value = {
            indent_with_tab: true,
            tab_size: 2,
            theme: 'githubLight',
            font_size: 14
        }
    }

    // 获取特定主题的配置
    const getThemeByValue = (value: string) => {
        return themeOptions.value.find(theme => theme.value === value)
    }

    // 验证配置
    const validateConfig = (config: EditorConfig): boolean => {
        if (config.tab_size && (config.tab_size < 1 || config.tab_size > 8)) {
            toast.error('缩进空格数必须在 1-8 之间')
            return false
        }

        if (config.theme && !getThemeByValue(config.theme)) {
            toast.error('选择的主题不存在')
            return false
        }

        return true
    }

    return {
        // 状态
        globalConfig,
        editorConfig,
        themeOptions,

        // 方法
        loadConfig,
        updateConfig,
        saveConfig,
        resetToDefault,
        getThemeByValue,
        validateConfig
    }
}
