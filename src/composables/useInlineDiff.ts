// 编辑器行内差异标记（当前内容 vs HEAD 基线）。
import {ref, watch, type Ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {debounce} from 'lodash-es'
import {computeDiffMarkers, setDiffMarkers} from '../editor/diffGutter'

interface Deps {
  editorView: Ref<any>
  code: Ref<string>
  rootDir: Ref<string | null>
  currentFilePath: Ref<string | null>
}

export function useInlineDiff({editorView, code, rootDir, currentFilePath}: Deps) {
  // 当前文件在 HEAD 中的内容；null 表示无基线（新文件/非 git/未跟踪），不显示标记
  const gitBaseline = ref<string | null>(null)

  // 计算并派发标记到编辑器
  const applyDiffMarkers = () => {
    const view = editorView.value
    if (!view) {
      return
    }
    const markers = gitBaseline.value === null
      ? {changed: new Map(), deleted: new Set<number>()}
      : computeDiffMarkers(gitBaseline.value, code.value)
    view.dispatch({effects: setDiffMarkers.of(markers)})
  }
  const applyDiffMarkersDebounced = debounce(applyDiffMarkers, 250)

  const fetchBaseline = async () => {
    if (!rootDir.value || !currentFilePath.value || !currentFilePath.value.startsWith(rootDir.value)) {
      gitBaseline.value = null
      applyDiffMarkers()
      return
    }
    const rel = currentFilePath.value.slice(rootDir.value.length + 1)
    try {
      const head = await invoke<{ exists: boolean, content: string }>('git_file_head', {
        root: rootDir.value,
        relPath: rel
      })
      gitBaseline.value = head.exists ? head.content : null
    }
    catch {
      gitBaseline.value = null
    }
    applyDiffMarkers()
  }

  // 切换文件取新基线；编辑时重算；编辑器重挂时重新派发
  watch(currentFilePath, () => fetchBaseline())
  watch(code, () => applyDiffMarkersDebounced())
  watch(editorView, () => applyDiffMarkers())

  return {fetchBaseline}
}
