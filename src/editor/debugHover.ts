// 调试悬停求值：会话停驻时，悬停标识符即在当前帧上下文求值并显示其值。
import {EditorView, hoverTooltip} from '@codemirror/view'
import {useDebug} from '../composables/useDebug'

const debugHoverTooltip = hoverTooltip(async (view, pos) => {
  const debug = useDebug()
  if (debug.status.value !== 'stopped') {
    return null
  }
  const line = view.state.doc.lineAt(pos)
  const rel = pos - line.from
  // 取光标处的标识符（含成员访问 a.b.c）
  const re = /[\w$.]+/g
  let word = ''
  let start = -1
  let end = -1
  let m: RegExpExecArray | null
  while ((m = re.exec(line.text))) {
    if (m.index <= rel && rel <= m.index + m[0].length) {
      word = m[0]
      start = line.from + m.index
      end = line.from + m.index + m[0].length
      break
    }
  }
  if (!word) {
    return null
  }
  try {
    const {result} = await debug.evaluate(word, 'hover')
    if (!result) {
      return null
    }
    return {
      pos: start,
      end,
      above: true,
      create: () => {
        const dom = document.createElement('div')
        dom.className = 'cm-debug-hover'
        dom.textContent = `${word} = ${result}`
        return {dom}
      }
    }
  }
  catch {
    return null
  }
})

const debugHoverTheme = EditorView.baseTheme({
  '.cm-debug-hover': {
    padding: '4px 8px',
    fontFamily: 'monospace',
    fontSize: '12px',
    whiteSpace: 'pre-wrap',
    maxWidth: '420px'
  }
})

export const debugHover = [debugHoverTooltip, debugHoverTheme]
