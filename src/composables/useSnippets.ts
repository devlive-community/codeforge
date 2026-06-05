import {ref} from 'vue'

export interface Snippet
{
    id: string
    prefix: string
    body: string
    description?: string
    // 适用语言；空或 '*' 表示所有语言
    language?: string
}

const STORAGE_KEY = 'snippets'

const load = (): Snippet[] => {
    try {
        const arr = JSON.parse(localStorage.getItem(STORAGE_KEY) || '[]')
        return Array.isArray(arr) ? arr : []
    }
    catch {
        return []
    }
}

// 模块级共享，保证编辑器扩展与管理面板看到同一份数据
const snippets = ref<Snippet[]>(load())

export function useSnippets()
{
    const persist = () => {
        localStorage.setItem(STORAGE_KEY, JSON.stringify(snippets.value))
    }
    const reload = () => {
        snippets.value = load()
    }

    const add = (s: Omit<Snippet, 'id'>) => {
        snippets.value.push({...s, id: `sn-${Date.now()}-${Math.random().toString(36).slice(2, 6)}`})
        persist()
    }
    const update = (id: string, patch: Partial<Snippet>) => {
        const i = snippets.value.findIndex(x => x.id === id)
        if (i >= 0) {
            snippets.value[i] = {...snippets.value[i], ...patch}
            persist()
        }
    }
    const remove = (id: string) => {
        snippets.value = snippets.value.filter(x => x.id !== id)
        persist()
    }

    // 查找某语言下前缀完全匹配的片段（精确前缀 + 语言匹配）
    const findByPrefix = (prefix: string, lang: string): Snippet | undefined =>
        snippets.value.find(s =>
            s.prefix === prefix && (!s.language || s.language === '*' || s.language === lang)
        )

    return {snippets, reload, persist, add, update, remove, findByPrefix}
}
