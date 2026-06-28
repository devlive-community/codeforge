// 多根工作区：主根之外额外挂载的文件夹（持久化到 KV）。
import {ref, type Ref} from 'vue'
import {open as openDialog} from '@tauri-apps/plugin-dialog'
import {kvGetJSON, kvSetJSON} from './useKvStore'

const WORKSPACE_EXTRA_KEY = 'workspace-extra-roots'

export function useWorkspaceRoots(rootDir: Ref<string | null>) {
  const extraRoots = ref<string[]>(kvGetJSON<string[]>(WORKSPACE_EXTRA_KEY, []))

  const persist = () => kvSetJSON(WORKSPACE_EXTRA_KEY, extraRoots.value)

  const addWorkspaceFolder = async () => {
    const selected = await openDialog({directory: true, multiple: false})
    if (selected && typeof selected === 'string' && selected !== rootDir.value && !extraRoots.value.includes(selected)) {
      extraRoots.value = [...extraRoots.value, selected]
      persist()
    }
  }

  const removeWorkspaceFolder = (path: string) => {
    extraRoots.value = extraRoots.value.filter(p => p !== path)
    persist()
  }

  // 打开新文件夹（新工作区）时清空额外挂载的根
  const resetExtraRoots = () => {
    extraRoots.value = []
    persist()
  }

  return {extraRoots, addWorkspaceFolder, removeWorkspaceFolder, resetExtraRoots}
}
