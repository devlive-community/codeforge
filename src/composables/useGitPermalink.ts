// 远程仓库永久链接：复制 / 在浏览器打开当前文件指定行的链接。
import type {Ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {open as openExternalUrl} from '@tauri-apps/plugin-shell'
import {useI18n} from 'vue-i18n'
import {useToast} from '../plugins/toast'

interface CursorInfo {
  line: number
}

export function useGitPermalink(
  rootDir: Ref<string | null>,
  currentFilePath: Ref<string | null>,
  cursorInfo: Ref<CursorInfo>
) {
  const {t} = useI18n()
  const toast = useToast()

  // 生成当前文件指定行的永久链接（含路径/仓库校验）
  const buildPermalink = async (): Promise<string | null> => {
    if (!rootDir.value || !currentFilePath.value) {
      toast.info(t('app.noFileToReveal'))
      return null
    }
    const root = rootDir.value
    const p = currentFilePath.value
    if (!(p === root || p.startsWith(root + '/') || p.startsWith(root + '\\'))) {
      toast.info(t('app.permalinkOutside'))
      return null
    }
    const rel = p.slice(root.length).replace(/^[\\/]/, '')
    try {
      return await invoke<string>('git_permalink', {root, relPath: rel, line: cursorInfo.value.line})
    }
    catch (error) {
      toast.error(t('app.permalinkFailed') + ': ' + error)
      return null
    }
  }

  const copyPermalink = async () => {
    const url = await buildPermalink()
    if (url) {
      await navigator.clipboard.writeText(url)
      toast.success(t('app.permalinkCopied'))
    }
  }

  const openPermalink = async () => {
    const url = await buildPermalink()
    if (url) {
      await openExternalUrl(url).catch((e) => toast.error(t('app.permalinkFailed') + ': ' + e))
    }
  }

  return {copyPermalink, openPermalink}
}
