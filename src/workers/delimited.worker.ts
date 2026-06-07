// CSV / TSV 解析 Worker：在后台线程解析，避免超大文件阻塞 UI
import {parseTable} from '../utils/delimited'

interface ParseRequest {
  id: number
  text: string
}

self.onmessage = (e: MessageEvent<ParseRequest>) => {
  const {id, text} = e.data
  try {
    const table = parseTable(text)
    ;(self as unknown as Worker).postMessage({id, ...table})
  }
  catch {
    ;(self as unknown as Worker).postMessage({id, columns: [], rows: []})
  }
}
