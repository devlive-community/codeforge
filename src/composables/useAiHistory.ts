import {ref} from 'vue'

export interface AiMsg
{
    role: 'user' | 'assistant'
    content: string
}

export interface AiConversation
{
    id: string
    title: string
    updatedAt: number
    messages: AiMsg[]
}

const STORAGE_KEY = 'ai-conversations'
const MAX = 50

const load = (): AiConversation[] => {
    try {
        return JSON.parse(localStorage.getItem(STORAGE_KEY) || '[]')
    }
    catch {
        return []
    }
}

const persist = (list: AiConversation[]) => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(list.slice(0, MAX)))
}

export function useAiHistory()
{
    const conversations = ref<AiConversation[]>(load())

    const reload = () => {
        conversations.value = load()
    }

    // 保存/更新一条会话（置顶）
    const saveConversation = (conv: AiConversation) => {
        const list = conversations.value.filter(c => c.id !== conv.id)
        list.unshift(conv)
        conversations.value = list.slice(0, MAX)
        persist(conversations.value)
    }

    const remove = (id: string) => {
        conversations.value = conversations.value.filter(c => c.id !== id)
        persist(conversations.value)
    }

    return {conversations, reload, saveConversation, remove}
}
