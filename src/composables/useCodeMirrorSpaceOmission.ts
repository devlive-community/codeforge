import {EditorView, ViewPlugin, ViewUpdate} from '@codemirror/view'

export function useCodeMirrorSpaceOmission()
{
    const spaceOmissionPlugin = ViewPlugin.fromClass(class
    {
        private styleElement: HTMLStyleElement | null = null

        constructor(private view: EditorView)
        {
            this.updateCSS()
        }

        update(update: ViewUpdate)
        {
            if (update.selectionSet) {
                this.updateCSS()
            }
        }

        updateCSS()
        {
            const selection = this.view.state.selection.main

            // 移除之前的样式
            if (this.styleElement) {
                this.styleElement.remove()
                this.styleElement = null
            }

            if (selection.empty) {
                return
            }

            const doc = this.view.state.doc
            const fromLine = doc.lineAt(selection.from)
            const toLine = doc.lineAt(selection.to)

            const cssRules: string[] = []

            // 为每个有缩进的选中行生成CSS规则
            for (let lineNum = fromLine.number; lineNum <= toLine.number; lineNum++) {
                const line = doc.line(lineNum)
                const text = line.text

                // 计算空格数
                let spaceCount = 0
                for (let i = 0; i < text.length; i++) {
                    if (text[i] === ' ') {
                        spaceCount++
                    }
                    else {
                        break
                    }
                }

                if (spaceCount > 0) {
                    // 使用行号作为选择器，添加伪元素
                    const arrowText = '·'.repeat(spaceCount)
                    cssRules.push(`
                        .cm-editor .cm-line:nth-child(${lineNum}) {
                            position: relative;
                        }
                        .cm-editor .cm-line:nth-child(${lineNum})::before {
                            content: "${arrowText}";
                            position: absolute;
                            left: 0.4em;
                            top: 0;
                            color: #9ca3af;
                            font-weight: 500;
                            pointer-events: none;
                            z-index: 2;
                            font-family: monospace;
                            font-size: inherit;
                            line-height: inherit;
                        }
                        .cm-editor .cm-selectionBackground .cm-line:nth-child(${lineNum})::before,
                        .cm-editor .cm-line:nth-child(${lineNum}).cm-selectionBackground::before {
                            background-color: var(--cm-selectionBackground, #b3d4fc);
                        }
                    `)
                }
            }

            // 插入动态样式
            if (cssRules.length > 0) {
                this.styleElement = document.createElement('style')
                this.styleElement.textContent = cssRules.join('\n')
                document.head.appendChild(this.styleElement)
            }
        }

        destroy()
        {
            if (this.styleElement) {
                this.styleElement.remove()
            }
        }
    })

    const spaceOmissionTheme = EditorView.theme({
        '.cm-line': {
            position: 'relative'
        }
    })

    return {
        spaceOmissionPlugin,
        spaceOmissionTheme
    }
}
