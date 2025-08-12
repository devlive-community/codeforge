import type { HighlightRule, LanguageHighlighter } from '../types'

export class GoHighlighter
    implements LanguageHighlighter
{
    getLanguageName(): string
    {
        return 'go'
    }

    getDisplayName(): string
    {
        return 'Go'
    }

    getOrder(): number
    {
        return 4
    }

    getRules(): HighlightRule[]
    {
        return [
            // 1. 注释 (最高优先级)
            {
                pattern: /\/\/.*$/gm,
                className: 'text-gray-500 italic',
                priority: 1
            },
            {
                pattern: /\/\*[\s\S]*?\*\//g,
                className: 'text-gray-500 italic',
                priority: 1
            },

            // 2. 字符串 (高优先级)
            {
                pattern: /`(?:[^`\\]|\\.)*`/g,
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

            // 3. Go 关键字
            {
                pattern: /\b(package|import|func|var|const|type|struct|interface|map|chan|select|go|defer|return|break|continue|fallthrough|if|else|switch|case|default|for|range|goto|true|false|nil|iota)\b/g,
                className: 'text-purple-600 font-semibold',
                priority: 3
            },

            // 4. Go 内置类型
            {
                pattern: /\b(bool|byte|complex64|complex128|error|float32|float64|int|int8|int16|int32|int64|rune|string|uint|uint8|uint16|uint32|uint64|uintptr)\b/g,
                className: 'text-blue-600 font-semibold',
                priority: 4
            },

            // 5. 数字
            {
                pattern: /\b\d+(?:\.\d+)?(?:[eE][+-]?\d+)?[fFlL]?\b/g,
                className: 'text-blue-600',
                priority: 5
            },

            // 6. 十六进制、八进制、二进制数字
            {
                pattern: /\b0[xX][0-9a-fA-F]+\b|\b0[0-7]+\b|\b0[bB][01]+\b/g,
                className: 'text-blue-600',
                priority: 5
            },

            // 7. 函数定义
            {
                pattern: /\bfunc\s+([a-zA-Z_]\w*)/g,
                className: 'text-orange-600 font-semibold',
                priority: 6
            },

            // 8. 函数调用
            {
                pattern: /\b([a-zA-Z_]\w*)(?=\s*\()/g,
                className: 'text-orange-600',
                priority: 7
            },

            // 9. 类型定义
            {
                pattern: /\btype\s+([A-Z]\w*)/g,
                className: 'text-yellow-600 font-semibold',
                priority: 8
            },

            // 10. 结构体字段
            {
                pattern: /\b([a-zA-Z_]\w*)\s*:/g,
                className: 'text-cyan-600',
                priority: 9
            },

            // 11. 包名
            {
                pattern: /\bpackage\s+([a-zA-Z_]\w*)/g,
                className: 'text-indigo-600 font-semibold',
                priority: 10
            },

            // 12. import 语句
            {
                pattern: /\bimport\s*\(\s*[\s\S]*?\)/g,
                className: 'text-emerald-600',
                priority: 11
            },
            {
                pattern: /\bimport\s+"[^"]+"/g,
                className: 'text-emerald-600',
                priority: 11
            },

            // 13. 常见的 Go 内置函数
            {
                pattern: /\b(make|new|len|cap|append|copy|delete|close|panic|recover|print|println)\b/g,
                className: 'text-pink-600 font-medium',
                priority: 12
            },

            // 14. 指针操作符
            {
                pattern: /[*&]/g,
                className: 'text-red-600 font-bold',
                priority: 13
            },

            // 15. 通道操作符
            {
                pattern: /<-/g,
                className: 'text-violet-600 font-bold',
                priority: 14
            },

            // 16. 结构体标签
            {
                pattern: /`[^`]*`/g,
                className: 'text-teal-600 bg-gray-100',
                priority: 15
            },

            // 17. 大写开头的导出标识符
            {
                pattern: /\b[A-Z]\w*/g,
                className: 'text-amber-600',
                priority: 16
            }
        ]
    }

    preProcess(code: string): string
    {
        // Go 特定的预处理
        return code
    }

    postProcess(highlighted: string): string
    {
        // Go 特定的后处理
        return highlighted
    }
}
