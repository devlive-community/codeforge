// 根据语言/文件构建 CodeMirror LSP 扩展（补全、悬浮、诊断、跳转、重命名）
import {invoke} from '@tauri-apps/api/core'
import {languageServerWithTransport} from 'codemirror-languageserver'
import {TauriLspTransport} from './lspTransport'

// CodeForge 语言 key → LSP languageId（与后端 server_cmd 对应）
const LANGUAGE_ID: Record<string, string> = {
  python3: 'python',
  python2: 'python',
  python: 'python',
  typescript: 'typescript',
  'typescript-nodejs': 'typescript',
  'typescript-browser': 'typescript',
  'javascript-nodejs': 'javascript',
  'javascript-browser': 'javascript',
  'javascript-jquery': 'javascript',
  nodejs: 'javascript',
  rust: 'rust',
  go: 'go',
  c: 'c',
  cpp: 'cpp',
  'objective-c': 'objective-c',
  'objective-cpp': 'objective-cpp',
  lua: 'lua',
  php: 'php',
  ruby: 'ruby',
  html: 'html',
  css: 'css',
  json: 'json'
}

export const lspSupportsLanguage = (language?: string): boolean =>
  !!language && language in LANGUAGE_ID

const toUri = (p: string): string =>
  'file://' + encodeURI(p.replace(/\\/g, '/')).replace(/#/g, '%23').replace(/\?/g, '%3F')

/**
 * 构建当前文件的 LSP 扩展；语言不支持 / 无文件 / 服务器不可用时返回 null。
 */
export async function createLspExtensions(
  language: string | undefined,
  filePath: string | null | undefined,
  rootDir?: string | null
): Promise<any | null> {
  if (!language || !filePath) {
    return null
  }
  const languageId = LANGUAGE_ID[language]
  if (!languageId) {
    return null
  }
  let available = false
  try {
    available = await invoke<boolean>('lsp_available', {language})
  }
  catch {
    available = false
  }
  if (!available) {
    return null
  }
  try {
    const transport = new TauriLspTransport(language)
    const rootUri = rootDir ? toUri(rootDir) : null
    return languageServerWithTransport({
      transport,
      rootUri,
      workspaceFolders: rootUri ? [{uri: rootUri, name: 'workspace'}] : null,
      documentUri: toUri(filePath),
      languageId,
      allowHTMLContent: true
    })
  }
  catch {
    return null
  }
}
