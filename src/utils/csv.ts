// 表格数据导出为 CSV 的通用工具（SQL 结果表 / CSV 数据表 / 图表底层数据共用）

const escapeCell = (v: any): string => {
  const s = v === null || v === undefined ? '' : typeof v === 'object' ? JSON.stringify(v) : String(v)
  return /[",\r\n]/.test(s) ? `"${s.replace(/"/g, '""')}"` : s
}

/** 将列名与行数据序列化为 CSV 文本 */
export function toCsv(columns: string[], rows: any[][]): string {
  const lines = [columns.map(escapeCell).join(',')]
  for (const row of rows) {
    lines.push(columns.map((_c, i) => escapeCell(row[i])).join(','))
  }
  return lines.join('\n')
}

/** 触发浏览器下载（带 BOM，便于 Excel 正确识别中文） */
export function downloadCsv(columns: string[], rows: any[][], filename?: string): void {
  const blob = new Blob(['﻿' + toCsv(columns, rows)], {type: 'text/csv;charset=utf-8'})
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = filename || `data-${Date.now()}.csv`
  a.click()
  URL.revokeObjectURL(url)
}
