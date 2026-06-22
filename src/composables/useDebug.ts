// 调试状态共享 store（单例）。P2 仅承载断点与当前执行位置；
// 会话/调用栈/变量等在 P3+ 扩展。
import {reactive, ref} from 'vue'

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

export function useDebug() {
  return {breakpoints, bpVersion, stopped, fileBreakpoints, toggleBreakpoint, setStopped}
}
