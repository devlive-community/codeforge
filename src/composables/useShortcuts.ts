import {computed, ref} from 'vue'
import {kvGetJSON, kvSetJSON} from './useKvStore'
import {i18n} from '../i18n'

export interface ShortcutAction
{
    id: string
    label: string
    default: string
}

// 可自定义的快捷键动作及默认绑定（Mod = mac 上 ⌘，其他平台 Ctrl）。
// 动作名（label）由 i18n 提供（shortcutAction.<id>），随界面语言切换。
const SHORTCUT_DEFS: { id: string; default: string }[] = [
    {id: 'run', default: 'Mod+Enter'},
    {id: 'runSelection', default: 'Mod+Shift+Enter'},
    {id: 'save', default: 'Mod+S'},
    {id: 'saveAs', default: 'Mod+Shift+S'},
    {id: 'open', default: 'Mod+O'},
    {id: 'quickOpen', default: 'Mod+P'},
    {id: 'commandPalette', default: 'Mod+Shift+P'},
    {id: 'gotoLine', default: 'Mod+G'},
    {id: 'outline', default: 'Mod+Shift+O'},
    {id: 'searchInFiles', default: 'Mod+Shift+F'},
    {id: 'generate', default: 'Mod+K'},
    {id: 'newTab', default: 'Mod+N'},
    {id: 'closeTab', default: 'Mod+W'},
    {id: 'toggleSidebar', default: 'Mod+B'},
    {id: 'toggleTerminal', default: 'Mod+`'},
    {id: 'toggleWordWrap', default: 'Alt+Z'}
]

const STORAGE_KEY = 'shortcuts'

const isMac = typeof navigator !== 'undefined' && /Mac/i.test(navigator.platform)

// 根据键盘事件构造组合字符串（顺序固定：Mod, Alt, Shift, Key）；仅按修饰键时返回 null
export const comboFromEvent = (e: KeyboardEvent): string | null => {
    if (['Control', 'Meta', 'Shift', 'Alt'].includes(e.key)) {
        return null
    }
    const parts: string[] = []
    if (e.metaKey || e.ctrlKey) parts.push('Mod')
    if (e.altKey) parts.push('Alt')
    if (e.shiftKey) parts.push('Shift')

    let key = e.key
    if (key === ' ') key = 'Space'
    else if (key.length === 1) key = key.toUpperCase()

    parts.push(key)
    return parts.join('+')
}

// 用于展示的可读形式
export const formatCombo = (combo: string): string => {
    return combo
        .split('+')
        .map(part => {
            if (part === 'Mod') return isMac ? '⌘' : 'Ctrl'
            if (part === 'Shift') return isMac ? '⇧' : 'Shift'
            if (part === 'Alt') return isMac ? '⌥' : 'Alt'
            return part
        })
        .join(isMac ? '' : '+')
}

export function useShortcuts()
{
    const overrides = ref<Record<string, string>>({})

    const load = () => {
        overrides.value = kvGetJSON<Record<string, string>>(STORAGE_KEY, {})
    }
    load()

    const persist = () => {
        kvSetJSON(STORAGE_KEY, overrides.value)
    }

    // 当前生效的绑定（默认 + 覆盖）
    const bindings = computed<Record<string, string>>(() => {
        const map: Record<string, string> = {}
        for (const action of SHORTCUT_DEFS) {
            map[action.id] = overrides.value[action.id] || action.default
        }
        return map
    })

    const getBinding = (id: string) => bindings.value[id] || ''

    const setBinding = (id: string, combo: string) => {
        overrides.value = {...overrides.value, [id]: combo}
        persist()
    }

    const resetBinding = (id: string) => {
        const next = {...overrides.value}
        delete next[id]
        overrides.value = next
        persist()
    }

    const resetAll = () => {
        overrides.value = {}
        persist()
    }

    // 匹配事件对应的动作 id，无匹配返回 null
    const matchAction = (e: KeyboardEvent): string | null => {
        const combo = comboFromEvent(e)
        if (!combo) {
            return null
        }
        for (const action of SHORTCUT_DEFS) {
            if (bindings.value[action.id] === combo) {
                return action.id
            }
        }
        return null
    }

    // 动作列表：label 从 i18n 取，随语言响应式更新
    const actions = computed<ShortcutAction[]>(() =>
        SHORTCUT_DEFS.map(a => ({id: a.id, label: i18n.global.t(`shortcutAction.${a.id}`), default: a.default}))
    )

    return {
        actions,
        bindings,
        getBinding,
        setBinding,
        resetBinding,
        resetAll,
        matchAction,
        formatCombo,
        reload: load
    }
}
