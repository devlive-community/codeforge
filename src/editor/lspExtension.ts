// 根据语言/文件构建 CodeMirror LSP 扩展（补全、悬浮、诊断、跳转、重命名）
import {invoke} from '@tauri-apps/api/core'
import {LanguageServerClient, languageServerWithTransport} from 'codemirror-languageserver'
import {TauriLspTransport} from './lspTransport'
import {setLspState} from './lspStatus'
import {lspCustomHover} from './lspHover'

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

// 草稿(未保存)时用的文件扩展名，构造 untitled 文档 URI
const LANGUAGE_EXT: Record<string, string> = {
  python: 'py', typescript: 'ts', javascript: 'js', rust: 'rs', go: 'go',
  c: 'c', cpp: 'cpp', 'objective-c': 'm', 'objective-cpp': 'mm',
  lua: 'lua', php: 'php', ruby: 'rb', html: 'html', css: 'css', json: 'json'
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
  if (!language) {
    return null
  }
  const languageId = LANGUAGE_ID[language]
  if (!languageId) {
    setLspState(language || '', 'off')
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
    setLspState(language, 'off')
    return null
  }
  setLspState(language, 'connecting')
  try {
    const transport = new TauriLspTransport(language)
    const rootUri = rootDir ? toUri(rootDir) : null
    // 已保存文件用真实路径；草稿用 untitled 文档 URI（语言服务器按内存内容分析）
    const documentUri = filePath
      ? toUri(filePath)
      : `untitled:Untitled.${LANGUAGE_EXT[languageId] || 'txt'}`
    const workspaceFolders = rootUri ? [{uri: rootUri, name: 'workspace'}] : null
    // 自建 client 以获取初始化完成(capabilities)信号，驱动状态栏的"索引中→就绪"
    const client = new LanguageServerClient({
      transport,
      rootUri,
      workspaceFolders,
      documentUri,
      languageId,
      autoClose: true,
      onCapabilities: () => setLspState(language, 'on'),
      onError: () => setLspState(language, 'off'),
      onClose: () => setLspState(language, 'off')
    })
    const base = languageServerWithTransport({
      client,
      transport,
      rootUri,
      workspaceFolders,
      documentUri,
      languageId,
      allowHTMLContent: true,
      autoClose: true
    })
    // 用自绘悬浮替代库的 hover/诊断 tooltip（定位精确到鼠标 + 主题适配）
    return [base, lspCustomHover]
  }
  catch {
    return null
  }
}
