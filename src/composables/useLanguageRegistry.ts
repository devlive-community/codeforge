import {ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'

/**
 * 语言注册表：基于插件配置构建「扩展名 ↔ 语言」双向映射。
 * 用于打开文件时按扩展名自动识别语言，是文件树 / 多标签 / 按文件运行的基石。
 */
export function useLanguageRegistry()
{
    // language -> extension（如 python3 -> py）
    const langToExt = ref<Record<string, string>>({})
    // extension -> language[]（同一扩展名可能对应多个插件，如 js）
    const extToLangs = ref<Record<string, string[]>>({})

    const normalizeExt = (ext: string) => ext.replace(/^\./, '').toLowerCase()

    const build = async () => {
        try {
            const config = await invoke<any>('get_app_config')
            const plugins = [
                ...(config?.plugins || []),
                ...(config?.custom_plugins || [])
            ]

            const l2e: Record<string, string> = {}
            const e2l: Record<string, string[]> = {}

            for (const plugin of plugins) {
                if (!plugin?.language || !plugin?.extension) {
                    continue
                }
                // 禁用的语言不参与识别
                if (plugin.enabled === false) {
                    continue
                }
                // 支持一个插件声明多个扩展名（逗号/空格分隔，如 yaml 的 "yaml,yml"）
                const exts = String(plugin.extension)
                    .split(/[,\s]+/)
                    .map(normalizeExt)
                    .filter(Boolean)
                if (exts.length === 0) {
                    continue
                }

                // 语言 -> 扩展名取第一个作为主扩展名
                if (!l2e[plugin.language]) {
                    l2e[plugin.language] = exts[0]
                }
                for (const ext of exts) {
                    if (!e2l[ext]) {
                        e2l[ext] = []
                    }
                    if (!e2l[ext].includes(plugin.language)) {
                        e2l[ext].push(plugin.language)
                    }
                }
            }

            langToExt.value = l2e
            extToLangs.value = e2l
        }
        catch (error) {
            console.error('构建语言注册表失败:', error)
        }
    }

    const getExtension = (language: string) => langToExt.value[language] || ''

    // 返回某文件名/路径对应的全部候选语言（同扩展名可能多个引擎）
    const getCandidates = (filePathOrName: string): string[] => {
        const ext = filePathOrName.split('.').pop()
        if (!ext || ext === filePathOrName) {
            return []
        }
        return extToLangs.value[normalizeExt(ext)] || []
    }

    /**
     * 根据文件名/路径推断语言，匹配不到返回 null。
     * 同一扩展名对应多语言（如 .js 的 Browser/jQuery/Node.js）时：
     * - 若 preferred 已是候选之一，则保持 preferred（不切换）；
     * - 否则取第一个候选作为默认。
     */
    const detectLanguage = (filePathOrName: string, preferred?: string): string | null => {
        const candidates = getCandidates(filePathOrName)
        if (candidates.length === 0) {
            return null
        }
        if (preferred && candidates.includes(preferred)) {
            return preferred
        }
        return candidates[0]
    }

    return {
        langToExt,
        extToLangs,
        build,
        getExtension,
        getCandidates,
        detectLanguage
    }
}
