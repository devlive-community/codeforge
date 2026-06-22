// 调试状态共享 store（单例）。P2：断点与执行位置；P3：会话编排与控制。
import {reactive, ref} from 'vue'
import {DapClient} from '../debug/dapClient'

export type DebugStatus = 'inactive' | 'starting' | 'running' | 'stopped'
export interface LaunchConfig { filePath: string; language: string; cwd?: string | null }
export interface StackFrame { id: number; name: string; line: number; column: number; source?: {path?: string; name?: string} }
export interface Scope { name: string; variablesReference: number; expensive: boolean }
export interface DapVariable { name: string; value: string; type?: string; variablesReference: number }

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
const frames = ref<StackFrame[]>([])
const selectedFrameId = ref<number | null>(null)
// 跳转请求（停驻/选择帧时触发，由 App 打开文件并定位）
const reveal = ref<{path: string; line: number; seq: number} | null>(null)
let revealSeq = 0
let client: DapClient | null = null
let threadId = 0
let lastConfig: LaunchConfig | null = null
let stopping = false

function requestReveal(path: string, line: number): void {
  reveal.value = {path, line, seq: ++revealSeq}
}

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

// 停驻后加载调用栈，定位栈顶帧
async function loadStopState(): Promise<void> {
  if (!client) {
    return
  }
  try {
    const res = await client.request('stackTrace', {threadId, startFrame: 0, levels: 20})
    frames.value = (res?.stackFrames ?? []) as StackFrame[]
    const top = frames.value[0]
    selectedFrameId.value = top?.id ?? null
    if (top?.source?.path && top.line) {
      setStopped({path: top.source.path, line: top.line})
      requestReveal(top.source.path, top.line)
    }
  }
  catch {
    // 忽略
  }
}

// 选择调用栈帧：跳转到其源码位置（不改变执行行高亮）
function selectFrame(id: number): void {
  selectedFrameId.value = id
  const f = frames.value.find(x => x.id === id)
  if (f?.source?.path && f.line) {
    requestReveal(f.source.path, f.line)
  }
}

async function requestScopes(frameId: number): Promise<Scope[]> {
  if (!client) {
    return []
  }
  try {
    const res = await client.request('scopes', {frameId})
    return (res?.scopes ?? []) as Scope[]
  }
  catch {
    return []
  }
}

async function requestVariables(variablesReference: number): Promise<DapVariable[]> {
  if (!client || variablesReference <= 0) {
    return []
  }
  try {
    const res = await client.request('variables', {variablesReference})
    return (res?.variables ?? []) as DapVariable[]
  }
  catch {
    return []
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
    await loadStopState()
  })
  c.on('continued', () => {
    status.value = 'running'
    setStopped(null)
    frames.value = []
    selectedFrameId.value = null
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

function clearStopUi(): void {
  setStopped(null)
  frames.value = []
  selectedFrameId.value = null
}

function doContinue(): void {
  if (status.value !== 'stopped') {
    return
  }
  status.value = 'running'
  clearStopUi()
  withThread('continue')
}
function pause(): void {
  withThread('pause')
}
function stepOver(): void {
  if (status.value !== 'stopped') {
    return
  }
  clearStopUi()
  withThread('next')
}
function stepIn(): void {
  if (status.value !== 'stopped') {
    return
  }
  clearStopUi()
  withThread('stepIn')
}
function stepOut(): void {
  if (status.value !== 'stopped') {
    return
  }
  clearStopUi()
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
  frames.value = []
  selectedFrameId.value = null
}

export function useDebug() {
  return {
    breakpoints,
    bpVersion,
    stopped,
    status,
    consoleLines,
    frames,
    selectedFrameId,
    reveal,
    fileBreakpoints,
    toggleBreakpoint,
    setStopped,
    syncBreakpoints,
    selectFrame,
    requestScopes,
    requestVariables,
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
