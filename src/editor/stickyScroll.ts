import {EditorView, ViewPlugin, ViewUpdate} from '@codemirror/view'

// 粘性滚动（sticky scroll）：把当前视口顶部所在作用域的外层「头部行」固定在编辑器顶部。
// 采用语言无关的「缩进链」启发式：从顶部可见行向上，依次收集缩进严格更小的非空行，
// 即为其外层作用域的头部（函数/类/块的起始行）。适配缩进型与花括号型语言。

const MAX_ROWS = 5

function indentOf(text: string, tabSize: number): number {
    let n = 0
    for (const ch of text) {
        if (ch === ' ') {
            n++
        }
        else if (ch === '\t') {
            n += tabSize
        }
        else {
            break
        }
    }
    return n
}

export function stickyScroll(tabSize = 4) {
    return ViewPlugin.fromClass(
        class {
            dom: HTMLElement
            onScroll: () => void

            constructor(view: EditorView) {
                this.dom = document.createElement('div')
                this.dom.className = 'cm-sticky-scroll'
                this.dom.style.display = 'none'
                view.dom.appendChild(this.dom)
                // 视口变化不一定每次滚动都触发 update，故额外监听滚动
                this.onScroll = () => this.compute(view)
                view.scrollDOM.addEventListener('scroll', this.onScroll, {passive: true})
                this.compute(view)
            }

            update(u: ViewUpdate) {
                if (u.docChanged || u.viewportChanged || u.geometryChanged) {
                    this.compute(u.view)
                }
            }

            compute(view: EditorView) {
                const {state} = view
                const scrollTop = view.scrollDOM.scrollTop
                let topLineNo: number
                try {
                    const block = view.lineBlockAtHeight(scrollTop)
                    topLineNo = state.doc.lineAt(block.from).number
                }
                catch {
                    this.hide()
                    return
                }
                if (topLineNo <= 1) {
                    this.hide()
                    return
                }

                const rows: Array<{no: number; text: string}> = []
                let minIndent = indentOf(state.doc.line(topLineNo).text, tabSize)
                for (let n = topLineNo - 1; n >= 1; n--) {
                    const line = state.doc.line(n)
                    if (!line.text.trim()) {
                        continue
                    }
                    const ind = indentOf(line.text, tabSize)
                    if (ind < minIndent) {
                        rows.unshift({no: n, text: line.text})
                        minIndent = ind
                        if (ind === 0) {
                            break
                        }
                    }
                }

                // 仅保留最靠近的若干层
                const shown = rows.slice(-MAX_ROWS)
                if (!shown.length) {
                    this.hide()
                    return
                }

                // 定位到滚动区域顶部（避开上方的查找面板等）
                const scroller = view.scrollDOM
                this.dom.style.top = `${scroller.offsetTop}px`
                this.dom.style.left = `${scroller.offsetLeft}px`
                this.dom.style.width = `${scroller.clientWidth}px`
                this.dom.style.display = 'block'

                this.dom.textContent = ''
                for (const r of shown) {
                    const el = document.createElement('div')
                    el.className = 'cm-sticky-line'
                    el.textContent = r.text || ' '
                    el.addEventListener('click', () => {
                        const pos = state.doc.line(r.no).from
                        view.dispatch({
                            selection: {anchor: pos},
                            effects: EditorView.scrollIntoView(pos, {y: 'start'})
                        })
                        view.focus()
                    })
                    this.dom.appendChild(el)
                }
            }

            hide() {
                this.dom.style.display = 'none'
            }

            destroy() {
                this.dom.remove()
            }
        }
    )
}
