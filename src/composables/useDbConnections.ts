import {ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {kvGet, kvGetJSON, kvSet, kvRemove} from './useKvStore'
import {i18n} from '../i18n'

export interface DataSource
{
    kind: 'memory' | 'sqlite' | 'mysql' | 'postgres' | 'clickhouse' | 'duckdb'
    file?: string
    host?: string
    port?: number
    user?: string
    password?: string
    database?: string
}

export interface DbConnection extends DataSource
{
    id: string
    name: string
}

const LEGACY_CONN_KEY = 'sql-connections' // 旧版：连接存于 KV 的一个 JSON 数组（已迁移到独立表）
const REF_KEY = 'sql-source-ref'

const genId = () => `db-${Date.now()}-${Math.random().toString(36).slice(2, 6)}`

// 模块级共享：连接列表（来自独立表 db_connections）+ 当前数据源引用（token）
const connections = ref<DbConnection[]>([])
const activeRef = ref<string>('memory')
let loadedConnections = false
let loadedRef = false

// 启动时调用（须在 loadKvStore 之后）：从独立表载入连接；首次发现旧 KV 数据则迁移过来
export const loadDbConnections = async () => {
    loadedConnections = true
    try {
        const list = await invoke<DbConnection[]>('db_connections_list')
        if (list.length === 0) {
            const legacy = kvGetJSON<DbConnection[]>(LEGACY_CONN_KEY, [])
            if (legacy.length) {
                for (const c of legacy) {
                    try {
                        await invoke('db_connection_save', {c})
                    }
                    catch (e) {
                        console.error('迁移连接失败:', e)
                    }
                }
                kvRemove(LEGACY_CONN_KEY)
                connections.value = legacy
                return
            }
        }
        connections.value = list
    }
    catch (e) {
        console.error('载入数据库连接失败:', e)
    }
}

export function useDbConnections()
{
    if (!loadedRef) {
        loadedRef = true
        activeRef.value = kvGet(REF_KEY) || 'memory'
    }
    // 兜底：若启动时未显式加载（如仅在某组件中首次用到），自动异步载入一次
    if (!loadedConnections) {
        loadDbConnections()
    }

    const add = (c: Omit<DbConnection, 'id'>) => {
        const conn = {...c, id: genId()} as DbConnection
        connections.value.push(conn)
        invoke('db_connection_save', {c: conn}).catch(e => console.error('保存连接失败:', e))
    }
    const update = (id: string, patch: Partial<DbConnection>) => {
        const i = connections.value.findIndex(x => x.id === id)
        if (i >= 0) {
            const merged = {...connections.value[i], ...patch}
            connections.value[i] = merged
            invoke('db_connection_save', {c: merged}).catch(e => console.error('保存连接失败:', e))
        }
    }
    const remove = (id: string) => {
        connections.value = connections.value.filter(x => x.id !== id)
        invoke('db_connection_delete', {id}).catch(e => console.error('删除连接失败:', e))
        if (activeRef.value === `conn:${id}`) {
            setActiveRef('memory')
        }
    }

    const setActiveRef = (token: string) => {
        activeRef.value = token
        kvSet(REF_KEY, token)
    }

    // 把当前引用解析为可执行的数据源
    const resolveActiveSource = (): DataSource => {
        const t = activeRef.value
        if (t.startsWith('conn:')) {
            const conn = connections.value.find(c => c.id === t.slice(5))
            if (conn) {
                const {id: _i, name: _n, ...rest} = conn
                return rest
            }
        }
        else if (t.startsWith('file:')) {
            return {kind: 'sqlite', file: t.slice(5)}
        }
        return {kind: 'memory'}
    }

    // 当前数据源展示名
    const activeLabel = (): string => {
        const t = activeRef.value
        if (t.startsWith('conn:')) {
            return connections.value.find(c => c.id === t.slice(5))?.name || i18n.global.t('sql.deleted')
        }
        if (t.startsWith('file:')) {
            const p = t.slice(5)
            return p.split(/[\\/]/).pop() || p
        }
        return i18n.global.t('sql.memoryDb')
    }

    return {connections, activeRef, add, update, remove, setActiveRef, resolveActiveSource, activeLabel}
}
