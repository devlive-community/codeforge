import {ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'

export interface AiMsg
{
    role: 'user' | 'assistant'
    content: string
}

// 列表项（不含完整消息，按需再取）
export interface AiConversationMeta
{
    id: string
    title: string
    updated_at: number
}

export interface AiConversation
{
    id: string
    title: string
    updatedAt: number
    messages: AiMsg[]
}

/**
 * AI 对话历史，存于与执行历史相同的 SQLite 库（后端命令）。
 */
export function useAiHistory()
{
    const conversations = ref<AiConversationMeta[]>([])

    const reload = async () => {
        try {
            conversations.value = await invoke<AiConversationMeta[]>('list_ai_conversations')
        }
        catch (error) {
            console.error('读取 AI 对话历史失败:', error)
        }
    }

    const saveConversation = async (conv: AiConversation) => {
        await invoke('save_ai_conversation', {
            id: conv.id,
            title: conv.title,
            messages: JSON.stringify(conv.messages),
            updatedAt: conv.updatedAt
        })
        await reload()
    }

    const getMessages = async (id: string): Promise<AiMsg[]> => {
        const json = await invoke<string>('get_ai_conversation', {id})
        try {
            return JSON.parse(json)
        }
        catch {
            return []
        }
    }

    const remove = async (id: string) => {
        await invoke('delete_ai_conversation', {id})
        await reload()
    }

    return {conversations, reload, saveConversation, getMessages, remove}
}
