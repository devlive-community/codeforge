// CSV / TSV 等分隔文本解析（纯函数，供主线程与 Web Worker 复用）

export interface DelimitedTable {
  columns: string[]
  rows: string[][]
}

/** 根据首行推断分隔符（制表符优先支持 TSV，其次分号，默认逗号） */
export function detectDelimiter(text: string): string {
  const firstLine = text.split('\n', 1)[0] || ''
  const tabs = (firstLine.match(/\t/g) || []).length
  const commas = (firstLine.match(/,/g) || []).length
  const semis = (firstLine.match(/;/g) || []).length
  if (tabs > 0 && tabs >= commas) {
    return '\t'
  }
  if (semis > commas) {
    return ';'
  }
  return ','
}

/** 支持引号、转义引号("")、字段内换行的分隔解析，返回二维数组 */
export function parseDelimited(text: string, delim: string): string[][] {
  const rows: string[][] = []
  let field = ''
  let row: string[] = []
  let inQuotes = false
  for (let i = 0; i < text.length; i++) {
    const c = text[i]
    if (inQuotes) {
      if (c === '"') {
        if (text[i + 1] === '"') {
          field += '"'
          i++
        }
        else {
          inQuotes = false
        }
      }
      else {
        field += c
      }
    }
    else if (c === '"') {
      inQuotes = true
    }
    else if (c === delim) {
      row.push(field)
      field = ''
    }
    else if (c === '\n') {
      row.push(field)
      rows.push(row)
      row = []
      field = ''
    }
    else if (c !== '\r') {
      field += c
    }
  }
  if (field.length > 0 || row.length > 0) {
    row.push(field)
    rows.push(row)
  }
  return rows
}

/** 解析为表格：首行作列名，其余对齐为等长行 */
export function parseTable(text: string): DelimitedTable {
  const trimmed = text.trim()
  if (!trimmed) {
    return {columns: [], rows: []}
  }
  const delim = detectDelimiter(trimmed)
  const all = parseDelimited(trimmed, delim).filter(r => r.length > 1 || (r.length === 1 && r[0] !== ''))
  if (all.length === 0) {
    return {columns: [], rows: []}
  }
  const columns = all[0].map((c, i) => c.trim() || `列${i + 1}`)
  const rows = all.slice(1).map(r => {
    const out: string[] = new Array(columns.length).fill('')
    for (let i = 0; i < columns.length; i++) {
      out[i] = r[i] ?? ''
    }
    return out
  })
  return {columns, rows}
}
