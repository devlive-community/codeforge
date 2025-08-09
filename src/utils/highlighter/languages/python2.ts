import type { HighlightRule, LanguageHighlighter } from '../types'

export class Python2Highlighter
    implements LanguageHighlighter
{
    getLanguageName(): string
    {
        return 'python2'
    }

    getDisplayName(): string
    {
        return 'Python 2'
    }

    getOrder(): number
    {
        return 2
    }

    getRules(): HighlightRule[]
    {
        return [
            // 1. 注释 (最高优先级)
            {
                pattern: /#.*$/gm,
                className: 'text-gray-500 italic',
                priority: 1
            },

            // 2. 字符串 (高优先级)
            {
                pattern: /"""[\s\S]*?"""/g,
                className: 'text-green-600',
                priority: 2
            },
            {
                pattern: /'''[\s\S]*?'''/g,
                className: 'text-green-600',
                priority: 2
            },
            {
                pattern: /"(?:[^"\\]|\\.)*"/g,
                className: 'text-green-600',
                priority: 2
            },
            {
                pattern: /'(?:[^'\\]|\\.)*'/g,
                className: 'text-green-600',
                priority: 2
            },

            // 3. Python 2 关键字
            {
                pattern: /\b(def|class|if|elif|else|for|while|try|except|finally|with|as|import|from|return|yield|break|continue|pass|lambda|and|or|not|in|is|True|False|None|print|exec|raw_input)\b/g,
                className: 'text-purple-600 font-semibold',
                priority: 3
            },

            // 4. 数字
            {
                pattern: /\b\d+(?:\.\d+)?(?:[eE][+-]?\d+)?\b/g,
                className: 'text-blue-600',
                priority: 4
            },

            // 5. 函数调用
            {
                pattern: /\b([a-zA-Z_]\w*)(?=\s*\()/g,
                className: 'text-orange-600',
                priority: 5
            },

            // 6. 类名定义
            {
                pattern: /\bclass\s+([A-Z]\w*)/g,
                className: 'text-yellow-600 font-semibold',
                priority: 6
            }
        ]
    }

    preProcess(code: string): string
    {
        // Python 2 特定的预处理
        return code
    }

    postProcess(highlighted: string): string
    {
        // Python 2 特定的后处理
        return highlighted
    }
}