import {ref} from 'vue'

// 当前编辑器语言的 LSP 状态（供状态栏显示）
export type LspStatus = 'off' | 'on'
export const lspState = ref<{ language: string; status: LspStatus }>({language: '', status: 'off'})

export const setLspState = (language: string, status: LspStatus) => {
  lspState.value = {language, status}
}
