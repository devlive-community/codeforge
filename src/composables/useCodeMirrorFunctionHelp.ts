import {EditorView, hoverTooltip} from '@codemirror/view'

export function useCodeMirrorFunctionHelp()
{
    // 提示框
    const createHelpTooltip = (text: string) => {
        const dom = document.createElement('div')
        dom.className = 'cm-custom-function-tooltip'

        dom.innerHTML = `
            <div class="relative mb-1">
                <div class="px-2 py-1 bg-gray-800 text-white text-xs rounded">
                    <span class="font-mono">${text}</span>
                </div>
                <div class="absolute left-1/2 transform -translate-x-1/2 top-full">
                    <div class="w-0 h-0 border-l-4 border-r-4 border-t-4 border-transparent border-t-gray-800"></div>
                </div>
            </div>
        `

        return {dom}
    }

    // 显示函数提示扩展
    const showFunctionHelpHover = hoverTooltip((view, pos, side) => {
        let {from, to, text} = view.state.doc.lineAt(pos)
        let start = pos, end = pos
        while (start > from && /\w/.test(text[start - from - 1])) {
            start--
        }
        while (end < to && /\w/.test(text[end - from])) {
            end++
        }
        if (start == pos && side < 0 || end == pos && side > 0) {
            return null
        }
        return {
            pos: start,
            end,
            above: true,
            create(_view)
            {
                return createHelpTooltip(text.slice(start - from, end - from))
            }
        }
    }, {hoverTime: 300})

    // 提示框样式主题
    const functionHelpTheme = EditorView.theme({
        '.cm-custom-function-tooltip': {
            zIndex: '100',
            animation: 'fadeIn 0.2s ease-out'
        },
        '.cm-tooltip.cm-tooltip-hover': {
            backgroundColor: 'transparent !important',
            border: 'none !important',
            boxShadow: 'none !important'
        },
        '@keyframes fadeIn': {
            'from': {opacity: '0', transform: 'translateY(-2px)'},
            'to': {opacity: '1', transform: 'translateY(0)'}
        }
    })

    return {
        showFunctionHelpHover,
        functionHelpTheme
    }
}
