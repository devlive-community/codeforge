import type { HighlightRule, LanguageHighlighter } from '../types'

export class NodeJSHighlighter
    implements LanguageHighlighter
{
    getLanguageName(): string
    {
        return 'nodejs'
    }

    getDisplayName(): string
    {
        return 'Node.js'
    }

    getOrder(): number
    {
        return 3
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

            // 3. JavaScript/Node.js 关键字
            {
                pattern: /\b(const|let|var|function|class|if|else|for|while|do|switch|case|default|break|continue|return|try|catch|finally|throw|new|this|super|extends|import|export|from|as|async|await|yield|typeof|instanceof|in|of|delete|void|null|undefined|true|false)\b/g,
                className: 'text-purple-600 font-semibold',
                priority: 3
            },

            // 4. Node.js 特定关键字和全局对象
            {
                pattern: /\b(require|module|exports|process|global|__dirname|__filename|Buffer|console|setTimeout|setInterval|clearTimeout|clearInterval)\b/g,
                className: 'text-indigo-600 font-semibold',
                priority: 3
            },

            // 5. 数字
            {
                pattern: /\b\d+(?:\.\d+)?(?:[eE][+-]?\d+)?\b/g,
                className: 'text-blue-600',
                priority: 4
            },

            // 6. 函数调用
            {
                pattern: /\b([a-zA-Z_$]\w*)(?=\s*\()/g,
                className: 'text-orange-600',
                priority: 5
            },

            // 7. 类名定义
            {
                pattern: /\bclass\s+([A-Z]\w*)/g,
                className: 'text-yellow-600 font-semibold',
                priority: 6
            },

            // 8. 对象属性
            {
                pattern: /\b([a-zA-Z_$]\w*)\s*:/g,
                className: 'text-cyan-600',
                priority: 7
            },

            // 9. 箭头函数
            {
                pattern: /=>/g,
                className: 'text-pink-600 font-bold',
                priority: 8
            },

            // 10. 模板字符串插值
            {
                pattern: /\$\{[^}]*\}/g,
                className: 'text-red-600 bg-yellow-100',
                priority: 9
            },

            // 11. require 路径
            {
                pattern: /require\s*\(\s*(['"`])([^'"`]+)\1\s*\)/g,
                className: 'text-emerald-600',
                priority: 10
            },

            // 12. 常见的 Node.js 模块名
            {
                pattern: /\b(fs|path|http|https|url|crypto|util|events|stream|os|cluster|child_process|readline|querystring|zlib|assert|buffer|domain|punycode|string_decoder|tls|tty|dgram|dns|net|vm|repl)\b/g,
                className: 'text-teal-600 font-medium',
                priority: 11
            },

            // 13. 正则表达式
            {
                pattern: /\/(?:[^\/\\\r\n]|\\.)+\/[gimuy]*/g,
                className: 'text-rose-600',
                priority: 12
            },

            // 14. 装饰器 (如果使用 TypeScript)
            {
                pattern: /@[a-zA-Z_$]\w*/g,
                className: 'text-violet-600',
                priority: 13
            }
        ]
    }

    preProcess(code: string): string
    {
        // Node.js 特定的预处理
        // 可以在这里处理一些特殊的语法转换
        return code
    }

    postProcess(highlighted: string): string
    {
        // Node.js 特定的后处理
        // 可以在这里添加一些额外的样式处理
        return highlighted
    }
}
