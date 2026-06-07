// CSV / TSV 解析 Worker：后台解析并上报进度，避免超大文件阻塞 UI
import {parseTable} from '../utils/delimited'

interface ParseRequest {
  id: number
  text: string
}

const post = (msg: any) => (self as unknown as Worker).postMessage(msg)

self.onmessage = (e: MessageEvent<ParseRequest>) => {
  const {id, text} = e.data
  try {
    let lastPercent = -1
    const table = parseTable(text, (p) => {
      const percent = Math.floor(p * 100)
      if (percent !== lastPercent) {
        lastPercent = percent
        post({id, type: 'progress', percent})
      }
    })
    post({id, type: 'done', columns: table.columns, rows: table.rows})
  }
  catch {
    post({id, type: 'done', columns: [], rows: []})
  }
}
