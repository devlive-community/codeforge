// 显式接入 LSP 自动补全：补全源 + 输入即触发 + 键位（Ctrl+Space 触发、Enter/Tab 接受）
import {autocompletion, completionKeymap, type CompletionContext, type CompletionResult} from '@codemirror/autocomplete'
import {keymap} from '@codemirror/view'
import {Prec} from '@codemirror/state'
import {languageServerPlugin} from 'codemirror-languageserver'

const offsetToPos = (doc: any, offset: number) => {
  const line = doc.lineAt(offset)
  return {line: line.number - 1, character: offset - line.from}
}

// 向 LSP 请求补全（复用库内 plugin 的 requestCompletion）
const lspSource = async (ctx: CompletionContext): Promise<CompletionResult | null> => {
  const {state, pos, explicit, view} = ctx
  const plugin: any = view?.plugin(languageServerPlugin as any)
  if (!plugin?.requestCompletion) {
    return null
  }
  const line = state.doc.lineAt(pos)
  const before = line.text[pos - line.from - 1]
  const triggers: string[] | undefined = plugin.client?.capabilities?.completionProvider?.triggerCharacters
  let triggerKind = 1 // Invoked
  let triggerCharacter: string | undefined
  if (!explicit && triggers && before && triggers.includes(before)) {
    triggerKind = 2 // TriggerCharacter
    triggerCharacter = before
  }
  // 非显式触发且不在单词中：不打扰
  if (!explicit && triggerKind === 1 && !ctx.matchBefore(/[\w.]$/)) {
    return null
  }
  try {
    return await plugin.requestCompletion(ctx, offsetToPos(state.doc, pos), {triggerKind, triggerCharacter})
  }
  catch {
    return null
  }
}

// 高优先级，确保 LSP 补全源生效；并补上补全键位
export const lspCompletion = [
  Prec.highest(autocompletion({override: [lspSource], activateOnTyping: true, defaultKeymap: false})),
  Prec.high(keymap.of(completionKeymap))
]
