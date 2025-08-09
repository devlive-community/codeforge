import type { HighlightMatch, LanguageHighlighter } from './types'
import { Python2Highlighter } from './languages/python2'
import { Python3Highlighter } from './languages/python3'

export class HighlightManager
{
    private highlighters = new Map<string, LanguageHighlighter>()

    constructor()
    {
        this.registerDefaultHighlighters()
    }

    /**
     * 注册默认高亮器
     */
    private registerDefaultHighlighters(): void
    {
        this.register(new Python2Highlighter())
        this.register(new Python3Highlighter())
    }

    /**
     * 注册高亮器
     */
    register(highlighter: LanguageHighlighter): void
    {
        this.highlighters.set(highlighter.getLanguageName(), highlighter)
    }

    /**
     * 注销高亮器
     */
    unregister(languageName: string): boolean
    {
        return this.highlighters.delete(languageName)
    }

    /**
     * 获取高亮器
     */
    getHighlighter(languageName: string): LanguageHighlighter | undefined
    {
        return this.highlighters.get(languageName)
    }

    /**
     * 获取支持的语言列表
     */
    getSupportedLanguages(): Array<{ name: string; displayName: string; order: number }>
    {
        const languages = Array.from(this.highlighters.values())
                               .map(highlighter => ({
                                   name: highlighter.getLanguageName(),
                                   displayName: highlighter.getDisplayName(),
                                   order: highlighter.getOrder()
                               }))

        // 按 order 排序
        return languages.sort((a, b) => a.order - b.order)
    }

    /**
     * 检查是否支持某种语言
     */
    isSupported(languageName: string): boolean
    {
        return this.highlighters.has(languageName)
    }

    /**
     * 对代码进行语法高亮
     */
    highlight(code: string, languageName: string): string
    {
        if (!code) {
            return ''
        }

        const highlighter = this.getHighlighter(languageName)
        if (!highlighter) {
            // 如果不支持该语言，返回转义后的原始代码
            return this.escapeHtml(code)
        }

        // 预处理
        let processedCode = highlighter.preProcess ? highlighter.preProcess(code) : code

        // HTML 转义
        processedCode = this.escapeHtml(processedCode)

        // 应用高亮规则
        const highlighted = this.applyHighlightRules(processedCode, highlighter.getRules())

        // 后处理
        return highlighter.postProcess ? highlighter.postProcess(highlighted) : highlighted
    }

    /**
     * 应用高亮规则
     */
    private applyHighlightRules(code: string, rules: Array<{ pattern: RegExp; className: string; priority?: number }>): string
    {
        // 收集所有匹配
        const matches: HighlightMatch[] = []

        for (const rule of rules) {
            const regex = new RegExp(rule.pattern.source, rule.pattern.flags)
            let match

            while ((match = regex.exec(code)) !== null) {
                matches.push({
                    start: match.index,
                    end: match.index + match[0].length,
                    className: rule.className,
                    priority: rule.priority || 999
                })

                // 防止无限循环
                if (!rule.pattern.global) {
                    break
                }
            }
        }

        // 按优先级排序，优先级高的（数字小的）在前
        matches.sort((a, b) => a.priority - b.priority || a.start - b.start)

        // 移除重叠的匹配（保留高优先级的）
        const filteredMatches = this.removeOverlappingMatches(matches)

        // 按开始位置倒序排序，从后往前插入标签
        filteredMatches.sort((a, b) => b.start - a.start)

        // 插入高亮标签
        let highlighted = code
        for (const match of filteredMatches) {
            const before = highlighted.substring(0, match.start)
            const content = highlighted.substring(match.start, match.end)
            const after = highlighted.substring(match.end)

            highlighted = before + `<span class="${ match.className }">${ content }</span>` + after
        }

        return highlighted
    }

    /**
     * 移除重叠的匹配
     */
    private removeOverlappingMatches(matches: HighlightMatch[]): HighlightMatch[]
    {
        const filteredMatches: HighlightMatch[] = []

        for (const match of matches) {
            const hasOverlap = filteredMatches.some(existing =>
                (match.start < existing.end && match.end > existing.start)
            )

            if (!hasOverlap) {
                filteredMatches.push(match)
            }
        }

        return filteredMatches
    }

    /**
     * HTML 转义
     */
    private escapeHtml(text: string): string
    {
        return text
        // .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        // .replace(/"/g, '&quot;')
        // .replace(/'/g, '&#39;')
    }
}
