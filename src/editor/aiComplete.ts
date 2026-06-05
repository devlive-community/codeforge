import {Decoration, EditorView, WidgetType, keymap} from '@codemirror/view'
import {Prec, StateEffect, StateField} from '@codemirror/state'

export interface Ghost
{
    text: string
    pos: number
}

// 设置/清除幽灵补全文本
export const setGhost = StateEffect.define<Ghost>()
export const clearGhost = StateEffect.define<null>()

// 幽灵文本 widget（灰色、不可选中，支持多行）
class GhostWidget extends WidgetType
{
    constructor(readonly text: string)
    {
        super()
    }

    eq(other: GhostWidget)
    {
        return other.text === this.text
    }

    toDOM()
    {
        const span = document.createElement('span')
        span.className = 'cm-ai-ghost'
        span.textContent = this.text
        return span
    }

    ignoreEvent()
    {
        return true
    }
}

// 持有当前幽灵补全：任何文档改动/光标移动都会清除，仅 setGhost 显式设置
const ghostField = StateField.define<Ghost | null>({
    create: () => null,
    update(value, tr) {
        for (const e of tr.effects) {
            if (e.is(setGhost)) {
                return e.value
            }
            if (e.is(clearGhost)) {
                return null
            }
        }
        if (tr.docChanged || tr.selection) {
            return null
        }
        return value
    },
    provide: f => EditorView.decorations.from(f, (g) => {
        if (!g) {
            return Decoration.none
        }
        const deco = Decoration.widget({
            widget: new GhostWidget(g.text),
            side: 1
        })
        return Decoration.set([deco.range(g.pos)])
    })
})

// 是否有幽灵补全显示
export const hasGhost = (view: EditorView): boolean => view.state.field(ghostField, false) != null

// 接受补全：插入文本并把光标移到末尾
const acceptGhost = (view: EditorView): boolean => {
    const g = view.state.field(ghostField, false)
    if (!g) {
        return false
    }
    view.dispatch({
        changes: {from: g.pos, insert: g.text},
        selection: {anchor: g.pos + g.text.length},
        effects: clearGhost.of(null)
    })
    return true
}

// 取消补全
const dismissGhost = (view: EditorView): boolean => {
    if (!view.state.field(ghostField, false)) {
        return false
    }
    view.dispatch({effects: clearGhost.of(null)})
    return true
}

// Tab 接受、Esc 取消；无补全时返回 false 回落到默认行为
const ghostKeymap = Prec.highest(keymap.of([
    {key: 'Tab', run: acceptGhost},
    {key: 'Escape', run: dismissGhost}
]))

const ghostTheme = EditorView.baseTheme({
    '.cm-ai-ghost': {
        opacity: '0.4',
        color: '#8b949e',
        whiteSpace: 'pre-wrap',
    },
})

// 清除当前补全（外部调用，如关闭功能时）
export const clearGhostIn = (view: EditorView | null | undefined) => {
    if (view && hasGhost(view)) {
        view.dispatch({effects: clearGhost.of(null)})
    }
}

export const aiCompleteExtension = [ghostField, ghostKeymap, ghostTheme]
