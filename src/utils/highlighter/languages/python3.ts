import type { HighlightRule, LanguageHighlighter } from '../types'

export class Python3Highlighter
    implements LanguageHighlighter
{
    getLanguageName(): string
    {
        return 'python3'
    }

    getDisplayName(): string
    {
        return 'Python 3'
    }

    getOrder(): number
    {
        return 1
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
                pattern: /f"(?:[^"\\]|\\.)*"/g,
                className: 'text-emerald-600',
                priority: 2
            }, // f-strings
            {
                pattern: /f'(?:[^'\\]|\\.)*'/g,
                className: 'text-emerald-600',
                priority: 2
            }, // f-strings
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

            // 3. Python 3 关键字
            {
                pattern: /\b(def|class|if|elif|else|for|while|try|except|finally|with|as|import|from|return|yield|break|continue|pass|lambda|and|or|not|in|is|True|False|None|async|await|nonlocal)\b/g,
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

            // 6. 装饰器
            {
                pattern: /@[a-zA-Z_]\w*/g,
                className: 'text-yellow-600',
                priority: 6
            },

            // 7. 类名定义
            {
                pattern: /\bclass\s+([A-Z]\w*)/g,
                className: 'text-yellow-600 font-semibold',
                priority: 7
            }
        ]
    }

    preProcess(code: string): string
    {
        // Python 3 特定的预处理
        return code
    }

    postProcess(highlighted: string): string
    {
        // Python 3 特定的后处理
        return highlighted
    }
}