import {EditorView} from '@codemirror/view'

export function useCodeMirrorFontFamily(fontFamily?: string)
{
    const defaultFont = "monospace"

    const fontFamilyTheme = EditorView.theme({
        '&': {
            fontFamily: fontFamily || defaultFont
        },
        '.cm-content': {
            fontFamily: fontFamily || defaultFont
        },
        '.cm-scroller': {
            fontFamily: fontFamily || defaultFont
        }
    })

    return {
        fontFamilyTheme
    }
}
