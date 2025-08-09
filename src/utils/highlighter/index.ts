export * from './types'
export * from './manager'
export { Python2Highlighter } from './languages/python2'
export { Python3Highlighter } from './languages/python3'

// 导入并重新导出管理器实例
import { HighlightManager } from './manager'

// 创建单例实例
export const highlightManager = new HighlightManager()

/**
 * 高亮代码的便捷函数
 * @param code 源代码
 * @param language 语言名称
 * @returns 高亮后的 HTML
 */
export function highlightCode(code: string, language: string): string
{
    return highlightManager.highlight(code, language)
}

/**
 * 获取支持的语言列表
 */
export function getSupportedLanguages()
{
    return highlightManager.getSupportedLanguages()
}

/**
 * 注册自定义高亮器
 */
export function registerHighlighter(highlighter: import('./types').LanguageHighlighter)
{
    highlightManager.register(highlighter)
}