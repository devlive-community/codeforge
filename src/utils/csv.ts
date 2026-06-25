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

function triggerDownload(blob: Blob, filename: string): void {
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = filename
  a.click()
  URL.revokeObjectURL(url)
}

/** 触发浏览器下载（带 BOM，便于 Excel 正确识别中文） */
export function downloadCsv(columns: string[], rows: any[][], filename?: string): void {
  triggerDownload(
    new Blob(['﻿' + toCsv(columns, rows)], {type: 'text/csv;charset=utf-8'}),
    filename || `data-${Date.now()}.csv`
  )
}

/** 导出为 JSON（每行一个对象，列名为键） */
export function downloadJson(columns: string[], rows: any[][], filename?: string): void {
  const data = rows.map((row) => {
    const o: Record<string, any> = {}
    columns.forEach((c, i) => (o[c] = row[i]))
    return o
  })
  triggerDownload(
    new Blob([JSON.stringify(data, null, 2)], {type: 'application/json;charset=utf-8'}),
    filename || `data-${Date.now()}.json`
  )
}

/** 导出为 Excel(.xlsx)。按需加载 SheetJS，避免拖累其它引用此工具的页面 */
export async function downloadXlsx(columns: string[], rows: any[][], filename?: string): Promise<void> {
  const XLSX = await import('xlsx')
  const ws = XLSX.utils.aoa_to_sheet([columns, ...rows])
  const wb = XLSX.utils.book_new()
  XLSX.utils.book_append_sheet(wb, ws, 'Sheet1')
  XLSX.writeFile(wb, filename || `data-${Date.now()}.xlsx`)
}
