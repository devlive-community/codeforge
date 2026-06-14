// 把编辑器内的 LSP 诊断聚合到响应式 store，供「问题面板」展示与跳转。
// 收集做去抖：索引期诊断高频刷新，只在停顿后聚合一次，顺带降低渲染压力。
import {ref} from 'vue'
import {EditorView} from '@codemirror/view'
import {forEachDiagnostic} from '@codemirror/lint'

export interface DiagItem {
  severity: string // 'error' | 'warning' | 'info' | 'hint'
  message: string
  line: number     // 1 基行号
  col: number      // 1 基列号
  from: number     // 文档偏移，用于排序
}

// 当前(活动)编辑器的诊断列表
export const diagnostics = ref<DiagItem[]>([])

let timer: ReturnType<typeof setTimeout> | null = null

// 在每次编辑器更新后去抖重算诊断列表
export const diagnosticsCollector = EditorView.updateListener.of((update) => {
  const view = update.view
  if (timer) {
    clearTimeout(timer)
  }
  timer = setTimeout(() => {
    const items: DiagItem[] = []
    forEachDiagnostic(view.state, (d, from) => {
      const line = view.state.doc.lineAt(from)
      items.push({
        severity: d.severity,
        message: d.message,
        line: line.number,
        col: from - line.from + 1,
        from
      })
    })
    items.sort((a, b) => a.from - b.from)
    diagnostics.value = items
  }, 200)
})
