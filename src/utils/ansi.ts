// 轻量 ANSI SGR → HTML（支持 16 色 + 加粗，自带 HTML 转义，规避 XSS）

const FG: Record<number, string> = {
  30: '#3b4048', 31: '#e06c75', 32: '#98c379', 33: '#e5c07b',
  34: '#61afef', 35: '#c678dd', 36: '#56b6c2', 37: '#dcdfe4',
  90: '#5c6370', 91: '#e06c75', 92: '#98c379', 93: '#e5c07b',
  94: '#61afef', 95: '#c678dd', 96: '#56b6c2', 97: '#ffffff'
}

const BG: Record<number, string> = {
  40: '#3b4048', 41: '#e06c75', 42: '#98c379', 43: '#e5c07b',
  44: '#61afef', 45: '#c678dd', 46: '#56b6c2', 47: '#dcdfe4',
  100: '#5c6370', 101: '#e06c75', 102: '#98c379', 103: '#e5c07b',
  104: '#61afef', 105: '#c678dd', 106: '#56b6c2', 107: '#ffffff'
}

const ESC = String.fromCharCode(27)
// 匹配 SGR 转义序列 ESC[...m（以 ESC 开头，避免误匹配普通文本中的 [..m）
const SGR = new RegExp(ESC + '\\[([0-9;]*)m', 'g')

const escapeHtml = (s: string) =>
    s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')

export function ansiToHtml(input: string): string {
  if (!input) {
    return ''
  }
  let fg: string | null = null
  let bg: string | null = null
  let bold = false
  let result = ''

  const flush = (text: string) => {
    if (!text) {
      return
    }
    const styles: string[] = []
    if (fg) styles.push(`color:${fg}`)
    if (bg) styles.push(`background:${bg}`)
    if (bold) styles.push('font-weight:600')
    result += styles.length
        ? `<span style="${styles.join(';')}">${escapeHtml(text)}</span>`
        : escapeHtml(text)
  }

  SGR.lastIndex = 0
  let last = 0
  let m: RegExpExecArray | null
  while ((m = SGR.exec(input)) !== null) {
    flush(input.slice(last, m.index))
    last = SGR.lastIndex
    const codes = m[1] === '' ? [0] : m[1].split(';').map(Number)
    for (const c of codes) {
      if (c === 0 || Number.isNaN(c)) {
        fg = null
        bg = null
        bold = false
      }
      else if (c === 1) bold = true
      else if (c === 22) bold = false
      else if (c === 39) fg = null
      else if (c === 49) bg = null
      else if (FG[c]) fg = FG[c]
      else if (BG[c]) bg = BG[c]
    }
  }
  flush(input.slice(last))
  return result
}
