import {ref} from 'vue'
import {kvGet, kvGetJSON, kvSet, kvSetJSON} from './useKvStore'

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

const CONN_KEY = 'sql-connections'
const REF_KEY = 'sql-source-ref'

const genId = () => `db-${Date.now()}-${Math.random().toString(36).slice(2, 6)}`

// 模块级共享：连接列表 + 当前数据源引用（token：memory / conn:<id> / file:<path>）
// 注意：不能在模块顶层读 KV——模块在 loadKvStore() 之前就被 import，缓存还是空的。
// 改为首次调用 useDbConnections() 时（组件 setup 阶段，已在 loadKvStore 之后）再载入。
const connections = ref<DbConnection[]>([])
const activeRef = ref<string>('memory')
let loaded = false

export function useDbConnections()
{
    if (!loaded) {
        loaded = true
        connections.value = kvGetJSON<DbConnection[]>(CONN_KEY, [])
        activeRef.value = kvGet(REF_KEY) || 'memory'
    }

    const persist = () => kvSetJSON(CONN_KEY, connections.value)

    const add = (c: Omit<DbConnection, 'id'>) => {
        connections.value.push({...c, id: genId()})
        persist()
    }
    const update = (id: string, patch: Partial<DbConnection>) => {
        const i = connections.value.findIndex(x => x.id === id)
        if (i >= 0) {
            connections.value[i] = {...connections.value[i], ...patch}
            persist()
        }
    }
    const remove = (id: string) => {
        connections.value = connections.value.filter(x => x.id !== id)
        persist()
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
            return connections.value.find(c => c.id === t.slice(5))?.name || '(已删除)'
        }
        if (t.startsWith('file:')) {
            const p = t.slice(5)
            return p.split(/[\\/]/).pop() || p
        }
        return '内存数据库'
    }

    return {connections, activeRef, add, update, remove, setActiveRef, resolveActiveSource, activeLabel}
}
