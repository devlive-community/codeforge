// 将 codemirror-languageserver 的 Transport 桥接到 Tauri 后端 LSP 命令/事件
import {invoke} from '@tauri-apps/api/core'
import {listen, type UnlistenFn} from '@tauri-apps/api/event'
import type {Transport} from 'codemirror-languageserver'

export class TauriLspTransport implements Transport {
  private language: string
  private msgCb: ((m: string) => void) | null = null
  private closeCb: (() => void) | null = null
  private errorCb: ((e: Error) => void) | null = null
  private unlisten: UnlistenFn[] = []
  private ready: Promise<void>

  constructor(language: string) {
    this.language = language
    this.ready = this.init()
  }

  private async init() {
    const un1 = await listen<{ language: string; messages: string[] }>('lsp:messages', (e) => {
      if (e.payload.language === this.language) {
        for (const m of e.payload.messages) {
          this.msgCb?.(m)
        }
      }
    })
    const un2 = await listen<string>('lsp:exit', (e) => {
      if (e.payload === this.language) {
        this.closeCb?.()
      }
    })
    this.unlisten = [un1, un2]
    await invoke('lsp_start', {language: this.language})
  }

  send(message: string): void {
    // 等服务器启动后再发，保持顺序
    this.ready
      .then(() => invoke('lsp_send', {language: this.language, message}))
      .catch((e) => this.errorCb?.(new Error(String(e))))
  }

  onMessage(callback: (message: string) => void): void {
    this.msgCb = callback
  }

  onClose(callback: () => void): void {
    this.closeCb = callback
  }

  onError(callback: (error: Error) => void): void {
    this.errorCb = callback
  }

  close(): void {
    for (const u of this.unlisten) {
      u()
    }
    this.unlisten = []
    // 不停止服务器进程，保持热以便同语言其它文件复用
  }
}
