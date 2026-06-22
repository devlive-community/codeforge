// 调试状态共享 store（单例）。P2：断点与执行位置；P3：会话编排与控制。
import {reactive, ref} from 'vue'
import {DapClient} from '../debug/dapClient'

export type DebugStatus = 'inactive' | 'starting' | 'running' | 'stopped'
export interface LaunchConfig { filePath: string; language: string; cwd?: string | null }
export interface StackFrame { id: number; name: string; line: number; column: number; source?: {path?: string; name?: string} }
export interface Scope { name: string; variablesReference: number; expensive: boolean }
export interface DapVariable { name: string; value: string; type?: string; variablesReference: number }

// 文件绝对路径 -> (行号(1-based) -> 条件表达式，空串表示无条件)
const breakpoints = reactive<Map<string, Map<number, string>>>(new Map())
// 断点变更版本号：Map 深层响应不够可靠，统一用版本号驱动外部同步
const bpVersion = ref(0)
// 当前停驻（执行）位置
const stopped = ref<{path: string; line: number} | null>(null)

function fileBreakpoints(path: string | null | undefined): number[] {
  if (!path) {
    return []
  }
  const m = breakpoints.get(path)
  return m ? [...m.keys()].sort((a, b) => a - b) : []
}

function breakpointCondition(path: string | null | undefined, line: number): string {
  if (!path) {
    return ''
  }
  return breakpoints.get(path)?.get(line) ?? ''
}

function toggleBreakpoint(path: string | null | undefined, line: number): void {
  if (!path || line < 1) {
    return
  }
  let m = breakpoints.get(path)
  if (!m) {
    m = new Map()
    breakpoints.set(path, m)
  }
  if (m.has(line)) {
    m.delete(line)
  }
  else {
    m.set(line, '')
  }
  if (m.size === 0) {
    breakpoints.delete(path)
  }
  bpVersion.value++
}

// 设置/更新某断点的条件（行不存在则新建该断点）
function setBreakpointCondition(path: string | null | undefined, line: number, condition: string): void {
  if (!path || line < 1) {
    return
  }
  let m = breakpoints.get(path)
  if (!m) {
    m = new Map()
    breakpoints.set(path, m)
  }
  m.set(line, condition)
  bpVersion.value++
}

function setStopped(loc: {path: string; line: number} | null): void {
  stopped.value = loc
}

// 列出全部断点（跨文件，供断点列表面板）
function allBreakpoints(): {path: string; line: number; condition: string}[] {
  const out: {path: string; line: number; condition: string}[] = []
  for (const [path, lines] of breakpoints) {
    for (const [line, condition] of lines) {
      out.push({path, line, condition})
    }
  }
  return out.sort((a, b) => (a.path === b.path ? a.line - b.line : a.path.localeCompare(b.path)))
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

// 监视表达式（停驻/选帧时重算）
const watches = ref<{expr: string; value: string}[]>([])

// 求值：在选中帧上下文中执行表达式。context: watch | repl | hover
async function evaluate(expression: string, context: string): Promise<{result: string; variablesReference: number}> {
  if (!client || !expression.trim()) {
    return {result: '', variablesReference: 0}
  }
  const args: Record<string, any> = {expression, context}
  if (selectedFrameId.value != null) {
    args.frameId = selectedFrameId.value
  }
  const res = await client.request('evaluate', args)
  return {result: res?.result ?? '', variablesReference: res?.variablesReference ?? 0}
}

async function refreshWatches(): Promise<void> {
  if (status.value !== 'stopped') {
    return
  }
  for (const w of watches.value) {
    try {
      const {result} = await evaluate(w.expr, 'watch')
      w.value = result
    }
    catch (e) {
      w.value = `<${e}>`
    }
  }
}

function addWatch(expr: string): void {
  if (expr.trim()) {
    watches.value.push({expr: expr.trim(), value: ''})
    refreshWatches()
  }
}
function removeWatch(i: number): void {
  watches.value.splice(i, 1)
}

async function evalRepl(expr: string): Promise<void> {
  if (!expr.trim()) {
    return
  }
  pushOut('input', `› ${expr}\n`)
  try {
    const {result} = await evaluate(expr, 'repl')
    pushOut('result', `${result}\n`)
  }
  catch (e) {
    pushOut('stderr', `${e}\n`)
  }
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
  for (const path of breakpoints.keys()) {
    await client.request('setBreakpoints', bpArgs(path)).catch(() => {})
  }
}

// 某文件的 setBreakpoints 参数（带条件）
function bpArgs(path: string): Record<string, any> {
  return {
    source: {path},
    breakpoints: fileBreakpoints(path).map((line) => {
      const cond = breakpointCondition(path, line)
      return cond ? {line, condition: cond} : {line}
    })
  }
}

// 会话进行中时，断点变更后实时下发（重发当前文件断点）
async function syncBreakpoints(path: string | null | undefined): Promise<void> {
  if (!client || status.value === 'inactive' || !path) {
    return
  }
  await client.request('setBreakpoints', bpArgs(path)).catch(() => {})
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
    await refreshWatches()
  }
  catch {
    // 忽略
  }
}

// 选择调用栈帧：跳转到其源码位置（不改变执行行高亮），并按该帧重算监视
function selectFrame(id: number): void {
  selectedFrameId.value = id
  const f = frames.value.find(x => x.id === id)
  if (f?.source?.path && f.line) {
    requestReveal(f.source.path, f.line)
  }
  refreshWatches()
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

// 各语言 launch 请求参数（Python=debugpy 源码级 / Go=delve 源码级）
function buildLaunchArgs(config: LaunchConfig): Record<string, any> {
  const base: Record<string, any> = {
    request: 'launch',
    name: 'CodeForge',
    program: config.filePath,
    cwd: config.cwd || undefined
  }
  if (config.language === 'go') {
    return {...base, mode: 'debug'}
  }
  return {...base, type: 'python', console: 'internalConsole', justMyCode: true, stopOnEntry: false}
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
    c.request('launch', buildLaunchArgs(config)).catch((e) => {
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
    watches,
    fileBreakpoints,
    breakpointCondition,
    allBreakpoints,
    toggleBreakpoint,
    setBreakpointCondition,
    setStopped,
    revealLocation: requestReveal,
    syncBreakpoints,
    selectFrame,
    requestScopes,
    requestVariables,
    evaluate,
    addWatch,
    removeWatch,
    evalRepl,
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
