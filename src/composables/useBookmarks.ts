// 行书签：按文件记忆书签行，提供切换/上一处/下一处/清空，并在切换文件时同步到编辑器。
import {ref, watch, type Ref} from 'vue'
import {setBookmarks} from '../editor/bookmark'

export function useBookmarks(editorView: Ref<any>, currentFilePath: Ref<string | null>) {
  // 按文件路径记忆书签行号(1-based)；未命名文件用空串作 key
  const map = ref<Record<string, number[]>>({})
  const keyOf = () => currentFilePath.value ?? ''
  const current = () => map.value[keyOf()] ?? []

  // 把当前文件的书签派发到编辑器
  const apply = () => {
    editorView.value?.dispatch({effects: setBookmarks.of(current())})
  }

  const cursorLine = (view: any): number => view.state.doc.lineAt(view.state.selection.main.head).number
  const gotoLine = (view: any, ln: number) => {
    const pos = view.state.doc.line(ln).from
    view.dispatch({selection: {anchor: pos}, scrollIntoView: true})
    view.focus()
  }

  const toggleBookmark = () => {
    const view = editorView.value
    if (!view) {
      return
    }
    const ln = cursorLine(view)
    const key = keyOf()
    const arr = map.value[key] ? [...map.value[key]] : []
    const i = arr.indexOf(ln)
    if (i >= 0) {
      arr.splice(i, 1)
    }
    else {
      arr.push(ln)
    }
    map.value = {...map.value, [key]: arr}
    apply()
  }

  const nextBookmark = () => {
    const view = editorView.value
    const arr = [...current()].sort((a, b) => a - b)
    if (!view || !arr.length) {
      return
    }
    const ln = cursorLine(view)
    gotoLine(view, arr.find(l => l > ln) ?? arr[0])
  }

  const prevBookmark = () => {
    const view = editorView.value
    const arr = [...current()].sort((a, b) => a - b)
    if (!view || !arr.length) {
      return
    }
    const ln = cursorLine(view)
    gotoLine(view, [...arr].reverse().find(l => l < ln) ?? arr[arr.length - 1])
  }

  const clearBookmarks = () => {
    const key = keyOf()
    if (map.value[key]?.length) {
      map.value = {...map.value, [key]: []}
      apply()
    }
  }

  // 切换文件 / 编辑器重挂时同步当前文件的书签
  watch(currentFilePath, () => apply())
  watch(editorView, () => apply())

  return {toggleBookmark, nextBookmark, prevBookmark, clearBookmarks}
}
