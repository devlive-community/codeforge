import {invoke} from '@tauri-apps/api/core'

export interface AiMsg
{
    role: 'user' | 'assistant'
    content: string
}

/**
 * AI 对话与某次执行记录（executionId）绑定，存于与执行历史同一个 SQLite 库。
 * 未关联执行的对话属临时会话，不保存。
 */
export function useAiHistory()
{
    const saveConversation = async (executionId: number, messages: AiMsg[]) => {
        await invoke('save_ai_conversation', {
            executionId,
            messages: JSON.stringify(messages),
            updatedAt: Date.now()
        })
    }

    const getMessages = async (executionId: number): Promise<AiMsg[]> => {
        const json = await invoke<string>('get_ai_conversation', {executionId})
        if (!json) {
            return []
        }
        try {
            return JSON.parse(json)
        }
        catch {
            return []
        }
    }

    const deleteConversation = async (executionId: number) => {
        await invoke('delete_ai_conversation', {executionId})
    }

    return {saveConversation, getMessages, deleteConversation}
}
