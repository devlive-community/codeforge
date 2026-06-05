import {invoke} from '@tauri-apps/api/core'

/**
 * 通用键值存储，替代 localStorage，持久化到 SQLite。
 * 启动时一次性把全部键值载入内存缓存，使读取保持同步；写入更新缓存并异步落库。
 */
const cache = new Map<string, string>()
let loaded = false

// 在应用挂载前调用一次，载入全部键值
export const loadKvStore = async () => {
    if (loaded) {
        return
    }
    try {
        const all = await invoke<Record<string, string>>('kv_get_all')
        for (const [k, v] of Object.entries(all)) {
            cache.set(k, v)
        }
    }
    catch (error) {
        console.error('载入键值存储失败:', error)
    }
    loaded = true
}

// 同步读取（缓存）；不存在返回 null
export const kvGet = (key: string): string | null => {
    return cache.has(key) ? cache.get(key)! : null
}

// 读取并 JSON 解析，失败/不存在返回默认值
export const kvGetJSON = <T>(key: string, fallback: T): T => {
    const raw = kvGet(key)
    if (raw == null) {
        return fallback
    }
    try {
        return JSON.parse(raw) as T
    }
    catch {
        return fallback
    }
}

// 写入（更新缓存 + 异步落库）
export const kvSet = (key: string, value: string) => {
    cache.set(key, value)
    invoke('kv_set', {key, value}).catch(e => console.error('保存键值失败:', e))
}

// 写入对象（JSON 序列化）
export const kvSetJSON = (key: string, value: unknown) => {
    kvSet(key, JSON.stringify(value))
}

// 删除
export const kvRemove = (key: string) => {
    cache.delete(key)
    invoke('kv_delete', {key}).catch(e => console.error('删除键值失败:', e))
}
