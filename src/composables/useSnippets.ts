import {ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'

export interface Snippet
{
    id: string
    prefix: string
    body: string
    description?: string
    // 适用语言；空或 '*' 表示所有语言
    language?: string
}

// 模块级共享，保证编辑器扩展与管理面板看到同一份数据
const snippets = ref<Snippet[]>([])
let loaded = false

const genId = () => `sn-${Date.now()}-${Math.random().toString(36).slice(2, 6)}`

const load = async () => {
    try {
        snippets.value = await invoke<Snippet[]>('get_snippets')
    }
    catch (error) {
        console.error('加载代码片段失败:', error)
        snippets.value = []
    }
}

// 应用启动时调用一次：从数据库载入
export const initSnippets = async () => {
    if (loaded) {
        return
    }
    loaded = true
    await load()
}

export function useSnippets()
{
    const add = async (s: Omit<Snippet, 'id'>) => {
        const snip: Snippet = {id: genId(), ...s}
        await invoke('save_snippet', {snippet: {description: '', language: '*', ...snip}})
        snippets.value.push(snip)
    }

    const update = async (id: string, patch: Partial<Snippet>) => {
        const i = snippets.value.findIndex(x => x.id === id)
        if (i < 0) {
            return
        }
        const merged = {...snippets.value[i], ...patch}
        await invoke('save_snippet', {snippet: {description: '', language: '*', ...merged}})
        snippets.value[i] = merged
    }

    const remove = async (id: string) => {
        await invoke('delete_snippet', {id})
        snippets.value = snippets.value.filter(x => x.id !== id)
    }

    // 查找某语言下前缀完全匹配的片段（同步，供编辑器 Tab 展开）
    const findByPrefix = (prefix: string, lang: string): Snippet | undefined =>
        snippets.value.find(s =>
            s.prefix === prefix && (!s.language || s.language === '*' || s.language === lang)
        )

    return {snippets, reload: load, add, update, remove, findByPrefix}
}
