// 跨文件诊断聚合：拦截 LSP 的 lsp:messages 原始流，按文件(URI)收集 publishDiagnostics，
// 供「问题」面板跨文件展示。语言服务通常至少推送已打开文件的诊断，部分服务会推送整个工程。
import {ref} from 'vue'
import {listen} from '@tauri-apps/api/event'

export interface FileDiag {
  severity: string // 'error' | 'warning' | 'info' | 'hint'
  message: string
  line: number     // 1-based
  col: number      // 1-based
}

// 绝对路径 → 诊断列表
export const allDiagnostics = ref<Record<string, FileDiag[]>>({})

const SEVERITY = ['', 'error', 'warning', 'info', 'hint']

// file:// URI → 本地路径（兼容 Windows 的 file:///C:/…）
const uriToPath = (uri: string): string => {
  let p = uri.replace(/^file:\/\//, '')
  try {
    p = decodeURIComponent(p)
  }
  catch { /* 保留原样 */ }
  if (/^\/[A-Za-z]:\//.test(p)) {
    p = p.slice(1)
  }
  return p
}

let started = false

export const initDiagnosticsAggregator = async () => {
  if (started) {
    return
  }
  started = true
  await listen<{ language: string; messages: string[] }>('lsp:messages', (e) => {
    for (const raw of e.payload.messages) {
      let msg: any
      try {
        msg = JSON.parse(raw)
      }
      catch {
        continue
      }
      if (msg?.method !== 'textDocument/publishDiagnostics' || !msg.params) {
        continue
      }
      const path = uriToPath(msg.params.uri)
      const items: FileDiag[] = (msg.params.diagnostics || []).map((d: any) => ({
        severity: SEVERITY[d.severity] || 'info',
        message: d.message,
        line: (d.range?.start?.line ?? 0) + 1,
        col: (d.range?.start?.character ?? 0) + 1
      }))
      const next = {...allDiagnostics.value}
      if (items.length) {
        next[path] = items
      }
      else {
        delete next[path]
      }
      allDiagnostics.value = next
    }
  })
}

export const clearAllDiagnostics = () => {
  allDiagnostics.value = {}
}
