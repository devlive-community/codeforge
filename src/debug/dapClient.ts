// DAP 客户端：把后端 dap_* 命令/事件封装为请求-响应 + 事件分发模型。
// 后端已剥离 Content-Length 帧，这里收发的是完整 JSON 字符串。
// 与 lspTransport 同构；launch 握手由上层(useDebug)用这些原语编排。
import {invoke} from '@tauri-apps/api/core'
import {listen, type UnlistenFn} from '@tauri-apps/api/event'

type Handler = (body: any) => void
type ReverseHandler = (args: any) => any | Promise<any>

interface Pending {
  resolve: (v: any) => void
  reject: (e: Error) => void
}

export class DapClient {
  readonly session: string
  private language: string
  private seq = 1
  private pending = new Map<number, Pending>()
  private eventHandlers = new Map<string, Set<Handler>>()
  private reverseHandlers = new Map<string, ReverseHandler>()
  private unlisten: UnlistenFn[] = []
  private closeCb: (() => void) | null = null

  constructor(session: string, language: string) {
    this.session = session
    this.language = language
  }

  // 启动适配器并开始监听消息
  async start(): Promise<void> {
    const un1 = await listen<{session: string; messages: string[]}>('dap:messages', (e) => {
      if (e.payload.session === this.session) {
        for (const m of e.payload.messages) {
          this.handle(m)
        }
      }
    })
    const un2 = await listen<string>('dap:exit', (e) => {
      if (e.payload === this.session) {
        this.closeCb?.()
      }
    })
    this.unlisten = [un1, un2]
    await invoke('dap_start', {session: this.session, language: this.language})
  }

  private handle(raw: string): void {
    let msg: any
    try {
      msg = JSON.parse(raw)
    }
    catch {
      return
    }
    if (msg.type === 'response') {
      const p = this.pending.get(msg.request_seq)
      if (p) {
        this.pending.delete(msg.request_seq)
        if (msg.success) {
          p.resolve(msg.body)
        }
        else {
          p.reject(new Error(msg.message || `${msg.command} 失败`))
        }
      }
    }
    else if (msg.type === 'event') {
      const hs = this.eventHandlers.get(msg.event)
      if (hs) {
        for (const h of [...hs]) {
          h(msg.body)
        }
      }
    }
    else if (msg.type === 'request') {
      // 适配器反向请求（runInTerminal / startDebugging 等）
      const handler = this.reverseHandlers.get(msg.command)
      Promise.resolve(handler ? handler(msg.arguments) : null)
        .then((body) => this.sendRaw({type: 'response', request_seq: msg.seq, success: true, command: msg.command, body}))
        .catch((err) => this.sendRaw({type: 'response', request_seq: msg.seq, success: false, command: msg.command, message: String(err)}))
    }
  }

  private sendRaw(obj: Record<string, any>): void {
    const message = JSON.stringify({seq: this.seq++, ...obj})
    invoke('dap_send', {session: this.session, message}).catch(() => {})
  }

  // 发送一个 DAP 请求，按 request_seq 关联响应
  request<T = any>(command: string, args?: any): Promise<T> {
    const seq = this.seq++
    const message = JSON.stringify({seq, type: 'request', command, arguments: args ?? {}})
    return new Promise<T>((resolve, reject) => {
      this.pending.set(seq, {resolve, reject})
      invoke('dap_send', {session: this.session, message}).catch((e) => {
        this.pending.delete(seq)
        reject(new Error(String(e)))
      })
    })
  }

  // 订阅适配器事件，返回取消订阅函数
  on(event: string, handler: Handler): () => void {
    let set = this.eventHandlers.get(event)
    if (!set) {
      set = new Set()
      this.eventHandlers.set(event, set)
    }
    const s = set
    s.add(handler)
    return () => s.delete(handler)
  }

  // 注册反向请求处理器
  onReverseRequest(command: string, handler: ReverseHandler): void {
    this.reverseHandlers.set(command, handler)
  }

  onClose(cb: () => void): void {
    this.closeCb = cb
  }

  async stop(): Promise<void> {
    try {
      await invoke('dap_stop', {session: this.session})
    }
    catch {
      // 忽略
    }
    for (const u of this.unlisten) {
      u()
    }
    this.unlisten = []
    for (const p of this.pending.values()) {
      p.reject(new Error('调试会话已结束'))
    }
    this.pending.clear()
    this.eventHandlers.clear()
  }
}
