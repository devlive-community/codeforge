// 标签会话持久化：记住上次打开的文件标签（含置顶与激活项），启动时恢复。
import {watch, type Ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {kvGetJSON, kvSetJSON} from './useKvStore'
import type {WorkspaceTab} from './useWorkspace'

const SESSION_TABS_KEY = 'session-tabs'

interface SessionDeps {
  editorTabs: Ref<WorkspaceTab[]>
  activeTabId: Ref<string>
  editorConfig: Ref<{ max_open_file_size?: number } | null | undefined>
  openPath: (path: string) => Promise<void> | void
  restorePinned: (paths: string[]) => void
  switchTab: (id: string) => void
}

export function useSessionTabs(deps: SessionDeps) {
  const {editorTabs, activeTabId, editorConfig, openPath, restorePinned, switchTab} = deps

  const persistSession = () => {
    const paths = editorTabs.value.map(t => t.filePath).filter((p): p is string => !!p)
    const pinned = editorTabs.value.filter(t => t.pinned && t.filePath).map(t => t.filePath as string)
    const activePath = editorTabs.value.find(t => t.id === activeTabId.value)?.filePath || null
    kvSetJSON(SESSION_TABS_KEY, {paths, pinned, activePath})
  }

  // 标签集合/文件/激活项/置顶变化时持久化（不含正文编辑，避免频繁写入）
  watch(
    () => editorTabs.value.map(t => (t.pinned ? '*' : '') + (t.filePath || '')).join('|') + '#' + activeTabId.value,
    () => persistSession()
  )

  // 启动时恢复上次打开的文件标签（仅已保存且可读的文本文件）
  const restoreSession = async () => {
    const saved = kvGetJSON<{ paths: string[], pinned?: string[], activePath: string | null } | null>(SESSION_TABS_KEY, null)
    if (!saved || !saved.paths?.length) {
      return
    }

    const limitBytes = (editorConfig.value?.max_open_file_size ?? 5) * 1024 * 1024
    for (const p of saved.paths) {
      try {
        const meta = await invoke<{ size_bytes: number, is_text: boolean }>('get_text_file_meta', {path: p})
        if (meta.is_text && meta.size_bytes <= limitBytes) {
          await openPath(p)
        }
      }
      catch {
        // 跳过已删除/无法读取的文件
      }
    }

    if (saved.pinned?.length) {
      restorePinned(saved.pinned)
    }

    if (saved.activePath) {
      const t = editorTabs.value.find(tab => tab.filePath === saved.activePath)
      if (t) {
        switchTab(t.id)
      }
    }
  }

  return {restoreSession}
}
