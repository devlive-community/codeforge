// 命名工作区：把一组根（主根 + 额外挂载根）存为命名条目，便于切换。
import {ref} from 'vue'
import {kvGetJSON, kvSetJSON} from './useKvStore'

export interface NamedWorkspace {
  name: string
  rootDir: string
  extraRoots: string[]
}

const KEY = 'named-workspaces'

export function useNamedWorkspaces() {
  const workspaces = ref<NamedWorkspace[]>(kvGetJSON<NamedWorkspace[]>(KEY, []))

  const persist = () => kvSetJSON(KEY, workspaces.value)

  // 保存/覆盖同名条目
  const save = (name: string, rootDir: string, extraRoots: string[]) => {
    const n = name.trim()
    if (!n || !rootDir) {
      return
    }
    const entry: NamedWorkspace = {name: n, rootDir, extraRoots: [...extraRoots]}
    const idx = workspaces.value.findIndex(w => w.name === n)
    if (idx >= 0) {
      workspaces.value = workspaces.value.map((w, i) => (i === idx ? entry : w))
    }
    else {
      workspaces.value = [...workspaces.value, entry]
    }
    persist()
  }

  const remove = (name: string) => {
    workspaces.value = workspaces.value.filter(w => w.name !== name)
    persist()
  }

  return {workspaces, save, remove}
}
