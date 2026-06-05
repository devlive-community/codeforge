import {ref} from 'vue'
import {invoke} from '@tauri-apps/api/core'

export type AppTheme = 'system' | 'light' | 'dark'

const theme = ref<AppTheme>('system')
let media: MediaQueryList | null = null
let mediaHandler: (() => void) | null = null

const apply = () => {
  let dark: boolean
  if (theme.value === 'dark') {
    dark = true
  }
  else if (theme.value === 'light') {
    dark = false
  }
  else {
    dark = window.matchMedia('(prefers-color-scheme: dark)').matches
  }
  document.documentElement.classList.toggle('dark', dark)
}

/**
 * 应用外观主题（跟随系统 / 浅色 / 深色），并在"跟随系统"时监听系统切换。
 */
export function useTheme()
{
  const setTheme = (value: AppTheme) => {
    theme.value = value
    apply()
  }

  // 从后端配置加载并应用
  const init = async () => {
    try {
      const config = await invoke<any>('get_app_config')
      const t = config?.theme
      theme.value = (t === 'light' || t === 'dark') ? t : 'system'
    }
    catch {
      theme.value = 'system'
    }
    apply()

    // 监听系统主题变化（仅"跟随系统"时生效）
    if (!media) {
      media = window.matchMedia('(prefers-color-scheme: dark)')
      mediaHandler = () => {
        if (theme.value === 'system') {
          apply()
        }
      }
      media.addEventListener('change', mediaHandler)
    }
  }

  return {theme, setTheme, init, apply}
}
