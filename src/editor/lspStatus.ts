import {ref} from 'vue'

// 当前编辑器语言的 LSP 状态（供状态栏显示）
// off=未启用 connecting=启动/索引中 on=就绪
export type LspStatus = 'off' | 'connecting' | 'on'
export const lspState = ref<{ language: string; status: LspStatus }>({language: '', status: 'off'})

export const setLspState = (language: string, status: LspStatus) => {
  lspState.value = {language, status}
}
