import {computed, reactive} from 'vue'
import {kvGetJSON, kvSetJSON} from './useKvStore'

export interface AiProviderConfig
{
    apiKey: string
    baseUrl: string
    model: string
}

export interface AiProviderMeta
{
    value: string
    label: string
    defaultModel: string
    defaultBase: string
}

export const AI_PROVIDERS: AiProviderMeta[] = [
    {value: 'anthropic', label: 'Claude (Anthropic)', defaultModel: 'claude-3-5-sonnet-latest', defaultBase: 'https://api.anthropic.com'},
    {value: 'openai', label: 'OpenAI', defaultModel: 'gpt-4o-mini', defaultBase: 'https://api.openai.com/v1'},
    {value: 'deepseek', label: 'DeepSeek', defaultModel: 'deepseek-chat', defaultBase: 'https://api.deepseek.com'}
]

interface AiConfig
{
    provider: string
    providers: Record<string, AiProviderConfig>
}

const STORAGE_KEY = 'ai-config'

const buildDefault = (): AiConfig => {
    const providers: Record<string, AiProviderConfig> = {}
    for (const p of AI_PROVIDERS) {
        providers[p.value] = {apiKey: '', baseUrl: '', model: p.defaultModel}
    }
    return {provider: 'anthropic', providers}
}

const loadRaw = (): AiConfig => {
    const base = buildDefault()
    const saved = kvGetJSON<any>(STORAGE_KEY, null)
    if (saved && saved.providers) {
        base.provider = saved.provider || base.provider
        for (const key of Object.keys(base.providers)) {
            if (saved.providers[key]) {
                base.providers[key] = {...base.providers[key], ...saved.providers[key]}
            }
        }
    }
    return base
}

export function useAiConfig()
{
    const state = reactive<AiConfig>(loadRaw())

    const save = () => {
        kvSetJSON(STORAGE_KEY, state)
    }

    const reload = () => {
        Object.assign(state, loadRaw())
    }

    // 当前生效的 provider 配置（含 provider 名与默认 base）
    const active = computed(() => {
        const meta = AI_PROVIDERS.find(p => p.value === state.provider) || AI_PROVIDERS[0]
        const cfg = state.providers[state.provider]
        return {
            provider: state.provider,
            apiKey: cfg.apiKey,
            baseUrl: cfg.baseUrl || meta.defaultBase,
            model: cfg.model || meta.defaultModel
        }
    })

    return {state, providers: AI_PROVIDERS, save, reload, active}
}
