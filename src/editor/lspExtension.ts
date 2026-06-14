// 根据语言/文件构建 CodeMirror LSP 扩展（补全、悬浮、诊断、跳转、重命名、格式化）
import {invoke} from '@tauri-apps/api/core'
import {
  LanguageServerClient,
  languageServerWithTransport,
  formatDocument,
  formatSelection,
  formattingOptions,
  renameSymbol
} from 'codemirror-languageserver'
import {keymap} from '@codemirror/view'
import {TauriLspTransport} from './lspTransport'
import {setLspState} from './lspStatus'
import {lspCustomHover} from './lspHover'

// 代次：每次构建 LSP 扩展自增，过期 client 的回调据此忽略
let stateGen = 0

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
  rootDir?: string | null,
  fmtOptions?: {tabSize?: number; insertSpaces?: boolean}
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
  // 代次令牌：扩展重建时旧 client 的 onClose/onError 不应覆盖最新状态
  const myGen = ++stateGen
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
      onCapabilities: () => {
        if (myGen === stateGen) {
          setLspState(language, 'on')
        }
      },
      onError: () => {
        if (myGen === stateGen) {
          setLspState(language, 'off')
        }
      },
      onClose: () => {
        if (myGen === stateGen) {
          setLspState(language, 'off')
        }
      }
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
    // base 已内置：补全(源+键位)、悬浮、文档高亮、跳转定义(F12/Cmd+Click)、
    // 重命名基建(renameExtension)。不要重复添加这些，否则触发配置冲突。
    // 此处仅补 base 未绑定的触发键与未接入的能力：
    //   - F2：触发重命名(base 有 renameExtension 但未绑键)
    //   - Shift-Alt-F / Mod-Shift-I：格式化整篇文档(base 未接入格式化)
    //   - Mod-K Mod-F：格式化选中区域
    //   - formattingOptions：将格式化的缩进与编辑器配置对齐
    const lspKeymap = keymap.of([
      {key: 'F2', run: renameSymbol},
      {key: 'Shift-Alt-f', run: formatDocument},
      {key: 'Mod-Shift-i', run: formatDocument},
      {key: 'Mod-k Mod-f', run: formatSelection}
    ])
    const fmt = formattingOptions.of({
      tabSize: fmtOptions?.tabSize ?? 4,
      insertSpaces: fmtOptions?.insertSpaces ?? true
    })
    return [base, lspCustomHover, lspKeymap, fmt]
  }
  catch {
    return null
  }
}
