// 自绘 LSP 悬浮提示：完全不依赖 CodeMirror 的 tooltip 定位，
// 监听鼠标 → posAtCoords 定位 → 取诊断 + textDocument/hover → 在鼠标处用挂到 body 的 fixed 浮层显示。
import {EditorView, ViewPlugin} from '@codemirror/view'
import {forEachDiagnostic} from '@codemirror/lint'
import {languageServerPlugin} from 'codemirror-languageserver'
import {useTheme} from '../composables/useTheme'

const offsetToPos = (doc: any, offset: number) => {
  const line = doc.lineAt(offset)
  return {line: line.number - 1, character: offset - line.from}
}

const SEVERITY_COLOR: Record<string, string> = {
  error: '#ef4444',
  warning: '#f59e0b',
  info: '#3b82f6',
  hint: '#6b7280'
}

function makeHoverPlugin() {
  return ViewPlugin.fromClass(class {
    view: EditorView
    box: HTMLElement
    timer: any = null
    reqId = 0
    onMove: (e: MouseEvent) => void
    onLeave: () => void
    onScroll: () => void

    constructor(view: EditorView) {
      this.view = view
      this.box = document.createElement('div')
      this.box.className = 'cf-lsp-hover'
      this.box.style.cssText = 'position:fixed;z-index:9999;display:none;max-width:520px;max-height:340px;overflow:auto;border-radius:8px;padding:8px 10px;font-size:12px;line-height:1.5;pointer-events:none;'
      document.body.appendChild(this.box)

      this.onMove = (e: MouseEvent) => {
        if (this.timer) {
          clearTimeout(this.timer)
        }
        this.timer = setTimeout(() => this.show(e.clientX, e.clientY), 280)
      }
      this.onLeave = () => {
        if (this.timer) {
          clearTimeout(this.timer)
        }
        this.hide()
      }
      this.onScroll = () => this.hide()

      view.dom.addEventListener('mousemove', this.onMove)
      view.dom.addEventListener('mouseleave', this.onLeave)
      view.scrollDOM.addEventListener('scroll', this.onScroll, true)
    }

    applyTheme() {
      const {isDark} = useTheme()
      const dark = isDark.value
      this.box.style.background = dark ? '#1f2937' : '#ffffff'
      this.box.style.color = dark ? '#e5e7eb' : '#1f2937'
      this.box.style.border = `1px solid ${dark ? '#374151' : '#e5e7eb'}`
      this.box.style.boxShadow = dark ? '0 8px 28px rgba(0,0,0,0.5)' : '0 8px 28px rgba(0,0,0,0.14)'
    }

    async show(x: number, y: number) {
      const pos = this.view.posAtCoords({x, y})
      if (pos == null) {
        this.hide()
        return
      }
      const myReq = ++this.reqId
      const parts: string[] = []

      // 1) 该位置的诊断
      forEachDiagnostic(this.view.state, (d, from, to) => {
        if (pos >= from && pos <= to) {
          const color = SEVERITY_COLOR[d.severity] || SEVERITY_COLOR.info
          parts.push(`<div style="border-left:3px solid ${color};padding-left:8px;margin:2px 0;white-space:pre-wrap">${escapeHtml(d.message)}</div>`)
        }
      })

      // 2) LSP textDocument/hover
      try {
        const plugin: any = this.view.plugin(languageServerPlugin as any)
        if (plugin?.requestHoverTooltip) {
          const spec = await plugin.requestHoverTooltip(this.view, offsetToPos(this.view.state.doc, pos))
          if (myReq !== this.reqId) {
            return // 过期
          }
          if (spec && typeof spec.create === 'function') {
            const built = spec.create(this.view)
            const dom: HTMLElement | undefined = built?.dom
            if (dom && dom.textContent && dom.textContent.trim()) {
              parts.push(`<div class="cf-lsp-hover-doc">${dom.innerHTML}</div>`)
            }
          }
        }
      }
      catch {
        // 忽略
      }

      if (myReq !== this.reqId) {
        return
      }
      if (parts.length === 0) {
        this.hide()
        return
      }

      this.applyTheme()
      this.box.innerHTML = parts.join('<div style="height:1px;background:rgba(128,128,128,0.2);margin:6px -10px"></div>')
      this.box.style.display = 'block'
      // 定位到鼠标下方，越界则翻转/夹紧
      const rect = this.box.getBoundingClientRect()
      let left = x + 2
      let top = y + 18
      if (left + rect.width > window.innerWidth - 8) {
        left = window.innerWidth - rect.width - 8
      }
      if (top + rect.height > window.innerHeight - 8) {
        top = y - rect.height - 12
      }
      this.box.style.left = Math.max(8, left) + 'px'
      this.box.style.top = Math.max(8, top) + 'px'
    }

    hide() {
      this.box.style.display = 'none'
    }

    destroy() {
      if (this.timer) {
        clearTimeout(this.timer)
      }
      this.view.dom.removeEventListener('mousemove', this.onMove)
      this.view.dom.removeEventListener('mouseleave', this.onLeave)
      this.view.scrollDOM.removeEventListener('scroll', this.onScroll, true)
      this.box.remove()
    }
  })
}

function escapeHtml(s: string): string {
  return s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')
}

// 自绘 hover + 隐藏 CodeMirror 默认的 hover / 诊断 tooltip（避免重复且定位错乱）
export const lspCustomHover = [
  makeHoverPlugin(),
  EditorView.theme({
    '.cm-tooltip.cm-tooltip-hover': {display: 'none !important'},
    '.cm-tooltip.cm-tooltip-lint': {display: 'none !important'},
    '.cf-lsp-hover-doc pre': {whiteSpace: 'pre-wrap', margin: '4px 0'},
    '.cf-lsp-hover-doc code': {fontFamily: 'monospace'}
  })
]
