import {EditorView} from '@codemirror/view'
import {ref} from 'vue'

export interface CursorInfo
{
    line: number
    col: number
    // 选中的字符数（无选中为 0）
    selLen: number
}

// 当前光标位置/选中长度（供状态栏显示）
export const cursorInfo = ref<CursorInfo>({line: 1, col: 1, selLen: 0})

// 监听选区/文档变化，更新光标信息
export const cursorListener = EditorView.updateListener.of((u) => {
    if (!u.selectionSet && !u.docChanged) {
        return
    }
    const sel = u.state.selection.main
    const line = u.state.doc.lineAt(sel.head)
    cursorInfo.value = {
        line: line.number,
        col: sel.head - line.from + 1,
        selLen: Math.abs(sel.to - sel.from)
    }
})
