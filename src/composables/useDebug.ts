// 调试状态共享 store（单例）。P2：断点与执行位置；P3：会话编排与控制。
import {reactive, ref} from 'vue'
import {DapClient} from '../debug/dapClient'

export type DebugStatus = 'inactive' | 'starting' | 'running' | 'stopped'
export interface LaunchConfig { filePath: string; language: string; cwd?: string | null }

// 文件绝对路径 -> 断点行号集合（1-based）
const breakpoints = reactive<Map<string, Set<number>>>(new Map())
// 断点变更版本号：Map/Set 深层响应不够可靠，统一用版本号驱动外部同步
const bpVersion = ref(0)
// 当前停驻（执行）位置
const stopped = ref<{path: string; line: number} | null>(null)

function fileBreakpoints(path: string | null | undefined): number[] {
  if (!path) {
    return []
  }
  const s = breakpoints.get(path)
  return s ? [...s].sort((a, b) => a - b) : []
}

function toggleBreakpoint(path: string | null | undefined, line: number): void {
  if (!path || line < 1) {
    return
  }
  let s = breakpoints.get(path)
  if (!s) {
    s = new Set()
    breakpoints.set(path, s)
  }
  if (s.has(line)) {
    s.delete(line)
  }
  else {
    s.add(line)
  }
  if (s.size === 0) {
    breakpoints.delete(path)
  }
  bpVersion.value++
}

function setStopped(loc: {path: string; line: number} | null): void {
  stopped.value = loc
}

// ===== P3：会话编排与控制 =====
const status = ref<DebugStatus>('inactive')
const consoleLines = ref<{category: string; text: string}[]>([])
let client: DapClient | null = null
let threadId = 0
let lastConfig: LaunchConfig | null = null
let stopping = false

function pushOut(category: string, text: string): void {
  consoleLines.value.push({category, text})
  if (consoleLines.value.length > 2000) {
    consoleLines.value.splice(0, consoleLines.value.length - 2000)
  }
}

async function sendBreakpoints(): Promise<void> {
  if (!client) {
    return
  }
  for (const [path, lines] of breakpoints) {
    await client
      .request('setBreakpoints', {
        source: {path},
        breakpoints: [...lines].sort((a, b) => a - b).map(line => ({line}))
      })
      .catch(() => {})
  }
}

// 会话进行中时，断点变更后实时下发（重发当前文件断点）
async function syncBreakpoints(path: string | null | undefined): Promise<void> {
  if (!client || status.value === 'inactive' || !path) {
    return
  }
  await client
    .request('setBreakpoints', {
      source: {path},
      breakpoints: fileBreakpoints(path).map(line => ({line}))
    })
    .catch(() => {})
}

async function revealTopFrame(): Promise<void> {
  if (!client) {
    return
  }
  try {
    const res = await client.request('stackTrace', {threadId, startFrame: 0, levels: 1})
    const frame = res?.stackFrames?.[0]
    const path = frame?.source?.path
    const line = frame?.line
    if (path && line) {
      setStopped({path, line})
    }
  }
  catch {
    // 忽略
  }
}

async function startSession(config: LaunchConfig): Promise<void> {
  if (status.value !== 'inactive') {
    await stopSession()
  }
  if (!config.filePath) {
    throw new Error('需要已保存的文件')
  }
  lastConfig = config
  status.value = 'starting'
  consoleLines.value = []
  threadId = 0
  const c = new DapClient('debug', config.language)
  client = c

  c.onClose(() => cleanup())
  c.on('initialized', async () => {
    await sendBreakpoints()
    await c.request('setExceptionBreakpoints', {filters: []}).catch(() => {})
    await c.request('configurationDone').catch(() => {})
  })
  c.on('stopped', async (body) => {
    threadId = body?.threadId ?? threadId
    status.value = 'stopped'
    await revealTopFrame()
  })
  c.on('continued', () => {
    status.value = 'running'
    setStopped(null)
  })
  c.on('output', (body) => pushOut(body?.category || 'console', body?.output || ''))
  c.on('terminated', () => stopSession())

  try {
    await c.start()
    await c.request('initialize', {
      clientID: 'codeforge',
      clientName: 'CodeForge',
      adapterID: config.language === 'go' ? 'go' : 'debugpy',
      locale: 'en',
      linesStartAt1: true,
      columnsStartAt1: true,
      pathFormat: 'path',
      supportsRunInTerminalRequest: false
    })
    // launch 在 configurationDone 后才返回，故不在主流程等待
    c.request('launch', {
      request: 'launch',
      type: config.language === 'go' ? 'go' : 'python',
      name: 'CodeForge',
      program: config.filePath,
      console: 'internalConsole',
      cwd: config.cwd || undefined,
      justMyCode: true,
      stopOnEntry: false
    }).catch((e) => {
      pushOut('stderr', `launch 失败: ${e}\n`)
      stopSession()
    })
    status.value = 'running'
  }
  catch (e) {
    cleanup()
    throw e
  }
}

function withThread(command: string): void {
  if (!client || status.value === 'inactive') {
    return
  }
  client.request(command, {threadId}).catch(() => {})
}

function doContinue(): void {
  if (status.value !== 'stopped') {
    return
  }
  status.value = 'running'
  setStopped(null)
  withThread('continue')
}
function pause(): void {
  withThread('pause')
}
function stepOver(): void {
  if (status.value !== 'stopped') {
    return
  }
  setStopped(null)
  withThread('next')
}
function stepIn(): void {
  if (status.value !== 'stopped') {
    return
  }
  setStopped(null)
  withThread('stepIn')
}
function stepOut(): void {
  if (status.value !== 'stopped') {
    return
  }
  setStopped(null)
  withThread('stepOut')
}

async function stopSession(): Promise<void> {
  if (stopping) {
    return
  }
  stopping = true
  const c = client
  if (c) {
    try {
      await c.request('disconnect', {terminateDebuggee: true})
    }
    catch {
      // 忽略
    }
    await c.stop()
  }
  cleanup()
  stopping = false
}

async function restart(): Promise<void> {
  const cfg = lastConfig
  await stopSession()
  if (cfg) {
    await startSession(cfg)
  }
}

function cleanup(): void {
  client = null
  threadId = 0
  status.value = 'inactive'
  setStopped(null)
}

export function useDebug() {
  return {
    breakpoints,
    bpVersion,
    stopped,
    status,
    consoleLines,
    fileBreakpoints,
    toggleBreakpoint,
    setStopped,
    syncBreakpoints,
    startSession,
    stopSession,
    restart,
    continue: doContinue,
    pause,
    stepOver,
    stepIn,
    stepOut
  }
}
