// 断点 gutter + 当前执行行高亮。断点数据由外部(App)按当前文件 dispatch 填充。
import {Decoration, EditorView, GutterMarker, gutter} from '@codemirror/view'
import {StateEffect, StateField} from '@codemirror/state'

export interface BreakpointData {
  lines: number[]      // 断点行号(1-based)
  exec: number | null  // 当前执行行(1-based)，无则 null
}

// 由外部派发：设置当前文件的断点与执行行
export const setBreakpointData = StateEffect.define<BreakpointData>()

interface BpState {
  lines: Set<number>
  exec: number | null
}

const bpField = StateField.define<BpState>({
  create: () => ({lines: new Set(), exec: null}),
  update(val, tr) {
    for (const e of tr.effects) {
      if (e.is(setBreakpointData)) {
        return {lines: new Set(e.value.lines), exec: e.value.exec}
      }
    }
    return val
  }
})

class BreakpointMarker extends GutterMarker {
  toDOM() {
    const span = document.createElement('span')
    span.className = 'cm-bp-dot'
    span.textContent = '●'
    return span
  }
}
const bpMarker = new BreakpointMarker()

// 当前执行行整行高亮
const execLineDeco = EditorView.decorations.compute([bpField], (state) => {
  const f = state.field(bpField)
  if (!f.exec || f.exec < 1 || f.exec > state.doc.lines) {
    return Decoration.none
  }
  const line = state.doc.line(f.exec)
  return Decoration.set([Decoration.line({class: 'cm-debug-exec-line'}).range(line.from)])
})

const bpTheme = EditorView.baseTheme({
  '.cm-breakpoint-gutter': {width: '14px', cursor: 'pointer'},
  '.cm-breakpoint-gutter .cm-gutterElement': {paddingLeft: '2px'},
  '.cm-bp-dot': {color: '#e51400', fontSize: '12px', lineHeight: '1'},
  '.cm-debug-exec-line': {backgroundColor: 'rgba(250, 204, 21, 0.18)'}
})

// 断点 gutter 扩展。onToggle 在点击 gutter 时回调行号(1-based)。
export function breakpointExtension(onToggle: (line: number) => void) {
  const bpGutter = gutter({
    class: 'cm-breakpoint-gutter',
    lineMarker(view, block) {
      const f = view.state.field(bpField, false)
      if (!f) {
        return null
      }
      const lineNo = view.state.doc.lineAt(block.from).number
      return f.lines.has(lineNo) ? bpMarker : null
    },
    // 断点数据变化时强制重算 gutter 标记
    lineMarkerChange: (update) =>
      update.transactions.some(tr => tr.effects.some(e => e.is(setBreakpointData))),
    domEventHandlers: {
      mousedown(view, block) {
        const lineNo = view.state.doc.lineAt(block.from).number
        onToggle(lineNo)
        return true
      }
    }
  })
  return [bpField, bpGutter, execLineDeco, bpTheme]
}
