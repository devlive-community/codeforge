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
                const ext = normalizeExt(String(plugin.extension))
                if (!ext) {
                    continue
                }

                if (!l2e[plugin.language]) {
                    l2e[plugin.language] = ext
                }
                if (!e2l[ext]) {
                    e2l[ext] = []
                }
                if (!e2l[ext].includes(plugin.language)) {
                    e2l[ext].push(plugin.language)
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

    /**
     * 根据文件名/路径推断语言，匹配不到返回 null。
     * 同一扩展名对应多语言时，暂取第一个（后续可加优先级或让用户选）。
     */
    const detectLanguage = (filePathOrName: string): string | null => {
        const ext = filePathOrName.split('.').pop()
        if (!ext || ext === filePathOrName) {
            return null
        }
        const langs = extToLangs.value[normalizeExt(ext)]
        return langs && langs.length > 0 ? langs[0] : null
    }

    return {
        langToExt,
        extToLangs,
        build,
        getExtension,
        detectLanguage
    }
}
