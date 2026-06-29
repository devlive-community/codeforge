// 全局快捷键：捕获阶段拦截 keydown，匹配到动作则阻止默认并派发。
import {onMounted, onUnmounted} from 'vue'

export function useGlobalShortcuts(
  matchShortcut: (e: KeyboardEvent) => string | null,
  dispatch: Record<string, () => void>,
  isOverlayOpen: () => boolean
) {
  const onGlobalKeydown = (e: KeyboardEvent) => {
    if (isOverlayOpen()) {
      return
    }
    const action = matchShortcut(e)
    if (action && dispatch[action]) {
      // 捕获阶段拦截：阻止事件到达编辑器（避免 Cmd+Enter 等被插入换行）
      e.preventDefault()
      e.stopPropagation()
      dispatch[action]()
    }
  }

  onMounted(() => window.addEventListener('keydown', onGlobalKeydown, true))
  onUnmounted(() => window.removeEventListener('keydown', onGlobalKeydown, true))
}
