// 国际化（vue-i18n）。
// 内置语言包以 JSON 文件初始化（locales/*.json），用户自定义语言包存于本地数据库(KV)，
// 启动时合并；可在「设置 → 语言包」中新增/编辑/删除，方便扩展。
import {ref} from 'vue'
import {createI18n} from 'vue-i18n'
import {kvGet, kvSet, kvGetJSON, kvSetJSON} from '../composables/useKvStore'
import zhCN from './locales/zh-CN.json'
import en from './locales/en.json'

export interface LocaleDef {
  name: string
  messages: Record<string, any>
}

// 内置语言包（初始化数据源）
const BUILTIN: Record<string, LocaleDef> = {
  'zh-CN': {name: '简体中文', messages: zhCN},
  en: {name: 'English', messages: en}
}

const LOCALE_KEY = 'app-locale'
const CUSTOM_KEY = 'i18n-custom-locales' // DB(KV)：{ [code]: { name, messages } }

export const i18n = createI18n({
  legacy: false,
  locale: 'zh-CN',
  fallbackLocale: 'zh-CN',
  messages: {'zh-CN': zhCN, en} as any
})

// 可用语言列表（内置 + 自定义），响应式，供界面语言下拉与管理页使用
export const availableLocales = ref<{ value: string; label: string; builtin: boolean }[]>([])

const readCustom = (): Record<string, LocaleDef> => kvGetJSON<Record<string, LocaleDef>>(CUSTOM_KEY, {})
const writeCustom = (data: Record<string, LocaleDef>) => kvSetJSON(CUSTOM_KEY, data)

const rebuildAvailable = (custom: Record<string, LocaleDef>) => {
  const list: { value: string; label: string; builtin: boolean }[] = []
  for (const [code, def] of Object.entries(BUILTIN)) {
    list.push({value: code, label: custom[code]?.name || def.name, builtin: true})
  }
  for (const [code, def] of Object.entries(custom)) {
    if (!BUILTIN[code]) {
      list.push({value: code, label: def.name || code, builtin: false})
    }
  }
  availableLocales.value = list
}

// 启动时调用（须在 loadKvStore 之后）：合并 DB 自定义语言包并恢复上次语言
export const loadLocales = () => {
  const custom = readCustom()
  for (const [code, def] of Object.entries(custom)) {
    i18n.global.setLocaleMessage(code, def.messages as any)
  }
  rebuildAvailable(custom)
  const saved = kvGet(LOCALE_KEY)
  if (saved && availableLocales.value.some(l => l.value === saved)) {
    i18n.global.locale.value = saved as any
  }
}

export const setLocale = (locale: string) => {
  i18n.global.locale.value = locale as any
  kvSet(LOCALE_KEY, locale)
}
export const getLocale = (): string => i18n.global.locale.value as string

// —— 语言包管理（供「设置 → 语言包」）——

export const isBuiltinLocale = (code: string) => !!BUILTIN[code]

// 取某语言当前完整文案（自定义优先，否则内置）；用于编辑器预填
export const getLocaleMessages = (code: string): Record<string, any> => {
  const custom = readCustom()
  if (custom[code]) {
    return custom[code].messages
  }
  if (BUILTIN[code]) {
    return BUILTIN[code].messages
  }
  return {}
}

// 取内置默认文案（用于以某内置语言为模板新建）
export const getBuiltinMessages = (code: string): Record<string, any> => BUILTIN[code]?.messages ?? BUILTIN['zh-CN'].messages

// 新增/编辑：写入 DB、应用到 i18n、刷新列表
export const saveLocale = (code: string, name: string, messages: Record<string, any>) => {
  const custom = readCustom()
  custom[code] = {name, messages}
  writeCustom(custom)
  i18n.global.setLocaleMessage(code, messages as any)
  rebuildAvailable(custom)
}

// 删除自定义语言包（内置不可删）；删的是当前语言则切回 zh-CN
export const deleteLocale = (code: string) => {
  if (BUILTIN[code]) {
    return
  }
  const custom = readCustom()
  delete custom[code]
  writeCustom(custom)
  rebuildAvailable(custom)
  if (getLocale() === code) {
    setLocale('zh-CN')
  }
}

// 恢复内置默认（移除对内置语言的自定义覆盖）
export const resetBuiltin = (code: string) => {
  if (!BUILTIN[code]) {
    return
  }
  const custom = readCustom()
  if (custom[code]) {
    delete custom[code]
    writeCustom(custom)
  }
  i18n.global.setLocaleMessage(code, BUILTIN[code].messages as any)
  rebuildAvailable(custom)
}

// 模块初始化时先用内置填充列表（KV 尚未加载时的兜底；loadLocales 后会再刷新）
rebuildAvailable({})
