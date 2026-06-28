// 编辑器文本变换命令：排序行、大小写转换、去重、去行尾空白。
// 仅依赖当前 EditorView，无副作用，便于从 App.vue 抽离复用。
import type {Ref} from 'vue'
import type {EditorView} from '@codemirror/view'

export function useTextCommands(editorView: Ref<EditorView | null>) {
  // 对选区做变换；无选区时作用于当前行
  const transformSelectionOrLine = (fn: (s: string) => string) => {
    const view = editorView.value
    if (!view) {
      return
    }
    const sel = view.state.selection.main
    let from = sel.from
    let to = sel.to
    if (sel.empty) {
      const line = view.state.doc.lineAt(sel.head)
      from = line.from
      to = line.to
    }
    const out = fn(view.state.doc.sliceString(from, to))
    view.dispatch({changes: {from, to, insert: out}, selection: {anchor: from, head: from + out.length}})
    view.focus()
  }

  // 选中行（无选区则全文）按整行的范围
  const lineBlockRange = (view: EditorView) => {
    const sel = view.state.selection.main
    const from = sel.empty ? 0 : view.state.doc.lineAt(sel.from).from
    const to = sel.empty ? view.state.doc.length : view.state.doc.lineAt(sel.to).to
    return {from, to}
  }

  // 通用：对按行范围内的文本做整体替换并保留选区
  const replaceLineBlock = (transform: (text: string) => string) => {
    const view = editorView.value
    if (!view) {
      return
    }
    const {from, to} = lineBlockRange(view)
    const out = transform(view.state.doc.sliceString(from, to))
    view.dispatch({changes: {from, to, insert: out}, selection: {anchor: from, head: from + out.length}})
    view.focus()
  }

  // 排序选中行；无选区时排序整篇
  const sortLines = (desc: boolean) => {
    replaceLineBlock((text) => {
      const lines = text.split('\n')
      lines.sort((a: string, b: string) => a.localeCompare(b))
      if (desc) {
        lines.reverse()
      }
      return lines.join('\n')
    })
  }

  // 删除重复行（保留首次出现）
  const removeDuplicateLines = () => {
    replaceLineBlock((text) => {
      const seen = new Set<string>()
      const out: string[] = []
      for (const l of text.split('\n')) {
        if (!seen.has(l)) {
          seen.add(l)
          out.push(l)
        }
      }
      return out.join('\n')
    })
  }

  // 去除行尾空白
  const trimTrailingWhitespace = () => {
    replaceLineBlock((text) => text.replace(/[ \t]+(\r?\n)/g, '$1').replace(/[ \t]+$/, ''))
  }

  return {transformSelectionOrLine, sortLines, removeDuplicateLines, trimTrailingWhitespace}
}
