// 编辑器右键菜单：开关状态、按光标定位并夹取到视口内、全局 contextmenu 监听。
// 具体菜单项的动作仍由 App 提供（Blame/历史/AI/永久链接等），这里只管菜单本身。
import {nextTick, onMounted, onUnmounted, reactive, ref, type Ref} from 'vue'
import {lspSupportsLanguage} from '../editor/lspExtension'

interface Deps {
  editorView: Ref<any>
  currentLanguage: Ref<string>
  canBlame: Ref<boolean>
}

export function useEditorContextMenu({editorView, currentLanguage, canBlame}: Deps) {
  const editorCtx = reactive({visible: false, x: 0, y: 0, lsp: false})
  const editorMenuRef = ref<HTMLElement | null>(null)

  const closeEditorCtx = () => {
    editorCtx.visible = false
  }

  const onEditorContext = async (e: MouseEvent) => {
    const target = e.target as HTMLElement | null
    const lsp = lspSupportsLanguage(currentLanguage.value) && !!editorView.value
    // 在编辑器内容区，且支持 LSP 或可 Blame 时弹出
    if (!target?.closest('.cm-content') || (!lsp && !canBlame.value)) {
      return
    }
    editorCtx.lsp = lsp
    e.preventDefault()
    const view = editorView.value
    if (view) {
      const cur = view.state.selection.main
      const pos = view.posAtCoords({x: e.clientX, y: e.clientY})
      // 仅在无选区、或右键点在选区之外时才移动光标；点在选区内则保留选区（不清除高亮）
      const insideSel = !cur.empty && pos != null && pos >= cur.from && pos <= cur.to
      if (pos != null && !insideSel) {
        view.dispatch({selection: {anchor: pos}})
      }
    }
    // 先按光标位置弹出，渲染后测量真实尺寸再夹取到视口内（菜单项数量可变，避免贴底/贴右裁切）
    editorCtx.x = e.clientX
    editorCtx.y = e.clientY
    editorCtx.visible = true
    await nextTick()
    const el = editorMenuRef.value
    if (el) {
      const r = el.getBoundingClientRect()
      const margin = 8
      if (editorCtx.x + r.width > window.innerWidth) {
        editorCtx.x = Math.max(margin, window.innerWidth - r.width - margin)
      }
      if (editorCtx.y + r.height > window.innerHeight) {
        editorCtx.y = Math.max(margin, window.innerHeight - r.height - margin)
      }
    }
  }

  onMounted(() => window.addEventListener('contextmenu', onEditorContext))
  onUnmounted(() => window.removeEventListener('contextmenu', onEditorContext))

  return {editorCtx, editorMenuRef, closeEditorCtx}
}
