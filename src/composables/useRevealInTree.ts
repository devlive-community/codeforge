// 在文件树中定位当前文件，以及「切换文件自动定位」开关。
import {ref, watch, type Ref} from 'vue'
import {useI18n} from 'vue-i18n'
import {useToast} from '../plugins/toast'
import {kvGet, kvSet} from './useKvStore'

export function useRevealInTree(
  currentFilePath: Ref<string | null>,
  sidebarVisible: Ref<boolean>
) {
  const {t} = useI18n()
  const toast = useToast()

  // 传给 Sidebar 的定位请求（path + 递增序号，便于重复定位同一文件）
  const revealRequest = ref<{ path: string, n: number } | null>(null)
  let revealSeq = 0

  const revealInTree = (path?: string) => {
    const target = path ?? currentFilePath.value
    if (!target) {
      toast.info(t('app.noFileToReveal'))
      return
    }
    sidebarVisible.value = true
    revealRequest.value = {path: target, n: ++revealSeq}
  }

  // 切换文件时自动定位（仅侧栏已打开时触发，避免强开侧栏打扰）
  const autoRevealTree = ref(kvGet('auto-reveal-tree') === 'true')
  const toggleAutoReveal = () => {
    autoRevealTree.value = !autoRevealTree.value
    kvSet('auto-reveal-tree', String(autoRevealTree.value))
    toast.info(autoRevealTree.value ? t('app.autoRevealOn') : t('app.autoRevealOff'))
    if (autoRevealTree.value && sidebarVisible.value && currentFilePath.value) {
      revealRequest.value = {path: currentFilePath.value, n: ++revealSeq}
    }
  }
  watch(currentFilePath, (p) => {
    if (autoRevealTree.value && sidebarVisible.value && p) {
      revealRequest.value = {path: p, n: ++revealSeq}
    }
  })

  return {revealRequest, revealInTree, toggleAutoReveal}
}
