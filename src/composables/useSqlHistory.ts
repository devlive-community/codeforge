// SQL 查询历史 + 收藏（单例，KV 持久化）。
import {ref} from 'vue'
import {kvGetJSON, kvSetJSON} from './useKvStore'

export interface SqlHistoryItem { sql: string; ts: number; favorite: boolean }

const KEY = 'sql-history'
const MAX = 100 // 非收藏上限（收藏不计入裁剪）

const items = ref<SqlHistoryItem[]>(kvGetJSON<SqlHistoryItem[]>(KEY, []))

function persist(): void {
  kvSetJSON(KEY, items.value)
}

function addHistory(sql: string): void {
  const s = sql.trim()
  if (!s) {
    return
  }
  // 与已有完全相同则提到最前并更新时间（保留其收藏状态）
  const idx = items.value.findIndex(i => i.sql === s)
  if (idx >= 0) {
    const it = items.value[idx]
    it.ts = Date.now()
    items.value.splice(idx, 1)
    items.value.unshift(it)
  }
  else {
    items.value.unshift({sql: s, ts: Date.now(), favorite: false})
  }
  // 裁剪最旧的非收藏项
  let nonFav = items.value.filter(i => !i.favorite).length
  for (let i = items.value.length - 1; i >= 0 && nonFav > MAX; i--) {
    if (!items.value[i].favorite) {
      items.value.splice(i, 1)
      nonFav--
    }
  }
  persist()
}

function toggleFavorite(ts: number): void {
  const it = items.value.find(i => i.ts === ts)
  if (it) {
    it.favorite = !it.favorite
    persist()
  }
}

function remove(ts: number): void {
  items.value = items.value.filter(i => i.ts !== ts)
  persist()
}

// 清空（保留收藏）
function clearHistory(): void {
  items.value = items.value.filter(i => i.favorite)
  persist()
}

export function useSqlHistory() {
  return {items, addHistory, toggleFavorite, remove, clearHistory}
}
