// 语法高亮规则接口
export interface HighlightRule
{
    pattern: RegExp
    className: string
    priority?: number
}

// 匹配结果接口
export interface HighlightMatch
{
    start: number
    end: number
    className: string
    priority: number
}

// 语言高亮插件接口
export interface LanguageHighlighter
{
    /**
     * 获取语言名称
     */
    getLanguageName(): string

    /**
     * 获取显示名称
     */
    getDisplayName(): string

    /**
     * 获取高亮规则
     */
    getRules(): HighlightRule[]

    /**
     * 获取排序权重（数字越小越靠前）
     */
    getOrder(): number

    /**
     * 预处理代码（可选）
     */
    preProcess?(code: string): string

    /**
     * 后处理高亮结果（可选）
     */
    postProcess?(highlighted: string): string
}