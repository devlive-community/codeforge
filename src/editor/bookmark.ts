// 书签：按行高亮（左侧强调条 + 淡底色）。书签数据由外部(App)按当前文件 dispatch 填充。
import {Decoration, EditorView, type DecorationSet} from '@codemirror/view'
import {StateEffect, StateField} from '@codemirror/state'

// 由外部派发：设置当前文件的书签行号(1-based)
export const setBookmarks = StateEffect.define<number[]>()

const bookmarkLine = Decoration.line({class: 'cm-bookmark-line'})

export const bookmarkField = StateField.define<DecorationSet>({
  create: () => Decoration.none,
  update(deco, tr) {
    for (const e of tr.effects) {
      if (e.is(setBookmarks)) {
        const ranges = e.value
          .filter(ln => ln >= 1 && ln <= tr.state.doc.lines)
          .sort((a, b) => a - b)
          .map(ln => bookmarkLine.range(tr.state.doc.line(ln).from))
        return Decoration.set(ranges, true)
      }
    }
    // 文档变化时让标记跟随行偏移
    return deco.map(tr.changes)
  },
  provide: f => EditorView.decorations.from(f)
})

const bookmarkTheme = EditorView.baseTheme({
  '.cm-bookmark-line': {
    backgroundColor: 'rgba(99, 102, 241, 0.10)',
    boxShadow: 'inset 3px 0 0 #6366f1'
  }
})

export const bookmarkExtension = [bookmarkField, bookmarkTheme]
